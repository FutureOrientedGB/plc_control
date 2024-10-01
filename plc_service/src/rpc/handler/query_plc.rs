use tonic;

use plc_proto::plc::{query_plc_response, QueryPlcRequest, QueryPlcResponse, QueryPlcVersion};

use super::MyPlcService;

impl MyPlcService {
    // from plc_client
    pub async fn query_plc_handler(
        &self,
        request: tonic::Request<QueryPlcRequest>,
    ) -> std::result::Result<tonic::Response<QueryPlcResponse>, tonic::Status> {
        let req = request.into_inner();
        let mut resp = QueryPlcResponse::default();
        resp.version = Some(query_plc_response::Version {
            request: req.version,
            required: QueryPlcVersion::QueryPlcVersion20240927.into(),
        });

        // validate request version with required
        resp.status = Self::validate_version(
            req.version,
            QueryPlcVersion::QueryPlcVersion20240927.into(),
            std::any::type_name::<QueryPlcVersion>(),
        );

        return Ok(tonic::Response::new(resp));
    }
}
