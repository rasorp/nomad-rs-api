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

impl Nomad {
    pub async fn list_node_pools(
        &self,
        opts: Option<QueryOptions>,
    ) -> Result<Vec<NodePool>, ClientError> {
        let req = self.set_request_query_options(
            self.build_request(Method::GET, "/v1/node/pools"),
            &opts.unwrap_or_default(),
        );
        self.send_with_response::<Vec<NodePool>>(req).await
    }

    pub async fn get_node_pool(
        &self,
        name: &str,
        opts: Option<QueryOptions>,
    ) -> Result<NodePool, ClientError> {
        let req = self.set_request_query_options(
            self.build_request(Method::GET, &format!("/v1/node/pool/{}", name)),
            &opts.unwrap_or_default(),
        );
        self.send_with_response::<NodePool>(req).await
    }

    pub async fn create_node_pool(
        &self,
        node_pool: &NodePool,
        opts: Option<WriteOptions>,
    ) -> Result<(), ClientError> {
        let req = self
            .set_request_write_options(
                self.build_request(Method::PUT, "/v1/node/pool"),
                &opts.unwrap_or_default(),
            )
            .json(node_pool);
        self.send_without_response(req).await
    }

    pub async fn delete_node_pool(
        &self,
        name: &str,
        opts: Option<WriteOptions>,
    ) -> Result<(), ClientError> {
        let req = self.set_request_write_options(
            self.build_request(Method::DELETE, &format!("/v1/node/pool/{}", name)),
            &opts.unwrap_or_default(),
        );
        self.send_without_response(req).await
    }
}
