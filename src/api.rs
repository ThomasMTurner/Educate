use rocket::http::Header;
use rocket::{Request, Response, routes};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::serde::json::Json;
use rocket::{get, post, options, launch};
use crate::services::{fill_indices, get_search_results};
use crate::parser::Document;
use tokio::runtime::Runtime;

// Corrected OPTIONS handler
#[options("/<_..>")]
fn options() -> &'static str {
    "OK"
}

// Corrected GET route
#[get("/fill")]
async fn fill() {
    fill_indices(1, 30).await;
}

// Corrected POST route
#[post("/get-results", data = "<query>")]
pub fn get_results(query: String) -> Json<Vec<Document>> {
    match get_search_results(query) {
        Ok(results) => Json(results),
        Err(_) => { 
            eprintln!("Could not send HTTP response");
            Json(vec![])
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

