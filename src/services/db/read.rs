use log;
use deadpool_redis::{redis::cmd, Connection, Pool, PoolError};

pub async fn get(pool: &Pool, key: &str) -> Result<String, PoolError> {
    log:info!("Query on Redis");
    let mut conn: Connection = pool.get().await?;
    let data: String = cmd("GET").arg(&[key]).query_async(&mut conn).await.unwrap_or("Find nothing with this key.".to_string());
    Ok(data)
}