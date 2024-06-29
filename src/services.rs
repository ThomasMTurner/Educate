// Module includes API-accessible commands - such as filling indices on application startup - and
// obtaining query results.
    
    use std::collections::HashMap;
    use crate::rank::get_ranked_documents;
    use crate::index::{Indexer, read_index_file};
    use crate::discover::get_domains_and_webpages;
    use crate::parser::{parse_crawl_results, Document};
    use crate::crawl::{get_crawled, CrawlResult};
    use serde::{Serialize, Deserialize};
    
    // Fill indices on application startup with crawl bot results (or use cached instead - later
    // modification).
    pub async fn fill_indices (crawl_depth: u8, seed_count: u8) -> Option<HashMap<Document, Vec<String>>> {
        let new_index: bool;
        let index_map: HashMap<Document, Vec<String>>;

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
            let parsed_results = parse_crawl_results(results);
        
            // Creates raw indices - stores in file (if file isn't already filled) and stores indices raw for later use.
            let _ = Indexer::TermIndex(HashMap::new()).new(parsed_results.clone());

            // Skip for now - for some reason much more computationally intensive than TermIndex.
            // let _ = Indexer::InvertedIndex(HashMap::new()).new(parsed_results.clone());

            println!("Completed creating indices...");

            // Now read the indices again which have just been written to disk.
            match read_index_file("./indices/dterm.json") {
                Ok(Indexer::TermIndex(map)) => {
                    index_map = map;
                    Some(index_map)
                }
                Ok(Indexer::InvertedIndex(_)) => {
                    return None
                }
                Err(e) => {
                    eprintln!("Index not found - problem with creation: {}", e);
                    return None
                }
            }
        } 
        else {
            println!("Indices already exist!");
            return None
        }
    }

    pub fn get_search_results(query: String) -> Result<DocumentResult, String> {
        let index_map = match read_index_file("./indices/dterm.json") {
            Ok(Indexer::TermIndex(map)) => map,
            Ok(Indexer::InvertedIndex(_)) => return Err(String::from('2')),
            Err(_) => return Err(String::from('2'))
        };

        let num_indexed = index_map.len();

        println!("{:?} indexed results loaded", num_indexed);

        let results: Vec<Document> = get_ranked_documents(query, Indexer::TermIndex(index_map))?;
        Ok(DocumentResult{results, indexed: num_indexed})
    }

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentResult {
    pub results: Vec<Document>,
    pub indexed: usize
}



