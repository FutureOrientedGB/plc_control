use stdext::function_name;

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
                    func = function_name!(),
                    addr = self.addr,
                    request = serde_json::to_string(&request).unwrap(),
                    status = status.to_string(),
                );
                Err(())
            }
            Ok(resp) => Ok(resp.into_inner()),
        }
    }
}
