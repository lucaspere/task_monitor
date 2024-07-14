use chrono::Local;
use serde::{Deserialize, Serialize};
use sysinfo::System;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpuMetrics {
    /// The CPU usage in percentage.
    pub usage: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryMetrics {
    /// The total memory in bytes.
    pub total: u64,
    /// The memory used in bytes.
    pub used: u64,
    /// The memory free in bytes.
    pub free: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemMetrics {
    pub cpu: CpuMetrics,
    pub memory: MemoryMetrics,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Represents system information including CPU and memory usage.
pub struct SystemInfo {
    /// The name of the system.
    pub name: String,
    /// The host name of the system.
    pub host: String,
    /// The timestamp of the metrics.
    pub timestamp: u64,
    /// The CPU metrics.
    pub cpu: CpuMetrics,
    /// The memory metrics.
    pub memory: MemoryMetrics,
}

impl SystemInfo {
    pub fn new(
        name: String,
        host: String,
        timestamp: u64,
        cpu: CpuMetrics,
        memory: MemoryMetrics,
    ) -> Self {
        Self {
            name,
            host,
            timestamp,
            cpu,
            memory,
        }
    }

    pub fn get_system_info(sys: &System) -> Self {
        return Self::new(
            System::name().unwrap(),
            System::host_name().unwrap(),
            Local::now().timestamp() as u64,
            CpuMetrics {
                usage: sys.global_cpu_info().cpu_usage(),
            },
            MemoryMetrics {
                total: sys.total_memory(),
                used: sys.used_memory(),
                free: sys.free_memory(),
            },
        );
    }
}
