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

use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};

use crate::routes::{health_check, subscribe};

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
