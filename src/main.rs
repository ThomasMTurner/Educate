extern crate rocket;

mod crawl;
mod api;
mod services;
mod rank;
mod parser;
mod index;
mod discover;
mod auth;
mod config;
mod meta;

use crate::api::rocket;

fn main() {
    call_api();
}


#[rocket::main]
async fn call_api() {
    let _ = rocket().launch().await;
}

