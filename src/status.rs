use crate::option::QueryOptions;
use crate::{ClientError, Nomad};
use reqwest::Method;

impl Nomad {
    /// Get the current leader of the Nomad cluster.
    ///
    /// # Arguments
    /// `opts`: Optional query options for the request.
    ///
    /// # Returns
    /// A `Result` containing the leader's address as a `String` or an error if
    /// the request fails.
    pub async fn get_status_leader(
        &self,
        opts: Option<QueryOptions>,
    ) -> Result<String, ClientError> {
        let req = self.set_request_query_options(
            self.build_request(Method::GET, "/v1/status/leader"),
            &opts.unwrap_or_default(),
        );
        self.send_with_response::<String>(req).await
    }

    /// Get the list of peers in the Nomad cluster.
    ///
    /// # Returns
    /// A `Result` containing a vector of peer addresses as `String`s or an
    /// error if the request fails.
    pub async fn get_status_peers(&self) -> Result<Vec<String>, ClientError> {
        let req = self.build_request(Method::GET, "/v1/status/peers");
        self.send_with_response::<Vec<String>>(req).await
    }
}
