use crate::metrics::SystemInfo;
use redis::{Client, Commands, RedisResult};
use std::collections::HashMap;

pub struct RedisStreamClient {
    client: Client,
    stream_key: String,
    group_name: String,
}

impl RedisStreamClient {
    pub fn new(redis_url: &str, stream_key: &str, group_name: &str) -> RedisResult<Self> {
        let client = Client::open(redis_url)?;
        let stream_client = Self {
            client,
            stream_key: stream_key.to_string(),
            group_name: group_name.to_string(),
        };
        stream_client.create_stream_and_consumer_group()?;
        Ok(stream_client)
    }

    fn create_stream_and_consumer_group(&self) -> RedisResult<()> {
        let mut con = self.client.get_connection()?;

        // Create the stream if it doesn't exist and set retention policy to 1 day
        con.xgroup_create_mkstream(&self.stream_key, &self.group_name, "$")?;

        // Set retention policy to 1 day (86400000 milliseconds)
        redis::cmd("XTRIM")
            .arg(&self.stream_key)
            .arg("MAXLEN")
            .arg("~")
            .arg("86400000")
            .query(&mut con)?;

        Ok(())
    }

    pub fn publish_system_metrics(&self, system_info: &SystemInfo) -> RedisResult<String> {
        let mut con = self.client.get_connection()?;

        let mut fields: HashMap<String, String> = HashMap::new();
        fields.insert("name".to_string(), system_info.name.clone());
        fields.insert("host".to_string(), system_info.host.clone());
        fields.insert("timestamp".to_string(), system_info.timestamp.to_string());
        fields.insert("cpu_usage".to_string(), system_info.cpu.usage.to_string());
        fields.insert(
            "memory_total".to_string(),
            system_info.memory.total.to_string(),
        );
        fields.insert(
            "memory_used".to_string(),
            system_info.memory.used.to_string(),
        );
        fields.insert(
            "memory_free".to_string(),
            system_info.memory.free.to_string(),
        );

        // Add disk metrics
        for (disk_name, disk_metrics) in &system_info.disk {
            fields.insert(
                format!("disk_{}_total", disk_name),
                disk_metrics.total.to_string(),
            );
            fields.insert(
                format!("disk_{}_used", disk_name),
                disk_metrics.used.to_string(),
            );
            fields.insert(
                format!("disk_{}_free", disk_name),
                disk_metrics.free.to_string(),
            );
        }

        // Publish to Redis stream
        let result: String = con.xadd(
            &self.stream_key,
            "*",
            &fields.into_iter().collect::<Vec<_>>(),
        )?;

        // Trim the stream to keep only the last day of data
        redis::cmd("XTRIM")
            .arg(&self.stream_key)
            .arg("MAXLEN")
            .arg("~")
            .arg("86400000")
            .query(&mut con)?;

        Ok(result)
    }

    pub fn read_system_metrics(
        &self,
        consumer_name: &str,
    ) -> RedisResult<Vec<HashMap<String, String>>> {
        let mut con = self.client.get_connection()?;

        let result: Vec<HashMap<String, String>> = redis::cmd("XREADGROUP")
            .arg(&self.group_name)
            .arg(consumer_name)
            .arg(&[(&self.stream_key, ">")])
            .query(&mut con)?;

        Ok(result)
    }
}
