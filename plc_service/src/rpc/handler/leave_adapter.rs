use tonic;

use plc_proto::plc::{
    self, leave_adapter_response, LeaveAdapterRequest, LeaveAdapterResponse, LeaveAdapterVersion,
};

use super::MyPlcService;

impl MyPlcService {
    // from plc_client
    pub async fn leave_adapter_handler(
        &self,
        request: tonic::Request<LeaveAdapterRequest>,
    ) -> std::result::Result<tonic::Response<LeaveAdapterResponse>, tonic::Status> {
        let req = request.into_inner();
        let mut resp = LeaveAdapterResponse::default();
        resp.version = Some(leave_adapter_response::Version {
            request: req.version,
            required: LeaveAdapterVersion::LeaveAdapter20240930.into(),
        });

        // validate request version with required
        resp.status = Self::validate_version(
            req.version,
            LeaveAdapterVersion::LeaveAdapter20240930.into(),
            std::any::type_name::<LeaveAdapterVersion>(),
        );

        // clear adapter's info
        if let Some(device_type) = req.r#type {
            if self.store.leave_adapter(device_type).await {
                resp.status = Some(plc::ResponseStatus {
                    code: plc::ResponseCode::ServiceStoreError.into(),
                    name: plc::ResponseCode::ServiceStoreError
                        .as_str_name()
                        .to_string(),
                    message: String::new(),
                });
            }
        }

        return Ok(tonic::Response::new(resp));
    }
}
