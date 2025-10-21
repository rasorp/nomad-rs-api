use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AllocationStub {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "EvalID")]
    pub eval_id: String,
    pub name: String,
    pub namespace: String,
    #[serde(rename = "NodeID")]
    pub node_id: String,
    pub node_name: String,
    #[serde(rename = "JobID")]
    pub job_id: String,
    pub job_type: String,
    pub job_version: u64,
    pub task_group: String,
    pub desired_status: String,
    pub desired_description: String,
    pub client_status: String,
    pub client_description: String,
    pub task_states: Option<HashMap<String, AllocationTaskState>>,
    #[serde(rename = "DeploymentID")]
    pub deployment_id: Option<String>,
    pub deployment_status: Option<AllocationDeploymentStatus>,
    pub followup_eval_id: Option<String>,
    pub preempted_allocations: Option<Vec<String>>,
    pub preempted_by_allocation: String,
    pub create_index: u64,
    pub modify_index: u64,
    pub create_time: i64,
    pub modify_time: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AllocationTaskState {
    pub state: String,
    pub failed: bool,
    pub restarts: u64,
    pub last_restart: Option<String>,
    pub start_at: Option<String>,
    pub finish_at: Option<String>,
    pub events: Option<Vec<AllocationTaskEvent>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AllocationTaskEvent {
    #[serde(rename = "Type")]
    pub type_: String,
    pub time: i64,
    pub display_message: String,
    pub details: Option<HashMap<String, String>>,
    pub message: String,
    pub signal: i32,
    pub exit_code: i32,
    pub driver_error: String,
    pub kill_timeout: u64,
    pub kill_error: String,
    pub kill_reason: String,
    pub restart_reason: String,
    pub setup_error: String,
    pub driver_message: String,
    pub task_signal_reason: String,
    pub task_signal: String,
    pub download_error: String,
    pub validation_error: String,
    pub disk_limit: i64,
    pub disk_size: Option<i64>,
    pub failed_sibling: String,
    pub vault_error: String,
    pub generic_source: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AllocationDeploymentStatus {
    pub healthy: Option<bool>,
    pub timestamp: String,
    pub canary: bool,
    pub modify_index: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AllocationMetric {
    pub nodes_evaluated: i32,
    pub nodes_filtered: i32,
    pub nodes_available: HashMap<String, i32>,
    pub class_filtered: Option<HashMap<String, i32>>,
    pub constraint_filtered: Option<HashMap<String, i32>>,
    pub nodes_exhausted: i32,
    pub class_exhausted: Option<HashMap<String, i32>>,
    pub dimension_exhausted: Option<HashMap<String, i32>>,
    pub quota_exhausted: Option<Vec<String>>,
    pub resources_exhausted: Option<HashMap<String, AllocationResourceExhausted>>,
    pub scores_meta: Option<HashMap<String, f64>>,
    pub allocation_time: i64,
    pub coalesced_failures: i32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AllocationResourceExhausted {
    #[serde(rename = "CPU")]
    pub cpu: i32,
    #[serde(rename = "MemoryMB")]
    pub memory_mb: i32,
    pub disk_mb: i32,
}
