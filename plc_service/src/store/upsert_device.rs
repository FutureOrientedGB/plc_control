use prost::Message;

use redis::{AsyncCommands, RedisResult};

use tracing;

use stdext::function_name;

use super::RedisStore;

use crate::plc;

impl RedisStore {
    pub async fn upsert_device(&self, request: &plc::UpsertPlcDeviceRequest) -> bool {
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
                Ok(mut connection) => match &request.info {
                    None => {
                        tracing::error!(message = "info field missing", func = function_name!(),);
                        return false;
                    }
                    Some(info) => {
                        let mut buf = Vec::new();
                        info.encode(&mut buf).unwrap();
                        let result: RedisResult<u32> = connection
                            .hset(&self.key_hash_devices, info.id.clone(), buf)
                            .await;
                        match result {
                            Err(e) => {
                                tracing::error!(
                                    message = "hset plc device error",
                                    func = function_name!(),
                                    error = e.to_string()
                                );
                                return false;
                            }
                            Ok(_) => {
                                return true;
                            }
                        }
                    }
                },
            },
        }
    }
}
