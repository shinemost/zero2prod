use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
async fn health_check(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}
pub async fn run() -> std::io::Result<()> {
    // server会一直监听请求，处理到达的请求，用于不会自动关闭或者完成
    HttpServer::new(|| { App::new()
        .route("/health_check", web::get().to(health_check))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
// curl -i http://127.0.0.1:8000/health_check