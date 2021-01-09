//! Types that execute raw SQL within a DBMS.

use crate::errors::RowConversionError;
use crate::errors::SqlExecutionError;
use crate::sql::SqlRow;

/// The iterator that is returned by an executor.
pub type ResultIter = Box<dyn Iterator<Item = Result<SqlRow, RowConversionError>>>;

/// A type which can execute SQL.
pub trait SqlExecutor {
    /// Executes the supplied script
    fn execute_sql(&self, sql: &str) -> Result<ResultIter, SqlExecutionError>;
}
