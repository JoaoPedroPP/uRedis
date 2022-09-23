// extern crate redis;
use deadpool_redis::{redis::cmd, Connection, Pool, PoolError};
// use actix_web::{
//     web
// };

pub async fn save(pool: &Pool/*redis_url: String, key: &str, payload: Schema*/) -> Result<String, PoolError> /*-> Result<(), String>*/ {
    println!("Save redis");
    let mut conn: Connection = pool.get().await?;
    let pong: String = cmd("PING").query_async(&mut conn).await?;
    println!("{:?}", pong);
    Ok(pong)
    // let client = redis::Client::open(redis_url).unwrap();
    // let mut con = client.get_connection().unwrap();
    // let data: String = serde_json::to_string(&payload).unwrap();

    // let cache = con.set::<&str, String, String>(key, data);
    // println!("{:?}", cache);
    // Ok(())
}