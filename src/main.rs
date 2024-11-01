// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}", name)
// }

use std::net::TcpListener;

use zero2prod_lib::{configuration::get_configuration, startup::run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 从配置文件读配置
    let conf = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", conf.application_port);
    let listener = TcpListener::bind(&address)?;
    run(listener)?.await
}
