use actix_web::{web, HttpResponse, Responder};
use crate::service::merk_service::{MerkPostDto, MerkService, MerkPatchDto};
use log::error;
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
    MerkService::get_merk(pool.get_ref()).await.unwrap_or_else(|_| HttpResponse::InternalServerError().finish())
}

async fn post_merks(
    pool: web::Data<Pool>,
    data: web::Json<MerkPostDto>
) -> impl Responder{
    match MerkService::post_merk(pool.get_ref(), data.into_inner()).await {
        Ok(response) => response,
        Err(e) => {
            error!("Failed to post merk: {:?}",e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn patch_merks(
    pool: web::Data<Pool>,
    data: web::Json<MerkPatchDto>
) -> impl Responder{
    MerkService::patch_merk(pool.get_ref(), data.into_inner())
        .await
        .unwrap_or_else(|_| HttpResponse::InternalServerError().finish())
}

async fn delete_merks(
    pool: web::Data<Pool>,
    path: web::Path<String>,
) -> impl Responder{
    let id = path.into_inner();
    MerkService::delete_merk(pool.get_ref(), id)
        .await
        .unwrap_or_else(|_| HttpResponse::InternalServerError().finish())
}

pub fn init_routes(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/merks")
                    .route("", web::get().to(get_merks))
                    .route("/v2",web::get().to(get_merks_2))
                    .route("/v3",web::get().to(get_merks_3))
                    .route("", web::post().to(post_merks))
                    .route("/{id}",web::delete().to(delete_merks))
                    .route("/{id}",web::patch().to(patch_merks))
    );
}