use super::PlcServiceRpcClient;
use crate::plc;

impl PlcServiceRpcClient {
    pub async fn query_plc_types(
        &mut self,
        request: plc::QueryPlcTypesRequest,
    ) -> Result<plc::QueryPlcTypesResponse, ()> {
        match self.client.query_plc_types(request.clone()).await {
            Err(status) => {
                tracing::error!(
                    "PlcServiceRpcClient::query_plc_types error, addr: {}, request: {:?}, status: {}",
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
