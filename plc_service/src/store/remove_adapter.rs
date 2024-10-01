use redis::AsyncCommands;
use tracing;

use stdext::function_name;

use super::RedisStore;

impl RedisStore {
    pub async fn remove_expired_adapter(&self) -> bool {
        match self.client.as_ref() {
            None => {
                tracing::error!(message = "redis::Client::as_ref", func = function_name!());
                return false;
            }
            Some(client) => match client.get_multiplexed_async_connection().await {
                Err(e) => {
                    tracing::error!(
                        message = "redis::Client::get_multiplexed_async_connection",
                        func = function_name!(),
                        error = e.to_string()
                    );
                    return false;
                }
                Ok(mut connection) => {
                    let timestamp = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs_f64() - 60.0;

                    let expired_members: redis::RedisResult<Vec<String>> = connection
                        .zrevrangebyscore(&self.key_hash_device_type_heartbeat, timestamp, 0.0)
                        .await;

                    let mut expired = 0;
                    let mut pipe = redis::pipe();
                    if let Ok(members) = expired_members {
                        expired = members.len();
                        for item in members {
                            if let [name, id, ..] = item
                                .split(":")
                                .map(|v| v.to_string())
                                .collect::<Vec<String>>()
                                .as_slice()
                            {
                                pipe.cmd("HDEL")
                                    .arg(&self.key_hash_device_type_id)
                                    .arg(&id)
                                    .ignore();
                                pipe.cmd("HDEL")
                                    .arg(&self.key_hash_device_type_name)
                                    .arg(&name)
                                    .ignore();
                            }
                        }
                    }
                    if expired > 0 {
                        pipe.cmd("ZREMRANGEBYSCORE")
                            .arg(&self.key_hash_device_type_heartbeat)
                            .arg(0)
                            .arg((timestamp).to_string())
                            .ignore();
                        let _: redis::RedisResult<()> = pipe.query_async(&mut connection).await;

                        tracing::info!(func = function_name!(), expired = expired);
                    }

                    return true;
                }
            },
        }
    }
}
