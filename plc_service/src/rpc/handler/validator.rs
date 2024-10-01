use plc_proto::plc::{ResponseCode, ResponseStatus};

use super::MyPlcService;

impl MyPlcService {
    pub fn validate_version(
        request: i32,
        required: i32,
        enum_type_name: &str,
    ) -> Option<ResponseStatus> {
        if request < required {
            return Some(ResponseStatus {
                code: ResponseCode::Deprecated.into(),
                name: ResponseCode::Deprecated.as_str_name().to_string(),
                message: format!(
                    "{}({}) was deprecated, update your proto file and code",
                    enum_type_name, request,
                ),
            });
        }

        return Some(ResponseStatus {
            code: ResponseCode::Ok.into(),
            name: ResponseCode::Ok.as_str_name().to_string(),
            message: String::new(),
        });
    }
}
