//! Types that execute raw SQL within a DBMS.

use crate::errors::ConnectionError;
use crate::errors::RowConversionError;
use crate::errors::SqlExecutionError;
use crate::sql::SqlRow;

/// The iterator that is returned by an executor.
pub type ResultIter<'a> = Box<dyn Iterator<Item = Result<SqlRow, RowConversionError>> + 'a>;

/// A type which can execute SQL.
pub trait SqlExecutor {
    /// Creates a SqlExecutor with the provided connection string.
    fn connect(connection_string: &str) -> Result<Self, ConnectionError>
    where
        Self: Sized;

    /// Executes the supplied script
    fn execute_sql<'a>(&'a mut self, sql: &str) -> Result<ResultIter<'a>, SqlExecutionError>;
}
