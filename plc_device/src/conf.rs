use serde;

use structopt::StructOpt;

use toml;

use tracing;

#[derive(Clone, Debug, StructOpt, serde::Serialize, serde::Deserialize)]
pub struct Conf {
    // auto fill
    #[structopt(long, default_value = "#app_name#")]
    pub name: String,

    // auto fill
    #[structopt(long, default_value = "#app_version#")]
    pub version: String,

    // auto replace
    #[structopt(long, default_value = "#app_name#.toml")]
    pub toml: String,

    #[structopt(long, default_value = "0.0.0.0")]
    pub host: String,

    #[structopt(long, default_value = "50052")]
    pub port: i32,
}

impl Conf {
    pub fn update(&mut self, name: &str, version: &str) {
        let default_conf = Conf::from_iter(&["--help"]);

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
                        if self.name == default_conf.name {
                            self.name = c.name;
                        }
                        if self.version == default_conf.version {
                            self.version = c.version;
                        }
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
