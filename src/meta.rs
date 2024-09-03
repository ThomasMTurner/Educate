use std::error::Error;
use futures::stream::{self, StreamExt};
use serde::{Deserialize, Serialize};
use crate::services::DocumentResult;
use rocket::serde::json::Json;
use std::process::Command;
use regex::Regex;
//use geolocation;

// NOTE: thiserror is often recommended for error type building.

#[derive(Serialize, Deserialize, Debug)]
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
    pub title: String,
    pub url: String,
    pub description: String
}

impl MetaSearchResult {
    fn new(title: String, url: String, description: String) -> Self {
        MetaSearchResult { title, url, description }
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

    async fn collect(&self) -> Result<Vec<SearchResponse>, ()> {
       match self.browser.as_str() {
            "DuckDuckGo" => {
                println!("Collecting DuckDuckGo results");
                match self.ddgr_collect().await {
                    Ok(response) => Ok(response),
                    Err(e) => panic!("Need to implement error handling here: {}", e)
                }
            }
            /*
            "Google" => {
                //
            }
            */
            _ => {
                eprintln!("Unknown browser: {}", self.browser);
                Err(())
            }
        } 
    }
    
    // TO DO:
    // Define a proper regular expression for capture groups ((number . )(some text)([url])(some
    // text))
    // Test this works for ddgr and googler outputs.
    fn process_with_expression(&self, raw: &str) -> Result<Vec<SearchResponse>, regex::Error> {
        // Define regex for simple pattern (number.)(title)([url])(description).
        let re = Regex::new(r"(\d+)\.\s*(.+)\s*(\[.+?\])\s*(.+)")?;
        
        // Convert captures to MetaSearchResult.
        let captures: Vec<SearchResponse> = re.captures_iter(raw)
            .map(|cap| SearchResponse::MetaSearch(
                MetaSearchResult::new(cap[2].to_string(), cap[3].to_string(), cap[4].to_string())))
            .collect();

        Ok(captures)
    }
    

    async fn ddgr_collect(&self) -> Result<Vec<SearchResponse>, regex::Error> {
        let output = Command::new("ddgr")
            .args(self.q.split_whitespace())
            .output()
            .expect("Failed to execute ddgr");

        let raw = String::from_utf8_lossy(&output.stdout);
        //println!("Raw: {}", raw);
        let results = self.process_with_expression(&raw)?;
        println!("Results: {:?}", results);
        Ok(results)
    }
    
    // May want to extract more useful information from the engine directly
    // using some available Search API as opposed to simple command line tools.
    // This will be preferable due to issues with Googler (mostly deprecated).
    
}

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
            Ok(res) => responses.extend(res),
            Err(_) => eprintln!("Error obtaining meta search response"),
        }
    }
    
    Ok(responses)
}



