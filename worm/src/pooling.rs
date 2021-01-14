//! Types that allow for database connection pooling using [r2d2];

#![cfg(feature = "pooling")]

use crate::connections::Connection;
use crate::errors::WormError;
use crate::executors::SqlExecutor;
use crate::Script;
use r2d2::ManageConnection;
use std::marker::PhantomData;

struct TestScript {}

impl Script for TestScript {
    type Output = ();

    fn compile(self) -> String {
        String::from("SELECT 1;")
    }
}

/// A type that is used to manage pooled [Connection] instances.
pub struct ConnectionManager<E>
where
    E: SqlExecutor + Send,
{
    connection_string: String,
    __: PhantomData<E>,
}

unsafe impl<E> Sync for ConnectionManager<E> where E: SqlExecutor + Send {}

impl<E> ManageConnection for ConnectionManager<E>
where
    E: SqlExecutor + Send + 'static,
{
    type Connection = Connection<E>;

    type Error = WormError;

    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let executor = E::connect(&self.connection_string)?;
        Ok(Connection::new(executor))
    }

    fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        conn.execute(TestScript {}).map(|_| ())
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}
