use actix_web::{web, HttpResponse, Responder};
use crate::service::hello_service::HelloService;

async fn get_hello() -> impl Responder{
    let message = HelloService::say_hello();
    HttpResponse::Ok().body(message)
}

async fn get_hello2() -> impl Responder{
    let message = HelloService::say_hello_2();
    HttpResponse::Ok().body(message)
}

async fn get_hello3() -> impl Responder{
    HelloService::say_hello_3()
}

pub fn init_routes(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/hello")
                    .route("", web::get().to(get_hello))
                    .route("/v2", web::get().to(get_hello2))
                    .route("/v3", web::get().to(get_hello3))
    );
}


