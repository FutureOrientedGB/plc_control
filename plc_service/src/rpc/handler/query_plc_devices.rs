use tonic;

use plc_proto::plc::{QueryPlcDevicesRequest, QueryPlcDevicesResponse};

// from plc_client
pub async fn query_plc_devices_handler(
    request: tonic::Request<QueryPlcDevicesRequest>,
) -> std::result::Result<tonic::Response<QueryPlcDevicesResponse>, tonic::Status> {
    let _query_plc_devices_request = request.into_inner();
    let query_plc_devices_response = QueryPlcDevicesResponse::default();
    let response = tonic::Response::new(query_plc_devices_response);
    return Ok(response);
}
