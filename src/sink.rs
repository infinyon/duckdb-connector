use anyhow::{Context, Result};
use async_trait::async_trait;
use url::Url;

use fluvio::Offset;
use fluvio_connector_common::{tracing::info, LocalBoxSink, Sink, secret::SecretString};
use fluvio_model_sql::Operation;

use crate::{config::DuckDBConfig, db::DuckDB};

#[derive(Debug)]
pub(crate) struct DuckDBSink {
    url: String,
}

impl DuckDBSink {
    pub(crate) fn new(config: &DuckDBConfig) -> Result<Self> {
  
        Ok(Self { url: config.url.resolve()? })
    }
}

#[async_trait]
impl Sink<Operation> for DuckDBSink {
    async fn connect(self, _offset: Option<Offset>) -> Result<LocalBoxSink<Operation>> {
        let db = DuckDB::connect(self.url.as_str()).await?;
        info!("connected to duckdb database");
        let unfold = futures::sink::unfold(db, |mut db: DuckDB, record: Operation| async move {
            db.execute(record).await;
            Ok::<_, anyhow::Error>(db)
        });
        Ok(Box::pin(unfold))
    }
}
