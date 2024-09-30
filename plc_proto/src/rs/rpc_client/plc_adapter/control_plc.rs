use super::PlcAdapterRpcClient;
use crate::plc;

impl PlcAdapterRpcClient {
    pub async fn control_plc(
        client: &mut plc::plc_adapter_client::PlcAdapterClient<tonic::transport::Channel>,
        req: plc::ControlPlcRequest,
    ) -> Result<plc::ControlPlcResponse, ()> {
        match client.control_plc(req.clone()).await {
            Err(status) => {
                tracing::error!(
                    "PlcAdapterRpcClient::control_plc error, request: {:?}, status: {}",
                    &req,
                    status
                );
                Err(())
            }
            Ok(resp) => Ok(resp.into_inner()),
        }
    }
}
