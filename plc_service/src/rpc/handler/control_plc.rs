use tonic;

use plc_proto::plc::{ControlPlcRequest, ControlPlcResponse, ControlPlcVersion};

use super::validate_version;

// from plc_client
pub async fn control_plc_handler(
    request: tonic::Request<ControlPlcRequest>,
) -> std::result::Result<tonic::Response<ControlPlcResponse>, tonic::Status> {
    let req = request.into_inner();
    let mut resp = ControlPlcResponse::default();

    // validate request version with required
    resp.status = validate_version(
        req.version,
        ControlPlcVersion::ControlPlc20240927.into(),
        std::any::type_name::<ControlPlcVersion>(),
    );

    return Ok(tonic::Response::new(resp));
}
