use crate::option::{QueryOptions, WriteOptions};
use crate::{ClientError, Nomad};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use time;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Deployment {
    #[serde(rename = "ID")]
    pub id: String,
    pub namespace: String,
    #[serde(rename = "JobID")]
    pub job_id: String,
    pub job_version: u64,
    pub job_modify_index: u64,
    pub job_spec_modify_index: u64,
    pub job_create_index: u64,
    pub is_multiregion: bool,
    pub task_groups: std::collections::HashMap<String, DeploymentState>,
    pub status: String,
    pub status_description: String,
    pub create_index: u64,
    pub modify_index: u64,
    pub create_time: i64,
    pub modify_time: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeploymentState {
    pub placed_canaries: Option<Vec<String>>,
    pub auto_revert: bool,
    pub progress_deadline: i64,
    #[serde(with = "time::serde::rfc3339")]
    pub require_progress_by: time::OffsetDateTime,
    pub promoted: bool,
    pub desired_canaries: i32,
    pub desired_total: i32,
    pub placed_allocs: i32,
    pub healthy_allocs: i32,
    pub unhealthy_allocs: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeploymentUpdateResponse {
    #[serde(rename = "ID")]
    pub eval_id: String,
    pub eval_create_index: u64,
    pub deployment_modify_index: u64,
    pub reverted_job_version: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeploymentPauseRequest {
    #[serde(rename = "DeploymentID")]
    pub deployment_id: String,
    pub pause: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeploymentPromoteRequest {
    #[serde(rename = "DeploymentID")]
    pub deployment_id: String,
    pub all: bool,
    pub groups: Option<Vec<String>>,
}

pub struct Endpoint<'a> {
    client: &'a Nomad,
}

impl<'a> Endpoint<'a> {
    /// Create a new `Endpoint` with the given `Nomad` client to interact with
    /// the deployment endpoints.
    pub fn new(client: &'a Nomad) -> Self {
        Self { client }
    }

    /// Fail a deployment by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the deployment to fail.
    /// * `opts` - Optional write options for the request.
    ///
    /// # Returns
    /// A `Result` containing the deployment update response or an error if the
    /// request fails.
    pub async fn fail(
        &self,
        id: &str,
        opts: Option<WriteOptions>,
    ) -> Result<DeploymentUpdateResponse, ClientError> {
        let req = self.client.set_request_write_options(
            self.client
                .build_request(Method::POST, &format!("/v1/deployment/fail/{}", id)),
            &opts.unwrap_or_default(),
        );
        self.client
            .send_with_response::<DeploymentUpdateResponse>(req)
            .await
    }

    /// Get a specific deployment by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the deployment to retrieve.
    /// * `opts` - Optional query options for the request.
    ///
    /// # Returns
    /// A `Result` containing the deployment or an error if the request fails.
    pub async fn get(
        &self,
        id: &str,
        opts: Option<QueryOptions>,
    ) -> Result<Deployment, ClientError> {
        let req = self.client.set_request_query_options(
            self.client
                .build_request(Method::GET, &format!("/v1/deployment/{}", id)),
            &opts.unwrap_or_default(),
        );
        self.client.send_with_response::<Deployment>(req).await
    }

    /// Get the list of deployments in the Nomad cluster.
    ///
    /// # Arguments
    /// * `opts` - Optional query options for the request.
    ///
    /// # Returns
    /// A `Result` containing a vector of deployments or an error if the request
    /// fails.
    pub async fn list(&self, opts: Option<QueryOptions>) -> Result<Vec<Deployment>, ClientError> {
        let req = self.client.set_request_query_options(
            self.client.build_request(Method::GET, "/v1/deployments"),
            &opts.unwrap_or_default(),
        );
        self.client.send_with_response::<Vec<Deployment>>(req).await
    }

    /// Promote a deployment to the next stage.
    ///
    /// # Arguments
    /// * `deployment_promote_request` - The request containing the deployment
    ///   ID, all flag, and optional groups to promote.
    /// * `opts` - Optional write options for the request.
    ///
    /// # Returns
    /// A `Result` containing the deployment update response or an error if the
    /// request fails.
    pub async fn promote(
        &self,
        deployment_promote_request: DeploymentPromoteRequest,
        opts: Option<WriteOptions>,
    ) -> Result<DeploymentUpdateResponse, ClientError> {
        let req = self
            .client
            .set_request_write_options(
                self.client.build_request(
                    Method::POST,
                    &format!(
                        "/v1/deployment/promote/{}",
                        deployment_promote_request.deployment_id
                    ),
                ),
                &opts.unwrap_or_default(),
            )
            .json(&deployment_promote_request);
        self.client
            .send_with_response::<DeploymentUpdateResponse>(req)
            .await
    }

    /// Pause or resume a deployment.
    ///
    /// # Arguments
    /// * `deployment_pause_request` - The request containing the deployment ID
    ///   and pause status.
    /// * `opts` - Optional write options for the request.
    ///
    /// # Returns
    /// A `Result` containing the deployment update response or an error if the
    /// request fails.
    pub async fn set_pause(
        &self,
        deployment_pause_request: DeploymentPauseRequest,
        opts: Option<WriteOptions>,
    ) -> Result<DeploymentUpdateResponse, ClientError> {
        let req = self
            .client
            .set_request_write_options(
                self.client.build_request(
                    Method::POST,
                    &format!(
                        "/v1/deployment/progress/{}",
                        deployment_pause_request.deployment_id
                    ),
                ),
                &opts.unwrap_or_default(),
            )
            .json(&deployment_pause_request);
        self.client
            .send_with_response::<DeploymentUpdateResponse>(req)
            .await
    }
}
