use std::collections::HashMap;

use anyhow::Result;

use fluvio_connector_common::tracing::{debug, error, info};
use fluvio_model_sql::{Insert, Operation};

use duckdb::{params_from_iter, Appender, Connection as DuckDbConnection, ToSql};

use crate::model::DuckDBValue;

pub struct DuckDB {
    conn: DuckDbConnection,
    appends: HashMap<String, Appender<'static>>,
}

impl DuckDB {
    pub(crate) async fn connect(url: &str) -> anyhow::Result<Self> {
        info!(url, "opening duckdb");
        Ok(Self {
            conn: DuckDbConnection::open(url)?,
            appends: HashMap::new(),
        })
    }

    pub(crate) async fn execute(&mut self, operation: Operation) {
        match operation {
            Operation::Insert(row) => {
                if let Err(err) = self.insert(row) {
                    error!("unable to insert duckdb: {}", err);
                }
            }
            Operation::Upsert(_row) => {
                todo!()
            }
        }
    }

    fn insert(&mut self, row: Insert) -> anyhow::Result<()> {
        let appenders = &mut self.appends;
        if !appenders.contains_key(&row.table) {
            debug!(row.table, "creating appender for table");
            // This is a hack to get around the lifetime issue with Appender
            // This should be totally safe since we only are using appender internally
            let appender: Appender<'static> = unsafe {
                std::mem::transmute::<Appender, Appender<'static>>(self.conn.appender(&row.table)?)
            };
            appenders.insert(row.table.clone(), appender);
        }

        if let Some(appender) = appenders.get_mut(&row.table) {
            insert(row, appender)?;
        }

        Ok(())
    }
}

#[allow(clippy::single_char_add_str)]
pub(crate) fn insert(row: Insert, appender: &mut Appender) -> Result<()> {
    let duck_values: Vec<DuckDBValue> =
        row.values.into_iter().map(|v| v.into()).collect::<Vec<_>>();

    let mut sql_values = vec![];
    for duck_value in duck_values.iter() {
        sql_values.push(duck_value.to_sql()?);
    }

    print!("duck_values: {:?}", sql_values);

    let binding = sql_values
        .iter()
        .map(|v| v as &dyn duckdb::ToSql)
        .collect::<Vec<_>>();
    let params: &[&dyn duckdb::ToSql] = binding.as_slice();

    // this cause error
    // TODO: dyn is too hard to work with, instead use
    // low level: https://duckdb.org/docs/api/c/appender
    appender.append_row(params)?;

    // let params = params_from_iter(duck_values);
    // println!("params: {:?}", params);
    //   appender.append_row([params]);

    Ok(())
}
