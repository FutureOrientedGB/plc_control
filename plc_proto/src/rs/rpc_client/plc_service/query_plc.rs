use super::PlcServiceRpcClient;
use crate::plc;

impl PlcServiceRpcClient {
    pub async fn query_plc(
        &mut self,
        req: plc::QueryPlcRequest,
    ) -> Result<plc::QueryPlcResponse, ()> {
        match self.client.query_plc(req.clone()).await {
            Err(status) => {
                tracing::error!(
                    "PlcServiceRpcClient::query_plc error, addr: {}, request: {:?}, status: {}",
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
