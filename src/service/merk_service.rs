use actix_web::HttpResponse;
use serde::Serialize;
use deadpool_postgres::Pool;
use tokio_postgres::Row;

pub struct MerkService;

#[derive(Serialize)]
pub struct MerkData{
    c_nm: String,
    b_enabled: i32,
}

impl MerkService{
    pub fn say_merk() -> &'static str{
        "hello merk"
    }

    pub fn say_merk_2() -> & 'static str{
        "hello merk 2"
    }

    pub async fn say_merk_3(pool: &Pool) -> Result<HttpResponse, actix_web::Error> {
        let client = pool.get().await.map_err(|e| {
            actix_web::error::ErrorInternalServerError(e)
        })?;

        let rows: Vec<Row> = client.query("SELECT c_nm, b_enabled FROM merk_master", &[])
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        let merk_list: Vec<MerkData> = rows.iter().map(|row| MerkData {
            c_nm: row.get("c_nm"),
            b_enabled: row.get("b_enabled"),
        }).collect();

        let json_response = serde_json::to_string(&merk_list)
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(json_response))
    }
}