use tokio;
use tonic;

use plc_proto::plc;

pub mod rpc;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let plc_service = rpc::handler::MyPlcAdapter::default();

    println!("Plc service running on {}", addr);

    tonic::transport::Server::builder()
        .add_service(plc::plc_adapter_server::PlcAdapterServer::new(plc_service))
        .serve(addr)
        .await?;

    Ok(())
}