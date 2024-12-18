use crate::configuration::{DatabaseSettings, Settings};
use crate::email_client::EmailClient;
use crate::routes::{confirm, health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

// 这里需要一个包装类型，以便在subscribe处理器中获取URL
// 在actix-web中，从上下文中得到数据的方式是基于类型的：
// 如果使用的是原始的String类型，则可能会由于类型相同而冲突
pub struct ApplicationBaseUrl(pub String);

// 用于保存新构建的服务器及其端口
pub struct Application {
    port: u16,
    server: Server,
}
impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection_pool = get_connection_pool(&configuration.database);
        let sender_email = configuration
            .email_client
            .sender()
            .expect("Invalid sender email address.");
        let timeout = configuration.email_client.timeout();
        let email_client = EmailClient::new(
            configuration.email_client.base_url,
            sender_email,
            configuration.email_client.authorization_token,
            timeout,
        );
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();
        dbg!(configuration.application.base_url.clone());
        let server = run(
            listener,
            connection_pool,
            email_client,
            configuration.application.base_url,
        )?;
        // 将端口保存在新结构体中
        Ok(Self { port, server })
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    // 函数名更精确的表达此函数仅在应用程序停止时返回
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(database: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(database.with_db())
}

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
    base_url: String,
) -> Result<Server, std::io::Error> {
    // 将连接池使用Web::data包装起来，其本质上是一个Arc智能指针
    let db_pool = web::Data::new(db_pool);
    let email_client = web::Data::new(email_client);
    let base_url = web::Data::new(ApplicationBaseUrl(base_url));
    // 通过上下文捕获connection
    let server = HttpServer::new(move || {
        App::new()
            // 注册中间件，此处是日志
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .route("/subscriptions/confirm", web::get().to(confirm))
            // 将连接注册为应用程序状态的一部分
            // 获得一个指针的副本并将其绑定到应用程序状态
            // 类似于golang里的context透传元数据
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
            .app_data(base_url.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
