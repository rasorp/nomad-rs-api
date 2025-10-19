use crate::option::{QueryOptions, WriteOptions};
use crate::{ClientError, Nomad};
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ACLPolicy {
    pub name: String,
    pub description: Option<String>,
    pub rules: String,
    pub job_acl: Option<JobACL>,
    pub create_index: Option<u64>,
    pub modify_index: Option<u64>,
}

impl ACLPolicy {
    /// Create a new ACL policy object with the specified name and rules.
    ///
    /// # Arguments
    /// * `name` - The name of the ACL policy.
    /// * `rules` - The rules for the ACL policy in HCL format.
    ///
    /// # Returns
    /// A new `ACLPolicy` object.
    pub fn new(name: String, rules: String) -> Self {
        Self {
            name,
            description: None,
            rules,
            job_acl: None,
            create_index: None,
            modify_index: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ACLPolicyStub {
    pub name: String,
    pub description: Option<String>,
    pub job_acl: Option<JobACL>,
    pub create_index: Option<u64>,
    pub modify_index: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobACL {
    pub namespace: String,
    pub job_id: String,
    pub group: String,
    pub task: String,
}

impl Nomad {
    /// Create a new ACL policy in the Nomad cluster. This can also be used to
    /// update an existing ACL policy by providing the same name.
    ///
    /// # Arguments
    /// * `policy` - The ACL policy to create.
    /// * `opts` - Optional write options for the request.
    ///
    /// # Returns
    /// A `Result` indicating success or failure of the operation.
    pub async fn create_acl_policy(
        &self,
        policy: &ACLPolicy,
        opts: Option<WriteOptions>,
    ) -> Result<(), ClientError> {
        let req = self
            .set_request_write_options(
                self.build_request(Method::POST, &format!("/v1/acl/policy/{}", policy.name)),
                &opts.unwrap_or_default(),
            )
            .json(policy);
        self.send_without_response(req).await
    }

    /// Delete an ACL policy by name.
    ///
    /// # Arguments
    /// * `name` - The name of the ACL policy to delete.
    /// * `opts` - Optional write options for the request.
    ///
    /// # Returns
    /// A `Result` indicating success or failure of the deletion operation.
    pub async fn delete_acl_policy(
        &self,
        name: &str,
        opts: Option<WriteOptions>,
    ) -> Result<(), ClientError> {
        let req = self.set_request_write_options(
            self.build_request(Method::DELETE, &format!("/v1/acl/policy/{}", name)),
            &opts.unwrap_or_default(),
        );
        self.send_without_response(req).await
    }

    /// Get the ACL policy with the specified name.
    ///
    /// # Arguments
    /// * `name` - The name of the ACL policy to retrieve.
    /// * `opts` - Optional query options for the request.
    ///
    /// # Returns
    /// A `Result` containing the ACL policy object or an error if the request
    /// fails.
    pub async fn get_acl_policy(
        &self,
        name: &str,
        opts: Option<QueryOptions>,
    ) -> Result<ACLPolicy, ClientError> {
        let req = self.set_request_query_options(
            self.build_request(Method::GET, &format!("/v1/acl/policy/{}", name)),
            &opts.unwrap_or_default(),
        );
        self.send_with_response::<ACLPolicy>(req).await
    }

    /// Get a list of the ACL policies that are associated with the caller ACL
    /// token.
    ///
    /// # Arguments
    /// * `opts` - Optional query options for the request.
    ///
    /// # Returns
    /// A `Result` containing a vector of `ACLPolicyStub` objects or an error if
    /// the request fails.
    pub async fn get_acl_policy_self(
        &self,
        opts: Option<QueryOptions>,
    ) -> Result<Vec<ACLPolicyStub>, ClientError> {
        let req = self.set_request_query_options(
            self.build_request(Method::GET, "/v1/acl/policy/self"),
            &opts.unwrap_or_default(),
        );
        self.send_with_response::<Vec<ACLPolicyStub>>(req).await
    }

    /// Get the list of ACL policies in the Nomad cluster.
    ///
    /// # Arguments
    /// * `opts` - Optional query options for the request.
    ///
    /// # Returns
    /// A `Result` containing a vector of `ACLPolicyStub` objects or an error if
    /// the request fails.
    pub async fn list_acl_policies(
        &self,
        opts: Option<QueryOptions>,
    ) -> Result<Vec<ACLPolicyStub>, ClientError> {
        let req = self.set_request_query_options(
            self.build_request(Method::GET, "/v1/acl/policies"),
            &opts.unwrap_or_default(),
        );
        self.send_with_response::<Vec<ACLPolicyStub>>(req).await
    }
}
