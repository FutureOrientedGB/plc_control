use clap::Parser;

pub mod conf;
pub mod log;
pub mod version;

fn main() {
    // parse conf first from file, then from command lines
    let mut conf = conf::Conf::parse();
    conf.update(&version::APP_NAME, &version::APP_VERSION);

    // init log
    log::open_log_file(&conf);

}
