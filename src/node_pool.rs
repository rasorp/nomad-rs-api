use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::option::{QueryOptions, WriteOptions};
use crate::{ClientError, Nomad};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodePool {
    pub name: String,
    pub description: Option<String>,
    pub meta: Option<std::collections::HashMap<String, String>>,
    pub scheduler_configuration: Option<NodePoolSchedulerConfiguration>,
    pub create_index: Option<u64>,
    pub modify_index: Option<u64>,
}

impl NodePool {
    pub fn new(name: String) -> Self {
        NodePool {
            name,
            description: None,
            meta: None,
            scheduler_configuration: None,
            create_index: None,
            modify_index: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodePoolSchedulerConfiguration {
    pub scheduler_algorithm: Option<String>,
    pub memory_oversubscription_enabled: Option<bool>,
}

pub struct Endpoint<'a> {
    client: &'a Nomad,
}

impl<'a> Endpoint<'a> {
    /// Create a new `Endpoint` with the given `Nomad` client to interact with
    /// the node pool endpoints.
    pub fn new(client: &'a Nomad) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        node_pool: &NodePool,
        opts: Option<WriteOptions>,
    ) -> Result<(), ClientError> {
        let req = self
            .client
            .set_request_write_options(
                self.client.build_request(Method::PUT, "/v1/node/pool"),
                &opts.unwrap_or_default(),
            )
            .json(node_pool);
        self.client.send_without_response(req).await
    }

    pub async fn delete(&self, name: &str, opts: Option<WriteOptions>) -> Result<(), ClientError> {
        let req = self.client.set_request_write_options(
            self.client
                .build_request(Method::DELETE, &format!("/v1/node/pool/{}", name)),
            &opts.unwrap_or_default(),
        );
        self.client.send_without_response(req).await
    }

    pub async fn get(
        &self,
        name: &str,
        opts: Option<QueryOptions>,
    ) -> Result<NodePool, ClientError> {
        let req = self.client.set_request_query_options(
            self.client
                .build_request(Method::GET, &format!("/v1/node/pool/{}", name)),
            &opts.unwrap_or_default(),
        );
        self.client.send_with_response::<NodePool>(req).await
    }

    pub async fn list(&self, opts: Option<QueryOptions>) -> Result<Vec<NodePool>, ClientError> {
        let req = self.client.set_request_query_options(
            self.client.build_request(Method::GET, "/v1/node/pools"),
            &opts.unwrap_or_default(),
        );
        self.client.send_with_response::<Vec<NodePool>>(req).await
    }
}
