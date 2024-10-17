// Module includes API-accessible commands - such as filling indices on application startup - and
// obtaining query results.
    
    use std::collections::HashMap;
    use crate::rank::get_ranked_documents;
    use crate::index::{Indexer, read_index_file};
    use crate::discover::get_domains_and_webpages;
    use crate::parser::{parse_crawl_results, Document};
    use crate::crawl::{get_crawled, CrawlResult};
    use serde::{Serialize, Deserialize};
    use crate::meta::SearchResponse;
    use thiserror::Error;
    
    // To Do:
    // Later this will encapsulate all services errors.
    // Currently single error implemented for case where incorrect
    // index name contains index type.
    #[derive(Error, Debug)]
    pub enum ServiceError {
        #[error("Incorrect index type in file")]
        IndexTypeError(std::io::Error),
        #[error("Could not read domains JSON")]
        ReadDomainsError(std::io::Error)
    }
    
    pub async fn fill_indices (crawl_depth: u8, seed_count: u8) -> Result<(), ServiceError> {
        let new_forward_index: bool;
        let new_inverted_index: bool;

        match read_index_file("./indices/dterm.json") {
            Ok(Indexer::TermIndex(_)) => {
                new_forward_index = false
            }
            Ok(Indexer::InvertedIndex(_)) => {
                return Err(ServiceError::IndexTypeError(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Incorrect index type in file",
                )));
            }
            Err(_) => {
                new_forward_index = true
            }
        }

        match read_index_file("./indices/inverted.json") {
            Ok(Indexer::TermIndex(_)) => {
                return Err(ServiceError::IndexTypeError(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Incorrect index type in file",
            )));
            }
            Ok(Indexer::InvertedIndex(_)) => {
                new_inverted_index = false;
            }
            Err(_) => {
                new_inverted_index = true
            }

        }
        
        if new_inverted_index || new_forward_index {
            let seed_urls: Vec<String>;

            match get_domains_and_webpages() {
                Ok((urls, _)) => {
                    seed_urls = urls[0..seed_count as usize].to_vec();
                }
                Err(e) => {
                   return Err(ServiceError::ReadDomainsError(e)) 
                }
            }
            
            // Modify to handle error case explicitly.
            let results: Vec<CrawlResult> = get_crawled(seed_urls, crawl_depth.into()).await;
            let parsed_results = parse_crawl_results(results);
        
            // Creates raw indices - stores in file (if file isn't already filled) and stores indices raw for later use.
            if new_forward_index {
                let _ = Indexer::TermIndex(HashMap::new()).new(parsed_results.clone()); 
            }

            if new_inverted_index {
                let _ = Indexer::InvertedIndex(HashMap::new()).new(parsed_results.clone());
            }
            
            Ok(())
        } 

        else {
            println!("Indices already exist!");
            Ok(()) 
        }
    }

    // get_search_results can receive a selection of possible ranking procedures (supported).
    // These are 1. Word2Vec document clustering 2. Sentence Transformer (BERT) document clustering
    // 3. BM25 (TF-IDF improvement) sorted.
    // LATER (final extension).
    // Optimise all above approaches
    // (1) & (2) Compute Latent Semantic Analysis before comparisons.
    // (3) Take best K from BM25, compute Latent Semantic Analysis, cosine similarity compare.

    // We need information about the procedure type.
    // Simple by checking if script string is not None.
    // Where None this is asking for BM25 ranked.
    pub fn get_search_results(query: String, script: &str) -> Result<SearchResponse, String> {
        if script.is_empty() {
            println!("Using BM25 ranked search");
            match read_index_file("./indices/inverted.json") {
                Ok(Indexer::InvertedIndex(map)) => {
                    let num_indexed = map.len();
                    let results = get_ranked_documents(query, Indexer::InvertedIndex(map), "")?;
                    Ok(SearchResponse::Search(DocumentResult {results, indexed: num_indexed}))
                },
                Ok(Indexer::TermIndex(_)) => {
                    Err(String::from("2"))
                },
                Err(_) => Err(String::from('2'))
            }
        }

        else {
            println!("Using {} ranked search", script);
            match read_index_file("./indices/dterm.json") {
                Ok(Indexer::InvertedIndex(_)) => {
                    println!("Index is Inverted Index");
                    Err(String::from("2"))
                },
                Ok(Indexer::TermIndex(map)) => {
                    println!("Index is Term Index");
                    let num_indexed = map.len();
                    let results: Vec<Document> = get_ranked_documents(query, Indexer::TermIndex(map), script)?;
                    Ok(SearchResponse::Search(DocumentResult {results, indexed: num_indexed}))
                },
                Err(_) => {
                    println!("Index not found");
                    Err(String::from("2"))
                }       
            }
            
        }
    }


#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentResult {
    pub results: Vec<Document>,
    pub indexed: usize
}



