use redis::{aio::Connection, RedisError};

pub async fn get_data_as_json<T>(
    conn: &mut Connection,
    key:  &str,
) -> Result<Option<T>, RedisError>
where
    T: serde::de::DeserializeOwned,
{
    let result: Option<String> = redis::cmd("GET")
        .arg(key)
        .query_async(conn)
        .await?;

    match result {
        Some(json_data) => {
            let data: T = serde_json::from_str(&json_data).unwrap();
            Ok(Some(data))
        }
        None => Ok(None)
    }
}

pub async fn store_data_as_json<T>(
    conn: &mut Connection,
    key:  &str,
    data: &T,
) -> Result<(), RedisError>
where
    T: serde::Serialize,
{
    let json_data = serde_json::to_string(data).unwrap();

    redis::cmd("SET")
        .arg(key)
        .arg(json_data)
        .query_async(conn)
        .await?;

    Ok(())
}