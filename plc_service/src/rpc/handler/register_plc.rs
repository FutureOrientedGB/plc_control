use tonic;

use plc_proto::plc::{RegisterPlcRequest, RegisterPlcResponse};

/// from plc_client
pub async fn register_plc_handler(
    request: tonic::Request<RegisterPlcRequest>,
) -> std::result::Result<tonic::Response<RegisterPlcResponse>, tonic::Status> {
    let _register_plc_request = request.into_inner();
    let register_plc_response = RegisterPlcResponse::default();
    let response = tonic::Response::new(register_plc_response);
    return Ok(response);
}
