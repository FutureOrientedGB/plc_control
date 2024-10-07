use tracing;

use stdext::function_name;

use super::RedisStore;
use plc_proto::plc;

impl RedisStore {
    pub async fn find_adapter(&self, device_type: plc::DeviceType) -> String {
        match self.client.as_ref() {
            None => {
                tracing::error!(func = function_name!(), message = "redis::Client::as_ref");
                return String::new();
            }
            Some(client) => match client.get_multiplexed_async_connection().await {
                Err(e) => {
                    tracing::error!(
                        message = "redis::Client::get_multiplexed_async_connection",
                        func = function_name!(),
                        error = e.to_string()
                    );
                    return String::new();
                }
                Ok(mut connection) => {
                    let result: redis::RedisResult<Vec<Option<String>>> = redis::pipe()
                        .hget(&self.key_hash_device_type_id, device_type.id)
                        .hget(&self.key_hash_device_type_name, &device_type.name)
                        .zscore(
                            &self.key_zset_device_type_heartbeat,
                            format!("{}:{}", &device_type.name, device_type.id),
                        )
                        .query_async(&mut connection)
                        .await;

                    match result {
                        Err(e) => {
                            tracing::error!(
                                message = "redis::pipe HGET+HGET",
                                func = function_name!(),
                                error = e.to_string(),
                                device_type_id = device_type.id,
                                device_type_name = device_type.name,
                            );
                            return String::new();
                        }
                        Ok(data) => {
                            let (mut addr_log, mut type_id_log) = (String::new(), String::new());
                            if let [rpc_addr, type_id, last_heartbeat, ..] = data.as_slice() {
                                let mut errors = vec![];
                                match rpc_addr {
                                    None => {
                                        errors.push("adapter rpc addr not found");
                                    }
                                    Some(addr) => {
                                        addr_log = addr.clone();
                                        match type_id {
                                            None => {
                                                errors.push("adapter device type id not found");
                                            }
                                            Some(tid) => {
                                                type_id_log = tid.clone();
                                                match tid.parse::<i32>() {
                                                    Err(_) => {
                                                        errors.push(
                                                            "adapter device type id parse error",
                                                        );
                                                    }
                                                    Ok(id) => {
                                                        if id != device_type.id {
                                                            errors.push(
                                                                "adapter device type id mismatch",
                                                            );
                                                        } else {
                                                            let timestamp =
                                                                std::time::SystemTime::now()
                                                                    .duration_since(
                                                                        std::time::UNIX_EPOCH,
                                                                    )
                                                                    .unwrap()
                                                                    .as_secs_f64();
                                                            match last_heartbeat {
                                                                None => {
                                                                    errors.push(
                                                                        "adapter device type id expired",
                                                                    );
                                                                }
                                                                Some(heartbeat) => {
                                                                    match heartbeat.parse::<f64>() {
                                                                        Err(_) => {
                                                                            errors.push("adapter device type id heartbeat error");
                                                                        }
                                                                        Ok(beat) => {
                                                                            if timestamp - beat > 60.0
                                                                            {
                                                                                errors.push("adapter device type id expired");
                                                                            } else {
                                                                                return addr
                                                                                    .clone();
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                if !errors.is_empty() {
                                    tracing::error!(
                                        message = errors.join(", "),
                                        rpc_addr = addr_log,
                                        device_type_id = type_id_log,
                                        req_device_type_id = device_type.id,
                                        req_device_type_name = device_type.name,
                                    );
                                }

                                return String::new();
                            }
                        }
                    }

                    return String::new();
                }
            },
        }
    }
}
