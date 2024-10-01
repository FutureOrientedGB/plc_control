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

    #[arg(long, default_value = "")]
    pub my_ip: String,

    #[arg(long, default_value = "0.0.0.0")]
    pub host: String,

    #[arg(long, default_value = "50052")]
    pub port: u16,

    #[arg(long, default_value = "localhost")]
    pub service_host: String,

    #[arg(long, default_value = "50051")]
    pub service_port: u16,

    #[arg(long)]
    pub device_type_id: i32,

    #[arg(long)]
    pub device_type_name: String,
}

impl Conf {
    pub fn update(&mut self, name: &str, version: &str, my_ip: String) {
        let default_conf =
            Conf::parse_from(&["--help", "--device-type-id=0", "--device-type-name=a"]);

        self.app_name = name.to_string();
        self.app_version = version.to_string();
        self.toml = self.toml.replace("#app_name#", &name);

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
                        if self.my_ip == default_conf.my_ip {
                            self.my_ip = c.my_ip;
                        }
                        if self.host == default_conf.host {
                            self.host = c.host;
                        }
                        if self.port == default_conf.port {
                            self.port = c.port;
                        }
                        if self.service_host == default_conf.service_host {
                            self.service_host = c.service_host;
                        }
                        if self.service_port == default_conf.service_port {
                            self.service_port = c.service_port;
                        }
                    }
                }
            }
        }

        if self.my_ip == default_conf.my_ip {
            self.my_ip = my_ip;
        }

        self.port = default_conf.port;
        std::fs::write(self.toml.clone(), toml::to_string_pretty(&self).unwrap()).unwrap();

        if self.port == default_conf.port {
            self.port += self.device_type_id as u16 - 1;
        }
    }
}
