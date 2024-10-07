use serde;

use clap::Parser;

use toml;

use tracing;

#[derive(Clone, Debug, Parser, serde::Serialize, serde::Deserialize)]
#[command(version, about, long_about = None)]
pub struct Conf {
    // auto fill
    #[arg(long, default_value = "#app_name#")]
    pub app_name: String,

    // auto fill
    #[arg(long, default_value = "#app_version#")]
    pub app_version: String,

    // auto replace
    #[arg(long, default_value = "#app_name#.toml")]
    pub toml: String,

    #[arg(long, default_value = "0.0.0.0")]
    pub host: String,

    #[arg(long, default_value = "50051")]
    pub port: u16,

    // redis:://{user}:{password}@{host}:{port}
    // unix://{sock_path}?pass={password}
    #[arg(long, default_value = "redis://:@localhost:6379")]
    pub redis_url: String,

    // auto replace
    #[arg(long, default_value = "#app_name#:hash_devices")]
    pub redis_key_hash_devices: String,

    // auto replace
    #[arg(long, default_value = "#app_name#:hash_device_type_id")]
    pub redis_key_hash_device_type_id: String,

    // auto replace
    #[arg(long, default_value = "#app_name#:hash_device_type_name")]
    pub redis_key_hash_device_type_name: String,

    // auto replace
    #[arg(long, default_value = "#app_name#:zset_device_type_heartbeat")]
    pub redis_key_zset_device_type_heartbeat: String,
}

impl Conf {
    pub fn update(&mut self, name: &str, version: &str) {
        let default_conf = Conf::parse_from(&["--help"]);

        self.app_name = name.to_string();
        self.app_version = version.to_string();
        self.toml = self.toml.replace("#app_name#", &name);
        self.redis_key_hash_devices = self.redis_key_hash_devices.replace("#app_name#", &name);
        self.redis_key_hash_device_type_id = self.redis_key_hash_device_type_id.replace("#app_name#", &name);
        self.redis_key_hash_device_type_name = self.redis_key_hash_device_type_name.replace("#app_name#", &name);
        self.redis_key_zset_device_type_heartbeat = self.redis_key_zset_device_type_heartbeat.replace("#app_name#", &name);

        if std::path::Path::new(&self.toml).exists() {
            if let Ok(content) = std::fs::read_to_string(&self.toml) {
                match toml::from_str::<Conf>(&content) {
                    Err(e) => {
                        tracing::error!(
                            message = "toml::from_str error",
                            path = self.toml,
                            text = content,
                            error = e.to_string(),
                        );
                    }
                    Ok(c) => {
                        if self.host == default_conf.host {
                            self.host = c.host;
                        }
                        if self.port == default_conf.port {
                            self.port = c.port;
                        }
                    }
                }
            }
        }

        std::fs::write(self.toml.clone(), toml::to_string_pretty(&self).unwrap()).unwrap();
    }
}
