use std::error::Error;

use tonic::transport::Channel;

use crate::proto::system_monitor_client::SystemMonitorClient;

pub struct SystemMonitorService {
    client: SystemMonitorClient<Channel>,
}

impl SystemMonitorService {
    pub async fn new(address: String) -> Result<Self, Box<dyn Error>> {
        let client = SystemMonitorClient::connect(address).await?;
        Ok(Self { client })
    }
}
