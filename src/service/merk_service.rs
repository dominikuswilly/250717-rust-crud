use actix_web::{error::{self, ErrorInternalServerError}, HttpResponse, Error, ResponseError};
use serde::{Serialize, Deserialize};
use deadpool_postgres::Pool;
use tokio_postgres::{Row, error::SqlState};
use log::{info,error};
use std::fmt;
use std::error::Error as StdError;

pub struct MerkService;

#[derive(Serialize)]
pub struct MerkData{
    c_nm: String,
    b_enabled: i32,
    c_alias: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct MerkPostDto {
    pub c_nm: String,
    pub b_enabled: i32,
    pub c_alias: String,
}

#[derive(Deserialize, Debug)]
pub struct MerkPatchDto{
    pub c_nm: String,
    pub b_enabled: i32,
    pub c_alias: String,
}

#[derive(Serialize)]
struct MyResponse<T>{
    message: String,
    data: Option<T>,
}

#[derive(Serialize)]
struct ErrorResponse{
    message: String,
}

#[derive(Debug)]
struct MyError{
    cause: String,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cause)
    }
}

impl StdError for MyError {}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        let err_response = ErrorResponse {
            message: self.cause.clone(),
        };
        HttpResponse::InternalServerError().json(err_response)
    }
}

impl MerkService{
    pub fn say_merk() -> &'static str{
        "hello merk"
    }

    pub fn say_merk_2() -> & 'static str{
        "hello merk 2"
    }

    pub async fn get_merk(pool: &Pool) -> Result<HttpResponse, actix_web::Error> {
        let client = pool.get().await.map_err(|e| {
            ErrorInternalServerError(e)
        })?;

        let rows: Vec<Row> = client.query("SELECT c_nm, b_enabled, c_alias FROM merk_master", &[])
            .await
            .map_err(|e| ErrorInternalServerError(e))?;

        let merk_list: Vec<MerkData> = rows.iter().map(|row| MerkData {
            c_nm: row.get("c_nm"),
            b_enabled: row.get("b_enabled"),
            c_alias: row.get("c_alias"),
        }).collect();

        let json_response = serde_json::to_string(&merk_list)
            .map_err(|e| ErrorInternalServerError(e))?;

        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(json_response))
    }

    pub async fn delete_merk(pool: &Pool, id: String) -> Result<HttpResponse, actix_web::Error> {
        let client = pool.get().await.map_err(ErrorInternalServerError)?;
        let result = client.execute("DELETE FROM merk_master WHERE c_nm = $1", &[&id])
            .await.map_err(error::ErrorInternalServerError)?;
        if result == 0 {
            Ok(HttpResponse::NotFound().body("Merk not found"))
        } else {
            Ok(HttpResponse::Ok().body("Merk deleted"))
        }
    }

    pub async fn post_merk(pool: &Pool, data: MerkPostDto) -> Result<HttpResponse, actix_web::Error> {
        info!("Received request to post merk: c_nm={}, b_enabled={}, c_alias={}", data.c_nm, data.b_enabled, data.c_alias);

        let client = match pool.get().await{
            Ok(client) => client,
            Err(e) => {
                error!("Failed to get client from pool: {:?}", e);
                return Err(error::ErrorInternalServerError("Database Connection Error"));
            }
        };

        let query_result = client.execute(
            "INSERT INTO merk_master (c_nm, b_enabled, c_alias) VALUES ($1, $2, $3)", 
            &[&data.c_nm, &data.b_enabled, &data.c_alias]
        ).await;

        match query_result{
            Ok(rows_affected) => {
                info!("Successfully Inserted merk. Rows affected: {}", rows_affected);
                let response = MyResponse {
                    message: "Merk berhasil dibuat".to_string(),
                    data: Some(data),
                };
                Ok(HttpResponse::Created().json(response))
            },
            Err(e) =>{
                error!("Failed to insert merk: {:?}", e);

                // Cek apakah error adalah duplicate key violation
                if let Some(db_error) = e.as_db_error() {
                    if db_error.code() == &SqlState::UNIQUE_VIOLATION {
                        // Handle duplicate row error khusus di sini, misalnya return 409 Conflict
                        let response = MyResponse::<MerkPostDto>{
                            message: "Merk dengan data yang sama sudah ada".to_string(),
                            data: None,
                        };
                        return Ok(HttpResponse::Conflict().json(response));
                    }
                }

                // Err(error::ErrorInternalServerError("Failed to insert merk"))
                Err(MyError { cause: e.to_string() }.into())
            }
        }
    }

    pub async fn patch_merk(pool: &Pool, data: MerkPatchDto) -> Result<HttpResponse, Error>{
        info!("Mulai patch merk dengan data: {:?}", data);

        let client = match pool.get().await {
            Ok(c) => {
                info!("Berhasil ambil client dari pool");
                c
            },
            Err(e) => {
                error!("Gagal ambil client dari pool: {:?}",e);
                return Err(ErrorInternalServerError(e));
            }
        };

        if let Err(e) = client.execute(
            "UPDATE merk_master SET b_enabled = $1, c_alias = $2 WHERE c_nm = $3", 
            &[&data.b_enabled, &data.c_alias, &data.c_nm]
        ).await{
            error!("Gagal update merk: {:?}", e);
            return Err(ErrorInternalServerError(e));
        }

        info!("Berhasil patch merk dengan nama: {}", data.c_nm);
        Ok(HttpResponse::Created().body("Merk patched"))
    }
}