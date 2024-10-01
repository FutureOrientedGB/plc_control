use tonic;

use plc_proto::plc::{upsert_plc_device_response, UpsertPlcDeviceRequest, UpsertPlcDeviceResponse, UpsertPlcVersion};

use super::MyPlcService;

impl MyPlcService {
    // from plc_client
    pub async fn upsert_plc_device_handler(
        &self,
        request: tonic::Request<UpsertPlcDeviceRequest>,
    ) -> std::result::Result<tonic::Response<UpsertPlcDeviceResponse>, tonic::Status> {
        let req = request.into_inner();
        let mut resp = UpsertPlcDeviceResponse::default();
        resp.version = Some(upsert_plc_device_response::Version {
            request: req.version,
            required: UpsertPlcVersion::UpsertPlc20240928.into(),
        });

        // validate request version with required
        resp.status = Self::validate_version(
            req.version,
            UpsertPlcVersion::UpsertPlc20240928.into(),
            std::any::type_name::<UpsertPlcVersion>(),
        );

        return Ok(tonic::Response::new(resp));
    }
}
