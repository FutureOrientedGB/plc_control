use tonic;

use plc_proto::plc::{QueryPlcRequest, QueryPlcResponse};

/// from plc_client
pub async fn query_plc_handler(
    request: tonic::Request<QueryPlcRequest>,
) -> std::result::Result<tonic::Response<QueryPlcResponse>, tonic::Status> {
    let _query_plc_request = request.into_inner();
    let query_plc_response = QueryPlcResponse::default();
    let response = tonic::Response::new(query_plc_response);
    return Ok(response);
}
