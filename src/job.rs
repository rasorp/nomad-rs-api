use crate::allocation::{AllocationMetric, AllocationStub};
use crate::deployment::Deployment;
use crate::evaluation::Evaluation;
use crate::option::{QueryOptions, WriteOptions};
use crate::{ClientError, Nomad};

use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Job type constants
pub const JOB_TYPE_SERVICE: &str = "service";
pub const JOB_TYPE_BATCH: &str = "batch";
pub const JOB_TYPE_SYSTEM: &str = "system";
pub const JOB_TYPE_SYSBATCH: &str = "sysbatch";

// Job priority default
pub const JOB_DEFAULT_PRIORITY: i32 = 50;

// Namespace default
pub const JOB_DEFAULT_NAMESPACE: &str = "default";

// Region default
pub const JOB_DEFAULT_REGION: &str = "global";

/// Job is the main structure representing a Nomad job.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Job {
    pub region: Option<String>,
    pub namespace: Option<String>,
    #[serde(rename = "ID")]
    pub id: Option<String>,
    pub name: String,
    #[serde(rename = "Type")]
    pub type_: Option<String>,
    pub priority: Option<i32>,
    pub all_at_once: Option<bool>,
    pub datacenters: Option<Vec<String>>,
    pub node_pool: Option<String>,
    pub constraints: Option<Vec<Constraint>>,
    pub affinities: Option<Vec<Affinity>>,
    pub task_groups: Vec<JobTaskGroup>,
    pub update: Option<JobUpdateStrategy>,
    pub multiregion: Option<JobMultiregion>,
    pub spreads: Option<Vec<JobSpread>>,
    pub periodic: Option<JobPeriodicConfig>,
    pub parameterized_job: Option<JobParameterizedConfig>,
    pub reschedule: Option<ReschedulePolicy>,
    pub migrate: Option<JobMigrateStrategy>,
    pub meta: Option<HashMap<String, String>>,
    #[serde(rename = "UI")]
    pub ui: Option<JobUIConfig>,

    // The fields below are set by the server and are not set when submitting a
    // job.
    pub stop: Option<bool>,
    pub parent_id: Option<String>,
    pub dispatched: Option<bool>,
    pub dispatch_idempotency_token: Option<String>,
    pub payload: Option<Vec<u8>>,
    pub consul_namespace: Option<String>,
    pub vault_namespace: Option<String>,
    pub nomad_token_id: Option<String>,
    pub status: Option<String>,
    pub status_description: Option<String>,
    pub stable: Option<bool>,
    pub version: Option<u64>,
    pub submit_time: Option<i64>,
    pub create_index: Option<u64>,
    pub modify_index: Option<u64>,
    pub job_modify_index: Option<u64>,
    pub version_tag: Option<JobVersionTag>,
}

impl Job {
    pub fn new(
        name: String,
        region: String,
        job_type: String,
        task_groups: Vec<JobTaskGroup>,
    ) -> Self {
        Self {
            name,
            region: Some(region),
            type_: Some(job_type.to_string()),
            task_groups,
            ..Default::default()
        }
    }
}

