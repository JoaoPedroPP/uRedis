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
    log::info!("-----------> App Started");
    dotenv::dotenv().ok();
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL not set");
    let manager = Config::from_url(redis_url);
    let pool = manager.create_pool(Some(Runtime::Tokio1)).unwrap();

    HttpServer::new(move || {
        App::new()
            // .app_data(redis_url.clone())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::JsonConfig::default().limit(8192))
            .route("/api/save", web::post().to(services::save_record))
            // .route("/api/read", web::post().to(services::respond))
    })
    .bind("0.0.0.0:8080")?
    .workers(4)
    .run()
    .await
}
// use std::env;

// use actix_web::{error, get, middleware, web, App, Error, HttpResponse, HttpServer};
// use deadpool_redis::{redis::cmd, Config as RedisConfig, Connection, Pool, PoolError, Runtime};

// fn redis_uri() -> String {
//     match env::var("REDIS_URL") {
//         Ok(s) if !s.is_empty() => s,
//         _ => "rediss://ibm_cloud_97301482_35f9_456d_a542_2c032b038573:2fa033279ad7936286feb7d90d75291f5eaa17cbbc8ea20be30b58a89eaaa462@024c1252-4809-48ad-8007-2f19bf90634e.974550db55eb4ec0983f023940bf637f.databases.appdomain.cloud:32272/0".into(),
//     }
// }

// async fn redis_ping(pool: &Pool) -> Result<String, PoolError> {
//     let mut conn: Connection = pool.get().await?;
//     let pong: String = cmd("PING").query_async(&mut conn).await?;

//     Ok(pong)
// }

// #[get("/")]
// async fn index(redis_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
//     let pong = redis_ping(&redis_pool)
//         .await
//         .map_err(|pool_error| error::ErrorNotAcceptable(format!("{}", pool_error)))?;

//     Ok(HttpResponse::Ok().body(format!("Redis PING -> {}", pong)))
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let redis_config = RedisConfig::from_url(redis_uri());
//     let redis_pool = redis_config.create_pool(Some(Runtime::Tokio1)).unwrap();
//     let server_url = "127.0.0.1:8080";

//     let server = HttpServer::new(move || {
//         App::new()
//             .app_data(web::Data::new(redis_pool.clone()))
//             .wrap(middleware::Logger::default())
//             .service(index)
//     })
//     .bind(server_url)?
//     .run();

//     println!(
//         "Server running! Access the index page here: http://{}/",
//         server_url
//     );

//     server.await
// }