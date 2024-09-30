use super::PlcServiceRpcClient;
use crate::plc;

impl PlcServiceRpcClient {
    pub async fn control_plc(
        &mut self,
        request: plc::ControlPlcRequest,
    ) -> Result<plc::ControlPlcResponse, ()> {
        match self.client.control_plc(request.clone()).await {
            Err(status) => {
                tracing::error!(
                    "PlcServiceRpcClient::control_plc error, addr: {}, request: {:?}, status: {}",
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
