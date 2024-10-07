use futures::stream::StreamExt;

use prost::Message;

use redis::AsyncCommands;

use tracing;

use stdext::function_name;

use super::RedisStore;

use crate::plc;

impl RedisStore {
    pub async fn query_devices(
        &self,
        _request: &plc::QueryPlcDevicesRequest,
    ) -> Vec<plc::PlcDeviceInfo> {
        match self.client.as_ref() {
            None => {
                tracing::error!(message = "redis::Client::as_ref", func = function_name!());
                return vec![];
            }
            Some(client) => match client.get_multiplexed_async_connection().await {
                Err(e) => {
                    tracing::error!(
                        message = "redis::Client::get_multiplexed_async_connection",
                        func = function_name!(),
                        error = e.to_string()
                    );
                    return vec![];
                }
                Ok(mut connection) => {
                    let device_bytes: Vec<(String, Vec<u8>)> =
                        match connection.hscan(&self.key_hash_devices).await {
                            Err(e) => {
                                tracing::error!(
                                    message = "redis::hscan",
                                    func = function_name!(),
                                    error = e.to_string()
                                );
                                vec![]
                            }
                            Ok(data) => data.collect().await,
                        };

                    let mut devices = Vec::<plc::PlcDeviceInfo>::default();
                    for (_device_id, device_bytes) in device_bytes.iter() {
                        if let Ok(device_info) = plc::PlcDeviceInfo::decode(&device_bytes[..]) {
                            devices.push(device_info);
                        }
                    }

                    return devices;
                }
            },
        }
    }
}
