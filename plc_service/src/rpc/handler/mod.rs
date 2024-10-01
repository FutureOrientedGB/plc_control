use tonic;

use plc_proto::plc::{
    plc_service_server::PlcService, ActivateAdapterRequest, ActivateAdapterResponse,
    ControlPlcRequest, ControlPlcResponse, LeaveAdapterRequest, LeaveAdapterResponse,
    QueryPlcDevicesRequest, QueryPlcDevicesResponse, QueryPlcRequest, QueryPlcResponse,
    QueryPlcSchemaRequest, QueryPlcSchemaResponse, QueryPlcTypesRequest, QueryPlcTypesResponse,
    UpsertPlcDeviceRequest, UpsertPlcDeviceResponse,
};

use crate::conf::Conf;
use crate::store::RedisStore;

pub mod activate_adapter;
pub mod control_plc;
pub mod leave_adapter;
pub mod query_plc;
pub mod query_plc_devices;
pub mod query_plc_schema;
pub mod query_plc_types;
pub mod upsert_plc_device;
pub mod validator;

#[derive(Debug, Default)]
pub struct MyPlcService {
    pub store: RedisStore,
}

impl MyPlcService {
    pub fn new(conf: Conf) -> Self {
        Self {
            store: RedisStore::new(conf),
        }
    }

    pub fn connect_store(&mut self) -> bool {
        return self.store.open();
    }
}

#[tonic::async_trait]
impl PlcService for MyPlcService {
    // from plc_adapter
    async fn activate_adapter(
        &self,
        request: tonic::Request<ActivateAdapterRequest>,
    ) -> std::result::Result<tonic::Response<ActivateAdapterResponse>, tonic::Status> {
        self.activate_adapter_handler(request).await
    }

    async fn leave_adapter(
        &self,
        request: tonic::Request<LeaveAdapterRequest>,
    ) -> std::result::Result<tonic::Response<LeaveAdapterResponse>, tonic::Status> {
        self.leave_adapter_handler(request).await
    }

    // from plc_client
    async fn query_plc_types(
        &self,
        request: tonic::Request<QueryPlcTypesRequest>,
    ) -> std::result::Result<tonic::Response<QueryPlcTypesResponse>, tonic::Status> {
        self.query_plc_types_handler(request).await
    }

    async fn query_plc_schema(
        &self,
        request: tonic::Request<QueryPlcSchemaRequest>,
    ) -> std::result::Result<tonic::Response<QueryPlcSchemaResponse>, tonic::Status> {
        self.query_plc_schema_handler(request).await
    }

    async fn query_plc_devices(
        &self,
        request: tonic::Request<QueryPlcDevicesRequest>,
    ) -> std::result::Result<tonic::Response<QueryPlcDevicesResponse>, tonic::Status> {
        self.query_plc_devices_handler(request).await
    }

    async fn upsert_plc_device(
        &self,
        request: tonic::Request<UpsertPlcDeviceRequest>,
    ) -> std::result::Result<tonic::Response<UpsertPlcDeviceResponse>, tonic::Status> {
        self.upsert_plc_device_handler(request).await
    }

    // from plc_client
    async fn control_plc(
        &self,
        request: tonic::Request<ControlPlcRequest>,
    ) -> std::result::Result<tonic::Response<ControlPlcResponse>, tonic::Status> {
        self.control_plc_handler(request).await
    }

    async fn query_plc(
        &self,
        request: tonic::Request<QueryPlcRequest>,
    ) -> std::result::Result<tonic::Response<QueryPlcResponse>, tonic::Status> {
        self.query_plc_handler(request).await
    }
}
