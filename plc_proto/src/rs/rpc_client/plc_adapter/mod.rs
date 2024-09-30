use tonic;

use crate::plc;

pub mod control_plc;
pub mod query_plc;

pub struct PlcAdapterRpcClient {
    addr: String,
    client: plc::plc_adapter_client::PlcAdapterClient<tonic::transport::Channel>,
}

impl PlcAdapterRpcClient {
    pub fn new(host: &String, port: u16) -> Self {
        let addr = format!("tcp://{}:{}", &host, port);
        let client = plc::plc_adapter_client::PlcAdapterClient::new(
            tonic::transport::Channel::builder(addr.parse().unwrap()).connect_lazy(),
        );
        Self {
            addr: addr,
            client: client,
        }
    }
}
