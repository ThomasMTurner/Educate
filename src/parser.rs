use reqwest::Error;
use serde_json::json;
use crate::crawl::CrawlResult;
use scraper::{Html, Selector};
use serde::{Serialize, Deserialize};
use std::process::Command;
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
   

// Structure to store parsed data into Document types which can be indexed.
// Omitted body as this is wasted information.
#[derive(Debug, Hash, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Document {
    pub url: String,
    pub content: Vec<String>,
    pub description: String,
    images: Vec<String>, 
    links: Vec<String>,
    pub title: String,
}

// Directly spawns child process to docker instance of LibreTranslate server.
fn _start_translate_server() {
    let run_translate_server = "docker run -ti --rm -p 127.0.0.1:5000:5000 libretranslate/libretranslate";
    let output = Command::new("sh").arg("-c").arg(run_translate_server).output().expect("Failed to run translation server.");
    println!("Starting translation server.");
    println!("Status of output {}", output.status)
}
    
// Obtains the source language of the text using lingua library.
fn get_source_language(text: String) -> Option<Language> {
    let languages: Vec<Language> = vec![Language::English, Language::Ukrainian, Language::Turkish, Language::Thai, Language::Swedish,
    Language::Spanish, Language::Slovene, Language::Slovak, Language::Russian, Language::Romanian, Language::Portuguese,
    Language::Polish, Language::Korean, Language::Japanese, Language::Italian, Language::Hungarian, Language::French,
    Language::Chinese];

    let detector: LanguageDetector = LanguageDetectorBuilder::from_languages(&languages).build();
    return detector.detect_language_of(text);
}
    
// Sends POST request to LibreTranslate API, auto-detecting source language and finally
// converting to English.
pub async fn _convert_text_to_english(text: String) -> Result<String, Error> {
    let mut source = String::new();
    let mut skip_translation: bool = false;

    //Get the source language of the text
    let source_response = get_source_language(text.clone());

    match source_response {
        Some(language) => {
            let language = language.to_string();
            if language == "English" {
                // Skip translation if English (English -> English already satisfied).
                println!("Body of the text is English  should be skipping");
                skip_translation = true;
            }
            else {
                source = language;
            }
        }
        None => println!("No language detected.")
    }
        
    if !skip_translation {
        let client = reqwest::Client::new();
        let res = client.post("http://0.0.0.0:5000/translate")
            .header("Content-Type", "application/json")
            .json(&json!({
                "q": text,
                "source": source,
                "target": "en"
            }))
            .send().await?;

        let status = res.status();
        if status.is_success() {
            let text_response = res.text().await;
            match text_response {
                Ok(text) => {
                    println!("Outputs the translated text {}", text);
                    Ok(text)
                }
                Err(e) => panic!("Translation error: {}", e)
            }
        }

        else {
            println!("Translation error: {}", status);
            Ok("Skip".to_string())
        }
    }

    else {
        Ok("Skip".to_string())
    }
}


    
// Implement behaviour for println! on Document type.
impl std::fmt::Display for Document {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "URL: {}, description: {}, title: {}", self.url, self.description, self.title)?;
        Ok(())
    }
}


// Parse all non-metadata text content.
fn parse_content(document: Html, content_selector: &Selector, mut content:Vec<String>, skip_translation: bool) -> Result<Vec<String>, Error> {
    for element in document.select(content_selector) {
        let text = element.text().collect::<String>();
        if !text.is_empty() {
            if !skip_translation {
                println!("Skipping translation for {}", text);
                    /*
                    let text_translation = convert_text_to_english(text.clone()).await;
                    match text_translation {
                        Err(e) => panic!("Error in translation: {}", e),
                        Ok(result) => {
                            if result != "Skip" {
                                text = result;
                            }
                            else {
                                println!("Skipping translation for {}", text);
                
                            }
                        }       
                    }
                    */
            }
            content.push(text);
        }
    }

    Ok(content)

}
    
// Parse all results from crawl bot.
pub fn parse_crawl_results(crawl_results: Vec<CrawlResult>) -> Vec<Document> {
    //start_translate_server();
    let mut parsed_results: Vec<Document> = Vec::new();
    for crawl_result in crawl_results {
        let result = parse_crawl_result(crawl_result);
        match result {
            Ok(document) => parsed_results.push(document),
            Err(_) => println!("Skipping parsing the result")
        }
    }
    parsed_results
}
    
