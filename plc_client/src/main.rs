use structopt::StructOpt;

pub mod conf;
pub mod log;
pub mod version;

fn main() {
    // parse conf first from file, then from command lines
    let name = "plc_client";
    let app = conf::Conf::clap().name(name).version(version::GIT_COMMIT_VERSION);
    let mut conf = conf::Conf::from_clap(&app.get_matches());
    conf.update(&name, &version::GIT_COMMIT_VERSION);

    // init log
    log::open_log_file(&conf);

}
