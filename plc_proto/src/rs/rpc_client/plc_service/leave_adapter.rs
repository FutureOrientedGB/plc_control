use super::PlcServiceRpcClient;
use crate::plc;

impl PlcServiceRpcClient {
    pub async fn leave_adapter(
        client: &mut plc::plc_service_client::PlcServiceClient<tonic::transport::Channel>,
        req: plc::LeaveAdapterRequest,
    ) -> Result<plc::LeaveAdapterResponse, ()> {
        match client.leave_adapter(req.clone()).await {
            Err(status) => {
                tracing::error!(
                    "PlcServiceRpcClient::leave_adapter error, request: {:?}, status: {}",
                    &req,
                    status
                );
                Err(())
            }
            Ok(resp) => Ok(resp.into_inner()),
        }
    }
}
