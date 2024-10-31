use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}", name)
}

// impl Responder是为了将返回值转换成HttpResponse类型
async fn health_check(_req: HttpRequest) -> impl Responder {
    // 获取一个以200状态码为基础的HttpResponseBuilder
    //该结构体实现了Responder trait，其可以调用自己的finish()来获得一个具有空响应体的HttpResponse
    HttpResponse::Ok()
    //  HttpResponse::Ok().finish()
}

#[actix_web::main]
// #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
