use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::types::Uuid;
use sqlx::PgPool;

use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// 为NewsSubscriber结构体实现TryForm特质
// 完美的解决从FormData转化为NewSubscriber
// 专业的事交给专业的特质去干
// 将意图表达得更加清晰，有助于别人的理解
impl TryFrom<FormData> for NewSubscriber {
    type Error = String;

    fn try_from(form: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(form.name)?;
        let email = SubscriberEmail::parse(form.email)?;
        Ok(NewSubscriber { name, email })
    }
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
    // 此处直接使用try_from
    // 亦可使用TryInto特质提供的try_into方法
    // 详情可查看TryInto特质源码
    // let new_subscriber = match NewSubscriber::try_from(form.0) {
    let new_subscriber = match form.0.try_into() {
        Ok(subscriber) => subscriber,
        Err(_) => return HttpResponse::BadRequest().finish(),
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
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
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
