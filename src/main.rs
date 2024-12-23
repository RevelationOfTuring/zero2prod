use std::net::TcpListener;

use secrecy::ExposeSecret;
// use env_logger::Env;
use sqlx::PgPool;
use zero2prod_lib::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // actix-web的Logger中间件：只需要使用init方法，由它来调用set_logger
    // 如果环境变量RUST_LOG未被设置，则默认输出所有info及以上级别的日志。例子：RUST_LOG=trace cargo run
    // env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // 从配置文件读配置
    let conf = get_configuration().expect("Failed to read configuration");
    // 创建数据库连接池
    // 注：当&PgPool运行数据库查询时，sqlx将从连接池中借用PgConnection并用他来进行查询；
    // 如果此时没有可用的连接，&PgPool将创建一个新连接或者等待一个空闲的连接
    // let connection_pool = PgPool::connect(
    //     // 数据库url为隐私数据，使用expose_secret暴露其内部数据
    //     &conf.database.connection_string().expose_secret()
    // )
    // .await
    // .expect("Failed to connect to Progres");
    // 推迟在首次启动时建立连接
    let connection_pool = PgPool::connect_lazy(conf.database.connection_string().expose_secret())
        .expect("Failed to create Postgres connection pool.");
    let address = format!("{}:{}", conf.application.host, conf.application.port);
    dbg!(&address);
    let listener = TcpListener::bind(&address)?;
    run(listener, connection_pool)?.await
}
