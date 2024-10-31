use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};


// impl Responder是为了将返回值转换成HttpResponse类型
async fn health_check(_req: HttpRequest) -> impl Responder {
    // 获取一个以200状态码为基础的HttpResponseBuilder
    //该结构体实现了Responder trait，其可以调用自己的finish()来获得一个具有空响应体的HttpResponse
    HttpResponse::Ok()
    //  HttpResponse::Ok().finish()
}

// run()不再是二进制文件的入口点，因此可以将其标记为async而不需要任何宏
pub async fn run() -> std::io::Result<()>{
HttpServer::new(||{
    App::new()
    .route("/health_check", web::get().to(health_check))
})
.bind(("127.0.0.1",8080))?
.run()
.await
}