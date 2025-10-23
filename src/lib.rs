pub mod acl_policy;
pub mod acl_token;
pub mod allocation;
pub mod deployment;
pub mod evaluation;
pub mod namespace;
pub mod node_pool;
pub mod option;
pub mod region;
pub mod service;
pub mod status;

use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;
use std::env;
use thiserror::Error;

static NOMAD_ENV_VAR_ADDRESS: &str = "NOMAD_ADDRESS";
static NOMAD_ENV_VAR_REGION: &str = "NOMAD_REGION";
static NOMAD_ENV_VAR_TOKEN: &str = "NOMAD_TOKEN";

pub struct Nomad {
    config: Config,
    http_client: Client,
}

impl Nomad {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            http_client: Client::builder()
                .user_agent("nomad-rs-api/0.0.1-alpha.1")
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    fn build_request(&self, method: reqwest::Method, path: &str) -> RequestBuilder {
        let request = self
            .http_client
            .request(method, format!("{}{}", self.config.address, path));

        let mut request = request.query(&[("region", self.config.region.as_str())]);

        if let Some(ref token) = self.config.token {
            request = request.header("X-Nomad-Token", token);
        }
        request
    }

    fn set_request_query_options(
        &self,
        req: RequestBuilder,
        opts: &option::QueryOptions,
    ) -> RequestBuilder {
        let mut request = req;

        if let Some(ref region) = opts.region {
            request = request.query(&[("region", region)]);
        }
        if let Some(ref namespace) = opts.namespace {
            request = request.query(&[("namespace", namespace)]);
        }
        if let Some(allow_stale) = opts.allow_stale {
            request = request.query(&[("allow_stale", &allow_stale.to_string())]);
        }
        if let Some(wait_index) = opts.wait_index {
            request = request.query(&[("wait_index", &wait_index.to_string())]);
        }
        if let Some(wait_time) = opts.wait_time {
            request = request.query(&[("wait_time", &wait_time.to_string())]);
        }
        if let Some(ref prefix) = opts.prefix {
            request = request.query(&[("prefix", prefix)]);
        }
        if let Some(ref params) = opts.params {
            for (key, value) in params.iter() {
                request = request.query(&[(key.as_str(), value.as_str())]);
            }
        }
        if let Some(ref headers) = opts.headers {
            for (key, value) in headers.iter() {
                request = request.header(key, value);
            }
        }
        if let Some(ref auth_token) = opts.auth_token {
            request = request.header("X-Nomad-Token", auth_token);
        }
        if let Some(ref filter) = opts.filter {
            request = request.query(&[("filter", filter)]);
        }
        if let Some(per_page) = opts.per_page {
            request = request.query(&[("per_page", &per_page.to_string())]);
        }
        if let Some(ref next_token) = opts.next_token {
            request = request.query(&[("next_token", next_token)]);
        }
        if let Some(reverse) = opts.reverse {
            request = request.query(&[("reverse", &reverse.to_string())]);
        }

        request
    }

    fn set_request_write_options(
        &self,
        req: RequestBuilder,
        opts: &option::WriteOptions,
    ) -> RequestBuilder {
        let mut request = req;

        if let Some(ref region) = opts.region {
            request = request.query(&[("region", region)]);
        }
        if let Some(ref namespace) = opts.namespace {
            request = request.query(&[("namespace", namespace)]);
        }
        if let Some(ref auth_token) = opts.auth_token {
            request = request.header("X-Nomad-Token", auth_token);
        }
        if let Some(ref headers) = opts.headers {
            for (key, value) in headers.iter() {
                request = request.header(key, value);
            }
        }
        if let Some(ref idempotency_token) = opts.idempotency_token {
            request = request.query(&[("idempotency_token", idempotency_token)]);
        }

        request
    }

