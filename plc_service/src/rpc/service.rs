use tonic;

use plc_proto::plc::{
    plc_service_server::PlcService, ControlPlcRequest, ControlPlcResponse, QueryPlcDevicesRequest,
    QueryPlcDevicesResponse, QueryPlcRequest, QueryPlcResponse, QueryPlcSchemaRequest,
    QueryPlcSchemaResponse, QueryPlcTypesRequest, QueryPlcTypesResponse, RegisterPlcRequest,
    RegisterPlcResponse, UpsertPlcDeviceRequest, UpsertPlcDeviceResponse,
};

#[derive(Debug, Default)]
pub struct MyPlcService {}

#[tonic::async_trait]
impl PlcService for MyPlcService {
    /// from plc_adapter
    async fn register_plc(
        &self,
        request: tonic::Request<RegisterPlcRequest>,
    ) -> std::result::Result<tonic::Response<RegisterPlcResponse>, tonic::Status> {
        unimplemented!();
    }

    /// from plc_client
    async fn query_plc_types(
        &self,
        request: tonic::Request<QueryPlcTypesRequest>,
    ) -> std::result::Result<tonic::Response<QueryPlcTypesResponse>, tonic::Status> {
        unimplemented!();
    }

    async fn query_plc_schema(
        &self,
        request: tonic::Request<QueryPlcSchemaRequest>,
    ) -> std::result::Result<tonic::Response<QueryPlcSchemaResponse>, tonic::Status> {
        unimplemented!();
    }

    async fn query_plc_devices(
        &self,
        request: tonic::Request<QueryPlcDevicesRequest>,
    ) -> std::result::Result<tonic::Response<QueryPlcDevicesResponse>, tonic::Status> {
        unimplemented!();
    }

    async fn upsert_plc_device(
        &self,
        request: tonic::Request<UpsertPlcDeviceRequest>,
    ) -> std::result::Result<tonic::Response<UpsertPlcDeviceResponse>, tonic::Status> {
        unimplemented!();
    }

    /// from plc_client
    async fn control_plc(
        &self,
        request: tonic::Request<ControlPlcRequest>,
    ) -> std::result::Result<tonic::Response<ControlPlcResponse>, tonic::Status> {
        unimplemented!();
    }

    async fn query_plc(
        &self,
        request: tonic::Request<QueryPlcRequest>,
    ) -> std::result::Result<tonic::Response<QueryPlcResponse>, tonic::Status> {
        unimplemented!();
    }
}
