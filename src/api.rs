use rocket::http::{Header, Status};
use rocket::{Request, Response, routes};
use rocket::response::{Responder, Result};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::serde::json::Json;
use rocket::{get, post, options, launch};
use crate::services::{fill_indices, get_search_results};
use crate::parser::Document;

// Corrected OPTIONS handler
#[options("/<_..>")]
fn options() -> &'static str {
    "OK"
}

// TO DO: 
// Modify later for configs a third argument to fill_indices to restrict the index types to fill.
#[get("/fill")]
async fn fill() {
    let crawl_depth: u8 = 1;
    let seed_count: u8  = 30;
    fill_indices(crawl_depth, seed_count).await;
}

pub enum SearchResult {
    Documents(Json<Vec<Document>>),
    Error(Json<String>)
}


impl<'r> Responder<'r, 'static> for SearchResult {
    fn respond_to(self, request: &'r Request<'_>) -> Result<'static> {
        match self {
            SearchResult::Documents(docs) => Response::build_from(docs.respond_to(request)?)
                .status(Status::Ok)
                .ok(),
            SearchResult::Error(err) => Response::build_from(err.respond_to(request)?)
                .status(Status::InternalServerError)
                .ok(),
        }
    }
}


// Corrected POST route
#[post("/get-results", data = "<query>")]
pub fn get_results(query: String) -> SearchResult {
    match get_search_results(query) {
        Ok(results) => SearchResult::Documents(Json(results)),
        Err(e) => {
            match e.as_str() {
                "-2" => SearchResult::Error(Json(String::from("Unspecified error"))),
                "-1" => SearchResult::Error(Json(String::from("No terms could be found in model vocabulary"))),
                "1" => SearchResult::Error(Json(String::from("No results"))),
                "2" => SearchResult::Error(Json(String::from("Error processing query"))),
                "3" => SearchResult::Error(Json(String::from("Indexing error"))),
                "4" => SearchResult::Error(Json(String::from("Issue with embedding script"))),
                "5" => SearchResult::Error(Json(String::from("Issue with clustering script"))),
                _ => SearchResult::Error(Json(String::from("Unknown error")))
            }
        }
    }
}

// CORS Fairing
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", 9797))) 
        .attach(CORS)
        .mount("/search", routes![fill, get_results, options])
}



