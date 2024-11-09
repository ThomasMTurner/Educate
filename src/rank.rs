
    use std::collections::HashMap;
    use std::fmt;
    use ndarray::Array2;
    use rayon::prelude::*;
    use linfa_reduction::Pca;
    use linfa::Dataset;
    use linfa::traits::{Fit, Predict};
    use crate::parser::Document;
    use crate::index::{Indexer, InvertedInfo, read_index_file};
    use std::process::Command;
    use serde_json::Value;
    use serde::{Serialize, Deserialize};
    use ndarray::Array1;
    extern crate redis;
    // use redis::Commands;
    // use serde_json::json;
    // IMPLEMENTED:
    // EmbeddedDocument - intermediate placeholder for documents & their averaged embedding.
    // Cluster - intermediate map for documents & corresponding centroid.

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EmbeddedDocument {
        pub document: Document,
        pub embedding: Vec<f32>,

    }

    impl fmt::Display for EmbeddedDocument {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}: ({})", self.document, format!("{:?}", self.embedding))
        }
    }

    impl EmbeddedDocument {
        fn new(document: Document, embedding: Vec<f32>) -> Self {
            EmbeddedDocument { document, embedding }
        }
    }
    
    #[derive(Debug, Clone)]
    pub struct Cluster {
        pub centroid: Vec<f32>,
        pub documents: Vec<EmbeddedDocument>,
    }

    impl fmt::Display for Cluster {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}: [{}]", format!("{:?}", self.centroid), format!("{:?}", self.documents))
        }
    }

    fn collect_terms (index: Indexer) -> Option<HashMap<Document, Vec<String>>> {
        match index {
            Indexer::TermIndex (map) => { Some(map) }
            Indexer::InvertedIndex (_map) => { None }
        }
    }

    fn get_average_vector (features: Vec<Vec<f32>>) -> Vec<f32> {
        let acc: Array1<f32> = features.iter().fold(Array1::zeros(300), |acc ,f| {
            Array1::from_vec(f.to_vec()) + Array1::from_vec(acc.to_vec())
        });

        return acc.to_vec().par_iter().map(|x| x / features.len() as f32).collect();
    }
    
    
    fn make_embeddings (terms: Vec<String>, script: &str) -> Result<Vec<Vec<f32>>, String> {
        let mut embedding_script = Command::new("python3");
        embedding_script.arg(script);
        println!("Obtained script to embed with: {:?}", embedding_script);

        println!("Using terms: {:?}", terms);

        for term in &terms {
            embedding_script.arg(term);
        }
        
        let output;

        match embedding_script.output() {
            Ok(out) => output = out,
            Err(e) => {
                eprintln!("Embedding error: {:?}", e);
                return Err(String::from("4"))
            }
        }

        let json;

        match std::str::from_utf8(&output.stdout) {
            Ok(out) => json = out,
            Err(e) => {
                eprintln!("Embedding error {:?}: UTF-8 conversion error", e);
                return Err(String::from("4"))
            }
        }

        let value: Value;

        match serde_json::from_str(json) {
            Ok(val) => value = val,
            Err(e) => {
                eprintln!("Embedding error, JSON parsing error: {:?}", e);
                return Err(String::from("4"))
            }
        }

        println!("Obtained embeddings: {:?}", value);

        match value {
            Value::Array(embeddings) => {
                let embeds_with_f32: Vec<Vec<f32>> = embeddings
                    .into_iter()
                    .map(|embedding| {
                        embedding.as_array()
                        .unwrap_or(&Vec::new()) 
                        .iter()
                        .map(|num| num.as_f64().unwrap_or(0.0) as f32) 
                        .collect() 
                    })
                .collect(); 
                Ok(embeds_with_f32)
            }
            Value::Number(error) if error.is_i64() => {
                match error.as_i64().unwrap() {
                    1 => {
                        println!("Embedding error: {}", error);
                        return Err(String::from("1"));
                    }
                    _ => {
                        println!("Embedding error: {}", error);
                        return Err(String::from("-2"));
                    }
                }
            } 
            _ => {
                println!("Embedding error (out of match)");
                Err(String::from("-2"))
            }
        }
    }

    pub fn embed_documents(document_terms: HashMap<Document, Vec<String>>, num_terms: u32, script: &str) -> Result<Vec<EmbeddedDocument>, String> {
        let mut global_embeddings: Vec<EmbeddedDocument> = Vec::new();

        document_terms.iter().for_each(|(document, terms)| {
            if terms.len() < 3 {
                return; 
            }
            
            let local_embeddings: Vec<Vec<f32>>;
            let result_vector: Vec<f32>;
           
            let mut title: Vec<String> = document.title.split_whitespace().map(String::from).collect();
            let mut terms = terms[0..num_terms as usize].to_vec();
            terms.append(&mut title);
            match make_embeddings(terms, script) {
                Ok(embeddings) => local_embeddings = embeddings,
                Err(e) => {
                    println!("Embedding error: {}", e);
                    return
                }
            }

            match script {
                "scripts/embedding.py" => result_vector = get_average_vector(local_embeddings),
                _ => result_vector = local_embeddings[0].clone() 
            }

            global_embeddings.push(EmbeddedDocument::new(document.clone(), result_vector));
        });

        if global_embeddings.is_empty() {
            return Err(String::from("-1"));
        }

        Ok(global_embeddings)
    }


    pub fn generate_clusters (embeddings: Vec<EmbeddedDocument>) -> Result<Vec<Cluster>, String> {
        let samples = embeddings.par_iter()
            .map(|doc| doc.embedding.clone())
            .collect::<Vec<Vec<f32>>>();
        
        let script = "./scripts/cluster.py";

        let clusters_output;

        match Command::new("python3").arg(script).arg(serde_json::to_string(&samples).unwrap_or_default()).output() {
            Ok(out) => clusters_output = out,
            Err(_) => return Err(String::from("5"))
        }

        let clusters_json;

        match std::str::from_utf8(&clusters_output.stdout) {
            Ok(out) => clusters_json = out,
            Err(_) => return Err(String::from("5"))
        }

        let clusters_json_value: Value;

        match serde_json::from_str(clusters_json) {
            Ok(value) => clusters_json_value = value,
            Err(_) => {
                return Err(String::from("5"));
            }
        }

        let default = Vec::new();

        let clusters = clusters_json_value.as_array()
            // WARNING: unsure how use of empty vector will be handled.
            .unwrap_or(&Vec::new())
            .par_iter()
            .map(|cluster| {
                let cluster_arr = cluster.as_array().unwrap_or(&default);
                let cluster_embeddings: Vec<Vec<f32>> = serde_json::from_value(cluster_arr[1].clone()).unwrap_or(Vec::new());
                let centroid_out: Vec<f32> = serde_json::from_value(cluster_arr[0].clone()).unwrap_or(Vec::new());
                let mut documents_for_cluster = Vec::new();
                for embedded_document in &embeddings {
                    for embedding in &cluster_embeddings {
                        if *embedding == embedded_document.embedding {
                            documents_for_cluster.push(embedded_document.clone());
                        }
                    }
                }
                
                
                Cluster {centroid: centroid_out, documents: documents_for_cluster}
            })
            .collect::<Vec<Cluster>>();
        
        Ok(clusters)
    }
    
    // Minkowski distance used preferentially for 
    // document clustering methods.
    fn mink_distance (a: Vec<f32>, b: Vec<f32>) -> f32 {
        let p = a.len() as f32;
        a.par_iter()
            .zip(b.par_iter())
            .map(|(x, y)| (x - y).abs().powf(p))
            .sum::<f32>()
            .powf(1.0 / p)
    }
    
    // Cosine similarity which will be used 
    // for TF-IDF weighting based ranking.
    fn _cos_distance (a: Vec<f32>, b: Vec<f32>) -> f32 {
        // Compute dot product
        let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();

        // Compute magnitudes.
        let mag1: f32 = a.iter().map(|x| (x * x).sqrt()).sum();
        let mag2: f32 = b.iter().map(|x| (x * x).sqrt()).sum();

        // Return distance.
        return dot / (mag1 * mag2);
    }

    fn reduce_query (query: Vec<f32>, embeddings: Vec<EmbeddedDocument>, centroid_len: usize) -> Vec<f32> {
        let embeddings: Vec<Vec<f32>> = embeddings.par_iter().map(|doc| doc.embedding.clone()).collect();

        let embedding_dset = Array2::from_shape_vec(
            (embeddings.len(), embeddings[0].len()),
            embeddings.into_iter().flatten().map(|f| f as f64).collect(),
        ).unwrap();

        let pca = Pca::params(centroid_len).fit(&Dataset::from(embedding_dset)).unwrap();

        let query_embedding: Array2<f64> = Array2::from_shape_vec(
            (1, query.len()),
            query.into_iter().map(|f| f as f64).collect(),
        ).unwrap();

        pca.predict(&query_embedding).iter().map(|&x| x as f32).collect()
    }
    
    // TO DO:
    // Prevent use of PCA where cosine similarity is used.
    // ENSURE cosine similarity is not implemented as cosine distance for the above.
    pub fn get_clustered_rankings (query: String, index: Indexer, script: &str) -> Result<Vec<Document>, String> { 
        let document_terms;
        match collect_terms (index) {
            Some (map) => {
                document_terms = map;
            },
            None => {
                return Err(String::from("3"))
            }
        }
        
        // TO DO: limit term count for Word2Vec versus Sentence Transformers.
        // Currently we are testing term limit for sentence transformers.
        let num_terms;
        match script {
            "scripts/embedding.py" => num_terms = 5,
            "scripts/sentence_transform.py" => num_terms = 50,
            // TO DO: check if this is a correct error message.
            _ => return Err(String::from("4"))
        }
        
        println!("Script: {}", script);
        println!("Terms: {}", num_terms);
        println!("Making embeddings");

        let embeddings = embed_documents(document_terms, num_terms, script)?;

        let clusters: Vec<Cluster>;

        match generate_clusters(embeddings.clone()) {
            Ok(clusters_out) => clusters = clusters_out,
            Err(_) => return Err(String::from("4"))
        }

        let parsed_query = query.to_string().replace("\"", "").trim().split_whitespace().map(str::to_string).collect();
        
        let query_embeddings = make_embeddings(parsed_query, script)?;
        
        let mut query_embedding;
        
        match script {
            "scripts/embedding.py" => query_embedding = get_average_vector(query_embeddings),
            _ => query_embedding = query_embeddings[0].clone() 
        }

        //let mut query_embedding = get_average_vector(query_embeddings);

        let centroid_len = clusters[0].centroid.len();
        let query_len = query_embedding.len();
       
        if query_len < centroid_len {
            query_embedding.resize(centroid_len, 0.0);
        }

        else if query_len > centroid_len {
            query_embedding = reduce_query(query_embedding, embeddings, centroid_len);
        } 
        
        // WARNING: error may occur here now we have removed unwrap() call.
        // Could replace with cos_distance for testing.
        let min = clusters.iter().min_by_key(move |cluster| {
            mink_distance(query_embedding.clone(), cluster.centroid.clone())
            .partial_cmp(&mink_distance(query_embedding.clone(), cluster.centroid.clone()))
        });
        
        let mut ranked_docs: Vec<Document> = vec![];

        for doc in &min.unwrap().documents {
            ranked_docs.push(doc.document.clone());
        }

        Ok(ranked_docs)
    }
     

    struct BM25 {
        k1: f64,
        b: f64,
        avg_doc_len: f64,
        doc_terms: HashMap<Document, Vec<String>>,
        inverted: HashMap<String, Vec<InvertedInfo>>,
        doc_count: usize
    }

    impl BM25 {
        fn new(k1: f64, b: f64, document_terms: HashMap<Document, Vec<String>>, inverted: HashMap<String, Vec<InvertedInfo>>) -> Self {
            // We can compute doc_count using the document_terms map.
            // We can compute doc lengths on demand with document_terms map.
            // We can compute average document length using document_terms map.
            let mut total: f64 = 0.0;
            let doc_count = document_terms.len();

            // Compute avg_doc_len.
            for (_, terms) in &document_terms {
                // Compute doc length.
                let doc_len = terms.len() as f64;
                total += doc_len;
            }
            
            let avg_doc_len = total / doc_count as f64;
            BM25 { k1, b, avg_doc_len, doc_terms: document_terms, inverted, doc_count }
        }
        
      
        // Implement idf ranking used within bm25 score.
        fn idf(&self, term: String) -> f64 {
            // Empty created for longer lifetime.
            let empty: Vec<InvertedInfo> = vec![];
            let docs_containing_term = self.inverted.get(&term).unwrap_or(&empty);
            let num_docs_containing_term = docs_containing_term.len();

            let idf = if num_docs_containing_term > 0 {
                ((self.doc_count - num_docs_containing_term) as f64 / (num_docs_containing_term as f64) + 0.5).log(10.0)
            } else {
                0.0 // or some other value to handle the case when no documents contain the term
            };
            
            return idf
        }
        
        // Implement bm25 score for individual document & query.
        fn score(&self, query: &str, document: Document) -> f64 {
            let mut bm25_score = 0.0;
            // Assuming query form is delimited by 
            for term in query.split_whitespace() {
                let term = term.trim();
                let idf = self.idf(term.to_string());
                let mut tf = 0.0;
                if let Some(containers) = self.inverted.get(term) {
                    if let Some(container) = containers.iter().find(|container| container.document == document) {
                        tf = container.term_freq as f64;
                    }
                }

                // Taking document length as term count.
                let doc_length = self.doc_terms.get(&document).unwrap().len() as f64;
                let rhs = idf * (tf * (self.k1 + 1.0)) / (tf + self.k1 * (1.0 - self.b + self.b * (doc_length / self.avg_doc_len)));
                bm25_score += rhs;
            }
            bm25_score
        }
        
        // Apply score() and sort entries by the score.
        pub fn rank_documents(&self, query: String) -> Result<Vec<Document>, String> {
            // Obtain all documents (within the document term index).
            let mut documents: Vec<Document> = self.doc_terms.keys().cloned().collect();

            // Sort documents by bm25 scoring.
            documents.sort_by(|a, b| {
                let score_a = self.score(&query, a.clone());
                let score_b = self.score(&query, b.clone());
                score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
            });

            Ok(documents)
        }
    }


    pub fn get_bm25_rankings (query: String) -> Result<Vec<Document>, String> {
        let document_terms;
        let inverted;

        // Need to read out both indices.
        match read_index_file("./indices/dterm.json") {
            Ok(Indexer::TermIndex(map)) => document_terms = map,
            Ok(Indexer::InvertedIndex(_)) => return Err(String::from("2")),
            Err(_) => return Err(String::from("2"))
        }

        match read_index_file("./indices/inverted.json") {
            Ok(Indexer::InvertedIndex(map)) => inverted = map,
            Ok(Indexer::TermIndex(_)) => return Err(String::from("2")),
            Err(_) => return Err(String::from("2"))
        }

        let bm25 = BM25::new(1.5, 0.75, document_terms, inverted);

        match BM25::rank_documents(&bm25, query) {
            Ok(documents) => Ok(documents),
            Err(e) => return Err(e)
        }
    }
   
    pub fn get_ranked_documents (query: String, index: Indexer, script: &str) -> Result<Vec<Document>, String> {
        if script.is_empty() {
            println!("Using bm25 ranking");
            let bm = get_bm25_rankings(query);
            println!("bm25 result: {:?}", bm);
            return bm
        }  

        else {
            return get_clustered_rankings(query, index, script);
        }
    }
    

