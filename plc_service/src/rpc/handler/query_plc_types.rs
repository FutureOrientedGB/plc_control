use tonic;

use plc_proto::plc::{
    DeviceType, PlcDeviceTypeId, QueryPlcTypesVersion, QueryPlcTypesRequest,
    QueryPlcTypesResponse, ResponseCode, ResponseStatus,
};

// from plc_client
pub async fn query_plc_types_handler(
    request: tonic::Request<QueryPlcTypesRequest>,
) -> std::result::Result<tonic::Response<QueryPlcTypesResponse>, tonic::Status> {
    let query_plc_types_request = request.into_inner();
    let mut query_plc_types_response = QueryPlcTypesResponse::default();

    // validate request version with required
    if query_plc_types_request.version < QueryPlcTypesVersion::InitD699c2120240927.into() {
        query_plc_types_response.status = Some(ResponseStatus {
            code: ResponseCode::Deprecated.into(),
            name: ResponseCode::Deprecated.as_str_name().to_string(),
            message: format!(
                "QueryPlcTypesVersion({v}) was deprecated, update your proto file and code",
                v = query_plc_types_request.version
            ),
        })
    }

    let mut types = vec![];
    for typd_id in PlcDeviceTypeId::Begin as i32 + 1..PlcDeviceTypeId::End as i32 {
        types.push(DeviceType {
            id: typd_id,
            name: PlcDeviceTypeId::try_from(typd_id)
                .unwrap()
                .as_str_name()
                .to_string(),
        });
    }
    query_plc_types_response.plc_types = types;

    let response = tonic::Response::new(query_plc_types_response);
    return Ok(response);
}
