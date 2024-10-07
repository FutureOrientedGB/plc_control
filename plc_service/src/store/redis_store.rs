use tracing;

use stdext::function_name;

use crate::conf::Conf;

#[derive(Debug, Default)]
pub struct RedisStore {
    pub client: Option<redis::Client>,

    pub url: String,

    // hash { device_id -> device }
    pub key_hash_devices: String,

    // hash { device_type_id -> tcp://rpc_host:rpc_port }
    pub key_hash_device_type_id: String,
    // hash { device_type_name -> device_type_id }
    pub key_hash_device_type_name: String,
    // zset { (device_type_id, ts) }
    pub key_zset_device_type_heartbeat: String,
}

impl RedisStore {
    pub fn new(conf: Conf) -> Self {
        RedisStore {
            client: None,
            url: conf.redis_url,
            key_hash_devices: conf.redis_key_hash_devices,
            key_hash_device_type_id: conf.redis_key_hash_device_type_id,
            key_hash_device_type_name: conf.redis_key_hash_device_type_name,
            key_zset_device_type_heartbeat: conf.redis_key_zset_device_type_heartbeat,
        }
    }

    pub fn open(&mut self) -> bool {
        // connect to redis
        match redis::Client::open(self.url.clone()) {
            Ok(client) => {
                self.client = Some(client);
                return true;
            }
            Err(e) => {
                tracing::error!(
                    message = "redis::Client::open",
                    func = function_name!(),
                    redis_url = self.url,
                    error = e.to_string(),
                );
                return false;
            }
        }
    }
}
