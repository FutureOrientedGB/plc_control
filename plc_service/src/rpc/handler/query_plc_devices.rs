use tonic;

use plc_proto::plc::{QueryPlcDevicesRequest, QueryPlcDevicesResponse, QueryPlcDevicesVersion};

use super::MyPlcService;

impl MyPlcService {
    // from plc_client
    pub async fn query_plc_devices_handler(
        &self,
        request: tonic::Request<QueryPlcDevicesRequest>,
    ) -> std::result::Result<tonic::Response<QueryPlcDevicesResponse>, tonic::Status> {
        let req = request.into_inner();
        let mut resp = QueryPlcDevicesResponse::default();

        // validate request version with required
        resp.status = Self::validate_version(
            req.version,
            QueryPlcDevicesVersion::QueryPlcDevices20240927.into(),
            std::any::type_name::<QueryPlcDevicesVersion>(),
        );

        return Ok(tonic::Response::new(resp));
    }
}
