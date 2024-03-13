extern crate rocket;

mod crawl;
mod api;
mod services;
mod rank;
mod parser;
mod index;
mod discover;

use crate::api::rocket;
use tokio::runtime::Runtime;

fn main() {
    call_api();
}


#[rocket::main]
async fn call_api() {
    rocket().launch().await;
}
