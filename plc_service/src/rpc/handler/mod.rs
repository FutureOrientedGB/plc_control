use tonic;

use plc_proto::plc::{
    plc_service_server::PlcService, ControlPlcRequest, ControlPlcResponse, QueryPlcDevicesRequest,
    QueryPlcDevicesResponse, QueryPlcRequest, QueryPlcResponse, QueryPlcSchemaRequest,
    QueryPlcSchemaResponse, QueryPlcTypesRequest, QueryPlcTypesResponse, RegisterPlcRequest,
    RegisterPlcResponse, UpsertPlcDeviceRequest, UpsertPlcDeviceResponse,
};

pub mod control_plc;
pub use control_plc::*;

pub mod query_plc_devices;
pub use query_plc_devices::*;

pub mod query_plc_schema;
pub use query_plc_schema::*;

pub mod query_plc_types;
pub use query_plc_types::*;

pub mod query_plc;
pub use query_plc::*;

pub mod register_plc;
pub use register_plc::*;

pub mod upsert_plc_device;
pub use upsert_plc_device::*;


#[derive(Debug, Default)]
pub struct MyPlcService {}

#[tonic::async_trait]
impl PlcService for MyPlcService {
    // from plc_adapter
    async fn register_plc(
        &self,
        request: tonic::Request<RegisterPlcRequest>,
    ) -> std::result::Result<tonic::Response<RegisterPlcResponse>, tonic::Status> {
        register_plc_handler(request).await
    }

    // from plc_client
    async fn query_plc_types(
        &self,
        request: tonic::Request<QueryPlcTypesRequest>,
    ) -> std::result::Result<tonic::Response<QueryPlcTypesResponse>, tonic::Status> {
        query_plc_types_handler(request).await
    }

    async fn query_plc_schema(
        &self,
        request: tonic::Request<QueryPlcSchemaRequest>,
    ) -> std::result::Result<tonic::Response<QueryPlcSchemaResponse>, tonic::Status> {
        query_plc_schema_handler(request).await
    }

    async fn query_plc_devices(
        &self,
        request: tonic::Request<QueryPlcDevicesRequest>,
    ) -> std::result::Result<tonic::Response<QueryPlcDevicesResponse>, tonic::Status> {
        query_plc_devices_handler(request).await
    }

    async fn upsert_plc_device(
        &self,
        request: tonic::Request<UpsertPlcDeviceRequest>,
    ) -> std::result::Result<tonic::Response<UpsertPlcDeviceResponse>, tonic::Status> {
        upsert_plc_device_handler(request).await
    }

    // from plc_client
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
