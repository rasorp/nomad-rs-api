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
}
