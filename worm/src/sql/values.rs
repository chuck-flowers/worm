use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;

/// A raw SQL value.
pub enum SqlValue {
    /// A simple `true` or `false` value.
    #[cfg(feature = "sql-value-bool")]
    Boolean(bool),
    /// A 4-byte numeric value with a 'floating' decimal value.
    #[cfg(feature = "sql-value-f32")]
    Float32(f32),
    /// An 8-byte numeric value with a 'floating' decimal value.
    #[cfg(feature = "sql-value-f64")]
    Float64(f64),
    /// A non-existent value.
    Null,
    /// An unbounded sequence of text characters.
    #[cfg(feature = "sql-value-string")]
    String(String),
    /// A 1-byte whole number that can be negative or positive.
    #[cfg(feature = "sql-value-i8")]
    Signed8(i8),
    /// A 2-byte whole number that can be negative or positive.
    #[cfg(feature = "sql-value-i16")]
    Signed16(i16),
    /// A 4-byte whole number that can be negative or positive.
    #[cfg(feature = "sql-value-i32")]
    Signed32(i32),
    /// A 8-byte whole number that can be negative or positive.
    #[cfg(feature = "sql-value-i64")]
    Signed64(i64),
    /// A 16-byte whole number that can be negative or positive.
    #[cfg(feature = "sql-value-i128")]
    Signed128(i128),
    /// A 1-byte whole number that can be negative or positive.
    #[cfg(feature = "sql-value-u8")]
    Unsigned8(u8),
    /// A 2-byte whole number that can be negative or positive.
    #[cfg(feature = "sql-value-u16")]
    Unsigned16(u16),
    /// A 4-byte whole number that can be negative or positive.
    #[cfg(feature = "sql-value-u32")]
    Unsigned32(u32),
    /// A 8-byte whole number that can be negative or positive.
    #[cfg(feature = "sql-value-u64")]
    Unsigned64(u64),
    /// A 16-byte whole number that cannot be negative.
    #[cfg(feature = "sql-value-u128")]
    Unsigned128(u128),
}

impl Display for SqlValue {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            #[cfg(feature = "sql-value-bool")]
            SqlValue::Boolean(b) => b.fmt(f),
            #[cfg(feature = "sql-value-f32")]
            SqlValue::Float32(float) => float.fmt(f),
            #[cfg(feature = "sql-value-f64")]
            SqlValue::Float64(float) => float.fmt(f),
            SqlValue::Null => f.write_str("NULL"),
            #[cfg(feature = "sql-value-string")]
            SqlValue::String(string) => write!(f, "'{}'", string),
            #[cfg(feature = "sql-value-i8")]
            SqlValue::Signed8(integer) => integer.fmt(f),
            #[cfg(feature = "sql-value-i16")]
            SqlValue::Signed16(integer) => integer.fmt(f),
            #[cfg(feature = "sql-value-i32")]
            SqlValue::Signed32(integer) => integer.fmt(f),
            #[cfg(feature = "sql-value-i64")]
            SqlValue::Signed64(integer) => integer.fmt(f),
            #[cfg(feature = "sql-value-i128")]
            SqlValue::Signed128(integer) => integer.fmt(f),
            #[cfg(feature = "sql-value-u8")]
            SqlValue::Unsigned8(integer) => integer.fmt(f),
            #[cfg(feature = "sql-value-u16")]
            SqlValue::Unsigned16(integer) => integer.fmt(f),
            #[cfg(feature = "sql-value-u32")]
            SqlValue::Unsigned32(integer) => integer.fmt(f),
            #[cfg(feature = "sql-value-u64")]
            SqlValue::Unsigned64(integer) => integer.fmt(f),
            #[cfg(feature = "sql-value-u128")]
            SqlValue::Unsigned128(integer) => integer.fmt(f),
        }
    }
}
