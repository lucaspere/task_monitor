use crate::metrics::SystemInfo;
use async_trait::async_trait;

#[async_trait]
pub trait DataSendStrategy: Send + Sync {
    async fn send_data(&self, system_info: &SystemInfo) -> Result<(), Box<dyn std::error::Error>>;
}
