use tonic;

use plc_proto::plc::{UpsertPlcDeviceRequest, UpsertPlcDeviceResponse};

/// from plc_client
pub async fn upsert_plc_device_handler(
    request: tonic::Request<UpsertPlcDeviceRequest>,
) -> std::result::Result<tonic::Response<UpsertPlcDeviceResponse>, tonic::Status> {
    let _upsert_plc_device_request = request.into_inner();
    let upsert_plc_device_response = UpsertPlcDeviceResponse::default();
    let response = tonic::Response::new(upsert_plc_device_response);
    return Ok(response);
}
