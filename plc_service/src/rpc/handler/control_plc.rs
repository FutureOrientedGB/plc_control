use tonic;

use plc_proto::plc::{ControlPlcRequest, ControlPlcResponse};

/// from plc_client
pub async fn control_plc_handler(
    request: tonic::Request<ControlPlcRequest>,
) -> std::result::Result<tonic::Response<ControlPlcResponse>, tonic::Status> {
    let _control_request = request.into_inner();
    let control_response = ControlPlcResponse::default();
    let response = tonic::Response::new(control_response);
    return Ok(response);
}
