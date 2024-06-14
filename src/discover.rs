use serde::Deserialize;
use serde_json::from_str;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
struct EduDomain {
    name: String,
    domains: Vec<String>,
    web_pages: Vec<String>,
    country: String,
    alpha_two_code: String,
    state_province: Option<String>,
}

//should return error in the case of a problem with IO operation, otherwise should just return string contents.
fn read_domains_json() -> Result<String, std::io::Error> {
    println!("Reading domains.json");
    let mut file = File::open("./discovery_data/domains.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(contents)
}

pub fn get_domains_and_webpages() -> Result<(Vec<String> , Vec<String>), std::io::Error> {
    let edu_domains: Vec<EduDomain>;
    let mut domains: Vec<String>;
    let mut seed_urls: Vec<String>;

    match read_domains_json() {
        Ok(contents) => {
            println!("JSON: {}", contents);
            // Here we use Serde's deserialiser to convert JSON to string, and return the result of this conversion (admittedly unsafely)
            edu_domains = from_str(&contents).unwrap();
            domains = Vec::new();
            seed_urls = Vec::new();
        }
        Err(e) => {
            println!("Error: {}", e);
            return Err(e)
        }
    }

    for edu_domain in edu_domains {
        let mut domains_vec = edu_domain.domains.clone();
        let mut web_pages_vec = edu_domain.web_pages.clone();

        domains.append(&mut domains_vec);
        seed_urls.append(&mut web_pages_vec);
    }
        
    Ok((seed_urls, domains))
}



