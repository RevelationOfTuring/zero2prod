use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
#[derive(Deserialize, Debug)]
pub struct FormData {
    email: String,
    name: String,
}

// tracing::instrument过程宏，使得所有subscribe函数中的步骤都在request_span的上下文中，即将subscribe包装在一个跨度中
// #[tracing::instrument]在函数声明处创建了一个跨度，并将所有参数都放入这个跨度的上下文中（即form和pool）
// 注：tracing::instrument对于异步函数插桩也是有效的
#[tracing::instrument(
    // name为跨度自身的日志消息（这个参数如果被忽略，则会使用函数名字）
    name = "Adding a new subscriber",
    // 很多时候我们不希望日志中记录某些参数（如pool），这时就可以显式地指定如何捕获它们——可通过skip指令告诉tracing忽略它们
    // 注：tracing会自动记录显示所有传入跨度的参数，如果不希望在日志中记录某些变量，请使用skip();
    skip(form, pool),
    // 通过field将某些值添加到跨度是上下文中（语法同tracing::info_span!上的语法类似）
    fields(
        // 生成一个随机的请求id，用于将日志和请求关联起来（此处定义request_id会覆盖TracingLogger提供的request_id，所以要注释掉）
        // request_id = %Uuid::new_v4(),
        subscriber_emial = %form.email,
        subscriber_name = %form.name,
    )
)]
// 负责调用流程中所需的子程序，根据HTTP的规则和约定将它们返回的结果转换为请求响应
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    // 生成一个随机的请求id，用于将日志和请求关联起来
    // let request_id = Uuid::new_v4();
    // span和log一样，有一个关联级别
    // 与字符串插值不同，tracing允许我们将结构化信息以键值对方式与span关联起来
    // %作为前缀修饰变量
    // let request_span = tracing::info_span!(
    //     "Adding a new subscriber",
    //     // 隐式：request_id是request_id的key
    //     %request_id,
    //     // 显式：subscriber_name为key，form.name的变量值为该key对应的值
    //     subscriber_emial=%form.email,
    //     subscriber_name=%form.name
    // );
    // let query_span = tracing::info_span!("Saving new subscriber details in the database");

    // 在async函数中使用enter()可能导致灾难性的后果（这里先这么写）
    // 调用.enter()表示激活新创建的跨度request_span。返回值为一个Entered类型的守卫对象，在该守卫被析构前，所有的下跨度都会被注册为当前跨度的子跨度
    // let _request_span_guard = request_span.enter(); // _request_span_guard在'subscrib'结束时析构，此时就退出了这个span
    // 在Cargo.toml找中启用了tracing的log功能。当tracing宏记录一个事件或跨度时，log的记录器可以将其收集起来（这里的记录器是env_logger）

    // 我们不需要手动对这个跨度调用.enter()
    // 后面的.instrument()会在合适的时机根据future的状态（异步的轮询）来调用.enter（即future被轮询时，进入该跨度。而future被挂起时，退出该跨度）
    // let query_span = tracing::info_span!("Saving new subscriber details in the database");

    // tracing::info!(
    //     "request_id {} - Adding '{}' '{}' as a new subscriber",
    //     request_id,
    //     form.email,
    //     form.name
    // );

    // tracing::info!("request_id {request_id} - Saving new subscribe details in the database");

    // $1是传递给query!()的第一个参数
    // match sqlx::query!(
    //     r#"
    //     INSERT INTO subscriptions (id, email, name, subscribed_at)
    //     VALUES ($1, $2, $3, $4)
    //     "#,
    //     Uuid::new_v4(), // 生成一个随机Uuid用作id
    //     form.email,
    //     form.name,
    //     Utc::now() // 使用当前时区的时间戳作为subscribed_at的值
    // )
    // // execute的参数需要是实现Executor trait, 将连接池作为可替换组件
    // // get_ref的作用：从Data<T>返回&T
    // .execute(pool.get_ref())
    // // 首先要绑定query_span这个插桩，然后等待这个future完成
    // .instrument(query_span)
    // .await
    match insert_subscriber(&pool, &form).await {
        Ok(_) => {
            // tracing::info!("request_id {request_id} - New subscriber details have been saved");
            HttpResponse::Ok().finish()
        }
        // 一旦sqlx::query!()失败
        // Err(e) => {
        //     // 日志的读者主要是应用程序的维护人员，应该用std::fmt::Debug格式来输出日志，获取尽可能多的信息
        //     // std::fmt::Display则是用于展示给app的用户的
        //     tracing::error!("Failed to execute query: {:?}", e);
        //     HttpResponse::InternalServerError().finish()
        // }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// 负责数据库逻辑，并不关心web框架。我们并不会把web::Form和web::Data传给它
// 此时insert_subscriber相当于是subscribe的子跨度
#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
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
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
