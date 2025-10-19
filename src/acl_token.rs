use crate::option::{QueryOptions, WriteOptions};
use crate::{ClientError, Nomad};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use time;

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

impl Nomad {
    /// Delete an ACL token by its accessor ID.
    ///
    /// # Arguments
    /// * `accessor_id` - The accessor ID of the ACL token to delete.
    /// * `opts` - Optional write options for the request.
    ///
    /// # Returns
    /// A `Result` indicating success or failure of the operation.
    pub async fn delete_acl_token(
        &self,
        accessor_id: &str,
        opts: Option<WriteOptions>,
    ) -> Result<(), ClientError> {
        let req = self.set_request_write_options(
            self.build_request(Method::DELETE, &format!("/v1/acl/token/{}", accessor_id)),
            &opts.unwrap_or_default(),
        );
        self.send_without_response(req).await
    }

    /// Get the list of ACL tokens in the Nomad cluster.
    ///
    /// # Arguments
    /// * `opts` - Optional query options to filter the results.
    ///
    /// # Returns
    /// A `Result` containing a vector of `ACLTokenStub` objects or an error if
    /// the request fails.
    pub async fn list_acl_tokens(
        &self,
        opts: Option<QueryOptions>,
    ) -> Result<Vec<ACLTokenStub>, ClientError> {
        let req = self.set_request_query_options(
            self.build_request(Method::GET, "/v1/acl/tokens"),
            &opts.unwrap_or_default(),
        );
        self.send_with_response::<Vec<ACLTokenStub>>(req).await
    }
}
