use stdext::function_name;

use super::PlcServiceRpcClient;
use crate::plc;

impl PlcServiceRpcClient {
    pub async fn activate_adapter(
        &mut self,
        request: plc::ActivateAdapterRequest,
    ) -> Result<plc::ActivateAdapterResponse, ()> {
        match self.client.activate_adapter(request.clone()).await {
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
