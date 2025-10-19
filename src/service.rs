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

impl Nomad {
    /// Get the list of services registered in the Nomad cluster.
    ///
    /// # Returns
    /// A `Result` containing a vector of `ServiceRegistrationList` or an error
    /// if the request fails.
    pub async fn get_services(&self) -> Result<Vec<ServiceRegistrationList>, ClientError> {
        let req = self.build_request(reqwest::Method::GET, "/v1/services");
        self.send_with_response::<Vec<ServiceRegistrationList>>(req)
            .await
    }

    /// Get the details of a specific service by its ID.
    ///
    /// # Arguments
    /// * `name` - The name of the service to retrieve.
    ///
    /// # Returns
    /// A `Result` containing a vector of `ServiceRegistration` or an error if
    /// the request fails.
    pub async fn get_service(&self, name: &str) -> Result<Vec<ServiceRegistration>, ClientError> {
        let req = self.build_request(reqwest::Method::GET, &format!("/v1/service/{}", name));
        self.send_with_response::<Vec<ServiceRegistration>>(req)
            .await
    }

    /// Delete a service registration by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the service to delete.
    ///
    /// # Returns
    /// A `Result` indicating success or failure of the operation.
    pub async fn delete_service(&self, name: &str) -> Result<(), ClientError> {
        let req = self.build_request(reqwest::Method::DELETE, &format!("/v1/service/{}", name));
        self.send_without_response(req).await
    }
}
