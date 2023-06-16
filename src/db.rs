use anyhow::Result;

use fluvio_connector_common::tracing::{error, info};
use fluvio_model_sql::{Insert, Operation};

use duckdb::{params_from_iter, Connection as DuckDbConnection};

use crate::model::DuckDBValue;

pub struct DuckDB(DuckDbConnection);

impl DuckDB {
    pub(crate) async fn connect(url: &str) -> anyhow::Result<Self> {
        info!(url, "opening duckdb");

        let conn = DuckDbConnection::open(url)?;
        Ok(Self(conn))
    }

    pub(crate) async fn execute(&mut self, operation: Operation) -> anyhow::Result<()> {
        match operation {
            Operation::Insert(row) => {
                self.insert(row)?;
            }
            Operation::Upsert(_row) => {
                todo!()
            }
        }
        Ok(())
    }

    fn insert(&mut self, row: Insert) -> anyhow::Result<()> {
        if let Err(err) = insert(&self.0, row) {
            error!("unable to insert duckdb: {}", err);
        }
        Ok(())
    }
}

pub(crate) fn insert(conn: &DuckDbConnection, row: Insert) -> Result<()> {
    let mut query = String::from("INSERT INTO ");
    query.push_str(&row.table);
    query.push_str(" (");
    for value in &row.values {
        query.push_str(&value.column);
        query.push_str(",");
    }
    query.pop();
    query.push_str(") ");
    query.push_str(" VALUES (");
    for _ in 0..row.values.len() {
        query.push_str("?,");
    }
    query.pop();
    query.push_str(")");

    let mut stmt = conn.prepare(&query)?;

    let ducdb_values: Vec<DuckDBValue> =
        row.values.into_iter().map(|v| v.into()).collect::<Vec<_>>();
    let params = params_from_iter(&ducdb_values);
    stmt.execute(params)?;

    Ok(())
}
