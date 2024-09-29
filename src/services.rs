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
    
    pub async fn fill_indices (crawl_depth: u8, seed_count: u8) -> Option<HashMap<Document, Vec<String>>> {
        let new_index: bool;
        //let index_map: HashMap<Document, Vec<String>>;
        let index_map: HashMap<String, Vec<Document>>;

        match read_index_file("./indices/dterm.json") {
            Ok(Indexer::TermIndex(_)) => {
                new_index = false
            }
            Ok(Indexer::InvertedIndex(_)) => {
                return None
            }
            Err(e) => {
                eprintln!("Index not found - creating new index: {}", e);
                new_index = true
            }
        }
        
        if new_index {
            let seed_urls: Vec<String>;

            match get_domains_and_webpages() {
                Ok((urls, _)) => {
                    seed_urls = urls[0..seed_count as usize].to_vec();
                }
                Err(_) => {
                   return None 
                }
            }
            
            // Modify to handle error case explicitly.
            let results: Vec<CrawlResult> = get_crawled(seed_urls, crawl_depth.into()).await;
            println!("Crawled {} results", results.len());
            let parsed_results = parse_crawl_results(results);
            println!("Parsed {} results", parsed_results.len());
        
            // Creates raw indices - stores in file (if file isn't already filled) and stores indices raw for later use.
            //let _ = Indexer::TermIndex(HashMap::new()).new(parsed_results.clone());
            let _ = Indexer::InvertedIndex(HashMap::new()).new(parsed_results.clone());

            // TO DO:
            // Use configurations to select index path to read.
            // Currently we will test the inverted path manually.
            // Standard path (for forward index) is './indices/dterm.json'
            match read_index_file("./indices/inverted.json") {
                Ok(Indexer::TermIndex(_)) => {
                    //index_map = map;
                    //Some(index_map)
                    return None
                }
                Ok(Indexer::InvertedIndex(map)) => {
                    index_map = map;
                    Some(index_map)
                }
                Err(e) => {
                    eprintln!("Document term index not found - problem with creation: {}", e);
                    return None
                }
            }
        } 
        else {
            println!("Indices already exist!");
            return None
        }
    }

    pub fn get_search_results(query: String, script: &str) -> Result<SearchResponse, String> {
        // TO DO:
        // Choice of index should also be configurable.
        // Manually testing inverted.
        match read_index_file("./indices/inverted.json") {
            Ok(Indexer::TermIndex(_)) => Err(String::from('2')),
            Ok(Indexer::InvertedIndex(map)) => {
                let num_indexed = map.len();
                let results: Vec<Document> = get_ranked_documents(query, Indexer::InvertedIndex(map), script)?;
                Ok(SearchResponse::Search(DocumentResult {results, indexed: num_indexed}))

            },
            Err(_) => Err(String::from('2'))
        }
    }


#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentResult {
    pub results: Vec<Document>,
    pub indexed: usize
}



