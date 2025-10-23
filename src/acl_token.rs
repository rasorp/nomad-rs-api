use crate::option::{QueryOptions, WriteOptions};
use crate::{ClientError, Nomad};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use time;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ACLTokenBootstrapRequest {
    pub bootstrap_secret: String,
}

impl ACLTokenBootstrapRequest {
    pub fn new(bootstrap_secret: String) -> Self {
        ACLTokenBootstrapRequest { bootstrap_secret }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ACLTokenCreateRequest {
    pub name: Option<String>,
    #[serde(rename = "Type")]
    pub token_type: String,
    pub global: bool,
    pub policies: Option<Vec<String>>,
    pub roles: Option<Vec<ACLTokenRoleLink>>,
    pub expiration_time: Option<String>,
    pub expiration_ttl: Option<i64>,
}

impl ACLTokenCreateRequest {
    pub fn new(token_type: String, global: bool) -> Self {
        ACLTokenCreateRequest {
            token_type,
            global,
            name: None,
            policies: None,
            roles: None,
            expiration_time: None,
            expiration_ttl: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ACLToken {
    #[serde(rename = "AccessorID")]
    pub accessor_id: String,
    #[serde(rename = "SecretID")]
    pub secret_id: String,
    pub name: Option<String>,
    #[serde(rename = "Type")]
    pub token_type: String,
    pub policies: Option<Vec<String>>,
    pub roles: Option<Vec<ACLTokenRoleLink>>,
    pub global: bool,
    #[serde(with = "time::serde::rfc3339")]
    pub create_time: time::OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub expiration_time: Option<time::OffsetDateTime>,
    pub expiration_ttl: Option<time::Duration>,
    pub create_index: Option<u64>,
    pub modify_index: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ACLTokenStub {
    #[serde(rename = "AccessorID")]
    pub accessor_id: String,
    pub name: String,
    #[serde(rename = "Type")]
    pub token_type: String,
    pub policies: Option<Vec<String>>,
    pub roles: Option<Vec<ACLTokenRoleLink>>,
    pub global: bool,
    #[serde(with = "time::serde::rfc3339")]
    pub create_time: time::OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub expiration_time: Option<time::OffsetDateTime>,
    pub hash: String,
    pub create_index: Option<u64>,
    pub modify_index: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ACLTokenRoleLink {
    #[serde(rename = "ID")]
    pub id: String,
    pub name: String,
}

pub struct Endpoint<'a> {
    client: &'a Nomad,
}

impl<'a> Endpoint<'a> {
    /// Create a new `Endpoint` with the given `Nomad` client to interact with
    /// the ACL token endpoints.
    pub fn new(client: &'a Nomad) -> Self {
        Self { client }
    }

    /// Bootstrap the ACL subsystem.
    ///
    /// # Arguments
    /// * `acl_token_bootstrap_request` - Optional bootstrapping parameters
    ///   which allows specifying the secret ID of the bootstrap token.
    /// * `opts` - Optional write options to use for the request.
    ///
    /// # Returns
    /// A `Result` containing the created ACL bootstrap token object or an error
    /// if the request fails.
    pub async fn bootstrap(
        &self,
        acl_token_bootstrap_request: Option<&ACLTokenBootstrapRequest>,
        opts: Option<WriteOptions>,
    ) -> Result<ACLToken, ClientError> {
        let req = self
            .client
            .set_request_write_options(
                self.client.build_request(Method::POST, "/v1/acl/bootstrap"),
                &opts.unwrap_or_default(),
            )
            .json(&acl_token_bootstrap_request);
        self.client.send_with_response(req).await
    }

    /// Create an ACL token.
    ///
    /// # Arguments
    /// * `acl_token_create_request` - The request object that defines the ACL
    ///   token that should be created.
    /// * `opts` - Optional write options to use for the request.
    ///
    /// # Returns
    /// A `Result` containing the created ACL token object or an error if the
    /// request fails.
    pub async fn create(
        &self,
        acl_token_create_request: &ACLTokenCreateRequest,
        opts: Option<WriteOptions>,
    ) -> Result<ACLToken, ClientError> {
        let req = self
            .client
            .set_request_write_options(
                self.client.build_request(Method::POST, "/v1/acl/token"),
                &opts.unwrap_or_default(),
            )
            .json(&acl_token_create_request);
        self.client.send_with_response(req).await
    }

    /// Delete an ACL token by its accessor ID.
    ///
    /// # Arguments
    /// * `accessor_id` - The accessor ID of the ACL token to delete.
    /// * `opts` - Optional write options for the request.
    ///
    /// # Returns
    /// A `Result` indicating success or failure of the operation.
    pub async fn delete(
        &self,
        accessor_id: &str,
        opts: Option<WriteOptions>,
    ) -> Result<(), ClientError> {
        let req = self.client.set_request_write_options(
            self.client
                .build_request(Method::DELETE, &format!("/v1/acl/token/{}", accessor_id)),
            &opts.unwrap_or_default(),
        );
        self.client.send_without_response(req).await
    }

    /// Get an ACL token by its accessor ID.
    ///
    /// # Arguments
    /// * `accessor_id` - The accessor ID of the ACL token to retrieve.
    /// * `opts` - Optional query options for the request.
    ///
    /// # Returns
    /// A `Result` containing the ACL token object or an error if the request
    /// fails.
    pub async fn get(
        &self,
        accessor_id: &str,
        opts: Option<QueryOptions>,
    ) -> Result<ACLToken, ClientError> {
        let req = self.client.set_request_query_options(
            self.client
                .build_request(Method::GET, &format!("/v1/acl/token/{}", accessor_id)),
            &opts.unwrap_or_default(),
        );
        self.client.send_with_response::<ACLToken>(req).await
    }

    /// Get an ACL token for the token used to authenticate the request.
    ///
    /// # Arguments
    /// * `opts` - Optional query options for the request.
    ///
    /// # Returns
    /// A `Result` containing the ACL token object or an error if the request
    /// fails.
    pub async fn get_self(&self, opts: Option<QueryOptions>) -> Result<ACLToken, ClientError> {
        let req = self.client.set_request_query_options(
            self.client.build_request(Method::GET, "/v1/acl/token/self"),
            &opts.unwrap_or_default(),
        );
        self.client.send_with_response::<ACLToken>(req).await
    }

    /// Get the list of ACL tokens in the Nomad cluster.
    ///
    /// # Arguments
    /// * `opts` - Optional query options to filter the results.
    ///
    /// # Returns
    /// A `Result` containing a vector of `ACLTokenStub` objects or an error if
    /// the request fails.
    pub async fn list(&self, opts: Option<QueryOptions>) -> Result<Vec<ACLTokenStub>, ClientError> {
        let req = self.client.set_request_query_options(
            self.client.build_request(Method::GET, "/v1/acl/tokens"),
            &opts.unwrap_or_default(),
        );
        self.client
            .send_with_response::<Vec<ACLTokenStub>>(req)
            .await
    }
}
