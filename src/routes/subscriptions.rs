use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// 可以直接将返回类型定义为HttpResponse
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    // 生成一个随机的请求id，用于将日志和请求关联起来
    let request_id = Uuid::new_v4();
    log::info!(
        "request_id {} - Adding '{}' '{}' as a new subscriber",
        request_id,
        form.email,
        form.name
    );
    log::info!("request_id {request_id} - Saving new subscribe details in the database");

    // $1是传递给query!()的第一次参数
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(), // 生成一个随机Uuid用作id
        form.email,
        form.name,
        Utc::now() // 使用当前时区的时间戳作为subscribed_at的值
    )
    // execute的参数需要是实现Executor trait, 将连接池作为可替换组件
    // get_ref的作用：从Data<T>返回&T
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            log::info!("request_id {request_id} - New subscriber details have been saved");
            HttpResponse::Ok().finish()
        }
        // 一旦sqlx::query!()失败
        Err(e) => {
            // 日志的读者主要是应用程序的维护人员，应该用std::fmt::Debug格式来输出日志，获取尽可能多的信息
            // std::fmt::Display则是用于展示给app的用户的
            log::error!("request_id {request_id} - Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
