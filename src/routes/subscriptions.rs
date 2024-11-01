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
        Ok(_) => HttpResponse::Ok().finish(),
        // 一旦sqlx::query!()失败
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
