use deadpool_redis::{redis::cmd, Connection, Pool, PoolError};

pub async fn save(pool: &Pool, key: &str, payload: &str) -> Result<(), PoolError> {
    log::info!("Set on Redis");
    let mut conn: Connection = pool.get().await?;
    cmd("SET").arg(key).arg(payload).query_async::<_, ()>(&mut conn).await?;
    Ok(())
}