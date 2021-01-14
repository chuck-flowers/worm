//! Types for interacting with a database.

use crate::errors::WormError;
use crate::executors::SqlExecutor;
#[cfg(feature = "rocket-support")]
use crate::pooling::ConnectionManager;
use crate::results::QueryResults;
use crate::Script;
#[cfg(feature = "rocket-support")]
use rocket_contrib::databases::r2d2::Error;
#[cfg(feature = "rocket-support")]
use rocket_contrib::databases::r2d2::Pool;
#[cfg(feature = "rocket-support")]
use rocket_contrib::databases::DatabaseConfig;
#[cfg(feature = "rocket-support")]
use rocket_contrib::databases::Poolable;

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

#[cfg(feature = "rocket-support")]
impl<E> Poolable for Connection<E>
where
    E: SqlExecutor + Send + 'static,
{
    type Manager = ConnectionManager<E>;

    type Error = Error;

    fn pool(config: DatabaseConfig) -> Result<Pool<Self::Manager>, Self::Error> {
        let manager = Self::Manager::new(config.url.to_owned());
        Pool::builder().max_size(config.pool_size).build(manager)
    }
}
