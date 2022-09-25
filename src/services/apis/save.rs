use log;
use actix_web::{
    HttpResponse, HttpRequest, Error, web, error
};
use deadpool_redis::{ Pool };

use super::data::{
    Body,
    Response
};

use crate::services::db::save;

pub async fn save_record(pool: web::Data<Pool>, req: web::Json<Body>) -> Result<HttpResponse, Error> {
    let body: Body = req.clone();
    log::info!("Save Request Incoming");
    if body.payload == None {
        let resp = Response::resp_no_body();
        return Ok(HttpResponse::Ok().json(resp));
    }
    else {
        let payload: serde_json::Value = serde_json::to_value(&body.payload).unwrap();
        let query = match payload.as_str() {
            Some(value) => value.to_string(),
            None => payload.to_string()
        };
        let resp: Response = match save(&pool, &body.key, &query).await {
            Ok(_) => Response::default(),
            Err(error) => {
                log::error!("{}", error);
                Response::error()
            }
        };
        return Ok(HttpResponse::Ok().json(resp));
    }
}