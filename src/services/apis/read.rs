use log;
use actix_web::{
    HttpResponse, Error, web
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
    let resp = match get(&pool, &body.key).await {
        Ok(result) => {
            // redis nao encontrou a chave correta
            if result == None {
                let nil_msg: String = format!("No such key: \'{}\'", body.key);
                GETResponse::no_payload(nil_msg)
            }
            else {
                // redis encontrou a chave
                let x: String = result.unwrap();
                match serde_json::from_str(&x) {
                    Ok(payload) => GETResponse::resp_200(payload), // Caso seja um objeto JSON
                    Err(_) => GETResponse::resp_200_string(x) // Caso seja uma String
                }
            }
        },
        Err(error) => {
            log::error!("{}", error);
            GETResponse::error()
        }
    };
    return Ok(HttpResponse::Ok().json(resp));
}
