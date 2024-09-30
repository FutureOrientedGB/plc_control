use super::PlcServiceRpcClient;
use crate::plc;

impl PlcServiceRpcClient {
    pub async fn query_plc_devices(
        client: &mut plc::plc_service_client::PlcServiceClient<tonic::transport::Channel>,
        req: plc::QueryPlcDevicesRequest,
    ) -> Result<plc::QueryPlcDevicesResponse, ()> {
        match client.query_plc_devices(req.clone()).await {
            Err(status) => {
                tracing::error!(
                    "PlcServiceRpcClient::query_plc_devices error, request: {:?}, status: {}",
                    &req,
                    status
                );
                Err(())
            }
            Ok(resp) => Ok(resp.into_inner()),
        }
    }
}
