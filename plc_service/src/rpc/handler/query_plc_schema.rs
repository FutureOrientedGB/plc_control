use tonic;

use plc_proto::plc::{
    plc_device_info, BarTypePlcSchema, DeviceType, FooTypePlcSchema, PlcDeviceInfo, PlcDeviceTypeId, QueryPlcSchemaRequest, QueryPlcSchemaResponse, QueryPlcSchemaVersion, ResponseCode, ResponseStatus
};

use super::MyPlcService;

impl MyPlcService {
    // from plc_client
    pub async fn query_plc_schema_handler(
        &self,
        request: tonic::Request<QueryPlcSchemaRequest>,
    ) -> std::result::Result<tonic::Response<QueryPlcSchemaResponse>, tonic::Status> {
        let req = request.into_inner();
        let mut resp = QueryPlcSchemaResponse::default();

        // validate request version with required
        resp.status = Self::validate_version(
            req.version,
            QueryPlcSchemaVersion::QueryPlcSchema20240927.into(),
            std::any::type_name::<QueryPlcSchemaVersion>(),
        );

        // fill schema
        match req.plc_type {
            // type missing
            None => {
                resp.status = Some(ResponseStatus {
                    code: ResponseCode::FieldMissing.into(),
                    name: ResponseCode::FieldMissing.as_str_name().to_string(),
                    message: String::from("QueryPlcSchemaRequest.type missing"),
                })
            }
            Some(plc_type) => {
                resp.plc_type = Some(plc_type.clone());
                match plc_type.id() {
                    // type error
                    PlcDeviceTypeId::Begin | PlcDeviceTypeId::End => {
                        resp.status = Some(ResponseStatus {
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
                        resp.plc_info = Some(PlcDeviceInfo {
                            id: String::new(),
                            r#type: Some(plc_type),
                            address: None,
                            schema: Some(plc_device_info::Schema::Foo(FooTypePlcSchema::new(
                                self.store
                                    .find_adapter(
                                        DeviceType {
                                            id: PlcDeviceTypeId::Foo.into(),
                                            name: PlcDeviceTypeId::Foo.as_str_name().to_string(),
                                        }
                                    )
                                    .await
                                    .is_empty(),
                            ))),
                        })
                    }
                    // type bar
                    PlcDeviceTypeId::Bar => {
                        resp.plc_info = Some(PlcDeviceInfo {
                            id: String::new(),
                            r#type: Some(plc_type),
                            address: None,
                            schema: Some(plc_device_info::Schema::Bar(BarTypePlcSchema::new(
                                self.store
                                    .find_adapter(
                                        DeviceType {
                                            id: PlcDeviceTypeId::Bar.into(),
                                            name: PlcDeviceTypeId::Bar.as_str_name().to_string(),
                                        }
                                    )
                                    .await
                                    .is_empty(),
                            ))),
                        })
                    }
                }
            }
        }

        return Ok(tonic::Response::new(resp));
    }
}
