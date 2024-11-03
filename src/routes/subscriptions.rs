use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::Instrument;
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
    // 在Cargo.toml找中启用了tracing的log功能。当tracing宏记录一个事件或跨度时，log的记录器可以将其收集起来（这里的记录器是env_logger）
    // tracing::info!(
    //     "request_id {} - Adding '{}' '{}' as a new subscriber",
    //     request_id,
    //     form.email,
    //     form.name
    // );

    // span和log一样，有一个关联级别
    // 与字符串插值不同，tracing允许我们将结构化信息以键值对方式与span关联起来
    // %作为前缀修饰变量
    let request_span = tracing::info_span!(
        "Adding a new subscriber",
        // 隐式：request_id是request_id的key
        %request_id,
        // 显式：form.email是subscriber_emial的key
        subscriber_emial=%form.email,
        subscriber_name=%form.name
    );

    // 在async函数中使用enter可能导致灾难性的后果（这里先这么写）
    // 调用.enter()表示激活新创建的跨度request_span。返回值为一个Entered类型的守卫对象，在该守卫被析构前，所有的下跨度都会被注册为当前跨度的子跨度
    let _request_span_guard = request_span.enter(); // _request_span_guard在'subscrib'结束时析构，此时就退出了这个span
    tracing::info!("request_id {request_id} - Saving new subscribe details in the database");

    // 我们不需要手动对这个跨度调用.enter()
    // 后面的.instrument()会在合适的时机根据future的状态来调用.enter
    let query_span = tracing::info_span!("Saving new subscriber details in the database");

    // $1是传递给query!()的第一个参数
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
    // 绑定要绑定query_span这个插桩，然后等待这个future完成
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!("request_id {request_id} - New subscriber details have been saved");
            HttpResponse::Ok().finish()
        }
        // 一旦sqlx::query!()失败
        Err(e) => {
            // 日志的读者主要是应用程序的维护人员，应该用std::fmt::Debug格式来输出日志，获取尽可能多的信息
            // std::fmt::Display则是用于展示给app的用户的
            tracing::error!("request_id {request_id} - Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
