use super::PlcServiceRpcClient;
use crate::plc;

impl PlcServiceRpcClient {
    pub async fn query_plc_devices(
        &mut self,
        req: plc::QueryPlcDevicesRequest,
    ) -> Result<plc::QueryPlcDevicesResponse, ()> {
        match self.client.query_plc_devices(req.clone()).await {
            Err(status) => {
                tracing::error!(
                    "PlcServiceRpcClient::query_plc_devices error, addr: {}, request: {:?}, status: {}",
                    &self.addr,
                    &req,
                    status
                );
                Err(())
            }
            Ok(resp) => Ok(resp.into_inner()),
        }
    }
}
