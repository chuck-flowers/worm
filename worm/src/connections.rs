//! Types for interacting with a database.

use crate::errors::WormError;
use crate::executors::SqlExecutor;
use crate::results::QueryResults;
use crate::Script;

/// An active connection with a databse which automatically converts between
/// SQL types and Rust types.
pub struct Connection {
    /// The executor which connects to the database.
    executor: Box<dyn SqlExecutor>,
}

impl Connection {
    /// Creates a new connection with the provided executor.
    pub fn new<E>(executor: E) -> Self
    where
        E: SqlExecutor + 'static,
    {
        let executor = Box::new(executor);
        Self { executor }
    }

    /// Executes a provided script.
    pub fn execute<S>(&self, script: S) -> Result<QueryResults<S::Output>, WormError>
    where
        S: Script,
    {
        let sql = script.compile();
        let row_iter = self.executor.execute_sql(&sql)?;
        Ok(QueryResults::new(row_iter))
    }
}
