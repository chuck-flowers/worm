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
use worm::errors::ConnectionError;
use worm::errors::RawRowConversionError;
use worm::errors::RowConversionError;
use worm::errors::SqlExecutionError;
use worm::executors::ResultIter;
use worm::executors::SqlExecutor;
use worm::sql::RecordField;
use worm::sql::SqlRow;
use worm::sql::SqlValue;

struct PgWormSqlValue(SqlValue);

impl<'a> FromSql<'a> for PgWormSqlValue {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        fn make_mapper<'a, T>(
            ty: &Type,
            raw: &'a [u8],
        ) -> Result<PgWormSqlValue, Box<dyn Error + Sync + Send>>
        where
            T: RecordField + FromSql<'a>,
        {
            <T as FromSql<'a>>::from_sql(ty, raw).map(|t| PgWormSqlValue(t.into_sql()))
        }

        let results = [
            make_mapper::<bool>,
            make_mapper::<f32>,
            make_mapper::<f64>,
            make_mapper::<String>,
            make_mapper::<i8>,
            make_mapper::<i16>,
            make_mapper::<i32>,
            make_mapper::<i64>,
            make_mapper::<u32>,
        ]
        .iter()
        .map(|f| f(ty, raw));

        Ok(results.filter_map(Result::ok).next().unwrap())
    }

    fn accepts(ty: &Type) -> bool {
        [
            bool::accepts,
            f32::accepts,
            f64::accepts,
            String::accepts,
            i8::accepts,
            i16::accepts,
            i32::accepts,
            i64::accepts,
            u32::accepts,
        ]
        .iter()
        .any(|f| f(ty))
    }
}

/// An worm executor for postgres
pub struct PostgresExecutor(Client);

impl SqlExecutor for PostgresExecutor {
    fn connect(connection_string: &str) -> Result<Self, ConnectionError>
    where
        Self: Sized,
    {
        match Client::connect(connection_string, NoTls) {
            Ok(client) => Ok(Self(client)),
            Err(_) => Err(ConnectionError::new(connection_string.to_owned())),
        }
    }

    fn execute_sql<'a>(&'a mut self, sql: &str) -> Result<ResultIter<'a>, SqlExecutionError> {
        let client = &mut self.0;
        let pg_row_iter = match client.query_raw::<_, bool, _>(sql, core::iter::empty()) {
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
