use fluvio_connector_common::{connector, secret::SecretString};

#[derive(Debug)]
#[connector(config, name = "duckdb")]
pub(crate) struct DuckDBConfig {
    pub url: SecretString,
}
