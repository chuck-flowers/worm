//! Types that represent raw SQL.

mod fields;

pub use self::fields::RecordField;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::iter::FromIterator;

/// A raw SQL value.
pub enum SqlValue {
    /// A simple `true` or `false` value.
    Boolean(bool),
    /// A numeric value with a 'floating' decimal value.
    Float(f64),
    /// A non-existent value.
    Null,
    /// An unbounded sequence of text characters.
    String(String),
    /// A whole number that can be negative or positive.
    SignedInteger(i128),
    /// A whole number that cannot be negative.
    UnsignedInteger(u128),
}

impl Display for SqlValue {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            SqlValue::Boolean(b) => b.fmt(f),
            SqlValue::Float(float) => float.fmt(f),
            SqlValue::Null => f.write_str("NULL"),
            SqlValue::String(string) => write!(f, "'{}'", string),
            SqlValue::SignedInteger(integer) => integer.fmt(f),
            SqlValue::UnsignedInteger(integer) => integer.fmt(f),
        }
    }
}

/// A raw SQL row.
pub struct SqlRow(Vec<SqlValue>);

impl FromIterator<SqlValue> for SqlRow {
    fn from_iter<T: IntoIterator<Item = SqlValue>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
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
