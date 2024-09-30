use tonic;

use tracing;

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
    client: Option<plc::plc_service_client::PlcServiceClient<tonic::transport::Channel>>,
}

impl PlcServiceRpcClient {
    pub fn new(host: &String, port: u16) -> Self {
        Self {
            addr: format!("tcp://{}:{}", &host, port),
            client: None,
        }
    }

    pub async fn connect(&mut self) {
        match tonic::transport::Channel::builder(self.addr.parse().unwrap())
            .connect()
            .await
        {
            Err(e) => {
                tracing::error!(
                    "tonic::transport::Channel::connect error, addr: {}, e: {}",
                    &self.addr,
                    e
                );
            }
            Ok(channel) => {
                self.client = Some(plc::plc_service_client::PlcServiceClient::new(channel))
            }
        }
    }
}
