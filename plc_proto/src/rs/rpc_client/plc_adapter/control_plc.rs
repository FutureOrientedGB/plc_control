use super::PlcAdapterRpcClient;
use crate::plc;

impl PlcAdapterRpcClient {
    pub async fn control_plc(
        &mut self,
        request: plc::ControlPlcRequest,
    ) -> Result<plc::ControlPlcResponse, ()> {
        match self.client.control_plc(request.clone()).await {
            Err(status) => {
                tracing::error!(
                    "PlcAdapterRpcClient::control_plc error, addr: {}, request: {:?}, status: {}",
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
