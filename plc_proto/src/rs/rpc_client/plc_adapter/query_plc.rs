use super::PlcAdapterRpcClient;
use crate::plc;

impl PlcAdapterRpcClient {
    pub async fn query_plc(
        client: &mut plc::plc_adapter_client::PlcAdapterClient<tonic::transport::Channel>,
        req: plc::QueryPlcRequest,
    ) -> Result<plc::QueryPlcResponse, ()> {
        match client.query_plc(req.clone()).await {
            Err(status) => {
                tracing::error!(
                    "PlcAdapterRpcClient::query_plc error, request: {:?}, status: {}",
                    &req,
                    status
                );
                Err(())
            }
            Ok(resp) => Ok(resp.into_inner()),
        }
    }
}
