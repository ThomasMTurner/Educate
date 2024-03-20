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
    use std::process::Command;
    use crate::parser::Document;
    use crate::index::Indexer;
    use std::cmp::min;
    use serde_json::Value;
    use serde_json::json;
    use serde_json::error::Category;
    use serde::{Serialize, Deserialize};
    
    
    //METHOD:
    //
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

    // Obtain the average vector over a set of feature vectors - needed as each term generates
    // their own feature vector. Need a single representation for a document.

    fn get_average_vector (features: Vec<Vec<f32>>) -> Vec<f32> {
        let avg_vector: Vec<f32> = features.par_iter().map(|feature| {
            let sum: f32 = feature.iter().sum();
            let avg = sum / feature.len() as f32;
            avg
        })
        .collect();
        
        avg_vector
    } 
    

    fn make_embedding (term: String) -> Option<Result<Vec<f32>, serde_json::Error>> {
        // Obtain script output from scripts / embedding.py - returns term vector as a
        // JSON-encoded array.
        let script = "./scripts/embedding.py";

        let embedding_output = Command::new("python3")
            .arg(script)
            .arg(term)
            .output()
            .expect("Failed to make embedding");


        // Obtain JSON string from the script output.
        let embedding_json = std::str::from_utf8(&embedding_output.stdout).expect("Embedding output not UTF-8");

                    
        // Obtain deserialised value - skip pass of loop for EOF header error.
        let embedding_json_value: Value;
        match serde_json::from_str(embedding_json) {
            Ok(value) => {
                embedding_json_value = value;
            }
            Err(e) => {
                match e.classify() {
                    Category::Eof => { 
                        println!("Obtained end of file header error - skipping to next term.");
                        return None
                    }
                     _ => return Some(Err(e))
                }
            }
        }
 
        // Collect deserialised feature vector.
        let term_vector = embedding_json_value.as_array()
            // Again may consider modifying - as this panics the program.
            .expect("JSON is not an array")
            .par_iter()
            .map(|v| v.as_f64().expect("JSON value is not a number"))
            .map(|v| v as f32)
            .collect::<Vec<f32>>();


        Some(Ok(term_vector))
    }


    fn normalise_features(features: HashMap<Document, Vec<Vec<f32>>>) -> HashMap<Document, Vec<f32>> {
        // Take the map, reduce each features vector to the same length as the smallest one, then
        // return the document -> averaged features vector map.
        
        // Discover the minimum feature length.
        let mut min_feature_len: u32 = 10000;
        for (_, feature) in &features {
            min_feature_len = min(min_feature_len, feature.len() as u32);
        }

        // Reduce all feature vectors to that length.
        let mut normalised_features = HashMap::new();
        for (document, feature) in &features {
            normalised_features.insert(document.clone(), get_average_vector(feature[0..min_feature_len as usize].to_vec()));
        } 

        normalised_features

    }

    
    // Obtain feature vector for each document by pipelining text content into Word2Vec.
    // Need to make use of Python Gensim library from within Rust.
    // No Error type provided as each error case is handled by a panic.
    // Temporarily public for testing.
    



    pub fn embed_documents(document_terms: HashMap<Document, Vec<String>>) -> Vec<EmbeddedDocument> {
        // Store embeddings
        let mut embeddings = vec![];
        let features: HashMap<Document, Vec<Vec<f32>>> = HashMap::new();

        document_terms.par_iter().for_each(|(document, terms)| {
            if terms.len() < 8 {
                return; // Skip documents with less than 8 terms
            }

            let feature_vectors: Vec<Vec<f32>> = terms[0..15].par_iter()
                .filter_map(|term| {
                    match make_embedding(term.to_string()) {
                        Some(result) => match result {
                            Ok(vector) => Some(vector),
                            Err(e) => {
                                eprintln!("Obtained error embedding term: {}", e);
                                None
                            }
                        },
                        None => None,
                    }
                })
                .collect();

            features.clone().insert(document.clone(), feature_vectors);
        });


        // Now need to normalise all of the feature lengths, get their average vectors and cast
        // these to EmbeddedDocument's.
        let normalised_features: HashMap<Document, Vec<f32>> = normalise_features(features);
        for (document, feature) in normalised_features {
            embeddings.push(EmbeddedDocument {document: document.clone(), embedding: feature});
        }

        // Return the embeddings.
        embeddings
    }
    



    // Clusters similar documents together using k-means. Need to store the clusters, mean centroid
    // value and the distance for each document to their respective centroid. Need to specify a
    // different type for this.
    
    
    pub fn generate_clusters (embeddings: Vec<EmbeddedDocument>) -> Result<Vec<Cluster>, ()> {
         
        // Collect embedded samples - as float vectors.
        let samples = embeddings.par_iter()
            .map(|doc| doc.embedding.clone())
            .collect::<Vec<Vec<f32>>>();
        
        // Need to store partitioned cluster (that is the [[embeddings]] for each cluster).
        // Also need the centroid for each cluster.
        // I.e. [[([embeddings], centroid)]]
        
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
                // Possibly inefficient way to map embeddings documents back to their container
                // struct?
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
        println!("Query vector length (possibly reduced) {}", a.len());
        println!("Centroid vector length {}", b.len());

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
    
    
    pub fn get_ranked_documents (query: String, index: Indexer) -> Result<Vec<Document>, ()> {
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
        
        // Embed documents
        let embeddings = embed_documents(document_terms);
        let mut clusters: Vec<Cluster> = vec![];

        match generate_clusters(embeddings.clone()) {
            Ok(clusters_out) => clusters = clusters_out,
            Err(_) => {}
        }

        // Embed the query for similarity comparison with cluster centroids.
        // Split the query into multiple terms - then normalise these outputs and average them.
        let query_terms = query.split_whitespace().collect::<Vec<&str>>();
        let mut query_embeddings = vec![];
        
        for term in &query_terms {
            match make_embedding(term.to_string()) {
                Some(result) => {
                   match result {
                        Ok(embedding) => query_embeddings.push(embedding),
                        Err(e) => eprintln!("Embedding query failed due to: {}", e)
                    }
                },
                None => {}
            }
        }


        let mut query_embedding = get_average_vector(query_embeddings);

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
        
        Ok(ranked_docs)

    }
    

