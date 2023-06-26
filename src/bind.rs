use std::ffi::{c_void, CStr};
use std::marker::PhantomData;
use std::mem::size_of;
use std::ops::DerefMut;
use std::os::raw::c_char;
use std::path::Path;
use std::ptr::{self};
use std::{ffi::CString, ops::Deref};

use anyhow::{anyhow, Result};
use libduckdb_sys::{
    duckdb_append_bool, duckdb_appender, duckdb_appender_create, duckdb_appender_end_row,
    duckdb_appender_error, duckdb_config, duckdb_connect, duckdb_connection,
    duckdb_create_logical_type, duckdb_database, duckdb_destroy_value, duckdb_get_int64,
    duckdb_get_varchar, duckdb_logical_type, duckdb_malloc, duckdb_open_ext, duckdb_state,
    duckdb_value, duckdb_vector, duckdb_vector_assign_string_element_len, duckdb_vector_get_data,
    duckdb_vector_size, idx_t, DuckDBSuccess, DUCKDB_TYPE_DUCKDB_TYPE_BIGINT,
    DUCKDB_TYPE_DUCKDB_TYPE_BLOB, DUCKDB_TYPE_DUCKDB_TYPE_BOOLEAN, DUCKDB_TYPE_DUCKDB_TYPE_DATE,
    DUCKDB_TYPE_DUCKDB_TYPE_DECIMAL, DUCKDB_TYPE_DUCKDB_TYPE_DOUBLE, DUCKDB_TYPE_DUCKDB_TYPE_ENUM,
    DUCKDB_TYPE_DUCKDB_TYPE_FLOAT, DUCKDB_TYPE_DUCKDB_TYPE_HUGEINT,
    DUCKDB_TYPE_DUCKDB_TYPE_INTEGER, DUCKDB_TYPE_DUCKDB_TYPE_INTERVAL,
    DUCKDB_TYPE_DUCKDB_TYPE_LIST, DUCKDB_TYPE_DUCKDB_TYPE_MAP, DUCKDB_TYPE_DUCKDB_TYPE_SMALLINT,
    DUCKDB_TYPE_DUCKDB_TYPE_STRUCT, DUCKDB_TYPE_DUCKDB_TYPE_TIME,
    DUCKDB_TYPE_DUCKDB_TYPE_TIMESTAMP, DUCKDB_TYPE_DUCKDB_TYPE_TIMESTAMP_MS,
    DUCKDB_TYPE_DUCKDB_TYPE_TIMESTAMP_NS, DUCKDB_TYPE_DUCKDB_TYPE_TIMESTAMP_S,
    DUCKDB_TYPE_DUCKDB_TYPE_TINYINT, DUCKDB_TYPE_DUCKDB_TYPE_UBIGINT,
    DUCKDB_TYPE_DUCKDB_TYPE_UINTEGER, DUCKDB_TYPE_DUCKDB_TYPE_UNION,
    DUCKDB_TYPE_DUCKDB_TYPE_USMALLINT, DUCKDB_TYPE_DUCKDB_TYPE_UTINYINT,
    DUCKDB_TYPE_DUCKDB_TYPE_UUID, DUCKDB_TYPE_DUCKDB_TYPE_VARCHAR,
};

#[macro_export]
macro_rules! c_string {
    ($c_str:expr) => {
        $c_str.as_ptr().cast::<c_char>()
    };
}

#[macro_export]
macro_rules! duck_error {
    ($status:expr,$err:ident) => {
        let status = $status;
        if status != libduckdb_sys::DuckDBSuccess {
            let msg = std::ffi::CStr::from_ptr($err).to_str()?;
            let rust_err = Err(anyhow::anyhow!("Failed to connect to database: {msg}"));
            libduckdb_sys::duckdb_free($err as *mut c_void);
            return rust_err;
        }
    };
}

pub unsafe fn malloc_struct<T>() -> *mut T {
    duckdb_malloc(size_of::<T>()).cast::<T>()
}

pub struct Value(pub(crate) duckdb_value);

impl Value {
    pub fn get_varchar(&self) -> CString {
        unsafe { CString::from_raw(duckdb_get_varchar(self.0)) }
    }

    #[allow(unused)]
    pub fn get_int64(&self) -> i64 {
        unsafe { duckdb_get_int64(self.0) }
    }
}

