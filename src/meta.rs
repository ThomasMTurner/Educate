use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::error::Error;
use futures::stream::{self, StreamExt};
use std::env;
use serde::{Deserialize, Serialize};
use crate::services::DocumentResult;
use rocket::serde::json::Json;
use std::process::Command;
//use geolocation;

// NOTE: thiserror is often recommended for error type building.
// NOTE: probably should use issue tracking rather 

#[derive(Serialize, Deserialize)]
pub enum SearchResponse {
    Search(DocumentResult),
    MetaSearch(MetaSearchResult)
}

pub enum SearchResult {
    Documents(Json<Vec<SearchResponse>>),
    Error(Json<String>)
}


#[derive(Debug, Serialize, Deserialize)]
pub struct MetaSearchResult {
    anon: String,
}

impl MetaSearchResult {
    fn new(anon: String) -> Self {
        MetaSearchResult { anon }
    }
}

#[derive(Debug)]
pub struct MetaSearchRequest {
    pub browser: String,
    //location: String,
    pub q: String
}


// Approach - do some work for a single request, then aggregate in async context.
impl MetaSearchRequest {
    pub fn new(browser: String, q: String /*location: String*/) -> Self {
        MetaSearchRequest { browser, /*location,*/ q }
    }

    async fn collect(&self) -> Result<SearchResponse, ()> {
       match self.browser.as_str() {
            "DuckDuckGo" => {
                println!("Collecting DuckDuckGo results");
                Ok(self.ddgr_collect().await)

            }
            "Google" => {
                Ok(self.googler_collect().await)
            }
            _ => {
                eprintln!("Unknown browser: {}", self.browser);
                Err(())
            }
        } 
    }


    /// TO DO (LATER):
    /// BOTH OF THESE CAN BE MODIFIED TO INCORPORATE USER LOCALE AND 
    /// FURTHER SEARCH FILTERS.
    /// I.E. LOCALE INFORMATION CAN BE USED (AREA CODE) WITH -R 
    /// PARAMETER WITH DDGR TO GET RESULTS FOR A SPECIFIC COUNTRY.
    /// ALSO NEED TO SET PROPER ERROR HANDLING.
    
    // TO DO:
    // Does not require API key - simply need to build command for simple
    // search (currently we do not have search filters).
    async fn ddgr_collect(&self) -> SearchResponse {
        let output = Command::new("ddgr")
            .args(self.q.split_whitespace())
            .output()
            .expect("Failed to execute ddgr");

        println!("DDGR output: {}", String::from_utf8_lossy(&output.stdout));
        
        SearchResponse::MetaSearch(MetaSearchResult::new(
            String::from_utf8_lossy(&output.stdout).to_string()))
    }
    
    // TO DO:
    // Does require API key - need to see how to collect this appropriately.
    async fn googler_collect(&self) -> SearchResponse {
        let key = self.browser.to_uppercase() + "_API_KEY";
        if let Ok(v) = env::var(key) {
            // Set API key for Googler config.
            println!("API key for Googler: {}", v);
            
        } else {
            eprintln!("Could not find API key for Googler")
        }

       
        // Build output.
        

        SearchResponse::MetaSearch(MetaSearchResult::new(
            String::from(" ")
        ))

    }
    
    // May want to extract more useful information from the engine directly
    // using some available Search API as opposed to simple command line tools.
    
}

// CURRENTLY: we removed instantiation of Tokio runtime as this 
// is already done by Rocket.
pub async fn aggregate(requests: Vec<MetaSearchRequest>) -> Result<Vec<SearchResponse>, Box<dyn Error>> {
    let mut responses: Vec<SearchResponse> = Vec::with_capacity(requests.len());
    let concurrent = requests.len();

    let results: Vec<_> = stream::iter(requests.into_iter().map(|r| {
        async move {
            r.collect().await
        }
    }))
    .buffer_unordered(concurrent)
    .collect()
    .await;

    for result in results {
        match result {
            Ok(res) => responses.push(res),
            Err(_) => eprintln!("Error obtaining meta search response"),
        }
    }
    
    Ok(responses)
}



