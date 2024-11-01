// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}", name)
// }

use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod_lib::{configuration::get_configuration, startup::run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 从配置文件读配置
    let conf = get_configuration().expect("Failed to read configuration");
    // 创建数据库连接池
    // 注：当&PgPool运行数据库查询时，sqlx将从连接池中借用PgConnection并用他来进行查询；
    // 如果此时没有可用的连接，&PgPool将创建一个新连接或者等待一个空闲的连接
    let connection_pool = PgPool::connect(&conf.database.connection_string())
        .await
        .expect("Failed to connect to Progres");
    let address = format!("127.0.0.1:{}", conf.application_port);
    let listener = TcpListener::bind(&address)?;
    run(listener, connection_pool)?.await
}
