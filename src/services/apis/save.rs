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
    let x: Body = req.clone();
    println!("{:?}", x);
    log::info!("-----------> New Save Request Incoming");
    let _ = save(&pool).await.map_err(|pool_error| error::ErrorNotAcceptable(format!("{}", pool_error)))?;
    let resp = Response::default();
    return Ok(HttpResponse::Ok().json(resp));
}
