use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::types::Uuid;
use sqlx::PgPool;
use tracing::Instrument;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    // info_span宏会创建一个info级别的跨度
    let request_span = tracing::info_span!(
        "Adding subscribe to request",
        %request_id,
        subscription_name = %form.name,
        subscriber_email = %form.email
    );
    // 激活request_span
    // 离开作用域会调用析构函数Drop
    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!("Saving new subscriber details into database");
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(pool.get_ref())
    // 绑定这个插桩，然后等待这个future完成
    .instrument(query_span)
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            // 这条错误日志不在query_span中
            tracing::error!(
                "request_id {} - Failed to execute query: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}
// curl -i -X POST -d 'email=thomas_mann@hotmail.com&name=Tom' http://127.0.0.1:8000/subscriptions
