use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

// impl Responder是为了将返回值转换成HttpResponse类型
async fn health_check(_req: HttpRequest) -> impl Responder {
    // 获取一个以200状态码为基础的HttpResponseBuilder
    //该结构体实现了Responder trait，其可以调用自己的finish()来获得一个具有空响应体的HttpResponse
    HttpResponse::Ok()
    //  HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
struct FormData {
    email: String,
    name: String,
}

// 可以直接将返回类型定义为HttpResponse
async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

// run()不再是二进制文件的入口点，因此可以将其标记为async而不需要任何宏
// pub async fn run() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
// }

// // 重构run函数(不再是async函数)，返回一个Server，在外围操作其await（为了集成测试中的后台运行）
// pub fn run() -> Result<Server, std::io::Error> {
//     let server =
//         HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
//             .bind(("127.0.0.1", 8080))?
//             .run();
//     Ok(server)
// }

// 使用TcpListener来绑定端口，这样就可以使用端口0来做集成测试
// 注: 端口0会分配一个可用的随机端口，该端口可以从TcpListener获得
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
