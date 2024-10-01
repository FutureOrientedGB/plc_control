use tonic;

use plc_proto::plc::{
    DeviceType, PlcDeviceTypeId, QueryPlcTypesRequest, QueryPlcTypesResponse, QueryPlcTypesVersion,
};

use super::MyPlcService;

impl MyPlcService {
    // from plc_client
    pub async fn query_plc_types_handler(
        &self,
        request: tonic::Request<QueryPlcTypesRequest>,
    ) -> std::result::Result<tonic::Response<QueryPlcTypesResponse>, tonic::Status> {
        let req = request.into_inner();
        let mut resp = QueryPlcTypesResponse::default();

        // validate request version with required
        resp.status = Self::validate_version(
            req.version,
            QueryPlcTypesVersion::QueryPlcTypes20240927.into(),
            std::any::type_name::<QueryPlcTypesVersion>(),
        );

        let mut types = vec![];
        for type_id in PlcDeviceTypeId::Begin as i32 + 1..PlcDeviceTypeId::End as i32 {
            types.push(DeviceType {
                id: type_id,
                name: PlcDeviceTypeId::try_from(type_id)
                    .unwrap()
                    .as_str_name()
                    .to_string(),
            });
        }
        resp.plc_types = types;

        return Ok(tonic::Response::new(resp));
    }
}
