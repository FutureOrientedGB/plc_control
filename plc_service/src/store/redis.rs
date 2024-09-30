use redis::AsyncCommands;

use tracing;




/*
 * hash - 存储plc设备信息
 * {task_hash -> task_info}
 * vfs:tasks:hash {
 *  "1bba6e4362c6021f00683adea703956499877ed3": RedisVfsTaskHashValue
 * }
 * 
 */



pub struct RedisStore {
    client: Option<redis::Client>,

    redis_url: String,

    // hash
    key_task_info: String,
    // hash
    key_consumer_info: String,
    // hash
    key_consumer_to_task: String,
    

}


impl RedisStore {
    fn new() -> Self {
        RedisStore {
            client: None,

            // unix domain socket
            redis_url: vec![
                // schema
                "unix://",
                // path
                "/var/run/redis.sock",
                // password
                "?pass=f02068c11f6fa761122401c52e766b35fb4e0640328d4f164c7078c0d5c74ba1",
            ].join(""),

            key_task_info: String::from("vfs:tasks:hash"),
            key_consumer_info: String::from("vfs:consumers:hash"),
            key_consumer_to_task: String::from("vfs:consumer_to_task:hash"),

        }
    }
}


impl RedisStore {
    pub fn open(&mut self) -> bool {
        // connect to redis
        match redis::Client::open(self.redis_url.clone()) {
            Ok(client) => {
                self.client = Some(client);
                return true;
            },
            Err(e) => {
                tracing::error!("redis::Client::open fail, redis_url: {}, error: {:?}", &self.redis_url, e);
                return false;
            }
        }
    }





}
