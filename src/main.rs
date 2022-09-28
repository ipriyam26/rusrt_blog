
use rust_blog::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}