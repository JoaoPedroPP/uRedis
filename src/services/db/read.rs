// extern crate redis;
use deadpool_redis::{redis::cmd, Connection, Pool, PoolError};
// use actix_web::{
//     web
// };

pub async fn get(pool: &Pool, key: &str) -> Result<String, PoolError> {
    println!("GET redis");
    let mut conn: Connection = pool.get().await?;
    let data: String = cmd("GET").arg(&[key]).query_async(&mut conn).await.unwrap_or("Find nothing with this key.".to_string());
    // println!("{:?}", data);
    Ok(data)
}