// Parses raw HTML data from crawler and returns Document type.
fn parse_crawl_result(crawl_result: CrawlResult) -> Result<Document, String> {

    let mut skip_translation: bool = false;

    let available_languages: Vec<&str> = vec!["English", "Ukrainian", "Turkish", "Thai", "Swedish", 
        "Spanish", "Slovene", "Slovak", "Russian", "Romanian", "Portuguese", "Polish", "Korean", "Japanese", 
        "Italian", "Hungarian", "French"];

    // first initialise entries already in the crawled results
    let url     = crawl_result.url;
    let body    = crawl_result.body;
    let links   = crawl_result.new_urls;

    //metadata entries
    let mut description = String::new();

    //non-metadata entries
    let mut content: Vec<String> = Vec::new();
    let mut images: Vec<String> = Vec::new();
    let mut title                = String::new(); 
    let document = Html::parse_document(&body);
        


    // Intialise selectors corresponding to above data.
    let title_selector    = Selector::parse("title").unwrap();
    let p_selector        = Selector::parse("p").unwrap();
    let h_selector        = Selector::parse("h1, h2, h3, h4, h5, h6").unwrap();
    let metadata_selector = Selector::parse("meta").unwrap();
    let image_selector    = Selector::parse("img").unwrap();

    
    // First select the title to include as part of the results
    if let Some(title_element) = document.select(&title_selector).next() {

        //Get the title element
        title = title_element.text().collect::<String>();
        println!("Originally setting title: {}", title);

        // Check to see if title is in the list of available languages
        let lang = get_source_language(title.clone());

        match lang {
            Some(language) => {
                let language_str = language.to_string();
                if (!available_languages.contains(&language_str.as_str())) || language_str != "English" {
                    println!("Not translatable (or bypass - text is non-english): {}", title);

                    // Only return error - that is skip parsing the current page - if language
                    // is not in available LibreTranslate models.
                        
                    // BYPASS: default skip translation if non-English.

                    return Err("Skip".to_string())
                }
                    
                if language_str == "English" {
                    skip_translation = true;
                }
                    
            },

            None => ()
        }
        
        // In the event that it is non-English.
        if !skip_translation {
            ();

                /*
                let title_translation = convert_text_to_english(title.clone()).await;
                match title_translation {
                    // Immediately handle the error case from convert_text_to_english function. 
                    Err(e) => panic!("Translation error: {}", e),
                    // Modify to keep the title as is without translation.
                    Ok(text) => {
                        if text != "Skip" {
                            title = text;
                        }
                        else {
                            ();
                        }
                    }
                }
                */
        }
        // In event text data is English.
        else {
            println!("Title is fine as is without translation: {}", title);
        }
    }

    else {
        println!("Title not found");
    }
         

    // Parse first h elements and then p elements to gather all of the text content.
        
    content = parse_content(document.clone(), &p_selector, parse_content(document.clone(), &h_selector, content, skip_translation).unwrap(), skip_translation).unwrap();

    // Working with metadata: consists of two attributes (name) and (content), where name is covered by cases we want to include, such as description,
    //author, keywords, etc.

    // Hence need to parse all meta tags and then provide cases for all the above we would like to include.

    // Alternatively use the following syntax inside of the selector: let description_selector = Selector::parse("meta[name='description']").unwrap();
        
        
    for element in document.select(&metadata_selector) {
        if let Some(name) = element.value().attr("name") {
            match name {
                "description" => {
                    if let Some(content) = element.value().attr("content") {
                        if !skip_translation {
                            println!("Skipping translation");
                                /*
                                let description_translation = convert_text_to_english(content.to_string()).await;
                                match description_translation {
                                    Err(e) => panic!("Translation error: {}", e),
                                    Ok(text) => description = text
                                }
                                */
                                
                        }
                        else {
                            if description != "Skip" {
                                description = content.to_string();
                            }
                        }
                    }
                },
                _ => {
                    println!("Do nothing");
                }
            }
        }
    }
         

       
    // Working with images: need to convert string stored in the src attribute of the img tag as a U8 vector (vector of integer bytes), this can
    // then be converted back to base64 format which allows us to display the images in the search interface.
    // Currently, we assume the logo is at the root of the DOM tree - and will be the first image
    // processed by the selector.
   
    // TO DO: may need to modify to extract the MIME type string to decode the image.

    for element in document.select(&image_selector) {
        if images.len() > 3 {
            break
        }
        match element.value().attr("src") {
            Some(src) => images.push(String::from(src)),
            None => ()
        }
    }

    if url.is_empty() || body.is_empty() || title.is_empty() || content.is_empty() {
        return Err(String::from("Skip"));
    }

    let new_document = Document {
        url,
        content,
        description,
        images,
        links,
        title,
    };

    Ok(new_document)
}
    





