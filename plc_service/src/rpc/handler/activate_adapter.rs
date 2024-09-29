use tonic;

use plc_proto::plc::{ActivateAdapterRequest, ActivateAdapterResponse};

// from plc_client
pub async fn activate_adapter_handler(
    request: tonic::Request<ActivateAdapterRequest>,
) -> std::result::Result<tonic::Response<ActivateAdapterResponse>, tonic::Status> {
    let _activate_adapter_request = request.into_inner();
    let activate_adapter_response = ActivateAdapterResponse::default();
    let response = tonic::Response::new(activate_adapter_response);
    return Ok(response);
}