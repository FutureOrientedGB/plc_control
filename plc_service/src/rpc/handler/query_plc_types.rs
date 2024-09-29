use tonic;

use plc_proto::plc::{QueryPlcTypesRequest, QueryPlcTypesResponse};

/// from plc_client
pub async fn query_plc_types_handler(
    request: tonic::Request<QueryPlcTypesRequest>,
) -> std::result::Result<tonic::Response<QueryPlcTypesResponse>, tonic::Status> {
    let _query_plc_types_request = request.into_inner();
    let query_plc_types_response = QueryPlcTypesResponse::default();
    let response = tonic::Response::new(query_plc_types_response);
    return Ok(response);
}
