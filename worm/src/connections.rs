//! Types for interacting with a database.

use crate::errors::WormError;
use crate::executors::SqlExecutor;
use crate::results::QueryResults;
use crate::Script;

/// An active connection with a databse which automatically converts between
/// SQL types and Rust types.
pub struct Connection<E>
where
    E: SqlExecutor,
{
    /// The executor which connects to the database.
    executor: E,
}

impl<E> Connection<E>
where
    E: SqlExecutor,
{
    /// Creates a new connection with the provided executor.
    pub fn new(executor: E) -> Self {
        Self { executor }
    }

    /// Executes a provided script.
    pub fn execute<S>(&mut self, script: S) -> Result<QueryResults<S::Output>, WormError>
    where
        S: Script,
    {
        let sql = script.compile();
        let row_iter = self.executor.execute_sql(&sql)?;
        Ok(QueryResults::new(row_iter))
    }
}
