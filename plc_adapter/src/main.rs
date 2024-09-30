use clap::Parser;

use tokio;

use tonic;

use plc_log::open_log_file;
use plc_proto::plc;

pub mod conf;
pub mod rpc;
pub mod version;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse conf first from file, then from command lines
    let mut conf = conf::Conf::parse();
    conf.update(&version::APP_NAME, &version::APP_VERSION);
    let addr = format!("{}:{}", &conf.host, &conf.port);

    // init log
    open_log_file(&conf.app_name, &conf.app_version, conf.port);

    // validate device type
    if plc::PlcDeviceTypeId::try_from(conf.device_type_id).is_err()
        || plc::PlcDeviceTypeId::from_str_name(&conf.device_type_name).is_none()
    {
        let mut device_types = vec![];
        for typd_id in plc::PlcDeviceTypeId::Begin as i32 + 1..plc::PlcDeviceTypeId::End as i32 {
            device_types.push(format!(
                "    --device-type-id={} --device-type-name={}",
                typd_id,
                plc::PlcDeviceTypeId::try_from(typd_id)
                    .unwrap()
                    .as_str_name()
                    .to_string(),
            ));
        }
        eprintln!("available device types: \n{}", device_types.join("\n"));
        tracing::error!("available device types: \n{}", device_types.join("\n"))
    } else {
        // init rpc service
        let plc_adapter = rpc::handler::MyPlcAdapter::default();

        // serve rpc
        tonic::transport::Server::builder()
            .add_service(plc::plc_adapter_server::PlcAdapterServer::new(plc_adapter))
            .serve(addr.parse().unwrap())
            .await?;
    }

    Ok(())
}
