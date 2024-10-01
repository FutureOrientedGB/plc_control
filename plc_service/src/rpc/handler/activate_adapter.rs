use tonic;

use plc_proto::plc::{
    self, ActivateAdapterRequest, ActivateAdapterResponse, ActivateAdapterVersion,
};

use super::MyPlcService;

impl MyPlcService {
    // from plc_client
    pub async fn activate_adapter_handler(
        &self,
        request: tonic::Request<ActivateAdapterRequest>,
    ) -> std::result::Result<tonic::Response<ActivateAdapterResponse>, tonic::Status> {
        let req = request.into_inner();
        let mut resp = ActivateAdapterResponse::default();
        resp.version = Some(plc::activate_adapter_response::Version {
            request: req.version,
            required: ActivateAdapterVersion::ActivateAdapter20240930.into(),
        });

        // validate request version with required
        resp.status = Self::validate_version(
            req.version,
            ActivateAdapterVersion::ActivateAdapter20240930.into(),
            std::any::type_name::<ActivateAdapterVersion>(),
        );

        // store adapter's info
        if self.store.activate_adapter(&req).await {
            resp.status = Some(plc::ResponseStatus {
                code: plc::ResponseCode::ServiceStoreError.into(),
                name: plc::ResponseCode::ServiceStoreError
                    .as_str_name()
                    .to_string(),
                message: String::new(),
            });
        }

        return Ok(tonic::Response::new(resp));
    }
}
