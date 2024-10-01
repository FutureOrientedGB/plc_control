use tokio::sync::broadcast;
use std::pin::Pin;
use std::future::Future;

pub struct PlcTimer {
    quit_rx: tokio::sync::broadcast::Receiver<()>,
    task_infos: Vec<(String, u64, Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>> + Send>)>, // (name, interval_ms, task fn)
    task_ticks: std::collections::BinaryHeap<(tokio::time::Instant, usize)>, // (next execute time, task index)
}

impl PlcTimer {
    pub fn new(quit_rx: tokio::sync::broadcast::Receiver<()>) -> Self {
        Self {
            task_infos: Vec::new(),
            task_ticks: std::collections::BinaryHeap::new(),
            quit_rx: quit_rx,
        }
    }

    pub fn add<F>(&mut self, name: &str, interval_ms: u64, task: F)
    where
        F: Fn() -> Pin<Box<dyn Future<Output = ()>> + Send + 'static,
    {
        let execute_at =
            tokio::time::Instant::now() + tokio::time::Duration::from_millis(interval_ms);
        self.task_infos.push((name.to_string(), interval_ms, Box::new(task)));
        self.task_ticks
            .push((execute_at, self.task_infos.len() - 1));
    }

    pub async fn run(&mut self) {
        while let Some((next_time, _)) = self.task_ticks.peek() {
            let now = tokio::time::Instant::now();
            if *next_time > now {
                tokio::select! {
                    _ = self.quit_rx.recv() => {
                        tracing::warn!("PlcTimer stopped");
                        return;
                    },
                    _ = tokio::time::sleep_until(*next_time) => {}
                }
            }
            self.execute();
        }
    }

    fn execute(&mut self) {
        let now = tokio::time::Instant::now();
        while let Some(&(next_time, task_index)) = self.task_ticks.peek() {
            if next_time > now {
                break;
            }
            let task = &self.task_infos[task_index];

            tracing::info!(
                "execute task begin, index: {}, name: {}",
                task_index,
                task.0
            );
            task.1();
            tracing::info!("execute task end, index: {}, name: {}", task_index, task.0);

            self.task_ticks.pop();
            self.task_ticks
                .push((now + tokio::time::Duration::from_millis(task.1), task_index));
        }
    }
}