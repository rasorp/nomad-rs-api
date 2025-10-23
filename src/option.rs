#[derive(Default)]
pub struct QueryOptions {
    pub region: Option<String>,
    pub namespace: Option<String>,
    pub allow_stale: Option<bool>,
    pub wait_index: Option<u64>,
    pub wait_time: Option<u64>,
    pub prefix: Option<String>,
    pub params: Option<std::collections::HashMap<String, String>>,
    pub headers: Option<std::collections::HashMap<String, String>>,
    pub auth_token: Option<String>,
    pub filter: Option<String>,
    pub per_page: Option<i32>,
    pub next_token: Option<String>,
    pub reverse: Option<bool>,
}

impl QueryOptions {
    /// Creates a new instance of `QueryOptions` with default values, so callers
    /// don't have to specify all fields.
    pub fn new() -> Self {
        QueryOptions {
            region: None,
            namespace: None,
            allow_stale: None,
            wait_index: None,
            wait_time: None,
            prefix: None,
            params: None,
            headers: None,
            auth_token: None,
            filter: None,
            per_page: None,
            next_token: None,
            reverse: None,
        }
    }
    pub fn with_region(mut self, region: String) -> Self {
        self.region = Some(region);
        self
    }
    pub fn with_namespace(mut self, namespace: String) -> Self {
        self.namespace = Some(namespace);
        self
    }
    pub fn with_allow_stale(mut self, allow_stale: bool) -> Self {
        self.allow_stale = Some(allow_stale);
        self
    }
    pub fn with_wait_index(mut self, wait_index: u64) -> Self {
        self.wait_index = Some(wait_index);
        self
    }
    pub fn with_wait_time(mut self, wait_time: u64) -> Self {
        self.wait_time = Some(wait_time);
        self
    }
    pub fn with_prefix(mut self, prefix: String) -> Self {
        self.prefix = Some(prefix);
        self
    }
    pub fn with_params(mut self, params: std::collections::HashMap<String, String>) -> Self {
        self.params = Some(params);
        self
    }
    pub fn with_headers(mut self, headers: std::collections::HashMap<String, String>) -> Self {
        self.headers = Some(headers);
        self
    }
    pub fn with_auth_token(mut self, auth_token: String) -> Self {
        self.auth_token = Some(auth_token);
        self
    }
    pub fn with_filter(mut self, filter: String) -> Self {
        self.filter = Some(filter);
        self
    }
    pub fn with_per_page(mut self, per_page: i32) -> Self {
        self.per_page = Some(per_page);
        self
    }
    pub fn with_next_token(mut self, next_token: String) -> Self {
        self.next_token = Some(next_token);
        self
    }
    pub fn with_reverse(mut self, reverse: bool) -> Self {
        self.reverse = Some(reverse);
        self
    }
}

#[derive(Default)]
pub struct WriteOptions {
    pub region: Option<String>,
    pub namespace: Option<String>,
    pub auth_token: Option<String>,
    pub headers: Option<std::collections::HashMap<String, String>>,
    pub idempotency_token: Option<String>,
}

impl WriteOptions {
    /// Creates a new instance of `WriteOptions` with default values, so callers
    /// don't have to specify all fields.
    pub fn new() -> Self {
        WriteOptions {
            region: None,
            namespace: None,
            auth_token: None,
            headers: None,
            idempotency_token: None,
        }
    }
    pub fn with_region(mut self, region: String) -> Self {
        self.region = Some(region);
        self
    }
    pub fn with_namespace(mut self, namespace: String) -> Self {
        self.namespace = Some(namespace);
        self
    }
    pub fn with_auth_token(mut self, auth_token: String) -> Self {
        self.auth_token = Some(auth_token);
        self
    }
    pub fn with_headers(mut self, headers: std::collections::HashMap<String, String>) -> Self {
        self.headers = Some(headers);
        self
    }
    pub fn with_idempotency_token(mut self, token: String) -> Self {
        self.idempotency_token = Some(token);
        self
    }
}
