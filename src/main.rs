
// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}", name)
// }

use test_zero2prod;

#[actix_web::main]
// #[tokio::main]
async fn main() -> std::io::Result<()> {
    test_zero2prod::run().await
}
