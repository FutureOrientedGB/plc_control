use super::PlcServiceRpcClient;
use crate::plc;

impl PlcServiceRpcClient {
    pub async fn control_plc(
        client: &mut plc::plc_service_client::PlcServiceClient<tonic::transport::Channel>,
        req: plc::ControlPlcRequest,
    ) -> Result<plc::ControlPlcResponse, ()> {
        match client.control_plc(req.clone()).await {
            Err(status) => {
                tracing::error!(
                    "PlcServiceRpcClient::control_plc error, request: {:?}, status: {}",
                    &req,
                    status
                );
                Err(())
            }
            Ok(resp) => Ok(resp.into_inner()),
        }
    }
}
