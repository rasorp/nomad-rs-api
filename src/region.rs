use crate::{ClientError, Nomad};
use reqwest::Method;

pub struct Endpoint<'a> {
    client: &'a Nomad,
}

impl<'a> Endpoint<'a> {
    /// Create a new `Endpoint` with the given `Nomad` client to interact with
    /// the region endpoints.
    pub fn new(client: &'a Nomad) -> Self {
        Self { client }
    }

    /// Fetches the list of regions from the Nomad server.
    ///
    /// # Returns
    /// A `Result` containing a vector of region names or an error if the
    /// request fails.
    pub async fn list(&self) -> Result<Vec<String>, ClientError> {
        let req = self.client.build_request(Method::GET, "/v1/regions");
        self.client.send_with_response::<Vec<String>>(req).await
    }
}
