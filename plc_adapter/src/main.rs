use clap::Parser;

use conf::Conf;
use local_ip_address::local_ip;

use tokio;

use tonic;

use plc_log::open_log_file;
use plc_proto::{plc, rpc_client};

pub mod conf;
pub mod rpc;
pub mod version;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse conf first from file, then from command lines
    let mut conf = conf::Conf::parse();
    conf.update(
        &version::APP_NAME,
        &version::APP_VERSION,
        local_ip().unwrap().to_string(),
    );
    let addr = format!("{}:{}", &conf.host, &conf.port);

    // init log
    open_log_file(&conf.app_name, &conf.app_version, conf.port);

    // validate device type
    if plc::PlcDeviceTypeId::try_from(conf.device_type_id).is_err()
        || plc::PlcDeviceTypeId::from_str_name(&conf.device_type_name).is_none()
    {
        let mut device_types = vec![];
        for type_id in plc::PlcDeviceTypeId::Begin as i32 + 1..plc::PlcDeviceTypeId::End as i32 {
            device_types.push(format!(
                "    --device-type-id={} --device-type-name={}",
                type_id,
                plc::PlcDeviceTypeId::try_from(type_id)
                    .unwrap()
                    .as_str_name()
                    .to_string(),
            ));
        }
        eprintln!("available device types: \n{}", device_types.join("\n"));
        tracing::error!("available device types: \n{}", device_types.join("\n"))
    } else {
        // init rpc service
        let plc_adapter = rpc::handler::MyPlcAdapter::default();

        // background tasks
        let (quit_background_tasks_tx, task_handle) = run_background_tasks(conf.clone()).await;

        // serve rpc
        tonic::transport::Server::builder()
            .add_service(plc::plc_adapter_server::PlcAdapterServer::new(plc_adapter))
            .serve(addr.parse().unwrap())
            .await?;

        // stop background tasks
        stop_background_tasks(quit_background_tasks_tx, task_handle).await;
    }

    Ok(())
}

async fn run_background_tasks(
    conf: Conf,
) -> (
    tokio::sync::broadcast::Sender<()>,
    tokio::task::JoinHandle<()>,
) {
    // activate self every second
    let (quit_tx, mut quit_rx) = tokio::sync::broadcast::channel(1);
    let handle = tokio::spawn(async move {
        let mut service_rpc_client =
            rpc_client::PlcServiceRpcClient::new(&conf.service_host, conf.service_port);

        let req = plc::ActivateAdapterRequest {
            version: plc::ActivateAdapterVersion::ActivateAdapter20240930.into(),
            r#type: Some(plc::DeviceType {
                id: conf.device_type_id,
                name: conf.device_type_name,
            }),
            rpc_host: conf.my_ip,
            rpc_port: conf.port as u32,
        };

        let mut times: u16 = 0;
        loop {
            tokio::select! {
                _ = quit_rx.recv() => {
                    break;
                },
                _ = tokio::time::sleep(tokio::time::Duration::from_secs(1)) => {
                    if times % 60 == 0 {
                        tracing::info!("activate_adapter begin, times: {}", times);
                    }
                    let _ = service_rpc_client.activate_adapter(req.clone()).await;
                    if times % 60 == 0 {
                        tracing::info!("activate_adapter end, times: {}", times);
                    }
                    times += 1;
                }
            }
        }
    });

    return (quit_tx, handle);
}

async fn stop_background_tasks(
    quit_tx: tokio::sync::broadcast::Sender<()>,
    handle: tokio::task::JoinHandle<()>,
) {
    // notify
    tracing::warn!("background tasks stopping");
    let _ = quit_tx.send(());
    // wait to quit
    let _ = tokio::join!(handle);
    tracing::warn!("background tasks stopped");
}
