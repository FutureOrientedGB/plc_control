use clap::Parser;

use tokio;

use tonic;

use stdext::function_name;

use plc_log::open_log_file;
use plc_proto::plc;

pub mod conf;
pub mod rpc;
pub mod store;
pub mod version;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse conf first from file, then from command lines
    let mut conf = conf::Conf::parse();
    conf.update(&version::APP_NAME, &version::APP_VERSION);
    let addr = format!("{}:{}", &conf.host, &conf.port);

    // init log
    open_log_file(&conf.app_name, &conf.app_version, conf.port);

    // init rpc service
    let mut plc_service = rpc::handler::MyPlcService::new(conf.clone());
    if plc_service.connect_store() {
        // background tasks
        let (quit_background_tasks_tx, task_handle) = run_background_tasks(conf).await;

        // serve rpc
        tonic::transport::Server::builder()
            .add_service(plc::plc_service_server::PlcServiceServer::new(plc_service))
            .serve(addr.parse().unwrap())
            .await?;

        // stop background tasks
        stop_background_tasks(quit_background_tasks_tx, task_handle).await;
    }

    Ok(())
}

async fn run_background_tasks(
    conf: conf::Conf,
) -> (
    tokio::sync::broadcast::Sender<()>,
    tokio::task::JoinHandle<()>,
) {
    // remove expired adapter every 30 seconds
    let (quit_tx, mut quit_rx) = tokio::sync::broadcast::channel(1);
    let handle = tokio::spawn(async move {
        let mut times: u16 = 0;
        let mut seconds = 0;
        let mut store = store::RedisStore::new(conf);
        let _ = store.open();
        loop {
            tokio::select! {
                _ = quit_rx.recv() => {
                    break;
                },
                _ = tokio::time::sleep(tokio::time::Duration::from_secs(seconds)) => {
                    seconds = 30;
                    if times % 60 == 0 {
                        tracing::info!(message = "remove_expired_adapter", func = function_name!(), end = false, times = times);
                    }
                    store.remove_expired_adapter().await;
                    if times % 60 == 0 {
                        tracing::info!(message = "remove_expired_adapter", func = function_name!(), end = true, times = times);
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
    tracing::warn!(func = function_name!(), end = false);
    let _ = quit_tx.send(());
    // wait to quit
    let _ = tokio::join!(handle);
    tracing::warn!(func = function_name!(), end = true);
}
