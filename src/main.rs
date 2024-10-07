use std::net::TcpListener;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind a port");
    run(listener)?.await
}
// curl -v http://127.0.0.1:8000/health_check