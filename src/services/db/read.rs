use log;
use deadpool_redis::{redis::cmd, Connection, Pool, PoolError};

pub async fn get(pool: &Pool, key: &str) -> Result<Option<String>, PoolError> {
    log::info!("Query on Redis");
    let mut conn: Connection = pool.get().await?;
    let err_msg: String = format!("No such key: \'{}\'", key);
    // let data2: Option<String> = cmd("GET").arg(&[key]).query_async(&mut conn).await?;
    // println!("{:?}", data2);
    let data: Option<String> = cmd("GET").arg(&[key]).query_async(&mut conn).await?;//.unwrap_or(err_msg);
    Ok(data)
}