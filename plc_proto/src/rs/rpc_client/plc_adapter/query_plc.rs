use super::PlcAdapterRpcClient;
use crate::plc;

impl PlcAdapterRpcClient {
    pub async fn query_plc(
        &mut self,
        request: plc::QueryPlcRequest,
    ) -> Result<plc::QueryPlcResponse, ()> {
        match self.client.query_plc(request.clone()).await {
            Err(status) => {
                tracing::error!(
                    "PlcAdapterRpcClient::query_plc error, addr: {}, request: {:?}, status: {}",
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