impl From<duckdb_value> for Value {
    fn from(value: duckdb_value) -> Self {
        Self(value)
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        unsafe {
            duckdb_destroy_value(&mut self.0);
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
#[repr(u32)]
pub enum DuckDBTypeEnum {
    Boolean = DUCKDB_TYPE_DUCKDB_TYPE_BOOLEAN,
    Tinyint = DUCKDB_TYPE_DUCKDB_TYPE_TINYINT,
    Smallint = DUCKDB_TYPE_DUCKDB_TYPE_SMALLINT,
    /// Signed 32-bit integer
    Integer = DUCKDB_TYPE_DUCKDB_TYPE_INTEGER,
    Bigint = DUCKDB_TYPE_DUCKDB_TYPE_BIGINT,
    Utinyint = DUCKDB_TYPE_DUCKDB_TYPE_UTINYINT,
    Usmallint = DUCKDB_TYPE_DUCKDB_TYPE_USMALLINT,
    Uinteger = DUCKDB_TYPE_DUCKDB_TYPE_UINTEGER,
    Ubigint = DUCKDB_TYPE_DUCKDB_TYPE_UBIGINT,
    Float = DUCKDB_TYPE_DUCKDB_TYPE_FLOAT,
    Double = DUCKDB_TYPE_DUCKDB_TYPE_DOUBLE,
    Timestamp = DUCKDB_TYPE_DUCKDB_TYPE_TIMESTAMP,
    Date = DUCKDB_TYPE_DUCKDB_TYPE_DATE,
    Time = DUCKDB_TYPE_DUCKDB_TYPE_TIME,
    Interval = DUCKDB_TYPE_DUCKDB_TYPE_INTERVAL,
    Hugeint = DUCKDB_TYPE_DUCKDB_TYPE_HUGEINT,
    Varchar = DUCKDB_TYPE_DUCKDB_TYPE_VARCHAR,
    Blob = DUCKDB_TYPE_DUCKDB_TYPE_BLOB,
    Decimal = DUCKDB_TYPE_DUCKDB_TYPE_DECIMAL,
    TimestampS = DUCKDB_TYPE_DUCKDB_TYPE_TIMESTAMP_S,
    TimestampMs = DUCKDB_TYPE_DUCKDB_TYPE_TIMESTAMP_MS,
    TimestampNs = DUCKDB_TYPE_DUCKDB_TYPE_TIMESTAMP_NS,
    Enum = DUCKDB_TYPE_DUCKDB_TYPE_ENUM,
    List = DUCKDB_TYPE_DUCKDB_TYPE_LIST,
    Struct = DUCKDB_TYPE_DUCKDB_TYPE_STRUCT,
    Map = DUCKDB_TYPE_DUCKDB_TYPE_MAP,
    Uuid = DUCKDB_TYPE_DUCKDB_TYPE_UUID,
    Union = DUCKDB_TYPE_DUCKDB_TYPE_UNION,
}

pub struct LogicalType(pub(crate) duckdb_logical_type);

impl Default for LogicalType {
    fn default() -> Self {
        Self::new(DuckDBTypeEnum::Varchar)
    }
}

impl LogicalType {
    pub fn new(typ: DuckDBTypeEnum) -> Self {
        unsafe {
            Self(duckdb_create_logical_type(
                typ as libduckdb_sys::duckdb_type,
            ))
        }
    }
}

impl Deref for LogicalType {
    type Target = duckdb_logical_type;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Config(duckdb_config);

impl Default for Config {
    fn default() -> Self {
        Self(unsafe { std::ptr::null_mut() })
    }
}

impl Deref for Config {
    type Target = duckdb_config;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Config {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct Database(duckdb_database);

impl From<duckdb_database> for Database {
    fn from(db: duckdb_database) -> Self {
        Self(db)
    }
}

impl Database {
    /// open DuckDB using path
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path_string = match path.as_ref().to_str() {
            Some(path) => path,
            None => return Err(anyhow!("Invalid path")),
        };

        let mut db = ptr::null_mut();
        let config = Config::default();
        let mut c_err = std::ptr::null_mut();
        let c_path_str = CString::new(path_string)?;
        unsafe {
            duck_error!(
                duckdb_open_ext(
                    c_string!(c_path_str),
                    &mut db,
                    *config as duckdb_config,
                    &mut c_err
                ),
                c_err
            );
        }

        Ok(Self::from(db))
    }

    /// opens a connection to the database
    pub fn connect(&self) -> Connection {
        let mut conn = ptr::null_mut();
        unsafe {
            duckdb_connect(self.0, &mut conn);
        }
        Connection::from(conn)
    }
}

pub struct Connection(duckdb_connection);

impl From<duckdb_connection> for Connection {
    fn from(connection: duckdb_connection) -> Self {
        Self(connection)
    }
}

impl Connection {
    pub fn create_appender(&self, schema: &str, table: &str) -> Result<Appender> {
        let mut appender = ptr::null_mut();
        let c_schema = CString::new(schema)?;
        let c_table = CString::new(table)?;
        unsafe {
            duckdb_appender_create(
                self.0,
                c_string!(c_schema),
                c_string!(c_table),
                &mut appender,
            );
        }
        Ok(Appender(appender))
    }
}

pub struct Appender(duckdb_appender);

impl From<duckdb_appender> for Appender {
    fn from(appender: duckdb_appender) -> Self {
        Self(appender)
    }
}

impl Appender {
    /// marke end of row
    pub fn end_row(&mut self) -> Result<()> {
        self.check_error(unsafe { duckdb_appender_end_row(self.0) })
    }

    pub fn append_bool(&mut self, val: bool) -> Result<()> {
        self.check_error(unsafe { duckdb_append_bool(self.0, val) })
    }

    /// check error
    fn check_error(&mut self, state: duckdb_state) -> Result<()> {
        if state == DuckDBSuccess {
            return Ok(());
        }
        let err_msg = unsafe {
            let c_err = duckdb_appender_error(self.0);
            CStr::from_ptr(c_err).to_str()?
        };

        Err(anyhow::anyhow!("Failed to connect to database: {err_msg}"))
    }
}

pub struct Vector<T> {
    duck_ptr: duckdb_vector,
    _phantom: PhantomData<T>,
}

impl<T> From<duckdb_vector> for Vector<T> {
    fn from(duck_ptr: duckdb_vector) -> Self {
        Self {
            duck_ptr,
            _phantom: PhantomData,
        }
    }
}

impl<T> Vector<T> {
    /// set data
    pub fn set_data(&self, row: usize, data: T) {
        let data_ptr: *mut T = unsafe { duckdb_vector_get_data(self.duck_ptr).cast() };
        let data_slice: &mut [T] =
            unsafe { std::slice::from_raw_parts_mut(data_ptr, duckdb_vector_size() as usize) };
        data_slice[row] = data;
    }
}

impl Vector<&[u8]> {
    pub fn assign_string_element(&self, index: idx_t, str: &[u8]) {
        unsafe {
            duckdb_vector_assign_string_element_len(
                self.duck_ptr,
                index,
                str.as_ptr() as *const c_char,
                str.len() as idx_t,
            )
        }
    }
}
