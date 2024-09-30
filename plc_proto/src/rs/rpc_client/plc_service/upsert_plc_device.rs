use super::PlcServiceRpcClient;
use crate::plc;

impl PlcServiceRpcClient {
    pub async fn upsert_plc_device(
        client: &mut plc::plc_service_client::PlcServiceClient<tonic::transport::Channel>,
        req: plc::UpsertPlcDeviceRequest,
    ) -> Result<plc::UpsertPlcDeviceResponse, ()> {
        match client.upsert_plc_device(req.clone()).await {
            Err(status) => {
                tracing::error!(
                    "PlcServiceRpcClient::upsert_plc_device error, request: {:?}, status: {}",
                    &req,
                    status
                );
                Err(())
            }
            Ok(resp) => Ok(resp.into_inner()),
        }
    }
}
