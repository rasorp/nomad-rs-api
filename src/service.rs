use crate::{ClientError, Nomad};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceRegistrationList {
    pub namespace: String,
    pub services: Vec<ServiceRegistrationStub>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceRegistrationStub {
    pub service_name: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceRegistration {
    pub id: String,
    pub service_name: String,
    pub namespace: String,
    pub node_id: String,
    pub datacenter: String,
    pub job_id: String,
    pub alloc_id: String,
    pub tags: Vec<String>,
    pub address: String,
    pub port: u16,
    pub create_index: u64,
    pub modify_index: u64,
}

pub struct Endpoint<'a> {
    client: &'a Nomad,
}

impl<'a> Endpoint<'a> {
    /// Create a new `Endpoint` with the given `Nomad` client to interact with
    /// the service registrations endpoint.
    pub fn new(client: &'a Nomad) -> Self {
        Self { client }
    }

    /// Delete a service registration by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the service to delete.
    ///
    /// # Returns
    /// A `Result` indicating success or failure of the operation.
    pub async fn delete(&self, name: &str) -> Result<(), ClientError> {
        let req = self
            .client
            .build_request(reqwest::Method::DELETE, &format!("/v1/service/{}", name));
        self.client.send_without_response(req).await
    }

    /// Get the details of a specific service by its ID.
    ///
    /// # Arguments
    /// * `name` - The name of the service to retrieve.
    ///
    /// # Returns
    /// A `Result` containing a vector of `ServiceRegistration` or an error if
    /// the request fails.
    pub async fn get(&self, name: &str) -> Result<Vec<ServiceRegistration>, ClientError> {
        let req = self
            .client
            .build_request(reqwest::Method::GET, &format!("/v1/service/{}", name));
        self.client
            .send_with_response::<Vec<ServiceRegistration>>(req)
            .await
    }

    /// Get the list of services registered in the Nomad cluster.
    ///
    /// # Returns
    /// A `Result` containing a vector of `ServiceRegistrationList` or an error
    /// if the request fails.
    pub async fn list(&self) -> Result<Vec<ServiceRegistrationList>, ClientError> {
        let req = self
            .client
            .build_request(reqwest::Method::GET, "/v1/services");
        self.client
            .send_with_response::<Vec<ServiceRegistrationList>>(req)
            .await
    }
}
