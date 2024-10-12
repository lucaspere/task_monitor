use std::{error::Error, time::Duration};
use sysinfo::System;
use tokio::time;

use crate::{
    metrics::{self},
    strategy::DataSendStrategy,
};

pub struct Config {
    pub interval: u64,
    pub strategy: Box<dyn DataSendStrategy>,
}

impl Config {
    pub fn new(strategy: Box<dyn DataSendStrategy>) -> Self {
        let interval = std::env::var("INTERVAL").unwrap_or("1".to_string());
        Self {
            interval: interval.parse().expect("INTERVAL must be a number"),
            strategy,
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut interval = time::interval(Duration::from_secs(self.interval));

        loop {
            interval.tick().await;
            let mut sys = System::new_all();
            let system_info = metrics::SystemInfo::get_system_info(&mut sys);

            self.strategy.send_data(&system_info).await?;
        }
    }
}
