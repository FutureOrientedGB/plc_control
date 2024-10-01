use tonic;

use plc_proto::plc::{LeaveAdapterRequest, LeaveAdapterResponse, LeaveAdapterVersion};

use super::validate_version;

// from plc_client
pub async fn leave_adapter_handler(
    request: tonic::Request<LeaveAdapterRequest>,
) -> std::result::Result<tonic::Response<LeaveAdapterResponse>, tonic::Status> {
    let req = request.into_inner();
    let mut resp = LeaveAdapterResponse::default();

    // validate request version with required
    resp.status = validate_version(
        req.version,
        LeaveAdapterVersion::LeaveAdapter20240930.into(),
        std::any::type_name::<LeaveAdapterVersion>(),
    );

    return Ok(tonic::Response::new(resp));
}
