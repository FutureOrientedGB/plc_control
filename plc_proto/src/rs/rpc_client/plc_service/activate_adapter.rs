use super::PlcServiceRpcClient;
use crate::plc;

impl PlcServiceRpcClient {
    pub async fn activate_adapter(
        client: &mut plc::plc_service_client::PlcServiceClient<tonic::transport::Channel>,
        req: plc::ActivateAdapterRequest,
    ) -> Result<plc::ActivateAdapterResponse, ()> {
        match client.activate_adapter(req.clone()).await {
            Err(status) => {
                tracing::error!(
                    "PlcServiceRpcClient::activate_adapter error, request: {:?}, status: {}",
                    &req,
                    status
                );
                Err(())
            }
            Ok(resp) => Ok(resp.into_inner()),
        }
    }
}
