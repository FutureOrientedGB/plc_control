use structopt::StructOpt;

use tokio;

use tonic;

use plc_proto::plc;

pub mod conf;
pub mod log;
pub mod rpc;
pub mod version;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse conf first from file, then from command lines
    let name = "plc_service";
    let app = conf::Conf::clap().name(name).version(version::GIT_COMMIT_VERSION);
    let mut conf = conf::Conf::from_clap(&app.get_matches());
    conf.update(&name, &version::GIT_COMMIT_VERSION);
    let addr = format!("{}:{}", &conf.host, &conf.port);

    // init log
    log::open_log_file(&conf);

    // init rpc service
    let plc_service = rpc::handler::MyPlcService::default();

    // serve rpc
    tonic::transport::Server::builder()
        .add_service(plc::plc_service_server::PlcServiceServer::new(plc_service))
        .serve(addr.parse().unwrap())
        .await?;

    Ok(())
}
