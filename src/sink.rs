use anyhow::{Context, Result};
use async_trait::async_trait;
use url::Url;

use fluvio::Offset;
use fluvio_connector_common::{LocalBoxSink, Sink};
use fluvio_model_sql::Operation;

use crate::{config::DuckDBConfig, db::DuckDB};

#[derive(Debug)]
pub(crate) struct DuckDBSink {
    url: Url,
}

impl DuckDBSink {
    pub(crate) fn new(config: &DuckDBConfig) -> Result<Self> {
        let url = Url::parse(&config.url.resolve()?).context("unable to parse sql url")?;

        Ok(Self { url })
    }
}

#[async_trait]
impl Sink<Operation> for DuckDBSink {
    async fn connect(self, _offset: Option<Offset>) -> Result<LocalBoxSink<Operation>> {
        let db = DuckDB::connect(self.url.as_str()).await?;
        let unfold = futures::sink::unfold(db, |mut db: DuckDB, record: Operation| async move {
            db.execute(record).await?;
            Ok::<_, anyhow::Error>(db)
        });
        Ok(Box::pin(unfold))
    }
}
