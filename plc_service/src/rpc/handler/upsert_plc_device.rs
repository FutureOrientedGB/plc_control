use tonic;

use plc_proto::plc::{UpsertPlcDeviceRequest, UpsertPlcDeviceResponse, UpsertPlcVersion};

use super::validate_version;

// from plc_client
pub async fn upsert_plc_device_handler(
    request: tonic::Request<UpsertPlcDeviceRequest>,
) -> std::result::Result<tonic::Response<UpsertPlcDeviceResponse>, tonic::Status> {
    let req = request.into_inner();
    let mut resp = UpsertPlcDeviceResponse::default();

    // validate request version with required
    resp.status = validate_version(
        req.version,
        UpsertPlcVersion::UpsertPlc20240928.into(),
        std::any::type_name::<UpsertPlcVersion>(),
    );

    return Ok(tonic::Response::new(resp));
}
