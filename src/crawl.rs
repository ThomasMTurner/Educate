use scraper::{Html, Selector};
use std::collections::BinaryHeap;
use reqwest::get;

#[derive(Debug, Eq, PartialEq, Clone)]
struct UrlToVisit {
    url: String,
    crawl_depth: u32,
}
    
#[derive(Clone, Debug)]
pub struct CrawlResult {
    pub url: String,
    pub new_urls: Vec<String>,
    pub body: String,
}

// IMPLEMENTED:
// Partial ordering to compare two CrawlResults for priority, based on smallest crawl_depth.
impl PartialOrd for UrlToVisit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.crawl_depth.cmp(&other.crawl_depth).reverse())
    }
}

// IMPLEMENTED:
// Full ordering with error handling layer above to catch errors in comparing tuples.
impl Ord for UrlToVisit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.crawl_depth.cmp(&other.crawl_depth).reverse()
    }
}

async fn get_crawl_result(url: &str) -> Result<CrawlResult, reqwest::Error> {
    let mut new_urls: Vec<String> = Vec::new();
    let response = get(url).await?;
    let body = response.text().await?;
    let fragment = Html::parse_document(&body);
    let url_selector = Selector::parse("a").unwrap();
    
    for element in fragment.select(&url_selector) {
        if let Some(url) = element.value().attr("href") {
            new_urls.push(url.to_string());
        }
    }
    
    let crawl_result = CrawlResult {
        url: url.to_string(),
        new_urls,
        body,
    };
    
    Ok(crawl_result)
}
    

async fn crawl(seed_urls: &mut Vec<String>, max_depth: u32) -> Vec<CrawlResult> {
    // Initialise visited to not re-process
    let mut visited: Vec<UrlToVisit> = Vec::new();

    // Initialise priority queue to crawl URL's
    let mut url_queue: BinaryHeap<UrlToVisit> = BinaryHeap::new();
    let mut results: Vec<CrawlResult> = Vec::new();

    for seed_url in seed_urls {
        url_queue.push(UrlToVisit {
            url: seed_url.to_string(),
            crawl_depth: 0,
        })
    }
        
    //while the url_queue is not empty and depth is less than 10
    while !(url_queue.is_empty()) && (url_queue.peek().unwrap().crawl_depth < max_depth) {

        //pop the next url from the queue
        let next_url = match url_queue.pop() {
            Some(url) => url,
            None => {
                continue;
            }
        };
            
        // If not visited
        if !(visited.contains(&next_url)) {

            // Increment depth for new crawl result
            let new_depth: u32 = next_url.crawl_depth + 1;
                
            // Get body response from get request - do NOT propagate error to the caller,
            // Simply print out the error response if it happens internally.
            let crawl_result;

            match get_crawl_result(&next_url.url).await {
                Ok(result) => crawl_result = result,
                Err(_) => continue
            }
 
            for url in &crawl_result.new_urls {
                url_queue.push( UrlToVisit {
                    url: url.to_string(),
                    crawl_depth: new_depth,
                })
            }

            // Also add the URL to results.
            results.push(crawl_result);
    
            // Add the URL now to visited
            visited.push(next_url);
        }

    }

    results

}

    

pub async fn get_crawled (seed_urls: Vec<String>, max_depth: u32) -> Vec<CrawlResult> {
    let mut seed_urls = seed_urls;
    let results = crawl(&mut seed_urls, max_depth).await;
    results
}


