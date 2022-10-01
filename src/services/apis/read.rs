use log;
use actix_web::{
    HttpResponse, Error, web, http
};
use deadpool_redis::{ Pool };

use super::data::{
    GETResponse
};

use crate::services::db::get;

pub async fn read_record(pool: web::Data<Pool>, req: web::Path<String>) -> Result<HttpResponse, Error> {
    let key: String = req.into_inner();
    log::info!("Read Request Incoming");
    let resp = match get(&pool, &key).await {
        Ok(result) => {
            // redis nao encontrou a chave correta
            if result == None {
                let nil_msg: String = format!("No such key: \'{}\'", key);
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
    return Ok(HttpResponse::Ok().status(http::StatusCode::from_u16(resp.code.try_into().unwrap()).unwrap()).json(resp));
}
