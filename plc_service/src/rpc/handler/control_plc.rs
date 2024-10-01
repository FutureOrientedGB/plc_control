use tonic;

use plc_proto::plc::{self, ControlPlcRequest, ControlPlcResponse, ControlPlcVersion};

use super::MyPlcService;

impl MyPlcService {
    // from plc_client
    pub async fn control_plc_handler(
        &self,
        request: tonic::Request<ControlPlcRequest>,
    ) -> std::result::Result<tonic::Response<ControlPlcResponse>, tonic::Status> {
        let req = request.into_inner();
        let mut resp = ControlPlcResponse::default();
        resp.version = Some(plc::control_plc_response::Version {
            request: req.version,
            required: ControlPlcVersion::ControlPlc20240927.into(),
        });

        // validate request version with required
        resp.status = Self::validate_version(
            req.version,
            ControlPlcVersion::ControlPlc20240927.into(),
            std::any::type_name::<ControlPlcVersion>(),
        );

        return Ok(tonic::Response::new(resp));
    }
}
