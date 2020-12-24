//! A crate that provides a Worm SQL executor for PostgreSQL

#![warn(clippy::all)]
#![warn(missing_docs)]

use postgres::fallible_iterator::FallibleIterator;
use postgres::types::FromSql;
use postgres::types::Type;
use postgres::Client;
use postgres::Error as PostgresError;
use postgres::NoTls;
use postgres::Row;
use std::error::Error;
use worm::errors::RawRowConversionError;
use worm::errors::RowConversionError;
use worm::errors::SqlExecutionError;
use worm::executors::ResultIter;
use worm::executors::SqlExecutor;
use worm::sql::SqlRow;
use worm::sql::SqlValue;

struct PgWormSqlValue(SqlValue);

impl<'a> FromSql<'a> for PgWormSqlValue {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        match ty {
            &Type::INT2 | &Type::INT4 | &Type::INT8 => {
                let val = i64::from_sql(ty, raw)?;
                Ok(PgWormSqlValue(SqlValue::SignedInteger(val as i128)))
            }
            &Type::TEXT => {
                let val = String::from_sql(ty, raw)?;
                Ok(PgWormSqlValue(SqlValue::String(val)))
            }
            _ => todo!("Unhandled postgres => worm conversion"),
        }
    }

    fn accepts(ty: &Type) -> bool {
        [i16::accepts, i32::accepts, i64::accepts, String::accepts]
            .iter()
            .any(|f| f(ty))
    }
}

/// An worm executor for postgres
pub struct PostgresExecutor(Client);

impl PostgresExecutor {
    /// Creates a new executor that can execute SQL in a specified DBMS.
    pub fn new(conn_string: &str) -> Result<Self, PostgresError> {
        let client = Client::connect(conn_string, NoTls)?;
        Ok(Self(client))
    }
}

impl SqlExecutor for PostgresExecutor {
    fn execute_sql<'a>(&'a mut self, sql: &str) -> Result<ResultIter<'a>, SqlExecutionError> {
        let client = &mut self.0;
        let pg_row_iter = match client.query_raw(sql, core::iter::empty()) {
            Ok(row_iter) => row_iter,
            Err(err) => return Err(SqlExecutionError::Other { err: Box::new(err) }),
        };

        let worm_row_iter = pg_row_iter.iterator().map(convert_row);
        let worm_row_iter_box = Box::new(worm_row_iter);
        Ok(worm_row_iter_box)
    }
}

fn convert_row(pg_row: Result<Row, PostgresError>) -> Result<SqlRow, RowConversionError> {
    let pg_row = pg_row.map_err(|err| RawRowConversionError::Other { err: err.into() })?;

    let row = (0..pg_row.len())
        .map(|i| pg_row.get::<_, PgWormSqlValue>(i).0)
        .collect();

    Ok(row)
}
