use std::collections::HashMap;
use serpapi::serpapi::Client;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::error::Error;
use futures::stream::{self, StreamExt};
use std::env;
//use geolocation;


// Overview of the service.
// (1) Take a batch of search requests. This will be specified by the browser, query & location.
// Location may be specified automatically (WARN ON DEFAULT LOCATION PROVIDING SLOW REQUESTS) or
// through the user providing their IP address manually.
// (2) Distribute batch over parallel threads to aggregate results. 
// ALSO TO DO; need to specify a specific error type & also the structure of the output.

pub struct MetaSearchRequest {
    browser: String,
    q: String,
    location: String
}

#[derive(Debug)]
pub struct MetaSearchResult {
    anon: String,
}

impl MetaSearchResult {
    fn new(anon: String) -> Self {
        MetaSearchResult { anon }
    }
}

// Approach - do some work for a single request, then aggregate in async context.
impl MetaSearchRequest {
    fn new(browser: String, q: String, location: String) -> Self {
        MetaSearchRequest { browser, q, location }
    }
    
    // Boxed dynamically allocated error is used here due to the error type of the 
    // serpapi::Client::search() method.
    async fn collect(&self, client: Arc<Mutex<Client>>) -> Result<MetaSearchResult, Box<dyn Error>> {
        let mut parameter = HashMap::<String, String>::new();
        parameter.insert("q".to_string(), self.q.to_string());
        parameter.insert("location".to_string(), self.location.to_string());

        // Lock the client and perform the search
        let client = client.lock().await;
        let response = client.search(parameter).await?;
        
        // Obtain organic results.
        let organic = response["organic_results"].as_array().ok_or("No organic results")?;
        assert!(organic.len() > 0);
        println!("Got organic results: {:?}", organic);

        // Still have no idea what the organic output is.
        Ok(MetaSearchResult::new("".to_string()))
    } 
    
}

#[tokio::main]
async fn aggregate(requests: Vec<MetaSearchRequest>, concurrent_requests: usize) -> Result<(), Box<dyn Error>> {
    let mut browsers: Vec<String> = Vec::with_capacity(requests.len());
    let mut clients: Vec<Arc<Mutex<Client>>> = Vec::with_capacity(requests.len());

    // Create client objects.
    requests.iter().for_each(|r| {
        if !browsers.contains(&r.browser) {
            browsers.push(r.browser.clone());
            let mut default = HashMap::<String, String>::new();
            default.insert("engine".to_string(), r.browser.clone());

            // NOTE: user must export &BROWSER&_API_KEY first.
            let key = r.browser.to_string().to_uppercase() + "_API_KEY";

            if let Ok(v) = env::var(key) {
                default.insert("api_key".to_string(), v);
            } else {
                eprintln!("No API key found for browser: {}", r.browser)
            }

            default.insert("api_key".to_string(), "API_KEY".to_string());
            clients.push(Arc::new(Mutex::new(Client::new(default))));
        } else {
            println!("Skipping duplicate browser. Not creating Client object.");
        }
    });
    

    stream::iter(requests.into_iter().map(|r| {
        let client = Arc::clone(&clients[browsers.iter().position(|x| x == &r.browser).unwrap()]);
        async move {
            r.collect(client).await
        }
    }))
    .buffer_unordered(concurrent_requests)
    .for_each(|result| async {
        match result {
            Ok(res) => println!("Got result: {:?}", res),
            Err(e) => eprintln!("Error collecting result: {:?}", e),
        }
    })
    .await;


    // Aggregate results into MetaSearchResult
    Ok(())
}



