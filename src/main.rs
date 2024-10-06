use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run().await
}
// curl -v http://127.0.0.1:8000/health_check