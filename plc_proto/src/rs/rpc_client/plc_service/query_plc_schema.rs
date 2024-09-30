use super::PlcServiceRpcClient;
use crate::plc;

impl PlcServiceRpcClient {
    pub async fn query_plc_schema(
        &mut self,
        req: plc::QueryPlcSchemaRequest,
    ) -> Result<plc::QueryPlcSchemaResponse, ()> {
        match self.client.query_plc_schema(req.clone()).await {
            Err(status) => {
                tracing::error!(
                    "PlcServiceRpcClient::query_plc_schema error, addr: {}, request: {:?}, status: {}",
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
