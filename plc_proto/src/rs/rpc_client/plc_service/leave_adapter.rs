use super::PlcServiceRpcClient;
use crate::plc;

impl PlcServiceRpcClient {
    pub async fn leave_adapter(
        &mut self,
        request: plc::LeaveAdapterRequest,
    ) -> Result<plc::LeaveAdapterResponse, ()> {
        match self.client.leave_adapter(request.clone()).await {
            Err(status) => {
                tracing::error!(
                    "PlcServiceRpcClient::leave_adapter error, addr: {}, request: {:?}, status: {}",
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
