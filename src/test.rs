use serde_json::error::Category;
use std::process::Command;
use serde_json::Value;
use rayon::prelude::*;

fn make_embedding (term: String) -> Option<Result<Vec<f32>, serde_json::Error>> {
        // Obtain script output from scripts / embedding.py - returns term vector as a
        // JSON-encoded array.
        let script = "./scripts/embedding.py";

        let embedding_output = Command::new("python3")
            .arg(script)
            .arg(term)
            .output()
            .expect("Failed to make embedding");

        println!("{:?}", embedding_output);

        // Obtain JSON string from the script output.
        let embedding_json = std::str::from_utf8(&embedding_output.stdout).expect("Embedding output not UTF-8");

        println!("{:?}", embedding_json);
                    
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

fn main() {
    let terms = vec!["computer", "keyboard", "mouse", "determine", "opal"];
    let feature_vectors: Vec<Vec<f32>> = terms.par_iter().map(|t| make_embedding(t.to_string()).unwrap().unwrap()).collect();
    for v in feature_vectors {
        println!("{:?}", v);
    }
}




