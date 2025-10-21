use crate::allocation::{AllocationMetric, AllocationStub};
use crate::option::{QueryOptions, WriteOptions};
use crate::{ClientError, Nomad};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const EVALUATION_STATUS_BLOCKED: &str = "blocked";
pub const EVALUATION_STATUS_PENDING: &str = "pending";
pub const EVALUATION_STATUS_COMPLETE: &str = "complete";
pub const EVALUATION_STATUS_FAILED: &str = "failed";
pub const EVALUATION_STATUS_CANCELED: &str = "canceled";

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Evaluation {
    #[serde(rename = "ID")]
    pub id: String,
    pub priority: i32,
    #[serde(rename = "Type")]
    pub type_: String,
    pub triggered_by: String,
    pub namespace: String,
    #[serde(rename = "JobID")]
    pub job_id: String,
    pub job_modify_index: Option<u64>,
    #[serde(rename = "NodeID")]
    pub node_id: Option<String>,
    pub node_modify_index: Option<u64>,
    #[serde(rename = "DeploymentID")]
    pub deployment_id: Option<String>,
    pub status: String,
    pub status_description: Option<String>,
    pub wait: Option<u64>,
    pub wait_until: Option<String>,
    pub next_eval: Option<String>,
    pub previous_eval: Option<String>,
    pub blocked_eval: Option<String>,
    pub related_evals: Option<Vec<EvaluationStub>>,
    pub failed_tg_allocs: Option<HashMap<String, AllocationMetric>>,
    pub plan_annotations: Option<EvaluationPlanAnnotation>,
    pub class_eligibility: Option<HashMap<String, bool>>,
    pub escaped_computed_class: Option<bool>,
    pub quota_limit_reached: Option<String>,
    pub annotate_plan: Option<bool>,
    pub queued_allocations: Option<HashMap<String, i32>>,
    pub snapshot_index: u64,
    pub create_index: u64,
    pub modify_index: u64,
    pub create_time: i64,
    pub modify_time: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EvaluationStub {
    #[serde(rename = "ID")]
    pub id: String,
    pub priority: i32,
    #[serde(rename = "Type")]
    pub type_: String,
    pub triggered_by: String,
    pub namespace: String,
    #[serde(rename = "JobID")]
    pub job_id: String,
    #[serde(rename = "NodeID")]
    pub node_id: String,
    #[serde(rename = "DeploymentID")]
    pub deployment_id: String,
    pub status: String,
    pub status_description: String,
    pub wait_until: String,
    pub next_eval: String,
    pub previous_eval: String,
    pub blocked_eval: String,
    pub create_index: u64,
    pub modify_index: u64,
    pub create_time: i64,
    pub modify_time: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct EvalualtionDeleteRequest {
    #[serde(rename = "EvalIDs")]
    pub eval_ids: Option<Vec<String>>,
    pub filter: Option<String>,
}

impl EvalualtionDeleteRequest {
    /// Create a new EvalualtionDeleteRequest with a vector of evaluations ID
    /// that will be deleted.
    pub fn new_with_ids(eval_ids: Vec<String>) -> Self {
        Self {
            eval_ids: Some(eval_ids),
            filter: None,
        }
    }

    /// Create a new EvalualtionDeleteRequest with a filter that will be used to
    /// identify evaluations to delete.
    pub fn new_with_filter(filter: String) -> Self {
        Self {
            eval_ids: None,
            filter: Some(filter),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EvaluationDeleteResponse {
    pub count: i32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EvaluationCountResponse {
    pub count: i32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EvaluationPlanAnnotation {
    pub desired_tg_updates: Option<HashMap<String, EvaluationDesiredUpdate>>,
    pub preemptions: Option<Vec<AllocationStub>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EvaluationDesiredUpdate {
    pub ignore: u64,
    pub place: u64,
    pub migrate: u64,
    pub stop: u64,
    pub in_place_update: u64,
    pub destructive_update: u64,
    pub canary: u64,
    pub preemptions: u64,
}

impl Nomad {
    /// List all allocations for a specific evaluation.
    ///
    /// # Arguments
    /// * `evaluation_id` - The ID of the evaluation.
    /// * `opts` - Optional query options for the request.
    ///
    /// # Returns
    /// A `Result` containing a vector of `AllocationListStub` objects or an
    /// error if the request fails.
    pub async fn evaluation_allocations_list(
        &self,
        evaluation_id: &str,
        opts: Option<QueryOptions>,
    ) -> Result<Vec<AllocationStub>, ClientError> {
        let req = self.set_request_query_options(
            self.build_request(
                Method::GET,
                &format!("/v1/evaluation/{}/allocations", evaluation_id),
            ),
            &opts.unwrap_or_default(),
        );
        let mut allocations = self.send_with_response::<Vec<AllocationStub>>(req).await?;

        // Sort by CreateIndex descending (highest first)
        allocations.sort_by(|a, b| b.create_index.cmp(&a.create_index));

        Ok(allocations)
    }

    /// Get information about a specific evaluation.
    ///
    /// # Arguments
    /// * `evaluation_id` - The ID of the evaluation to retrieve.
    /// * `opts` - Optional query options for the request.
    ///
    /// # Returns
    /// A `Result` containing the `Evaluation` object or an error if the request
    /// fails.
    pub async fn evaluation_get(
        &self,
        evaluation_id: &str,
        opts: Option<QueryOptions>,
    ) -> Result<Evaluation, ClientError> {
        let req = self.set_request_query_options(
            self.build_request(
                Method::GET,
                &format!("/v1/evaluation/{}?related=true", evaluation_id),
            ),
            &opts.unwrap_or_default(),
        );
        self.send_with_response::<Evaluation>(req).await
    }

    /// Get a count of evaluations.
    ///
    /// # Arguments
    /// * `opts` - Optional query options for the request.
    ///
    /// # Returns
    /// A `Result` containing an `EvalCountResponse` or an error if the request
    /// fails.
    pub async fn evaluations_count(
        &self,
        opts: Option<QueryOptions>,
    ) -> Result<EvaluationCountResponse, ClientError> {
        let req = self.set_request_query_options(
            self.build_request(Method::GET, "/v1/evaluations/count"),
            &opts.unwrap_or_default(),
        );
        self.send_with_response::<EvaluationCountResponse>(req)
            .await
    }

    /// Delete evaluations using a filter or specific IDs.
    ///
    /// # Arguments
    /// * `evaluation_delete_request` - The delete request with eval IDs or
    ///   filter.
    /// * `opts` - Optional write options for the request.
    ///
    /// # Returns
    /// A `Result` containing an `EvaluationDeleteResponse` or an error if the
    /// request fails.
    pub async fn evaluations_delete(
        &self,
        evaluation_delete_request: &EvalualtionDeleteRequest,
        opts: Option<WriteOptions>,
    ) -> Result<EvaluationDeleteResponse, ClientError> {
        let req = self
            .set_request_write_options(
                self.build_request(Method::DELETE, "/v1/evaluations"),
                &opts.unwrap_or_default(),
            )
            .json(evaluation_delete_request);
        self.send_with_response::<EvaluationDeleteResponse>(req)
            .await
    }

    /// List all evaluations.
    ///
    /// # Arguments
    /// * `opts` - Optional query options for the request.
    ///
    /// # Returns
    /// A `Result` containing a vector of `Evaluation` objects or an error if
    /// the request fails.
    pub async fn evaluations_list(
        &self,
        opts: Option<QueryOptions>,
    ) -> Result<Vec<Evaluation>, ClientError> {
        let req = self.set_request_query_options(
            self.build_request(Method::GET, "/v1/evaluations"),
            &opts.unwrap_or_default(),
        );
        let mut evaluations = self.send_with_response::<Vec<Evaluation>>(req).await?;

        // Sort by CreateIndex descending (highest first)
        evaluations.sort_by(|a, b| b.create_index.cmp(&a.create_index));

        Ok(evaluations)
    }
}
