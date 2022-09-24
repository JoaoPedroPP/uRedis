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
    let result: String = get(&pool, &body.key).await.map_err(|pool_error| error::ErrorNotAcceptable(format!("{}", pool_error)))?;
    let resp: GETResponse = match serde_json::from_str(&result) {
        Ok(payload) => GETResponse::resp_200(payload),
        Err(_) => GETResponse::no_payload(result)
    };
    return Ok(HttpResponse::Ok().json(resp));
}
