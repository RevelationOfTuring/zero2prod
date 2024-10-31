use std::net::TcpListener;

// 注：集成测试要求main函数以库的形式向外暴露
use ::zero2prod_lib::run;

#[actix_web::test] // 是tokio::main的测试等价物，可以使用`cargo expand --test health_check`（<- 测试文件名）来看宏生成了哪些代码
async fn health_check_works() {
    // 准备，即在后台启动应用
    let address = spawn_app();
    // 使用 reqwest::Client对应用程序执行HTTP请求
    let client = reqwest::Client::new();

    // 执行
    let response = client
        .get(&format!("{address}/health_check"))
        .send()
        .await
        .expect("Failed to execute request");

    // 检查状态码为200
    dbg!(response.status());
    assert!(response.status().is_success());
    // 检查无响应体
    assert_eq!(Some(0), response.content_length());
}

// 定义在后台某处启动应用程序
fn spawn_app() -> String {
    // 尝试绑定端口0将触发操作系统扫描可用端口，即选择一个随机的可用的端口
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // 得到绑定的随机端口号
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");
    // 启动服务器作为后台任务
    let _ = actix_web::rt::spawn(server);
    format!("http://127.0.0.1:{port}")
}
