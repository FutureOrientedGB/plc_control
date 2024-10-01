use clap::Parser;

use plc_log::open_log_file;
use plc_proto::{plc, rs};

pub mod conf;
pub mod version;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse conf first from file, then from command lines
    let mut conf = conf::Conf::parse();
    conf.update(&version::APP_NAME, &version::APP_VERSION);

    // init log
    open_log_file(&conf.app_name, &conf.app_version, conf.service_port);

    // test service api
    let mut service_client = rs::rpc_client::PlcServiceRpcClient::new(&conf.service_host, conf.service_port);
    let resp_query_plc_types = service_client.query_plc_types(plc::QueryPlcTypesRequest {
        version: plc::QueryPlcTypesVersion::QueryPlcTypes20240927.into()
    }).await.unwrap();
    tracing::info!(plc_types = serde_json::to_string(&resp_query_plc_types).unwrap());
    for plc_type in resp_query_plc_types.plc_types {
        let resp_query_plc_schema = service_client.query_plc_schema(plc::QueryPlcSchemaRequest {
            version: plc::QueryPlcSchemaVersion::QueryPlcSchema20240927.into(),
            plc_type: Some(plc_type)
        }).await.unwrap();
        tracing::info!(plc_types = serde_json::to_string(&resp_query_plc_schema).unwrap());
    }

    Ok(())
}
