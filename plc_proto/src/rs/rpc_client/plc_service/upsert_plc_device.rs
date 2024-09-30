use super::PlcServiceRpcClient;
use crate::plc;

impl PlcServiceRpcClient {
    pub async fn upsert_plc_device(
        &mut self,
        req: plc::UpsertPlcDeviceRequest,
    ) -> Result<plc::UpsertPlcDeviceResponse, ()> {
        match self.client.upsert_plc_device(req.clone()).await {
            Err(status) => {
                tracing::error!(
                    "PlcServiceRpcClient::upsert_plc_device error, addr: {}, request: {:?}, status: {}",
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