impl Default for Job {
    fn default() -> Self {
        Self {
            region: Some(JOB_DEFAULT_REGION.to_string()),
            namespace: Some(JOB_DEFAULT_NAMESPACE.to_string()),
            id: None,
            name: "".to_string(),
            type_: Some(JOB_TYPE_SERVICE.to_string()),
            priority: Some(JOB_DEFAULT_PRIORITY),
            all_at_once: None,
            datacenters: None,
            node_pool: None,
            constraints: None,
            affinities: None,
            task_groups: vec![JobTaskGroup::default()],
            update: None,
            multiregion: None,
            spreads: None,
            periodic: None,
            parameterized_job: None,
            reschedule: None,
            migrate: None,
            meta: None,
            ui: None,
            stop: None,
            parent_id: None,
            dispatched: None,
            dispatch_idempotency_token: None,
            payload: None,
            consul_namespace: None,
            vault_namespace: None,
            nomad_token_id: None,
            status: None,
            status_description: None,
            stable: None,
            version: None,
            submit_time: None,
            create_index: None,
            modify_index: None,
            job_modify_index: None,
            version_tag: None,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobStub {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
    pub name: String,
    pub namespace: String,
    pub datacenters: Vec<String>,
    #[serde(rename = "Type")]
    pub type_: String,
    pub priority: i32,
    pub periodic: bool,
    pub parameterized_job: bool,
    pub stop: bool,
    pub status: String,
    pub status_description: String,
    pub job_summary: Option<JobSummary>,
    pub create_index: u64,
    pub modify_index: u64,
    pub job_modify_index: u64,
    pub submit_time: i64,
    pub meta: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobSummary {
    #[serde(rename = "JobID")]
    pub job_id: String,
    pub namespace: String,
    pub summary: HashMap<String, JobTaskGroupSummary>,
    pub children: Option<JobSummaryChildren>,
    pub create_index: u64,
    pub modify_index: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobSummaryChildren {
    pub pending: i64,
    pub running: i64,
    pub dead: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobTaskGroupSummary {
    pub queued: i32,
    pub complete: i32,
    pub failed: i32,
    pub running: i32,
    pub starting: i32,
    pub lost: i32,
    pub unknown: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobUpdateStrategy {
    pub stagger: Option<u64>,
    pub max_parallel: Option<i32>,
    pub health_check: Option<String>,
    pub min_healthy_time: Option<u64>,
    pub healthy_deadline: Option<u64>,
    pub progress_deadline: Option<u64>,
    pub canary: Option<i32>,
    pub auto_revert: Option<bool>,
    pub auto_promote: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobMultiregion {
    pub strategy: Option<JobMultiregionStrategy>,
    pub regions: Option<Vec<JobMultiregionRegion>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobMultiregionStrategy {
    pub max_parallel: Option<i32>,
    pub on_failure: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobMultiregionRegion {
    pub name: String,
    pub count: Option<i32>,
    pub datacenters: Option<Vec<String>>,
    pub node_pool: Option<String>,
    pub meta: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobPeriodicConfig {
    pub enabled: Option<bool>,
    pub spec: Option<String>,
    pub specs: Option<Vec<String>>,
    pub spec_type: Option<String>,
    pub prohibit_overlap: Option<bool>,
    pub time_zone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobParameterizedConfig {
    pub payload: Option<String>,
    pub meta_required: Option<Vec<String>>,
    pub meta_optional: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReschedulePolicy {
    pub attempts: Option<i32>,
    pub interval: Option<u64>,
    pub delay: Option<u64>,
    pub delay_function: Option<String>,
    pub max_delay: Option<u64>,
    pub unlimited: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobMigrateStrategy {
    pub max_parallel: Option<i32>,
    pub health_check: Option<String>,
    pub min_healthy_time: Option<u64>,
    pub healthy_deadline: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Constraint {
    pub l_target: Option<String>,
    pub r_target: Option<String>,
    pub operand: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Affinity {
    pub l_target: Option<String>,
    pub r_target: Option<String>,
    pub operand: Option<String>,
    pub weight: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobSpread {
    pub attribute: Option<String>,
    pub weight: Option<i32>,
    pub spread_target: Option<Vec<JobSpreadTarget>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobSpreadTarget {
    pub value: String,
    pub percent: u8,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobTaskGroup {
    pub name: String,
    pub count: Option<i32>,
    pub constraints: Option<Vec<Constraint>>,
    pub affinities: Option<Vec<Affinity>>,
    pub tasks: Vec<Task>,
    pub spreads: Option<Vec<JobSpread>>,
    pub volumes: Option<HashMap<String, VolumeRequest>>,
    pub restart_policy: Option<RestartPolicy>,
    pub reschedule_policy: Option<ReschedulePolicy>,
    pub ephemeral_disk: Option<EphemeralDisk>,
    pub update: Option<JobUpdateStrategy>,
    pub migrate: Option<JobMigrateStrategy>,
    pub networks: Option<Vec<NetworkResource>>,
    pub meta: Option<HashMap<String, String>>,
    pub services: Option<Vec<Service>>,
    pub shutdown_delay: Option<u64>,
    pub stop_after_client_disconnect: Option<u64>,
    pub max_client_disconnect: Option<u64>,
    pub scaling: Option<ScalingPolicy>,
    pub consul_namespace: Option<String>,
}

impl JobTaskGroup {
    pub fn new(name: String, tasks: Vec<Task>) -> Self {
        Self {
            name,
            tasks,
            count: None,
            constraints: None,
            affinities: None,
            spreads: None,
            volumes: None,
            restart_policy: None,
            reschedule_policy: None,
            ephemeral_disk: None,
            update: None,
            migrate: None,
            networks: None,
            meta: None,
            services: None,
            shutdown_delay: None,
            stop_after_client_disconnect: None,
            max_client_disconnect: None,
            scaling: None,
            consul_namespace: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Task {
    pub name: String,
    pub driver: String,
    pub config: Option<HashMap<String, serde_json::Value>>,
    pub constraints: Option<Vec<Constraint>>,
    pub affinities: Option<Vec<Affinity>>,
    pub env: Option<HashMap<String, String>>,
    pub services: Option<Vec<Service>>,
    pub resources: Option<TaskResources>,
    pub meta: Option<HashMap<String, String>>,
    pub kill_timeout: Option<u64>,
    pub kill_signal: Option<String>,
    pub leader: Option<bool>,
    pub shutdown_delay: Option<u64>,
    pub user: Option<String>,
    pub lifecycle: Option<JobTaskLifecycle>,
    pub templates: Option<Vec<TaskTemplate>>,
    pub vault: Option<Vault>,
    pub dispatch_payload: Option<DispatchPayloadConfig>,
}

impl Task {
    pub fn new(name: String, driver: String) -> Self {
        Task {
            name,
            driver,
            config: None,
            constraints: None,
            affinities: None,
            env: None,
            services: None,
            resources: None,
            meta: None,
            kill_timeout: None,
            kill_signal: None,
            leader: None,
            shutdown_delay: None,
            user: None,
            lifecycle: None,
            templates: None,
            vault: None,
            dispatch_payload: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobTaskLifecycle {
    pub hook: String,
    pub sidecar: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TaskTemplate {
    pub source_path: Option<String>,
    pub dest_path: Option<String>,
    pub embedded_tmpl: Option<String>,
    pub change_mode: Option<String>,
    pub change_signal: Option<String>,
    pub splay: Option<u64>,
    pub perms: Option<String>,
    pub left_delim: Option<String>,
    pub right_delim: Option<String>,
    pub envvars: Option<bool>,
    pub vault_grace: Option<u64>,
    pub wait: Option<TemplateWaitConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TemplateWaitConfig {
    pub min: Option<u64>,
    pub max: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Vault {
    pub policies: Option<Vec<String>>,
    pub namespace: Option<String>,
    pub env: Option<bool>,
    pub change_mode: Option<String>,
    pub change_signal: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DispatchPayloadConfig {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TaskResources {
    #[serde(rename = "CPU")]
    pub cpu: Option<i64>,
    pub cores: Option<i64>,
    #[serde(rename = "Memory")]
    pub memory_mb: Option<i64>,
    #[serde(rename = "MemoryMax")]
    pub memory_max_mb: Option<i64>,
    #[serde(rename = "Disk")]
    pub disk_mb: Option<i64>,
    pub networks: Option<Vec<NetworkResource>>,
    pub devices: Option<Vec<RequestedDevice>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkResource {
    pub mode: Option<String>,
    pub device: Option<String>,
    #[serde(rename = "CIDR")]
    pub cidr: Option<String>,
    #[serde(rename = "IP")]
    pub ip: Option<String>,
    pub mbits: Option<i32>,
    pub dns: Option<DNSConfig>,
    pub reserved_ports: Option<Vec<Port>>,
    pub dynamic_ports: Option<Vec<Port>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DNSConfig {
    pub servers: Option<Vec<String>>,
    pub searches: Option<Vec<String>>,
    pub options: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Port {
    pub label: String,
    pub value: Option<i32>,
    pub to: Option<i32>,
    pub host_network: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequestedDevice {
    pub name: String,
    pub count: Option<u64>,
    pub constraints: Option<Vec<Constraint>>,
    pub affinities: Option<Vec<Affinity>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Service {
    pub name: String,
    pub tags: Option<Vec<String>>,
    pub canary_tags: Option<Vec<String>>,
    pub port_label: Option<String>,
    pub address_mode: Option<String>,
    pub checks: Option<Vec<ServiceCheck>>,
    pub check_restart: Option<CheckRestart>,
    pub connect: Option<ConsulConnect>,
    pub meta: Option<HashMap<String, String>>,
    pub canary_meta: Option<HashMap<String, String>>,
    pub enable_tag_override: Option<bool>,
    pub on_update: Option<String>,
    pub provider: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceCheck {
    pub name: Option<String>,
    #[serde(rename = "Type")]
    pub type_: String,
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub path: Option<String>,
    pub protocol: Option<String>,
    pub port_label: Option<String>,
    pub address_mode: Option<String>,
    pub interval: Option<u64>,
    pub timeout: Option<u64>,
    pub initial_status: Option<String>,
    pub tls_skip_verify: Option<bool>,
    pub method: Option<String>,
    pub header: Option<HashMap<String, Vec<String>>>,
    pub check_restart: Option<CheckRestart>,
    pub grpc_service: Option<String>,
    pub grpc_use_tls: Option<bool>,
    pub success_before_passing: Option<i32>,
    pub failures_before_critical: Option<i32>,
    pub body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CheckRestart {
    pub limit: Option<i32>,
    pub grace: Option<u64>,
    pub ignore_warnings: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConsulConnect {
    pub native: Option<bool>,
    pub gateway: Option<ConsulGateway>,
    pub sidecar_service: Option<ConsulSidecarService>,
    pub sidecar_task: Option<SidecarTask>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConsulGateway {
    pub proxy: Option<ConsulGatewayProxy>,
    pub ingress: Option<ConsulIngressGateway>,
    pub terminating: Option<ConsulTerminatingGateway>,
    pub mesh: Option<ConsulMeshGateway>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConsulGatewayProxy {
    pub connect_timeout: Option<u64>,
    pub envoy_gateway_bind_tagged_addresses: Option<bool>,
    pub envoy_gateway_bind_addresses: Option<HashMap<String, ConsulGatewayBindAddress>>,
    pub envoy_gateway_no_default_bind: Option<bool>,
    pub config: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConsulGatewayBindAddress {
    pub address: String,
    pub port: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConsulIngressGateway {
    pub tls: Option<ConsulGatewayTLSConfig>,
    pub listeners: Option<Vec<ConsulIngressListener>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConsulGatewayTLSConfig {
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConsulIngressListener {
    pub port: i32,
    pub protocol: String,
    pub services: Vec<ConsulIngressService>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConsulIngressService {
    pub name: String,
    pub hosts: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConsulTerminatingGateway {
    pub services: Vec<ConsulLinkedService>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConsulMeshGateway {
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConsulLinkedService {
    pub name: String,
    #[serde(rename = "CAFile")]
    pub ca_file: Option<String>,
    pub cert_file: Option<String>,
    pub key_file: Option<String>,
    #[serde(rename = "SNI")]
    pub sni: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConsulSidecarService {
    pub port: Option<String>,
    pub proxy: Option<ConsulProxy>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConsulProxy {
    pub local_service_address: Option<String>,
    pub local_service_port: Option<i32>,
    pub config: Option<HashMap<String, serde_json::Value>>,
    pub upstreams: Option<Vec<ConsulUpstream>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConsulUpstream {
    pub destination_name: String,
    pub local_bind_port: i32,
    pub datacenter: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SidecarTask {
    pub name: Option<String>,
    pub driver: Option<String>,
    pub user: Option<String>,
    pub config: Option<HashMap<String, serde_json::Value>>,
    pub env: Option<HashMap<String, String>>,
    pub resources: Option<TaskResources>,
    pub meta: Option<HashMap<String, String>>,
    pub kill_timeout: Option<u64>,
    pub kill_signal: Option<String>,
    pub shutdown_delay: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RestartPolicy {
    pub attempts: Option<i32>,
    pub interval: Option<u64>,
    pub delay: Option<u64>,
    pub mode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EphemeralDisk {
    pub migrate: Option<bool>,
    pub size_mb: Option<i32>,
    pub sticky: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VolumeRequest {
    pub name: String,
    #[serde(rename = "Type")]
    pub type_: String,
    pub source: String,
    pub read_only: Option<bool>,
    pub mount_options: Option<VolumeMount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VolumeMount {
    pub fs_type: Option<String>,
    pub mount_flags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ScalingPolicy {
    pub enabled: Option<bool>,
    pub min: Option<i64>,
    pub max: Option<i64>,
    pub policy: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobUIConfig {
    pub description: Option<String>,
    pub links: Option<Vec<JobUILink>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobUILink {
    pub label: String,
    #[serde(rename = "URL")]
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobVersionTag {
    pub name: String,
    pub description: Option<String>,
    pub tagged_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobSubmission {
    pub source: String,
    pub format: String,
    pub variable_flags: Option<HashMap<String, String>>,
    pub variables: Option<String>,
}

#[derive(Debug, Default)]
pub struct RegisterOptions {
    pub enforce_index: bool,
    pub modify_index: u64,
    pub policy_override: bool,
    pub preserve_counts: bool,
    pub preserve_resources: bool,
    pub eval_priority: i32,
    pub submission: Option<JobSubmission>,
}

#[derive(Debug, Clone)]
pub struct JobDeregisterRequest {
    pub job_id: String,
    pub purge: bool,
    pub global: bool,
    pub eval_priority: i32,
    pub no_shutdown_delay: bool,
}

impl JobDeregisterRequest {
    pub fn new(job_id: String) -> Self {
        JobDeregisterRequest {
            job_id,
            purge: false,
            global: false,
            eval_priority: 0,
            no_shutdown_delay: false,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobRegisterRequest<'a> {
    pub job: &'a Job,
    pub enforce_index: Option<bool>,
    pub job_modify_index: Option<u64>,
    pub policy_override: Option<bool>,
    pub preserve_counts: Option<bool>,
    pub preserve_resources: Option<bool>,
    pub eval_priority: Option<i32>,
    pub submission: Option<JobSubmission>,
}

impl<'a> JobRegisterRequest<'a> {
    pub fn new(job: &'a Job) -> Self {
        JobRegisterRequest {
            job,
            enforce_index: None,
            job_modify_index: None,
            policy_override: None,
            preserve_counts: None,
            preserve_resources: None,
            eval_priority: None,
            submission: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobRegisterResponse {
    #[serde(rename = "EvalID")]
    pub eval_id: String,
    pub eval_create_index: u64,
    pub job_modify_index: u64,
    pub warnings: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobDeregisterResponse {
    #[serde(rename = "EvalID")]
    pub eval_id: String,
    pub eval_create_index: u64,
    pub job_modify_index: u64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobValidateRequest<'a> {
    pub job: &'a Job,
}

impl<'a> JobValidateRequest<'a> {
    pub fn new(job: &'a Job) -> Self {
        Self { job }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobValidateResponse {
    pub driver_config_validated: bool,
    pub validation_errors: Vec<String>,
    pub error: Option<String>,
    pub warnings: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobPlanRequest<'a> {
    pub job: &'a Job,
    pub diff: bool,
    pub policy_override: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobPlanResponse {
    pub job_modify_index: u64,
    pub created_evals: Vec<Evaluation>,
    pub diff: Option<JobDiff>,
    pub annotations: Option<PlanAnnotations>,
    pub failed_tg_allocs: Option<HashMap<String, AllocationMetric>>,
    pub next_periodic_launch: Option<String>,
    pub warnings: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobDiff {
    #[serde(rename = "Type")]
    pub type_: String,
    #[serde(rename = "ID")]
    pub id: String,
    pub fields: Option<Vec<FieldDiff>>,
    pub objects: Option<Vec<ObjectDiff>>,
    pub task_groups: Option<Vec<TaskGroupDiff>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TaskGroupDiff {
    #[serde(rename = "Type")]
    pub type_: String,
    pub name: String,
    pub fields: Option<Vec<FieldDiff>>,
    pub objects: Option<Vec<ObjectDiff>>,
    pub tasks: Option<Vec<TaskDiff>>,
    pub updates: Option<HashMap<String, u64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TaskDiff {
    #[serde(rename = "Type")]
    pub type_: String,
    pub name: String,
    pub fields: Option<Vec<FieldDiff>>,
    pub objects: Option<Vec<ObjectDiff>>,
    pub annotations: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FieldDiff {
    #[serde(rename = "Type")]
    pub type_: String,
    pub name: String,
    pub old: String,
    pub new: String,
    pub annotations: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectDiff {
    #[serde(rename = "Type")]
    pub type_: String,
    pub name: String,
    pub fields: Option<Vec<FieldDiff>>,
    pub objects: Option<Vec<ObjectDiff>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlanAnnotations {
    pub desired_tg_updates: Option<HashMap<String, DesiredUpdates>>,
    pub preempted_allocs: Option<Vec<AllocationStub>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DesiredUpdates {
    pub ignore: u64,
    pub place: u64,
    pub migrate: u64,
    pub stop: u64,
    pub in_place_update: u64,
    pub destructive_update: u64,
    pub canary: u64,
    pub preemptions: u64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobListDeploymentsRequest {
    #[serde(rename = "JobID")]
    pub job_id: String,
    pub all: bool,
}

impl JobListDeploymentsRequest {
    pub fn new(job_id: String, all: bool) -> Self {
        Self { job_id, all }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobDispatchRequest {
    #[serde(rename = "JobID")]
    pub job_id: String,
    pub payload: Option<Vec<u8>>,
    pub meta: Option<HashMap<String, String>>,
    pub id_prefix_template: Option<String>,
    pub priority: Option<i32>,
}

impl JobDispatchRequest {
    pub fn new(job_id: String) -> Self {
        JobDispatchRequest {
            job_id,
            payload: None,
            meta: None,
            id_prefix_template: None,
            priority: None,
        }
    }
    pub fn with_payload(mut self, payload: Vec<u8>) -> Self {
        self.payload = Some(payload);
        self
    }
    pub fn with_meta(mut self, meta: HashMap<String, String>) -> Self {
        self.meta = Some(meta);
        self
    }
    pub fn with_id_prefix_template(mut self, id_prefix_template: String) -> Self {
        self.id_prefix_template = Some(id_prefix_template);
        self
    }
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = Some(priority);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobDispatchResponse {
    pub dispatched_job_id: String,
    #[serde(rename = "EvalID")]
    pub eval_id: String,
    pub eval_create_index: u64,
    pub job_create_index: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobVersionsResponse {
    pub versions: Vec<Job>,
    pub diffs: Option<Vec<JobDiff>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobRevertRequest {
    #[serde(rename = "JobID")]
    pub job_id: String,
    pub job_version: u64,
    pub enforce_prior_version: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobStabilityRequest {
    #[serde(rename = "JobID")]
    pub job_id: String,
    pub job_version: u64,
    pub stable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobStabilityResponse {
    pub job_modify_index: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobEvaluationForceRequest {
    #[serde(rename = "JobID")]
    pub job_id: String,
    pub force_reschedule: bool,
    pub eval_options: Option<JobEvaluationForce>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobEvaluationForce {
    pub force_reschedule: bool,
}

impl JobEvaluationForceRequest {
    pub fn new(job_id: String, force_reschedule: bool) -> Self {
        JobEvaluationForceRequest {
            job_id,
            force_reschedule,
            eval_options: None,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobAllocationsListRequest {
    #[serde(rename = "JobID")]
    pub job_id: String,
    pub all_allocs: bool,
}

impl JobAllocationsListRequest {
    pub fn new(job_id: String, all_allocs: bool) -> Self {
        Self { job_id, all_allocs }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ScalingRequest {
    pub count: Option<i64>,
    pub target: HashMap<String, String>,
    pub error: Option<bool>,
    pub message: Option<String>,
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TaskGroupScaleStatus {
    pub desired: i32,
    pub placed: i32,
    pub running: i32,
    pub healthy: i32,
    pub unhealthy: i32,
    pub events: Option<Vec<ScalingEvent>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ScalingEvent {
    pub time: i64,
    pub count: Option<i64>,
    pub previous_count: i64,
    pub error: bool,
    pub message: Option<String>,
    pub meta: Option<HashMap<String, String>>,
    pub eval_id: Option<String>,
}

#[derive(Debug, Default)]
pub struct JobsListRequest {
    pub meta: Option<bool>,
}

pub struct Endpoint<'a> {
    client: &'a Nomad,
}

impl<'a> Endpoint<'a> {
    /// Create a new `Endpoint` with the given `Nomad` client to interact with
    /// the job endpoints.
    pub fn new(client: &'a Nomad) -> Self {
        Self { client }
    }

    /// Deregister a job.
    ///
    /// # Arguments
    /// * `job_deregister_request` - A reference to a `JobDeregisterRequest`
    ///   struct containing the job ID and deregistration options.
    /// * `opts` - Optional write options for the request.
    ///
    /// # Returns
    /// A `Result` containing a `JobDeregisterResponse` or a `ClientError`.
    pub async fn deregister(
        &self,
        job_deregister_request: &JobDeregisterRequest,
        opts: Option<WriteOptions>,
    ) -> Result<JobDeregisterResponse, ClientError> {
        let req = self.client.set_request_write_options(
            self.client.build_request(
                Method::DELETE,
                &format!(
                    "/v1/job/{}?purge={}&global={}&eval_priority={}&no_shutdown_delay={}",
                    job_deregister_request.job_id,
                    job_deregister_request.purge,
                    job_deregister_request.global,
                    job_deregister_request.eval_priority,
                    job_deregister_request.no_shutdown_delay
                ),
            ),
            &opts.unwrap_or_default(),
        );

        self.client
            .send_with_response::<JobDeregisterResponse>(req)
            .await
    }

    /// Dispatch a instance of a parameterized job.
    ///
    /// # Arguments
    /// * `job_dispatch_request` - The job dispatch request containing the job
    ///   ID and optional payload.
    /// * `opts` - Optional write options for the request.
    ///
    /// # Returns
    /// A `Result` containing a the job dispatch response or an error if the
    /// request fails.
    pub async fn dispatch(
        &self,
        job_dispatch_request: &JobDispatchRequest,
        opts: Option<WriteOptions>,
    ) -> Result<JobDispatchResponse, ClientError> {
        let req = self
            .client
            .set_request_write_options(
                self.client.build_request(
                    Method::POST,
                    &format!("/v1/job/{}/dispatch", job_dispatch_request.job_id),
                ),
                &opts.unwrap_or_default(),
            )
            .json(job_dispatch_request);

        self.client
            .send_with_response::<JobDispatchResponse>(req)
            .await
    }

    /// Force and evaluation of a job.
    ///
    /// # Arguments
    /// * `job_evaluation_force_request` - A reference to a
    ///   `JobEvaluationForceRequest` struct containing the job ID and force
    ///   evaluation options.
    /// * `opts` - Optional write options for the request.
    ///
    /// # Returns
    /// A `Result` containing a `JobRegisterResponse` or a `ClientError`.
    pub async fn force_evaluation(
        &self,
        job_evaluation_force_request: &JobEvaluationForceRequest,
        opts: Option<WriteOptions>,
    ) -> Result<JobRegisterResponse, ClientError> {
        let req = self
            .client
            .set_request_write_options(
                self.client.build_request(
                    Method::POST,
                    &format!("/v1/job/{}/evaluate", job_evaluation_force_request.job_id),
                ),
                &opts.unwrap_or_default(),
            )
            .json(job_evaluation_force_request);

        self.client
            .send_with_response::<JobRegisterResponse>(req)
            .await
    }

    /// Force an instance of a periodic job to run immediately.
    ///
    /// # Arguments
    /// * `job_id` - The ID of the periodic job to force run.
    /// * `opts` - Optional write options for the request.
    ///
    /// # Returns
    /// A `Result` containing a the periodic job registration response or an
    /// error if the request fails.
    pub async fn force_periodic(
        &self,
        job_id: &str,
        opts: Option<WriteOptions>,
    ) -> Result<JobRegisterResponse, ClientError> {
        let req = self.client.set_request_write_options(
            self.client
                .build_request(Method::POST, &format!("/v1/job/{}/periodic/force", job_id)),
            &opts.unwrap_or_default(),
        );

        self.client
            .send_with_response::<JobRegisterResponse>(req)
            .await
    }

    /// Get a specific job by its ID.
    ///
    /// # Arguments
    /// * `job_id` - A string representing the ID of the job to retrieve.
    /// * `opts` - Optional query options for the request.
    ///
    /// # Returns
    /// A `Result` containing a the job object or an error if the request
    /// fails.
    pub async fn get(&self, job_id: &str, opts: Option<QueryOptions>) -> Result<Job, ClientError> {
        let req = self.client.set_request_query_options(
            self.client
                .build_request(Method::GET, &format!("/v1/job/{}", job_id)),
            &opts.unwrap_or_default(),
        );
        self.client.send_with_response::<Job>(req).await
    }

    /// Get the latest deployment for a job.
    ///
    /// # Arguments
    /// * `job_id` - A string representing the ID of the job to query.
    /// * `opts` - Optional query options for the request.
    ///
    /// # Returns
    /// A `Result` containing the latest `Deployment` object or a `ClientError`.
    pub async fn get_latest_deployment(
        &self,
        job_id: &str,
        opts: Option<QueryOptions>,
    ) -> Result<Option<Deployment>, ClientError> {
        let req = self.client.set_request_query_options(
            self.client
                .build_request(Method::GET, &format!("/v1/job/{}/deployment", job_id)),
            &opts.unwrap_or_default(),
        );

        self.client
            .send_with_response::<Option<Deployment>>(req)
            .await
    }

    /// Get the summary of a job.
    ///
    /// # Arguments
    /// * `job_id` - A string representing the ID of the job to query.
    /// * `opts` - Optional query options for the request.
    ///
    /// # Returns
    /// A `Result` containing the `JobSummary` object or a `ClientError` if the
    /// request fails.
    pub async fn get_summary(
        &self,
        job_id: &str,
        opts: Option<QueryOptions>,
    ) -> Result<JobSummary, ClientError> {
        let req = self.client.set_request_query_options(
            self.client
                .build_request(Method::GET, &format!("/v1/job/{}/summary", job_id)),
            &opts.unwrap_or_default(),
        );

        self.client.send_with_response::<JobSummary>(req).await
    }

    /// List all registered jobs.
    ///
    /// # Arguments
    /// * `jobs_list_request` - An optional reference to a `JobsListRequest`
    ///   struct containing parameters for the request.
    /// * `opts` - Optional query options for the request.
    ///
    /// Returns a `Result` containing a vector of `JobStub` structs or a
    /// `ClientError`.
    pub async fn list(
        &self,
        jobs_list_request: Option<&JobsListRequest>,
        opts: Option<QueryOptions>,
    ) -> Result<Vec<JobStub>, ClientError> {
        // Determine if meta information is requested and set our meta value
        // accordingly for the query string.
        let meta = match jobs_list_request {
            Some(req) => req.meta.unwrap_or(false),
            None => false,
        };

        let req = self.client.set_request_query_options(
            self.client
                .build_request(Method::GET, &format!("/v1/jobs?meta={}", meta)),
            &opts.unwrap_or_default(),
        );
        let mut jobs = self.client.send_with_response::<Vec<JobStub>>(req).await?;
        jobs.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(jobs)
    }

    /// List all allocations for a job.
    ///
    /// # Arguments
    /// * `job_allocations_list_request` - A reference to a
    ///   `JobAllocationsListRequest` struct containing the job ID and allocation
    ///   options.
    /// * `opts` - Optional query options for the request.
    ///
    /// # Returns
    /// A `Result` containing a vector of `AllocationStub` structs or a
    /// `ClientError`.
    pub async fn list_allocations(
        &self,
        job_allocations_list_request: &JobAllocationsListRequest,
        opts: Option<QueryOptions>,
    ) -> Result<Vec<AllocationStub>, ClientError> {
        let req = self.client.set_request_query_options(
            self.client.build_request(
                Method::GET,
                &format!(
                    "/v1/job/{}/allocations?all={}",
                    job_allocations_list_request.job_id, job_allocations_list_request.all_allocs
                ),
            ),
            &opts.unwrap_or_default(),
        );

        let mut allocs = self
            .client
            .send_with_response::<Vec<AllocationStub>>(req)
            .await?;

        allocs.sort_by(|a, b| b.create_index.cmp(&a.create_index));
        Ok(allocs)
    }

    /// List all the deployments for a job.
    ///
    /// # Arguments
    /// * `job_list_deployments_request` - The job deployments list request
    ///   containing the job ID and deployment options.
    /// * `opts` - Optional query options for the request.
    ///
    /// # Returns
    /// A `Result` containing a vector of `Deployment` structs or a
    /// `ClientError`.
    pub async fn list_deployments(
        &self,
        job_list_deployments_request: &JobListDeploymentsRequest,
        opts: Option<QueryOptions>,
    ) -> Result<Vec<Deployment>, ClientError> {
        let req = self.client.set_request_query_options(
            self.client.build_request(
                Method::GET,
                &format!(
                    "/v1/job/{}/deployments?all={}",
                    job_list_deployments_request.job_id, job_list_deployments_request.all
                ),
            ),
            &opts.unwrap_or_default(),
        );

        let mut deployments = self
            .client
            .send_with_response::<Vec<Deployment>>(req)
            .await?;
        deployments.sort_by(|a, b| b.create_index.cmp(&a.create_index));
        Ok(deployments)
    }

    /// List all evaluations for a job.
    ///
    /// # Arguments
    /// * `job_id` - A string representing the ID of the job to query.
    /// * `opts` - Optional query options for the request.
    ///
    /// # Returns
    /// A `Result` containing a vector of `Evaluation` structs or a
    /// `ClientError`.
    pub async fn list_evaluations(
        &self,
        job_id: &str,
        opts: Option<QueryOptions>,
    ) -> Result<Vec<Evaluation>, ClientError> {
        let req = self.client.set_request_query_options(
            self.client
                .build_request(Method::GET, &format!("/v1/job/{}/evaluations", job_id)),
            &opts.unwrap_or_default(),
        );

        let mut evals = self
            .client
            .send_with_response::<Vec<Evaluation>>(req)
            .await?;
        evals.sort_by(|a, b| b.create_index.cmp(&a.create_index));
        Ok(evals)
    }

    /// Perform a job rgistration plan.
    ///
    /// # Arguments
    /// * `job_plan_request` - A reference to a `JobPlanRequest` struct
    ///   containing the job to plan.
    /// * `opts` - Optional write options for the request.
    ///
    /// # Returns
    /// A `Result` containing a `JobPlanResponse` or a `ClientError`.
    pub async fn plan(
        &self,
        job_plan_request: &JobPlanRequest<'_>,
        opts: Option<WriteOptions>,
    ) -> Result<JobPlanResponse, ClientError> {
        match job_plan_request.job.id {
            Some(_) => {}
            None => {
                return Err(ClientError::InvalidInputError(
                    "Job ID must be set".to_string(),
                ));
            }
        }

        let req = self
            .client
            .set_request_write_options(
                self.client.build_request(
                    Method::POST,
                    &format!("/v1/job/{}/plan", job_plan_request.job.id.as_ref().unwrap()),
                ),
                &opts.unwrap_or_default(),
            )
            .json(&job_plan_request);

        self.client.send_with_response::<JobPlanResponse>(req).await
    }

    /// Register is used to run a new job or update on existing job.
    ///
    /// # Arguments
    /// * `job_register_request` - A reference to a `JobRegisterRequest` struct
    ///   containing the job to register and any additional registration
    ///   options.
    /// * `opts` - Optional write options for the request.
    ///
    /// # Returns
    /// A `Result` containing a `JobRegisterResponse` or a `ClientError`.
    pub async fn regsiter(
        &self,
        job_register_request: &JobRegisterRequest<'_>,
        opts: Option<WriteOptions>,
    ) -> Result<JobRegisterResponse, ClientError> {
        let req = self
            .client
            .set_request_write_options(
                self.client.build_request(Method::POST, "/v1/jobs"),
                &opts.unwrap_or_default(),
            )
            .json(&job_register_request);

        self.client
            .send_with_response::<JobRegisterResponse>(req)
            .await
    }

    /// Validate a job.
    ///
    /// # Arguments
    /// * `job_validate_request` - A reference to a `JobValidateRequest` struct
    ///   containing the job to validate.
    /// * `opts` - Optional write options for the request.
    ///
    /// # Returns
    /// A `Result` containing a `JobValidateResponse` or a `ClientError`.
    pub async fn validate(
        &self,
        job_validate_request: &JobValidateRequest<'_>,
        opts: Option<WriteOptions>,
    ) -> Result<JobValidateResponse, ClientError> {
        let req = self
            .client
            .set_request_write_options(
                self.client.build_request(Method::POST, "/v1/validate/job"),
                &opts.unwrap_or_default(),
            )
            .json(&job_validate_request);

        self.client
            .send_with_response::<JobValidateResponse>(req)
            .await
    }
}
