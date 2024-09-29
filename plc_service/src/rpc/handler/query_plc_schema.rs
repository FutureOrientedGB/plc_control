use tonic;

use plc_proto::plc::{
    plc_device_info, BarTypePlcSchema, FooTypePlcSchema, PlcDeviceInfo, PlcDeviceTypeId,
    QueryPlcSchemaRequest, QueryPlcSchemaResponse, QueryPlcSchemaVersion, ResponseCode,
    ResponseStatus,
};

/// from plc_client
pub async fn query_plc_schema_handler(
    request: tonic::Request<QueryPlcSchemaRequest>,
) -> std::result::Result<tonic::Response<QueryPlcSchemaResponse>, tonic::Status> {
    let query_plc_schema_request = request.into_inner();
    let mut query_plc_schema_response = QueryPlcSchemaResponse::default();

    // validate request version with required
    if query_plc_schema_request.version < QueryPlcSchemaVersion::InitDd141e320240927.into() {
        query_plc_schema_response.status = Some(ResponseStatus {
            code: ResponseCode::Deprecated.into(),
            name: ResponseCode::Deprecated.as_str_name().to_string(),
            message: format!(
                "QueryPlcSchemaVersion({v}) was deprecated, update your proto file and code",
                v = query_plc_schema_request.version
            ),
        })
    }

    // fill schema
    match query_plc_schema_request.plc_type {
        // type missing
        None => {
            query_plc_schema_response.status = Some(ResponseStatus {
                code: ResponseCode::FieldMissing.into(),
                name: ResponseCode::FieldMissing.as_str_name().to_string(),
                message: String::from("QueryPlcSchemaRequest.type missing"),
            })
        }
        Some(plc_type) => {
            query_plc_schema_response.plc_type = Some(plc_type.clone());
            match plc_type.id() {
                // type error
                PlcDeviceTypeId::Begin | PlcDeviceTypeId::End => {
                    query_plc_schema_response.status = Some(ResponseStatus {
                        code: ResponseCode::FieldError.into(),
                        name: ResponseCode::FieldError.as_str_name().to_string(),
                        message: format!(
                            "QueryPlcSchemaRequest.type({v}, {n}) error",
                            v = plc_type.id,
                            n = plc_type.name
                        ),
                    })
                }
                // type foo
                PlcDeviceTypeId::Foo => {
                    query_plc_schema_response.plc_info = Some(PlcDeviceInfo {
                        r#type: Some(plc_type),
                        address: None,
                        schema: Some(plc_device_info::Schema::Foo(FooTypePlcSchema::new())),
                    })
                }
                // type bar
                PlcDeviceTypeId::Bar => {
                    query_plc_schema_response.plc_info = Some(PlcDeviceInfo {
                        r#type: Some(plc_type),
                        address: None,
                        schema: Some(plc_device_info::Schema::Bar(BarTypePlcSchema::new())),
                    })
                }
            }
        }
    }

    let response = tonic::Response::new(query_plc_schema_response);
    return Ok(response);
}
