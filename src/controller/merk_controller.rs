use actix_web::{web, HttpResponse, Responder};
use crate::service::merk_service::MerkService;
use deadpool_postgres::Pool;

async fn get_merks() -> impl Responder{
    let message = MerkService::say_merk();
    HttpResponse::Ok().body(message)
}

async fn get_merks_2() -> impl Responder{
    let message = MerkService::say_merk_2();
    HttpResponse::Ok().body(message)
}

async fn get_merks_3(pool: web::Data<Pool>) -> impl Responder{
    MerkService::say_merk_3(pool.get_ref()).await.unwrap_or_else(|_| HttpResponse::InternalServerError().finish())
}

pub fn init_routes(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/merks")
                    .route("", web::get().to(get_merks))
                    .route("/v2",web::get().to(get_merks_2))
                    .route("/v3",web::get().to(get_merks_3))
    );
}