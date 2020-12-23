//! Types that represent raw SQL.

mod fields;

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

/// A raw SQL row.
pub struct SqlRow(Vec<SqlValue>);

/// A type that can be returned by a SQL query.
pub trait SqlResult {
    /// Converts a SQL row into an instance of the type.
    fn from_row(row: SqlRow) -> Self
    where
        Self: Sized;
}
