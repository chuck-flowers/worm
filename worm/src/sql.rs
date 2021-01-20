//! Types that represent raw SQL.

mod fields;
mod rows;
mod values;

pub use self::fields::RecordField;
pub use self::rows::SqlRow;
pub use self::values::SqlValue;
use crate::errors::RowConversionError;
pub use worm_macros::SqlResult;

/// A type that can be returned by a SQL query.
pub trait SqlResult {
    /// Converts a SQL row into an instance of the type.
    fn from_row(row: SqlRow) -> Result<Self, RowConversionError>
    where
        Self: Sized;
}

impl SqlResult for () {
    fn from_row(_: SqlRow) -> Result<Self, RowConversionError>
    where
        Self: Sized,
    {
        Ok(())
    }
}
