use log;
use actix_web::{
    HttpResponse, Error, web
};
use deadpool_redis::{ Pool };

use super::data::{
    Body,
    Response
};

use crate::services::db::save;
use crate::services::db::cache;

pub async fn save_record(pool: web::Data<Pool>, req: web::Json<Body>) -> Result<HttpResponse, Error> {
    let body: Body = req.clone();
    log::info!("Save Request Incoming");
    let resp: Response = if body.payload == None {
        Response::resp_no_body()
    }
    else {
        let payload: serde_json::Value = serde_json::to_value(&body.payload).unwrap();
        let query = match payload.as_str() {
            Some(value) => value.to_string(),
            None => payload.to_string()
        };
        match save(&pool, &body.key, &query).await {
            Ok(_) => Response::default(),
            Err(error) => {
                log::error!("{}", error);
                Response::error()
            }
        }
    };
    return Ok(HttpResponse::Ok().json(resp));
}

pub async fn cache_record(pool: web::Data<Pool>, req: web::Json<Body>) -> Result<HttpResponse, Error> {
    let body: Body = req.clone();
    log::info!("Cache Request Incoming");
    let resp: Response = if body.payload == None {
        Response::resp_no_body()
    }
    else if body.ttl == None {
        Response::resp_no_ttl()
    }
    else {
        let payload: serde_json::Value = serde_json::to_value(&body.payload).unwrap();
        let query = match payload.as_str() {
            Some(value) => value.to_string(),
            None => payload.to_string()
        };
        match cache(&pool, &body.key, &query, body.ttl.unwrap()).await {
            Ok(_) => Response::default(),
            Err(error) => {
                log::error!("{}", error);
                Response::error()
            }
        }
    };
    return Ok(HttpResponse::Ok().json(resp));
}