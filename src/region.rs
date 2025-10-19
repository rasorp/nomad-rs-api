use crate::{ClientError, Nomad};
use reqwest::Method;

impl Nomad {
    /// Fetches the list of regions from the Nomad server.
    ///
    /// # Returns
    /// A `Result` containing a vector of region names or an error if the
    /// request fails.
    pub async fn get_regions(&self) -> Result<Vec<String>, ClientError> {
        let req = self.build_request(Method::GET, "/v1/regions");
        self.send_with_response::<Vec<String>>(req).await
    }
}
