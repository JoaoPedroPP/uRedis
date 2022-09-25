use log;
use actix_web::{
    HttpResponse, HttpRequest, Error, web, error
};
use deadpool_redis::{ Pool };

use super::data::{
    Body,
    GETResponse
};
use serde_json;
use crate::services::db::get;

pub async fn read_record(pool: web::Data<Pool>, req: web::Json<Body>) -> Result<HttpResponse, Error> {
    let body: Body = req.clone();
    log::info!("Read Request Incoming");
    // let result: String = get(&pool, &body.key).await.map_err(|pool_error| error::ErrorNotAcceptable(format!("{}", pool_error)))?;
    let resp = match get(&pool, &body.key).await/*.map_err(|pool_error| error::ErrorNotAcceptable(format!("{}", pool_error)))*/ {
        Ok(result) => {
            // redis nao encontrou a chave correta
            if result == None {
                GETResponse::no_payload("No such key".to_string())
            }
            else {
                // redis encontrou a chave
                let x: String = result.unwrap();
                match serde_json::from_str(&x) {
                    Ok(payload) => GETResponse::resp_200(payload), // Caso seja um objeto JSON
                    Err(_) => GETResponse::no_payload(x) // Caso seja uma String
                }
            }
        },
        Err(error) => {
            log::warn!("Error");
            GETResponse::no_payload("No such key".to_string())
        }
    };
    return Ok(HttpResponse::Ok().json(resp));
}
