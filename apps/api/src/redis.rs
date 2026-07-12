use redis::aio::ConnectionManager;
use redis::AsyncCommands;

pub type RedisPool = ConnectionManager;

pub async fn create_pool(redis_url: &str) -> anyhow::Result<RedisPool> {
    let client = redis::Client::open(redis_url)?;
    let manager = ConnectionManager::new(client).await?;
    Ok(manager)
}

pub async fn cache_set(
    conn: &mut RedisPool,
    key: &str,
    value: &str,
    ttl_secs: u64,
) -> anyhow::Result<()> {
    conn.set_ex::<_, _, ()>(key, value, ttl_secs).await?;
    Ok(())
}

pub async fn cache_get(conn: &mut RedisPool, key: &str) -> anyhow::Result<Option<String>> {
    let value: Option<String> = conn.get(key).await?;
    Ok(value)
}

pub async fn cache_del(conn: &mut RedisPool, key: &str) -> anyhow::Result<()> {
    conn.del::<_, ()>(key).await?;
    Ok(())
}
