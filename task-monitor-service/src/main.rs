#![recursion_limit = "256"]
use config::Config;
use dotenv::dotenv;
use std::error::Error;

mod metrics;
mod redis;
mod strategy;
mod proto {
    tonic::include_proto!("sysinfo");
}
mod config;
mod grpc;

use grpc::grpc_strategy::GrpcStrategy;
use redis::redis_strategy::RedisStrategy;
use strategy::DataSendStrategy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    // Choose the strategy based on environment variable or command-line argument
    let strategy = if std::env::var("USE_REDIS").is_ok() {
        let redis_url = std::env::var("REDIS_URL").unwrap_or("redis://127.0.0.1/".to_string());
        let stream_key = std::env::var("REDIS_STREAM_KEY").unwrap_or("system_metrics".to_string());
        Box::new(RedisStrategy::new(&redis_url, &stream_key)?) as Box<dyn DataSendStrategy>
    } else {
        let grpc_host = std::env::var("GRPC_HOST").unwrap_or("0.0.0.0".to_string());
        let grpc_port = std::env::var("GRPC_PORT").unwrap_or("50051".to_string());
        let grpc_address = format!("{}:{}", grpc_host, grpc_port);
        Box::new(GrpcStrategy::new(grpc_address).await?) as Box<dyn DataSendStrategy>
    };

    let config = Config::new(strategy);
    config.run().await?;

    Ok(())
}
