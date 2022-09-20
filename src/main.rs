use std::env;
use log;
use actix_web::{
    web,
    App,
    HttpServer
};

mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log::info!("App Started");
    dotenv::dotenv().ok();
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL not set");

    HttpServer::new(move || {
        App::new()
            .app_data(redis_url.clone())
            .app_data(web::JsonConfig::default().limit(8192))
            .route("/api/save", web::post().to(services::save_record))
            // .route("/api/read", web::post().to(services::respond))
    })
    .bind("0.0.0.0:8080")?
    .workers(4)
    .run()
    .await
}
