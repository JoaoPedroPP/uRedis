use deadpool_redis::{redis::cmd, Connection, Pool, PoolError};

pub async fn delete(pool: &Pool, key: &str) -> Result<(), PoolError> {
    log::info!("Delete on Redis");
    let mut conn: Connection = pool.get().await?;
    cmd("DEL").arg(key).query_async::<_, ()>(&mut conn).await?;
    Ok(())
}
