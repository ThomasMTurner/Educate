
    use std::collections::HashMap;
    use std::fmt;
    use ndarray::Array2;
    use rayon::prelude::*;
    use linfa_reduction::Pca;
    use linfa::Dataset;
    use linfa::traits::{Fit, Predict};
    use crate::parser::Document;
    use crate::index::Indexer;
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
        println!("Obtained terms to embed: {:?}", terms);

        for term in &terms {
            embedding_script.arg(term);
        }
        
        let output;

        match embedding_script.output() {
            Ok(out) => output = out,
            Err(_) => return Err(String::from("4"))
        }

        let json;

        match std::str::from_utf8(&output.stdout) {
            Ok(out) => json = out,
            Err(_) => return Err(String::from("4"))
        }

        let value: Value;

        match serde_json::from_str(json) {
            Ok(val) => value = val,
            Err(_) => return Err(String::from("4"))
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
                println!("Embeddings after second processing: {:?}", embeds_with_f32);
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
            println!("Embedding with following terms: {:?}", terms);
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
        
    
    pub fn get_ranked_documents (query: String, index: Indexer, script: &str) -> Result<Vec<Document>, String> {
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
    

