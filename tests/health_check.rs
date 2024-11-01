use std::net::TcpListener;

use sqlx::{Connection, PgConnection};
// 注：集成测试要求main函数以库的形式向外暴露
use zero2prod_lib::startup::run;

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
    let configuration =
        zero2prod_lib::configuration::get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    // 连接Postgres
    // 注：为了调用PgConnection::connect，必须也导入trait Connection。因为它不是该结构体的内在方法
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");

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

    // 测试postgres连接
    // 注：sqlx::query!返回一个匿名的记录类型。在编译时，在验证查询的有效性后生成结构体定义
    // 每个成员都对应结果中的一个列（如：saved.email对应email列）
    // sqlx在编译时依赖DATABASE_URL环境变量来确定postgres的位置，建议在根目录下增加一个.env
    // sqlxm每次都将从.env文件中读取DATABASE_URL，省去了每次都要导出环境变量的麻烦。
    // 在.env和configuration.yaml同时存数据库连接参数可能让人不爽。但没关系，.env仅与开发过程、构建和测试步骤相关。
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",) // 从subscriptions表中查询email和name列
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription.");

    println!("{:?}", saved);
    assert_eq!(saved.email, "revelationofturing@gmail.com");
    assert_eq!(saved.name, "michael wang");
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
