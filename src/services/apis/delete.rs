use log;
use actix_web::{
    HttpResponse, Error, web
};
use deadpool_redis::{ Pool };

use super::data::{
    Response
};

use crate::services::db::delete;

pub async fn delete_record(pool: web::Data<Pool>, req: web::Path<String>) -> Result<HttpResponse, Error> {
    let key: String = req.into_inner();
    log::info!("Delete Request Incoming");
    let resp: Response = match delete(&pool, &key).await {
        Ok(_) => Response::resp_generic("Deleted".to_string()),
        Err(error) => {
            log::error!("{}", error);
            Response::error()
        }
    };
    return Ok(HttpResponse::Ok().json(resp));
}
