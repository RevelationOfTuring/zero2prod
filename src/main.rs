// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}", name)
// }

use zero2prod_lib::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
