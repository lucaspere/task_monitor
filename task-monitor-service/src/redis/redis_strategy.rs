use crate::metrics::SystemInfo;
use crate::redis::stream::publish_system_metrics;
use crate::strategy::DataSendStrategy;
use async_trait::async_trait;
use redis::Client;

use super::stream::create_stream_and_consumer_group;

pub struct RedisStrategy {
    client: Client,
    stream_key: String,
}

impl RedisStrategy {
    pub fn new(redis_url: &str, stream_key: &str) -> Result<Self, redis::RedisError> {
        let client = Client::open(redis_url)?;
        create_stream_and_consumer_group(&client, stream_key, "metrics")?;
        Ok(Self {
            client,
            stream_key: stream_key.to_string(),
        })
    }
}

#[async_trait]
impl DataSendStrategy for RedisStrategy {
    async fn send_data(&self, system_info: &SystemInfo) -> Result<(), Box<dyn std::error::Error>> {
        publish_system_metrics(&self.client, &self.stream_key, system_info)?;
        Ok(())
    }
}
