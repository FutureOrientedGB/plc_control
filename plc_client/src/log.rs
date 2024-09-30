use chrono::Local;

use tracing::Level;
use tracing_appender;
use tracing_subscriber::{self, fmt::time::OffsetTime};

use time::{macros::format_description, UtcOffset};

pub fn open_log_file(conf: &super::conf::Conf) {
    let log_dir = format!(
        "{}/log",
        std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .to_owned()
            .into_os_string()
            .into_string()
            .unwrap()
            .to_string()
    );
    tracing_subscriber::fmt()
        .with_writer(tracing_appender::rolling::daily(
            &log_dir,
            format!("{}.{}.log", &conf.app_name, &conf.service_port),
        ))
        .with_max_level(Level::INFO)
        .with_timer(OffsetTime::new(
            UtcOffset::from_hms(8, 0, 0).unwrap(),
            format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]"),
        ))
        .with_line_number(true)
        .with_thread_ids(true)
        .with_ansi(false)
        .init();

    println!(
        "loggging to: {}/{}.{}.log.{}",
        &log_dir,
        &conf.app_name,
        &conf.service_port,
        Local::now().format("%Y-%m-%d")
    );

    tracing::info!(
        conf.app_name,
        conf.app_version,
        conf.service_host,
        conf.service_port,
        "open daily log file"
    );
}
