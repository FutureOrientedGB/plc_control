use structopt::StructOpt;

pub mod conf;
pub mod log;

fn main() {
    // parse conf first from file, then from command lines
    let name = "plc_client";
    let version = "773aa65.20240929.162250"; // update by build.rs
    let app = conf::Conf::clap().name(name).version(version);
    let mut conf = conf::Conf::from_clap(&app.get_matches());
    conf.update(&name, &version);

    // init log
    log::open_log_file(&conf);

}
