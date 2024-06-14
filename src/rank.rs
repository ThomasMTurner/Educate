    // TO DO:
    // Ranking procedures which can be served with Rocket API:
    // 1. Document clustering using K-means (can include other clustering methods later) - assign a
    //    representative for each cluster based on distance to centroid / rank based on distance to
    //    centroid.
    // 2. (Extra) PageRank to order clusters based on link strength.

    use std::collections::HashMap;
    use std::fmt;
    use ndarray::Array2;
    use rayon::prelude::*;
    use linfa_reduction::Pca;
    use linfa::Dataset;
    use linfa::traits::{Fit, Predict};
    use crate::parser::Document;
    use crate::index::Indexer;
    // Necessary imports for Python-Rust IPC
    use std::process::Command;
    use serde_json::Value;
    use serde_json::json;
    //use serde_json::error::Category;
    use serde::{Serialize, Deserialize};
    use ndarray::Array1;

    //METHOD:
    //  1. Users can choose between different embedding methods, specifically (1) TF-IDF (2) Word2Vec
    //  2. Word2Vec method:
    //  2a. Collect all terms for that document (specific methods for handling the document-term
    //  index and inverted index)
    //  2b. Pass this vector into Word2Vec to produce a feature vector - loop until all documents
    //  are embedded.
    //  2c. Produce a map from each document ID -> feature vector.
    //  2d. Use this list of embeddings in the k-means clustering.
    //  2e. Upon search query - collect k search terms from the nearest centroid to the query - may
    //  need to use dimensionality reduction since search query generally contains many less terms
    //  than the document feature vectors.
    

    // Define a simple structure to store our embedded documents.
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

    // Obtain vector of terms for both document-term index and inverted index.
    // Term collecting not currently supported for the inverted index - that is - Word2Vec
    // preferentially used for document-term and more suitable embedding (TF-IDF?) for the inverted.

    fn collect_terms (index: Indexer) -> Option<HashMap<Document, Vec<String>>> {
        match index {
            Indexer::TermIndex (map) => { Some(map) }
            Indexer::InvertedIndex (_map) => { None }
        }
    }

    fn get_average_vector (features: Vec<Vec<f32>>) -> Vec<f32> {
        // LATER: optimise with a fold operation.
        let mut acc: Array1<f32> = Array1::zeros(300);
        for f in &features {
            let arr = Array1::from_vec(f.to_vec());
            acc = &acc + &arr;
        }

        let casted_acc: Vec<f32> = acc.to_vec();
        return casted_acc.par_iter().map(|x| x / features.len() as f32).collect();
    }

    fn make_embeddings (terms: Vec<String>) -> Result<Vec<Vec<f32>>, String> {
        let mut embedding_script = Command::new("python3");
        embedding_script.arg("scripts/embedding.py");

        for term in &terms {
            embedding_script.arg(term);
        }
        
        let output = embedding_script.output().expect("Failed to make embeddings");

        let embedding_json = std::str::from_utf8(&output.stdout).expect("Embedding output not UTF-8");

        let value: Value = serde_json::from_str(embedding_json)
            .map_err(|e| format!("Failed to parse JSON {} due to: {}", embedding_json, e))?;

        match value {
            Value::Array(embeddings) => Ok(embeddings.into_iter().map(|embedding| embedding.as_array().unwrap().into_iter()
                .map(|num| num.as_f64().unwrap() as f32).collect()).collect()),
            Value::Number(error) if error.is_i64() => {
                match error.as_i64().unwrap() {
                    1 => {
                        return Err(1.to_string());
                    }
                    _ => {
                        return Err((-1).to_string());
                    }
                }
            } 
            _ => Err((-1).to_string())
        }
    }

    
    // Obtain feature vector for each document by pipelining text content into Word2Vec.
    // Need to make use of Python Gensim library from within Rust.
    // No Error type provided as each error case is handled by a panic.
    // Temporarily public for testing.
    pub fn embed_documents(document_terms: HashMap<Document, Vec<String>>, num_terms: u32) -> Result<Vec<EmbeddedDocument>, String> {
        let mut global_embeddings: Vec<EmbeddedDocument> = Vec::new();

        document_terms.iter().for_each(|(document, terms)| {
            if terms.len() < 3 {
                return; 
            }
            
            let local_embeddings: Vec<Vec<f32>>;

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
            
            println!("Embedded document: {:?}", doc.document.title);
            
            global_embeddings.push(doc);
            
        });

        if global_embeddings.is_empty() {
            return Err((-1).to_string());
        }

        Ok(global_embeddings)
            
    }


    
    // Clusters similar documents together using k-means. Need to store the clusters, mean centroid
    // value and the distance for each document to their respective centroid. Need to specify a
    // different type for this.
    pub fn generate_clusters (embeddings: Vec<EmbeddedDocument>) -> Result<Vec<Cluster>, ()> {
        for embedding in &embeddings {
            println!("Embedding: {:?}", embedding.embedding);
        }

        // Collect embedded samples - as float vectors.
        let samples = embeddings.par_iter()
            .map(|doc| doc.embedding.clone())
            .collect::<Vec<Vec<f32>>>();
        
        let script = "./scripts/cluster.py";    

        let clusters_output = Command::new("python3")
            .arg(script)
            .arg(json!(samples).to_string())
            .output()
            .expect("Failed to make clusters");

        let clusters_json = std::str::from_utf8(&clusters_output.stdout).expect("Clusters output not streamed as UTF-8 bytes");

        let clusters_json_value: Value;

        match serde_json::from_str(clusters_json) {
            Ok(value) => clusters_json_value = value,
            Err(e) => {
                eprintln!("Failed to parse JSON: {}", e);
                panic!("Exiting...");
            }
        }

        let clusters = clusters_json_value.as_array()
            .expect("JSON is not an array")
            .par_iter()
            .map(|cluster| {
                let cluster_arr = cluster.as_array().expect("JSON is not an array");
                let cluster_embeddings: Vec<Vec<f32>> = serde_json::from_value(cluster_arr[1].clone()).expect("Embeddings cluster JSON is not an array");
                let centroid_out: Vec<f32> = serde_json::from_value(cluster_arr[0].clone()).expect("Centroid JSON is not an array");
                let mut documents_for_cluster = vec![];
                // Possibly inefficient way to map embeddings documents back to their container struct?
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

    
    // Minkowski distance metric - uses Pca model to reduce query vector where necessary to size of the centroid.
    fn distance (a: Vec<f32>, b: Vec<f32>) -> f32 {
        let p = a.len() as f32;
        a.par_iter()
            .zip(b.par_iter())
            .map(|(x, y)| (x - y).abs().powf(p))
            .sum::<f32>()
            .powf(1.0 / p)
    }
    
    fn reduce_query (query: Vec<f32>, embeddings: Vec<EmbeddedDocument>, centroid_len: usize) -> Vec<f32> {
        let embeddings: Vec<Vec<f32>> = embeddings.par_iter().map(|doc| doc.embedding.clone()).collect();

        // Generate the dataset from the input embedding vectors.
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
                panic!("Index type not supported.");
            }
        }
        
        // TO DO: propagate error code, so that API can send "no documents could be embedded".
        let embeddings = embed_documents(document_terms, 2)?;
    
        let mut clusters: Vec<Cluster> = Vec::new();

        match generate_clusters(embeddings.clone()) {
            Ok(clusters_out) => clusters = clusters_out,
            Err(_) => {}
        }

        // Embed the query for similarity comparison with cluster centroids.
        // Split the query into multiple terms - then normalise these outputs and average them.
        let query_value: Value = serde_json::from_str(&query).expect("Query not in JSON format");
        let extracted_query = query_value.get("query").expect("No query field in JSON");
        let parsed_query = extracted_query.to_string().replace("\"", "").trim().split_whitespace().map(str::to_string).collect();
        
        println!("Parsed query: {:?}", parsed_query);
        
        // TO DO: here propagate error code, so that API can send "query not found in model vocab".
        let query_embeddings = make_embeddings(parsed_query)?;

        println!("Query embeddings: {:?}", query_embeddings);

        let mut query_embedding = get_average_vector(query_embeddings);

        println!("Averaged query embedding: {:?}", query_embedding);

        let centroid_len = clusters[0].centroid.len();
        let query_len = query_embedding.len();
       
        if query_len < centroid_len {
            query_embedding.resize(centroid_len, 0.0);
        }

        else if query_len > centroid_len {
            query_embedding = reduce_query(query_embedding, embeddings, centroid_len);
        } 
        
        let min = clusters.iter().min_by_key(move |cluster| {
            distance(query_embedding.clone(), cluster.centroid.clone())
            .partial_cmp(&distance(query_embedding.clone(), cluster.centroid.clone()))
            .expect("NaN values present")
        });
        
        let mut ranked_docs: Vec<Document> = vec![];

        for doc in &min.unwrap().documents {
            ranked_docs.push(doc.document.clone());
        }

        println!("Ranked documents: {:?}", ranked_docs.par_iter().map(|d| d.title.to_string()).collect::<Vec<String>>());

        Ok(ranked_docs)
    }
    

