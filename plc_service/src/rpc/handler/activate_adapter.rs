use tonic;

use plc_proto::plc::{ActivateAdapterRequest, ActivateAdapterResponse, LeaveAdapterRequest, LeaveAdapterResponse};

// from plc_client
pub async fn activate_adapter_handler(
    request: tonic::Request<ActivateAdapterRequest>,
) -> std::result::Result<tonic::Response<ActivateAdapterResponse>, tonic::Status> {
    let _activate_adapter_request = request.into_inner();
    let activate_adapter_response = ActivateAdapterResponse::default();
    let response = tonic::Response::new(activate_adapter_response);
    return Ok(response);
}

// from plc_client
pub async fn leave_adapter_handler(
    request: tonic::Request<LeaveAdapterRequest>,
) -> std::result::Result<tonic::Response<LeaveAdapterResponse>, tonic::Status> {
    let _leave_adapter_request = request.into_inner();
    let leave_adapter_response = LeaveAdapterResponse::default();
    let response = tonic::Response::new(leave_adapter_response);
    return Ok(response);
}