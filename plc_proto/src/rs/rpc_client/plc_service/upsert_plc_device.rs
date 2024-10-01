use stdext::function_name;

use super::PlcServiceRpcClient;
use crate::plc;

impl PlcServiceRpcClient {
    pub async fn upsert_plc_device(
        &mut self,
        request: plc::UpsertPlcDeviceRequest,
    ) -> Result<plc::UpsertPlcDeviceResponse, ()> {
        match self.client.upsert_plc_device(request.clone()).await {
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
