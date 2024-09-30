use clap::Parser;

pub mod conf;
pub mod log;
pub mod version;

fn main() {
    // parse conf first from file, then from command lines
    let name = "plc_client";
    let mut conf = conf::Conf::parse();
    conf.update(&name, &version::GIT_COMMIT_VERSION);

    // init log
    log::open_log_file(&conf);

}
