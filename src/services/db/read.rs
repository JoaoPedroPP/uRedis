use log;
use deadpool_redis::{redis::cmd, Connection, Pool, PoolError};

pub async fn get(pool: &Pool, key: &str) -> Result<Option<String>, PoolError> {
    log::info!("Query on Redis");
    let mut conn: Connection = pool.get().await?;
    let data: Option<String> = cmd("GET").arg(&[key]).query_async(&mut conn).await?;
    Ok(data)
}