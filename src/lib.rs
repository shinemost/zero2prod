use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};

async fn health_check(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}
pub fn run() -> Result<Server,std::io::Error> {
    // server会一直监听请求，处理到达的请求，用于不会自动关闭或者完成
    let server =HttpServer::new(|| { App::new()
        .route("/health_check", web::get().to(health_check))
    })
        .bind("127.0.0.1:8000")?
        .run();
    Ok(server)

}
// curl -i http://127.0.0.1:8000/health_check