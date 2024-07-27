    // IMPLEMENTED:
    // Complete ranking procedure using document clustering with K-means and Word2Vec embeddings.
    
    // TO DO:
    // (1) TF-IDF rankings as alternative & possibly more efficient embedding procedure.
    // (2) Implement PageRank as an alternative ranking procedure to document clustering. Provide user
    // with option to select ranking procedure in configs.

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
    use redis::Commands;
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

    // IMPLEMENTED: 
    // Obtain vector of terms for both document-term index and inverted index.
    // Term collecting not currently supported for the inverted index - that is - Word2Vec
    // preferentially used for document-term and more suitable embedding (TF-IDF?) for the inverted index

    fn collect_terms (index: Indexer) -> Option<HashMap<Document, Vec<String>>> {
        match index {
            Indexer::TermIndex (map) => { Some(map) }
            Indexer::InvertedIndex (_map) => { None }
        }
    }

    // IMPLEMENTED:
    // Compute component-wise average vector, useful to obtain global representation of document.
    // TO DO:
    // (1) Optimise the following with a fold operation - clearly follows this pattern.
    // (2) Error handling - ensure valid input (non-empty), and look for other possible errors to
    // place in Result type output.

    fn get_average_vector (features: Vec<Vec<f32>>) -> Vec<f32> {
        let acc: Array1<f32> = features.iter().fold(Array1::zeros(300), |acc ,f| {
            Array1::from_vec(f.to_vec()) + Array1::from_vec(acc.to_vec())
        });

        return acc.to_vec().par_iter().map(|x| x / features.len() as f32).collect();
    }

    // IMPLEMENTED:
    // Make embeddings using Word2Vec Python script. Opted to use JSON serialisation /
    // deserialisation pipeline for Python-Rust communication.

    fn make_embeddings (terms: Vec<String>) -> Result<Vec<Vec<f32>>, String> {
        let mut embedding_script = Command::new("python3");
        embedding_script.arg("scripts/embedding.py");

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

        match value {
            Value::Array(embeddings) => Ok(embeddings.into_iter().map(|embedding| embedding.as_array().unwrap_or(&Vec::new()).into_iter()
                .map(|num| num.as_f64().unwrap() as f32).collect()).collect()),
            Value::Number(error) if error.is_i64() => {
                match error.as_i64().unwrap() {
                    1 => {
                        return Err(String::from("1"));
                    }
                    _ => {
                        return Err(String::from("-2"));
                    }
                }
            } 
            _ => Err(String::from("-2"))
        }
    }

    // IMPLEMENTED: 
    // Obtain feature vector for each document by pipelining text content into Word2Vec.
    // WARNING: Temporarily public for testing.

    pub fn embed_documents(document_terms: HashMap<Document, Vec<String>>, num_terms: u32) -> Result<Vec<EmbeddedDocument>, String> {
        let mut global_embeddings: Vec<EmbeddedDocument> = Vec::new();

        document_terms.iter().for_each(|(document, terms)| {
            if terms.len() < 3 {
                return; 
            }
            
            let local_embeddings: Vec<Vec<f32>>;
            
            // TO DO: make use of PCA here also for dimensionality reduction. Currently
            // just clipping the values.
            // Should be in the form: terms[0..num_terms as usize] -> reduce(terms, size)
            match make_embeddings(terms[0..num_terms as usize].to_vec()) {
                Ok(embeddings) => local_embeddings = embeddings,
                Err(_) => {
                    return
                }
            }

            let doc = EmbeddedDocument { 
                document: document.clone(),
                embedding: get_average_vector(local_embeddings)
            };
            
            global_embeddings.push(doc);
            
        });

        if global_embeddings.is_empty() {
            return Err(String::from("-1"));
        }

        Ok(global_embeddings)
    }


    // IMPLEMENTED: 
    // Clusters similar documents together using k-means. Need to store the clusters, mean centroid
    // value and the distance for each document to their respective centroid. Need to specify a
    // different type for this.

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

    // IMPLEMENTED: 
    // Minkowski distance metric - uses Pca model to reduce query vector where necessary to size of the centroid.
    // TO DO:
    // Error handling for invalid input and other possible errors wrapped in Result type output.

    fn distance (a: Vec<f32>, b: Vec<f32>) -> f32 {
        let p = a.len() as f32;
        a.par_iter()
            .zip(b.par_iter())
            .map(|(x, y)| (x - y).abs().powf(p))
            .sum::<f32>()
            .powf(1.0 / p)
    }
    
    // IMPLEMENTED:
    // Principal component analysis utility used to reduce the size of the query vector to match
    // cluster embeddings size.
    // TO DO:
    // (1) Proper error handling
    // (2) Modifying for more general use for reducing elsewhere (we have opted in other cases to
    // simply cap off some of the higher dimensional data points - which will reduce the accuracy
    // of the ranking).

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
        
    
    pub fn get_ranked_documents (query: String, index: Indexer) -> Result<Vec<Document>, String> {
        let document_terms;
        
        // Unwrap the Indexer type
        match collect_terms (index) {
            Some (map) => {
                document_terms = map;
            },
            None => {
                // Code 3: indexing error.
                return Err(String::from("3"))
            }
        }

        println!("Terms collected.");

        let embeddings = embed_documents(document_terms, 2)?;

        println!("Embedding for documents complete.");
    
        let clusters: Vec<Cluster>;


        match generate_clusters(embeddings.clone()) {
            Ok(clusters_out) => clusters = clusters_out,
            Err(_) => return Err(String::from("4"))
        }

        println!("Generated clusters.");

        // Embed the query for similarity comparison with cluster centroids.
        // Split the query into multiple terms - then normalise these outputs and average them.
        let query_value: Value = serde_json::from_str(&query).map_err(|_| String::from("2"))?;

        let extracted_query;
        match query_value.get("query") {
            Some(q) => extracted_query = q,
            None => return Err(String::from("2"))
        }

        let parsed_query = extracted_query.to_string().replace("\"", "").trim().split_whitespace().map(str::to_string).collect();
        
        let query_embeddings = make_embeddings(parsed_query)?;

        let mut query_embedding = get_average_vector(query_embeddings);

        let centroid_len = clusters[0].centroid.len();
        let query_len = query_embedding.len();
       
        if query_len < centroid_len {
            query_embedding.resize(centroid_len, 0.0);
        }

        else if query_len > centroid_len {
            query_embedding = reduce_query(query_embedding, embeddings, centroid_len);
        } 
        
        // WARNING: error may occur here now we have removed unwrap() call.
        let min = clusters.iter().min_by_key(move |cluster| {
            distance(query_embedding.clone(), cluster.centroid.clone())
            .partial_cmp(&distance(query_embedding.clone(), cluster.centroid.clone()))
        });
        
        let mut ranked_docs: Vec<Document> = vec![];

        for doc in &min.unwrap().documents {
            ranked_docs.push(doc.document.clone());
        }

        println!("Ranked documents: {:?}", ranked_docs.par_iter().map(|d| d.title.to_string()).collect::<Vec<String>>());

        Ok(ranked_docs)
    }
    

