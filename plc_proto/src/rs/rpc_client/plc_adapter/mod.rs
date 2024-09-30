use tonic;

use tracing;

use crate::plc;

pub mod control_plc;
pub mod query_plc;

pub struct PlcAdapterRpcClient {
    addr: String,
    client: Option<plc::plc_adapter_client::PlcAdapterClient<tonic::transport::Channel>>,
}

impl PlcAdapterRpcClient {
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
                self.client = Some(plc::plc_adapter_client::PlcAdapterClient::new(channel))
            }
        }
    }
}