    async fn send_with_response<TResponse: DeserializeOwned>(
        &self,
        req: RequestBuilder,
    ) -> Result<TResponse, ClientError> {
        let req_result = req.build();
        if let Err(error) = req_result {
            return Err(ClientError::RequestCreationError(error.to_string()));
        }

        let req = req_result.unwrap();

        match self.http_client.execute(req).await {
            Ok(response) => {
                let status = response.status();
                if response.status().is_success() {
                    match response.json::<TResponse>().await {
                        Ok(body) => Ok(body),
                        Err(err) => Err(ClientError::DeserializationError(err.to_string())),
                    }
                } else {
                    match response.text().await {
                        Ok(body) => Err(ClientError::ServerError(status.as_u16(), body)),
                        Err(err) => Err(ClientError::NetworkError(err.to_string())),
                    }
                }
            }
            Err(err) => Err(ClientError::NetworkError(err.to_string())),
        }
    }

    async fn send_without_response(&self, req: RequestBuilder) -> Result<(), ClientError> {
        let req_result = req.build();
        if let Err(error) = req_result {
            return Err(ClientError::RequestCreationError(error.to_string()));
        }

        let req = req_result.unwrap();

        match self.http_client.execute(req).await {
            Ok(response) => {
                let status = response.status();

                match response.status().is_success() {
                    true => Ok(()),
                    false => match response.text().await {
                        Ok(body) => Err(ClientError::ServerError(status.as_u16(), body)),
                        Err(err) => Err(ClientError::NetworkError(err.to_string())),
                    },
                }
            }
            Err(err) => Err(ClientError::NetworkError(err.to_string())),
        }
    }

    /// Get access to the ACL Policy endpoint methods.
    pub fn acl_policy(&self) -> acl_policy::Endpoint<'_> {
        acl_policy::Endpoint::new(self)
    }

    /// Get access to the ACL Token endpoint methods.
    pub fn acl_token(&self) -> acl_token::Endpoint<'_> {
        acl_token::Endpoint::new(self)
    }

    /// Get access to the Deployment endpoint methods.
    pub fn deployment(&self) -> deployment::Endpoint<'_> {
        deployment::Endpoint::new(self)
    }

    /// Get access to the Evaluation endpoint methods.
    pub fn evaluation(&self) -> evaluation::Endpoint<'_> {
        evaluation::Endpoint::new(self)
    }

    /// Get access to the Namespace endpoint methods.
    pub fn namespace(&self) -> namespace::Endpoint<'_> {
        namespace::Endpoint::new(self)
    }

    /// Get access to the Node Pool endpoint methods.
    pub fn node_pool(&self) -> node_pool::Endpoint<'_> {
        node_pool::Endpoint::new(self)
    }

    /// Get access to the Region endpoint methods.
    pub fn region(&self) -> region::Endpoint<'_> {
        region::Endpoint::new(self)
    }

    /// Get access to the Service endpoint methods.
    pub fn service(&self) -> service::Endpoint<'_> {
        service::Endpoint::new(self)
    }

    /// Get access to the Status endpoint methods.
    pub fn status(&self) -> status::Endpoint<'_> {
        status::Endpoint::new(self)
    }
}

#[derive(Debug)]
pub struct Config {
    pub address: String,
    pub region: String,
    pub token: Option<String>,
}

impl Config {
    pub fn from_env() -> Config {
        let mut default = Config::default();
        default.address = env::var(NOMAD_ENV_VAR_ADDRESS).unwrap_or(default.address);
        default.region = env::var(NOMAD_ENV_VAR_REGION).unwrap_or(default.region);
        default.token = env::var(NOMAD_ENV_VAR_TOKEN).map_or(default.token, Some);
        default
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            address: "http://127.0.0.1:4646".to_string(),
            region: "global".to_string(),
            token: None,
        }
    }
}

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Request creation error: {0}")]
    RequestCreationError(String),
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
    #[error("Nomad API error: [{0}] '{1}'")]
    ServerError(u16, String),
    #[error("Network error: {0}")]
    NetworkError(String),
}
