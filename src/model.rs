use std::ops::Deref;

use duckdb::types::{TimeUnit, Value as DuckValue};
use duckdb::{types::ToSqlOutput, ToSql};

use fluvio_model_sql::Value;

pub(crate) struct DuckDBValue(Value);

impl Deref for DuckDBValue {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Value> for DuckDBValue {
    fn from(value: Value) -> Self {
        Self(value)
    }
}

impl ToSql for DuckDBValue {
    fn to_sql(&self) -> duckdb::Result<duckdb::types::ToSqlOutput<'_>> {
        use fluvio_model_sql::Type::{
            BigInt, Bool, Bytes, Char, Date, DoublePrecision, Float, Int, Json, Numeric, SmallInt,
            Text, Time, Timestamp, Uuid,
        };

        if self.raw_value == "NULL" {
            return Ok(ToSqlOutput::from(duckdb::types::Null));
        }
        match self.type_ {
            Bool => {
                let value: bool = self
                    .raw_value
                    .parse::<bool>()
                    .map_err(|err| duckdb::Error::ToSqlConversionFailure(Box::new(err)))?;
                Ok(ToSqlOutput::from(value))
            }
            Char => Ok(ToSqlOutput::from(self.raw_value.as_str())),
            SmallInt => {
                let value: i16 = self
                    .raw_value
                    .parse::<i16>()
                    .map_err(|err| duckdb::Error::ToSqlConversionFailure(Box::new(err)))?;
                Ok(ToSqlOutput::from(value))
            }
            Int => {
                let value: i32 = self
                    .raw_value
                    .parse::<i32>()
                    .map_err(|err| duckdb::Error::ToSqlConversionFailure(Box::new(err)))?;
                Ok(ToSqlOutput::from(value))
            }
            BigInt => {
                let value: i64 = self
                    .raw_value
                    .parse::<i64>()
                    .map_err(|err| duckdb::Error::ToSqlConversionFailure(Box::new(err)))?;
                Ok(ToSqlOutput::from(value))
            }
            Float => {
                let value: f32 = self
                    .raw_value
                    .parse::<f32>()
                    .map_err(|err| duckdb::Error::ToSqlConversionFailure(Box::new(err)))?;
                Ok(ToSqlOutput::from(value))
            }
            DoublePrecision => {
                let value: f64 = self
                    .raw_value
                    .parse::<f64>()
                    .map_err(|err| duckdb::Error::ToSqlConversionFailure(Box::new(err)))?;
                Ok(ToSqlOutput::from(value))
            }
            Text => Ok(ToSqlOutput::from(self.raw_value.as_str())),
            Bytes => Ok(ToSqlOutput::from(self.raw_value.as_bytes())),
            Numeric => todo!(),
            Timestamp => {
                // 2023-03-03T18:30:18.679Z
                //  println!("parsing timestamp: {}",self.raw_value.as_str());
                let timestamp = chrono::DateTime::parse_from_rfc3339(self.raw_value.as_str())
                    .map_err(|err| duckdb::Error::ToSqlConversionFailure(Box::new(err)))?;
                //  println!("timestamp: {:#?}",timestamp);
                Ok(ToSqlOutput::Owned(DuckValue::Timestamp(
                    TimeUnit::Millisecond,
                    timestamp.timestamp_millis(),
                )))
            }
            Date => todo!(),
            Time => todo!(),
            Uuid => todo!(),
            Json => todo!(),
        }
    }
}
