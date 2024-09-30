use super::PlcServiceRpcClient;
use crate::plc;

impl PlcServiceRpcClient {
    pub async fn activate_adapter(
        &mut self,
        request: plc::ActivateAdapterRequest,
    ) -> Result<plc::ActivateAdapterResponse, ()> {
        match self.client.activate_adapter(request.clone()).await {
            Err(status) => {
                tracing::error!(
                    "PlcServiceRpcClient::activate_adapter error, addr: {}, request: {:?}, status: {}",
                    &self.addr,
                    &request,
                    status
                );
                Err(())
            }
            Ok(resp) => Ok(resp.into_inner()),
        }
    }
}
