use tonic;

use crate::plc;

pub mod activate_adapter;
pub mod control_plc;
pub mod leave_adapter;
pub mod query_plc;
pub mod query_plc_devices;
pub mod query_plc_schema;
pub mod query_plc_types;
pub mod upsert_plc_device;

pub struct PlcServiceRpcClient {
    addr: String,
    client: plc::plc_service_client::PlcServiceClient<tonic::transport::Channel>,
}

impl PlcServiceRpcClient {
    pub fn new(host: &String, port: u16) -> Self {
        let addr = format!("tcp://{}:{}", &host, port);
        let client = plc::plc_service_client::PlcServiceClient::new(
            tonic::transport::Channel::builder(addr.parse().unwrap()).connect_lazy(),
        );
        Self {
            addr: addr,
            client: client,
        }
    }
}
