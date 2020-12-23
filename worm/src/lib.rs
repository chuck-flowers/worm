//! A Wannabe Object Relational Mapper (WORM)

#![warn(clippy::all)]
#![warn(missing_docs)]

pub mod connections;
pub mod errors;
pub mod executors;
pub mod results;
pub mod sql;

use self::sql::SqlResult;

/// A type that represents a templated script.
pub trait Script {
    /// The Rust type that rows returned by the script should be interpretted as.
    type Output: SqlResult;

    /// Compiles the instance of the type into its SQL form.
    fn compile(self) -> String;
}
