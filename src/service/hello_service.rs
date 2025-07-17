use actix_web::{HttpResponse,Responder};
use crate::dto::hello::hello_dto::{GetHelloDto};

pub struct HelloService;

impl HelloService{
    pub fn say_hello() -> &'static str{
        "hello world"
    }

    pub fn say_hello_2() -> & 'static str{
        "hello world 2"
    }

    pub fn say_hello_3() -> impl Responder {
        let hello: GetHelloDto = GetHelloDto {
            c_name : "a",
            c_alias : "b",
            b_enabled : 1,        
        };

        HttpResponse::Ok().json(hello)
    }

}


