use serde;

use clap::Parser;

use toml;

use tracing;

#[derive(Clone, Debug, Parser, serde::Serialize, serde::Deserialize)]
pub struct Conf {
    // auto fill
    #[arg(long, default_value = "#app_name#")]
    pub name: String,

    // auto fill
    #[arg(long, default_value = "#app_version#")]
    pub version: String,

    // auto replace
    #[arg(long, default_value = "#app_name#.toml")]
    pub toml: String,

    #[arg(long, default_value = "0.0.0.0")]
    pub host: String,

    #[arg(long, default_value = "50052")]
    pub port: i32,

    #[arg(long, default_value = "localhost")]
    pub service_host: String,

    #[arg(long, default_value = "50051")]
    pub service_port: i32,
}

impl Conf {
    pub fn update(&mut self, name: &str, version: &str) {
        let default_conf = Conf::parse_from(&["--help"]);

        self.name = name.to_string();
        self.version = version.to_string();
        self.toml = self.toml.replace("#app_name#", &name);

        if std::path::Path::new(&self.toml).exists() {
            if let Ok(content) = std::fs::read_to_string(&self.toml) {
                match toml::from_str::<Conf>(&content) {
                    Err(e) => {
                        tracing::error!(
                            "parse toml error, path: {}, text: {}, error: {}",
                            &self.toml,
                            &content,
                            e
                        );
                    }
                    Ok(c) => {
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

        std::fs::write(self.toml.clone(), toml::to_string_pretty(&self).unwrap()).unwrap();
    }
}
