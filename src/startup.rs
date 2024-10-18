use crate::email_client::EmailClient;
use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
) -> Result<Server, std::io::Error> {
    // 将连接池使用Web::data包装起来，其本质上是一个Arc智能指针
    let db_pool = web::Data::new(db_pool);
    let email_client = web::Data::new(email_client);
    // 通过上下文捕获connection
    let server = HttpServer::new(move || {
        App::new()
            // 注册中间件，此处是日志
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // 将连接注册为应用程序状态的一部分
            // 获得一个指针的副本并将其绑定到应用程序状态
            // 类似于golang里的context透传元数据
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
