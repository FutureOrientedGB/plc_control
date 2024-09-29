use tonic;

use plc_proto::plc::{
    plc_adapter_server::PlcAdapter, ControlPlcRequest, ControlPlcResponse, QueryPlcRequest,
    QueryPlcResponse,
};

pub mod control_plc;
pub use control_plc::*;

pub mod query_plc;
pub use query_plc::*;

#[derive(Debug, Default)]
pub struct MyPlcAdapter {}

#[tonic::async_trait]
impl PlcAdapter for MyPlcAdapter {
    /// from plc_client
    async fn control_plc(
        &self,
        request: tonic::Request<ControlPlcRequest>,
    ) -> std::result::Result<tonic::Response<ControlPlcResponse>, tonic::Status> {
        control_plc_handler(request).await
    }

    async fn query_plc(
        &self,
        request: tonic::Request<QueryPlcRequest>,
    ) -> std::result::Result<tonic::Response<QueryPlcResponse>, tonic::Status> {
        query_plc_handler(request).await
    }
}
