#![recursion_limit = "256"]

use chrono::Local;
use dotenv::dotenv;
use local_ip_address::linux::local_ip;
use proto::{
    system_info_create_request::Body, system_monitor_client::SystemMonitorClient,
    SystemInfoContext, SystemInfoCreateRequest,
};
use std::{error::Error, time::Duration};
use sysinfo::System;
use tokio::time;
use tonic::Request;
mod grpc;
mod metrics;
mod proto {
    tonic::include_proto!("sysinfo");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let grpc_port = std::env::var("GRPC_PORT").expect("Expected grpc port to be defined");
    let grpc_host = std::env::var("GRPC_HOST").expect("Expected grpc host to be defined");
    let address = format!("{grpc_host}:{grpc_port}");
    let mut client = SystemMonitorClient::connect(address).await?;
    let message = async_stream::stream! {
        let mut interval = time::interval(Duration::from_secs(1));
        let system_ctx = SystemInfoCreateRequest {
            body: Some(Body::Context(
                SystemInfoContext {
                    ip: local_ip().unwrap().to_string(),
                    start_time: Local::now().to_rfc3339(),
                },
            )),
        };

        yield system_ctx;

        while let time = interval.tick().await {
            let mut sys = System::new_all();

            let system = metrics::SystemInfo::get_system_info(&sys);
            let data = serde_json::to_string(&system).unwrap();
            let system_ctx = SystemInfoCreateRequest {
                body: Some(Body::Data(data.as_bytes().to_vec())),
            };

            yield system_ctx;
       }
    };

    let request = Request::new(message);

    client
        .send_system_info(request)
        .await
        .map(|response| println!("RESPONSE {:?}", response.get_ref()))?;

    Ok(())
}
