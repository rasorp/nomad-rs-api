use crate::option::{QueryOptions, WriteOptions};
use crate::{ClientError, Nomad};
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Namespace {
    pub name: String,
    pub description: Option<String>,
    pub quota: Option<String>,
    pub capabilities: Option<NamespaceCapabilities>,
    pub node_pool_configuration: Option<NamespaceNodePoolConfiguration>,
    pub vault_configuration: Option<NamespaceVaultConfiguration>,
    pub consul_configuration: Option<NamespaceConsulConfiguration>,
    pub meta: Option<std::collections::HashMap<String, String>>,
    pub create_index: Option<u64>,
    pub modify_index: Option<u64>,
}

impl Namespace {
    pub fn new(name: String) -> Self {
        Namespace {
            name,
            description: None,
            quota: None,
            capabilities: None,
            node_pool_configuration: None,
            vault_configuration: None,
            consul_configuration: None,
            meta: None,
            create_index: None,
            modify_index: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct NamespaceCapabilities {
    pub enabled_task_drivers: Option<Vec<String>>,
    pub disabled_task_drivers: Option<Vec<String>>,
    pub enabled_network_modes: Option<Vec<String>>,
    pub disabled_network_modes: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct NamespaceNodePoolConfiguration {
    pub default: Option<String>,
    pub allowed: Option<Vec<String>>,
    pub denied: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct NamespaceVaultConfiguration {
    pub default: String,
    pub allowed: Option<Vec<String>>,
    pub denied: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct NamespaceConsulConfiguration {
    pub default: String,
    pub allowed: Option<Vec<String>>,
    pub denied: Option<Vec<String>>,
}

pub struct Endpoint<'a> {
    client: &'a Nomad,
}

impl<'a> Endpoint<'a> {
    /// Create a new `Endpoint` with the given `Nomad` client to interact with
    /// the namespace endpoints.
    pub fn new(client: &'a Nomad) -> Self {
        Self { client }
    }

    /// Create a new namespace in the Nomad cluster. This can also be used to
    /// update an existing namespace by providing the same name.
    ///
    /// # Arguments
    /// * `namespace` - The `Namespace` object containing the details of the
    ///   namespace to create.
    /// * `opts` - Optional write options to use for the request.
    ///
    /// # Returns
    /// A `Result` containing the created namespace or an error if the request
    /// fails.
    pub async fn create(
        &self,
        namespace: &Namespace,
        opts: Option<WriteOptions>,
    ) -> Result<(), ClientError> {
        let req = self
            .client
            .set_request_write_options(
                self.client.build_request(Method::PUT, "/v1/namespace"),
                &opts.unwrap_or_default(),
            )
            .json(&namespace);
        self.client.send_without_response(req).await
    }

    /// Delete a namespace from the Nomad cluster.
    ///
    /// # Arguments
    /// * `name` - The name of the namespace to delete.
    /// * `opts` - Optional write options to use for the request.
    ///
    /// # Returns
    /// A `Result` indicating success or failure of the deletion operation.
    pub async fn delete(&self, name: &str, opts: Option<WriteOptions>) -> Result<(), ClientError> {
        let req = self.client.set_request_write_options(
            self.client
                .build_request(Method::DELETE, &format!("/v1/namespace/{}", name)),
            &opts.unwrap_or_default(),
        );
        self.client.send_without_response(req).await
    }

    /// Get details of a specific namespace by name.
    ///
    /// # Arguments
    /// * `name` - The name of the namespace to retrieve.
    /// * `opts` - Optional query options for the request.
    ///
    /// # Returns
    /// A `Result` containing the namespace object or an error if the request
    /// fails.
    pub async fn get(
        &self,
        name: &str,
        opts: Option<QueryOptions>,
    ) -> Result<Namespace, ClientError> {
        let req = self.client.set_request_query_options(
            self.client
                .build_request(Method::GET, &format!("/v1/namespace/{}", name)),
            &opts.unwrap_or_default(),
        );
        self.client.send_with_response::<Namespace>(req).await
    }

    /// Get the list of namespaces in the Nomad cluster.
    ///
    /// # Arguments
    /// * `opts` - Optional query options for the request.
    ///
    /// # Returns
    /// A `Result` containing a vector of namespaces or an error if the request
    /// fails.
    pub async fn list(&self, opts: Option<QueryOptions>) -> Result<Vec<Namespace>, ClientError> {
        let req = self.client.set_request_query_options(
            self.client.build_request(Method::GET, "/v1/namespaces"),
            &opts.unwrap_or_default(),
        );
        self.client.send_with_response::<Vec<Namespace>>(req).await
    }
}
