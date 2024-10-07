use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgConnection;
use std::net::TcpListener;

pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server, std::io::Error> {
    // 将连接包装在一个智能指针中
    let connection = web::Data::new(connection);
    // 通过上下文捕获connection
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // 将连接注册为应用程序状态的一部分
            // 获得一个指针的副本并将其绑定到应用程序状态
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

// curl -v http://127.0.0.1:8000/health_check
