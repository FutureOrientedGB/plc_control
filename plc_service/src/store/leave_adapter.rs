use tracing;

use stdext::function_name;

use super::RedisStore;

use crate::plc;

impl RedisStore {
    pub async fn leave_adapter(&self, device_type: plc::DeviceType) -> bool {
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
                    match redis::pipe()
                        .hdel(&self.key_hash_device_type_id, device_type.id)
                        .ignore()
                        .hdel(&self.key_hash_device_type_name, &device_type.name)
                        .ignore()
                        .zrem(
                            &self.key_zset_device_type_heartbeat,
                            format!("{}:{}", &device_type.name, device_type.id),
                        )
                        .ignore()
                        .query_async(&mut connection)
                        .await
                    {
                        Err(e) => {
                            tracing::error!(
                                message = "redis::pipe HDEL+HDEL+ZREM",
                                func = function_name!(),
                                error = e.to_string(),
                                device_type_id = device_type.id,
                                device_type_name = device_type.name,
                            );
                            return false;
                        }
                        Ok(()) => {
                            return true;
                        }
                    }
                }
            },
        }
    }
}
