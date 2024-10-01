use stdext::function_name;

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
