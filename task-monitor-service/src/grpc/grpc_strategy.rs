use crate::metrics::SystemInfo;
use crate::proto::{
    system_info_create_request::Body, system_monitor_client::SystemMonitorClient,
    SystemInfoCreateRequest,
};
use crate::strategy::DataSendStrategy;
use async_trait::async_trait;
use futures::stream;
use tokio::sync::Mutex;
use tonic::Request;

pub struct GrpcStrategy {
    client: Mutex<SystemMonitorClient<tonic::transport::Channel>>,
}

impl GrpcStrategy {
    pub async fn new(address: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = SystemMonitorClient::connect(address).await?;
        Ok(Self {
            client: Mutex::new(client),
        })
    }
}

#[async_trait]
impl DataSendStrategy for GrpcStrategy {
    async fn send_data(&self, system_info: &SystemInfo) -> Result<(), Box<dyn std::error::Error>> {
        let data = serde_json::to_string(system_info)?;
        let request = SystemInfoCreateRequest {
            body: Some(Body::Data(data.into_bytes())),
        };

        let stream = stream::once(async move { request });
        let response = self
            .client
            .lock()
            .await
            .send_system_info(Request::new(stream))
            .await?;

        println!("RESPONSE: {:?}", response);
        Ok(())
    }
}
