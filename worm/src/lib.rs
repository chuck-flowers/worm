//! A Wannabe Object Relational Mapper (WORM)

#![warn(clippy::all)]
#![warn(missing_docs)]

pub mod errors;
pub mod results;
pub mod sql;

use self::errors::WormError;
use self::results::QueryResults;
use self::sql::SqlResult;

/// A type that represents a templated script.
pub trait Script {
    /// The Rust type that rows returned by the script should be interpretted as.
    type Output: SqlResult;

    /// Executes the current value of the script.
    fn execute(self) -> Result<QueryResults<Self::Output>, WormError>;
}
