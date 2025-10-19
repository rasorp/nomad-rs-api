# nomad-rs-api
The Nomad Rust API is an experimental project that provides an ergonomic, type-safe interface for
interacting with the [HashiCorp Nomad](https://github.com/hashicorp/nomad) API using
[Rust](https://rust-lang.org).

⚠️ Status: This project is in an early experimental stage. Expect breaking changes, incomplete
endpoints, and evolving design patterns as we iterate.

## Usage
The client can be configured automatically using environment variabeles:
```rust
use nomad_rs_api::{Config, Nomad};

let client = Nomad::new(Config::from_env());
```

Alternatively, you can configure the client manually:
```rust
let config = Config {
    address: "http://localhost:4646".to_string(),
    token: None,
    ..Default::default()
};

let client = Nomad::new(config);
```

### Query and Write Options
The query and write options can be easily built and modified before making a request:
```rust
use nomad_rs_api::{option};

let mut query_opts = option::QueryOptions::new();
query_opts.namespace = Some("platform".to_string());

let mut write_opts = option::WriteOptions::new();
write_opts.namespace = Some("platform".to_string());
```
