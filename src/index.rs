use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::parser::Document;
use rayon::prelude::*;


//set the location to store indices at local subdirectory "indices"
const INDEX_DIR: &str = "./indices";
const DTERM_PATH: &str = "./indices/dterm.json";
const INVERTED_PATH: &str = "./indices/inverted.json";


     
// Pre-processing step before passing text content to indices.
fn tokenise (content: String) -> Vec<String> {

    //remove all non alphabetic characters - text encoded as a stream of UTF encoded bytes (UTF-8)
    //gets all of these chars, applies filter (keeping alphabetic characters), and then collects this stream back into a String
    let content: String = content.chars().filter(|c| c.is_alphabetic() || c.is_whitespace()).collect();

    //delimit words by comma
    let words: Vec<String> = content.split_whitespace().map(|s| s.to_string().to_lowercase()).collect();
        
    //remove stop words - words which do not add semantic value to the string as a sentence.
    let stop_words = vec![
   "a","your", "an", "the", "and", "but", "in", "on", "of", "with", "is", "was", "by", "at", "to", "from", "which", "you", "it", "this", "that", "or", "be", "are", "been", "were", "would", "will", "shall", "should", "can", "could", "has", "have", "had", "not", "if", "else", "then", "for", "but", "or", "so", "no", "nor", "on", "at", "to", "from", "by", "in", "of", "up", "out", "over", "under", "again", "further", "then", "once", "here", "there", "when", "where", "why", "how", "all", "any", "both", "each", "few", "more", "most", "other", "some", "such", "no", "nor", "not", "only", "own", "same", "so", "than", "too", "very", "s", "t", "can", "will", "just", "don", "should", "now", "we"
    ];      

    words.into_iter().filter(|word| !stop_words.contains(&word.as_str())).collect()
}


    
// Create fresh or filled index and place at the file path specified.
pub fn create_index_file(file_path: &str, index: &Indexer) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(file_path);
        
    // If indices directory doesn't exist, create it.
    if !path.exists() {
        fs::create_dir_all(INDEX_DIR)?;
    }

    let file = File::create(file_path)?;
    bincode::serialize_into(file, index)?;
    Ok(()) 
}
    
// Read fresh or filled index at file path specified.
// Originally set to place error on the heap (Box<dyn std::error::Error>)
pub fn read_index_file(file_path: &str) -> Result<Indexer, String> {
    let file;
    let index;

    match File::open(file_path) {
        Ok(f) => file = f,
        Err(e) => return Err(e.to_string())
    }

    match bincode::deserialize_from(file) {
        Ok(i) => index = i,
        Err(e) => return Err(e.to_string())
    }

    Ok(index)
}

// Delete the file at the file path specified.
pub fn _delete_index_file(file_path: &str) -> std::io::Result<()> {
    fs::remove_file(file_path)?;
    Ok(())
}
    
// Delete all indices, need to store file path locations in separate file.
pub fn _delete_all() -> std::io::Result<()> {
    for index in fs::read_dir(INDEX_DIR)? {
        let index = index?;
        if index.file_type()?.is_file() {
            let _ = fs::remove_file(index.path());
        }
    }
    Ok(())
    
}
  
    
// Create enum to store all index types as variants, currently includes document-term and
// term-document indices. Later will want to include B-tree.
#[derive(Serialize, Deserialize, Debug)]
pub enum Indexer {
    TermIndex(HashMap<Document, Vec<String>>),
    InvertedIndex(HashMap<String, Vec<Document>>),
    
}

// Implementation of standard Hash Map functions for each index type.
impl Indexer {
    pub fn new(&mut self, documents: Vec<Document>) -> &mut Self {
        match self {
            Indexer::TermIndex(_) => {
                for document in documents {
                    let pre_terms: Vec<Vec<String>> = document.content.par_iter().map(|content| tokenise(String::from(content))).collect();
                    let terms = pre_terms.into_iter().flatten().collect();
                    self.insert(document.clone(), terms);
                }

                let _ = create_index_file(DTERM_PATH, &self);
                self
            }
            Indexer::InvertedIndex(_) => {
                for document in documents {
                    let pre_terms: Vec<Vec<String>> = document.content.par_iter().map(|content| tokenise(String::from(content))).collect();
                    let terms = pre_terms.into_iter().flatten().collect();
                    self.insert(document.clone(), terms);
                }
                    
                let _ = create_index_file(INVERTED_PATH, &self);
                self
            }
        }
    }
    

    fn insert(&mut self, document: Document, terms: Vec<String>){
        match self {
            // Document-term implements the standard insert function.
            Indexer::TermIndex(map) => { 
                let _ = map.insert(document, terms);
            },
            // Term-document requires the reverse mapping, handled by below logic.
             Indexer::InvertedIndex(map) => { 
                for term in terms {
                    if let Some(documents) = map.get_mut(&term) { 
                        documents.push(document.clone());
                    } 
                    else {
                        map.insert(term, vec![document.clone()]);
                    }
                }
            }
        }
    }
   
}



