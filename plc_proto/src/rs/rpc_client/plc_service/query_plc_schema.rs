use stdext::function_name;

use super::PlcServiceRpcClient;
use crate::plc;

impl PlcServiceRpcClient {
    pub async fn query_plc_schema(
        &mut self,
        request: plc::QueryPlcSchemaRequest,
    ) -> Result<plc::QueryPlcSchemaResponse, ()> {
        match self.client.query_plc_schema(request.clone()).await {
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
