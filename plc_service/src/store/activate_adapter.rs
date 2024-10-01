use tracing;

use stdext::function_name;

use super::RedisStore;

use crate::plc;

impl RedisStore {
    pub async fn activate_adapter(&self, request: &plc::ActivateAdapterRequest) -> bool {
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
                    let device_type = request.r#type.as_ref().unwrap();

                    let timestamp = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs_f64();
                    match redis::pipe()
                        .cmd("HSET")
                        .arg(&self.key_hash_device_type_id)
                        .arg(device_type.id)
                        .arg(format!("tcp://{}:{}", request.rpc_host, request.rpc_port))
                        .ignore()
                        .cmd("HSET")
                        .arg(&self.key_hash_device_type_name)
                        .arg(&device_type.name)
                        .arg(device_type.id)
                        .ignore()
                        .cmd("ZADD")
                        .arg(&self.key_hash_device_type_heartbeat)
                        .arg(timestamp.to_string())
                        .arg(format!("{}:{}", &device_type.name, device_type.id))
                        .ignore()
                        .query_async(&mut connection)
                        .await
                    {
                        Err(e) => {
                            tracing::error!(
                                message = "redis::pipe HSET+HSET+ZADD",
                                func = function_name!(),
                                error = e.to_string(),
                                request = serde_json::to_string(&request).unwrap(),
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
