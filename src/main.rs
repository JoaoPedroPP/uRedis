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
            .service(
                web::scope("/api")
                .route("/save", web::post().to(services::save_record))
                .route("/cache", web::post().to(services::cache_record))
                .route("/read", web::post().to(services::read_record))
                .route("/delete/{key}", web::get().to(services::delete_record))
            )
    })
    .bind("0.0.0.0:8080")?
    .workers(4)
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use log;
    use std::env;
    use serde_json::json;
    use lazy_static::lazy_static;
    use actix_web::{ test, web, App };
    use deadpool_redis::{
        Config,
        Runtime,
        Pool
    };

    lazy_static! {
        static ref APP_STATE: web::Data<Pool> = {
            env_logger::builder().init();
            log::info!("Set App state for test");
            dotenv::dotenv().ok();
            let redis_url = env::var("REDIS_URL").expect("REDIS_URL not set");
            let manager = Config::from_url(redis_url);
            let pool = manager.create_pool(Some(Runtime::Tokio1)).unwrap();
    
            web::Data::new(pool.clone())
        };
    }

    #[actix_web::test]
    async fn test_save_200() {
        let app = test::init_service(App::new().app_data(APP_STATE.clone()).route("/api/save", web::post().to(services::save_record))).await;
        let payload = json!({ "key": "a", "payload": "a string" });
        let req = test::TestRequest::post().uri("/api/save").set_json(payload).to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 201);
    }

    #[actix_web::test]
    async fn test_cache_200() {
        let app = test::init_service(App::new().app_data(APP_STATE.clone()).route("/api/cache", web::post().to(services::cache_record))).await;
        let payload = json!({ "key": "b", "ttl": 10, "payload": "a string" });
        let req = test::TestRequest::post().uri("/api/cache").set_json(payload).to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 201);
    }

    #[actix_web::test]
    async fn test_read_200() {
        let app = test::init_service(App::new().app_data(APP_STATE.clone()).route("/api/read", web::post().to(services::read_record))).await;
        let payload = json!({ "key": "a"});
        let req = test::TestRequest::post().uri("/api/read").set_json(payload).to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
    }

    #[actix_web::test]
    async fn test_delete_200() {
        let app = test::init_service(App::new().app_data(APP_STATE.clone()).route("/api/delete/{key}", web::get().to(services::delete_record))).await;
        let req = test::TestRequest::get().uri("/api/delete/a").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
    }
}