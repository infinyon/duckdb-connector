mod config;
mod db;
mod model;
mod sink;

use config::DuckDBConfig;
use fluvio_connector_common::{
    connector,
    consumer::ConsumerStream,
    tracing::{debug, info, trace},
    Result, Sink,
};
use fluvio_model_sql::Operation;
use futures::SinkExt;
use sink::DuckDBSink;

#[connector(sink)]
async fn start(config: DuckDBConfig, mut stream: impl ConsumerStream) -> Result<()> {
    let sink = DuckDBSink::new(&config)?;
    let mut sink = sink.connect(None).await?;
    info!("Starting consumer loop");
    while let Some(item) = stream.next().await {
        debug!("received record");
        let operation: Operation = serde_json::from_slice(item?.as_ref())?;
        trace!(?operation);
        sink.send(operation).await?;
    }
    Ok(())
}
