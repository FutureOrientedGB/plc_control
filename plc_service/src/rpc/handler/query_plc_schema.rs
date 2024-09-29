use tonic;

use plc_proto::plc::{QueryPlcSchemaRequest, QueryPlcSchemaResponse};

/// from plc_client
pub async fn query_plc_schema_handler(
    request: tonic::Request<QueryPlcSchemaRequest>,
) -> std::result::Result<tonic::Response<QueryPlcSchemaResponse>, tonic::Status> {
    let _query_plc_schema_request = request.into_inner();
    let query_plc_schema_response = QueryPlcSchemaResponse::default();
    let response = tonic::Response::new(query_plc_schema_response);
    return Ok(response);
}
