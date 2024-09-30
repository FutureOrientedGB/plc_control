use chrono::Local;

use tracing::Level;
use tracing_appender;
use tracing_subscriber::{self, fmt::time::OffsetTime};

use time::{macros::format_description, UtcOffset};

pub fn open_log_file(app_name: &str, app_version: &str, app_port: u16) {
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
            format!("{}.{}.log", &app_name, app_port),
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
        &app_name,
        app_port,
        Local::now().format("%Y-%m-%d")
    );

    tracing::info!(app_name, app_version, app_port, "open daily log file");
}
