use std::net::TcpListener;

// 注：集成测试要求main函数以库的形式向外暴露
use ::zero2prod_lib::run;

#[actix_web::test] // 是actix_web::main的测试等价物，可以使用`cargo expand --test health_check`（<- 测试文件名）来看宏生成了哪些代码
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

#[actix_web::test]
async fn subscribe_return_a_200_for_valid_form_data() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=michael%20wang&email=revelationofturing%40gmail.com";
    let response = client
        .post(&format!("{address}/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[actix_web::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=michael%20wang", "email missed"),
        ("email=revelationofturing%40gmail.com", "name missed"),
        ("", "name and email missed"),
    ];

    for (invalid_body, msg) in test_cases {
        let response = client
            .post(&format!("{address}/subscriptions"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect(msg);

        assert_eq!(
            400,
            response.status().as_u16(),
            // 关于测试失败的附加自定义错误消息
            "The API did not fail with 400 Bad Request when the payload was {}.",
            invalid_body
        );
    }
}
