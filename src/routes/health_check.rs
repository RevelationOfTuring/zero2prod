use actix_web::{HttpRequest, HttpResponse, Responder};

// impl Responder是为了将返回值转换成HttpResponse类型
pub async fn health_check(_req: HttpRequest) -> impl Responder {
    // 获取一个以200状态码为基础的HttpResponseBuilder
    //该结构体实现了Responder trait，其可以调用自己的finish()来获得一个具有空响应体的HttpResponse
    HttpResponse::Ok()
    //  HttpResponse::Ok().finish()
}
