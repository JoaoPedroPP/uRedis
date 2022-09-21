use actix_web::{
    HttpResponse, HttpRequest, Error, web
};

use super::data::{
    Body,
    Response
};

pub async fn save_record(ctx: HttpRequest, req: web::Json<Body>) -> Result<HttpResponse, Error> {
    let x: Body = req.clone();
    println!("{:?}", x);
    let resp = Response::default();
    return Ok(HttpResponse::Ok().json(resp));
}
