use tonic;

use plc_proto::plc::{ActivateAdapterRequest, ActivateAdapterResponse, ActivateAdapterVersion};

use super::validate_version;

// from plc_client
pub async fn activate_adapter_handler(
    request: tonic::Request<ActivateAdapterRequest>,
) -> std::result::Result<tonic::Response<ActivateAdapterResponse>, tonic::Status> {
    let req = request.into_inner();
    let mut resp = ActivateAdapterResponse::default();

    // validate request version with required
    resp.status = validate_version(
        req.version,
        ActivateAdapterVersion::ActivateAdapter20240930.into(),
        std::any::type_name::<ActivateAdapterVersion>(),
    );

    return Ok(tonic::Response::new(resp));
}
