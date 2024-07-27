extern crate redis;
use rocket::http::{Header, Status};
use rocket::{Request, Response, routes};
use rocket::response::{Responder, Result};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::serde::json::Json;
use rocket::{get, post, options, launch};
use crate::services::{fill_indices, get_search_results, DocumentResult};
//use serde_json::{Value, json};
//use crate::parser::Document;
use crate::auth::{authenticate, Credentials, SearchHistoryResponse, make_registration, update_history};
use crate::config::Config;
use crate::meta::*;


// Corrected OPTIONS handler
#[options("/<_..>")]
fn options() -> &'static str {
    "OK"
}

// TO DO: enable setting by input search modification (hence be POST request).
#[get("/fill")]
async fn fill() {
    let crawl_depth: u8 = 1;
    let seed_count: u8  = 30;
    fill_indices(crawl_depth, seed_count).await;
}

pub enum SearchResult {
    Documents(Json<DocumentResult>),
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


#[post("/get-results", data = "<query>")]
pub fn get_results(query: String) -> SearchResult {
    match get_search_results(query) {
        Ok(results) => SearchResult::Documents(Json(results)),
        Err(e) => {
            match e.as_str() {
                "-2" => SearchResult::Error(Json(String::from("Unspecified error"))),
                "-1" => SearchResult::Error(Json(String::from("No terms could be found in model vocabulary"))),
                "1"  => SearchResult::Error(Json(String::from("No results"))),
                "2"  => SearchResult::Error(Json(String::from("Error processing query"))),
                "3"  => SearchResult::Error(Json(String::from("Indexing error"))),
                "4"  => SearchResult::Error(Json(String::from("Issue with embedding script"))),
                "5"  => SearchResult::Error(Json(String::from("Issue with clustering script"))),
                 _   => SearchResult::Error(Json(String::from("Unknown error")))
            }
        }
    }
}


// TO DO: specify to return (i) unspecified error (for now) (ii) user does not exist (iii)
// successful login with included user session object (also will include search history)
pub enum AuthResult {
    Error(Json<String>),   // Unspecified error case OR user does not exist.
    LoginConfirm(Json<SearchHistoryResponse>), // Confirmation object for login with username & search history (later to be modified with a session token).
    RegisterConfirm(Json<()>),
    UpdateConfirm(Json<()>)
}

impl<'r> Responder<'r, 'static> for AuthResult {
    fn respond_to(self, request: &'r Request<'_>) -> Result<'static> {
        match self {
            AuthResult::Error(err) => Response::build_from(err.respond_to(request)?)
                .status(Status::InternalServerError)
                .ok(),
            AuthResult::LoginConfirm(conf) => Response::build_from(conf.respond_to(request)?)
                .status(Status::Ok)
                .ok(),
            AuthResult::RegisterConfirm(conf) => Response::build_from(conf.respond_to(request)?)
                .status(Status::Ok)
                .ok(),
            AuthResult::UpdateConfirm(conf) => Response::build_from(conf.respond_to(request)?)
                .status(Status::Ok)
                .ok(),
        }
    }
}

#[post("/login", data="<credentials>")]
pub fn login(credentials: Json<Credentials>) -> AuthResult {
    match authenticate(&credentials) {
        Ok(response) => {
            return AuthResult::LoginConfirm(Json(response))
        }
        Err(e) => return AuthResult::Error(Json(e.to_string()))
    }
}

#[post("/register", data="<credentials>")]
pub fn register(credentials: Json<Credentials>) -> AuthResult {
    match make_registration(&credentials) {
        Ok(response) => return AuthResult::RegisterConfirm(Json(response)),
        Err(e) => {
            println!("{:?}", e.to_string());
            return AuthResult::Error(Json(e.to_string()))
        }
    }
}

// TO DO: IMPLEMENT WITH REDIS SETTING VALUES.
#[post("/add-history", data="<credentials>")]
pub fn add_history(credentials: Json<Credentials>) -> AuthResult {
    match update_history(&credentials) {
        Ok(response) => return AuthResult::UpdateConfirm(Json(response)),
        Err(e) => return AuthResult::Error(Json(e.to_string()))
    }
}

pub enum ConfigResult {
    WriteError(Json<String>),
    WriteSuccess(Json<()>),
    ReadError(Json<String>),
    ReadSuccess(Json<Config>)
}

impl<'r> Responder<'r, 'static> for ConfigResult {
    fn respond_to(self, request: &'r Request<'_>) -> Result<'static> {
        match self {
            ConfigResult::WriteError(err) => Response::build_from(err.respond_to(request)?)
                .status(Status::InternalServerError)
                .ok(),
            ConfigResult::WriteSuccess(conf) => Response::build_from(conf.respond_to(request)?)
                .status(Status::Ok)
                .ok(),
            ConfigResult::ReadSuccess(conf) => Response::build_from(conf.respond_to(request)?)
                .status(Status::Ok)
                .ok(),
            ConfigResult::ReadError(conf) => Response::build_from(conf.respond_to(request)?)
                .status(Status::InternalServerError)
                .ok(),
        }
    }
}


#[post("/write", data="<config>")]
pub fn write(config: Json<Config>) -> ConfigResult {
    let config: Config = config.into_inner();
    match config.write() {
        Ok(()) => return ConfigResult::WriteSuccess(Json(())),
        Err(e) => return ConfigResult::WriteError(Json(e.to_string())),
    }
}


#[post("/read", data="<config>")]
pub fn read(config: Json<Config>) -> ConfigResult {
    let mut config: Config = config.into_inner();
    match config.read() {
        Ok(()) => return ConfigResult::ReadSuccess(Json(config)),
        Err(e) => return ConfigResult::ReadError(Json(e.to_string())),
    }
}


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
        .mount("/auth", routes![login, register, add_history, options])
        .mount("/config", routes![write, read, options])
}



