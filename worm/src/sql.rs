//! Types that represent raw SQL.

mod fields;
mod values;

pub use self::fields::RecordField;
pub use self::values::SqlValue;
use core::iter::FromIterator;

/// A raw SQL row.
pub struct SqlRow(Vec<SqlValue>);

impl FromIterator<SqlValue> for SqlRow {
    fn from_iter<T: IntoIterator<Item = SqlValue>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl IntoIterator for SqlRow {
    type Item = SqlValue;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// A type that can be returned by a SQL query.
pub trait SqlResult {
    /// Converts a SQL row into an instance of the type.
    fn from_row(row: SqlRow) -> Self
    where
        Self: Sized;
}

impl SqlResult for () {
    fn from_row(_: SqlRow) -> Self
    where
        Self: Sized,
    {
    }
}
