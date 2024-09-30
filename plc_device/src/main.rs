use clap::Parser;

use plc_log::open_log_file;

pub mod conf;
pub mod version;

fn main() {
    // parse conf first from file, then from command lines
    let mut conf = conf::Conf::parse();
    conf.update(&version::APP_NAME, &version::APP_VERSION);

    // init log
    open_log_file(&conf.app_name, &conf.app_version, conf.port);
}
