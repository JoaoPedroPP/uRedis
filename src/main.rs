use std::env;
use log;
use actix_web::{
    web,
    App,
    HttpServer
};

use deadpool_redis::{
    Config,
    Runtime
};

mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::builder().init();
    log::info!("App Started");
    dotenv::dotenv().ok();
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL not set");
    let manager = Config::from_url(redis_url);
    let pool = manager.create_pool(Some(Runtime::Tokio1)).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::JsonConfig::default().limit(8192))
            .route("/api/save", web::post().to(services::save_record))
            .route("/api/read", web::post().to(services::read_record))
    })
    .bind("0.0.0.0:8080")?
    .workers(4)
    .run()
    .await
}