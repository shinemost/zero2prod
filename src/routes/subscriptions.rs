use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::types::Uuid;
use sqlx::PgPool;

use crate::domain::{NewSubscriber, SubscriberName};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// 使用打桩宏简化插桩代码，解耦，插桩代码与业务代码分离
// 必须要放在需要打桩的函数上面
// 每次函数被调用时打桩代码即会执行
#[tracing::instrument(
    name = "Adding a new subscription",
    // 在日志中忽略参数form,pool
    skip(form, pool),
    fields(
subscriber_email = %form.email,
subscriber_name = %form.name
    )
)]

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let new_subscriber = NewSubscriber {
        email: form.0.email,
        name: SubscriberName::parse(form.0.name),
    };
    match insert_subscriber(&pool, &new_subscriber).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
#[tracing::instrument(
    name = "Saving new subscriber details into database",
    skip(new_subscriber, pool)
)]
pub async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        new_subscriber.email,
        new_subscriber.name.inner_ref(),
        Utc::now(),
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
