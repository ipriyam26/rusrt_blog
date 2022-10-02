
use rust_blog::run;
// use actix_web::{web, App, HttpResponse, HttpServer, dev::Server};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run().await?.await
}