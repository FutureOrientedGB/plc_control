use structopt::StructOpt;

use tokio;

use tonic;

use plc_proto::plc;

pub mod conf;
pub mod log;
pub mod rpc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse conf first from file, then from command lines
    let name = "plc_service";
    let version = ""; // update by build.rs
    let app = conf::Conf::clap().name(name).version(version);
    let mut conf = conf::Conf::from_clap(&app.get_matches());
    conf.update(&name, &version);
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
