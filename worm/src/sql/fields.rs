use crate::errors::FieldConversionError;
use crate::sql::SqlValue;

/// A type which can be converted from/to a raw SQL literal.
pub trait RecordField {
    /// Creates a new instance of the type from a provided [SqlValue]
    fn from_sql(sql_value: SqlValue) -> Result<Self, FieldConversionError>
    where
        Self: Sized;

    /// Creates a [SqlValue] from an instance of the type.
    fn into_sql(self) -> SqlValue;
}

#[cfg(feature = "sql-value-bool")]
impl RecordField for bool {
    fn from_sql(sql_value: SqlValue) -> Result<Self, FieldConversionError>
    where
        Self: Sized,
    {
        if let SqlValue::Boolean(b) = sql_value {
            Ok(b)
        } else {
            Err(FieldConversionError::IncorrectType)
        }
    }

    fn into_sql(self) -> SqlValue {
        SqlValue::Boolean(self)
    }
}

#[cfg(feature = "sql-value-f32")]
impl RecordField for f32 {
    fn from_sql(sql_value: SqlValue) -> Result<Self, FieldConversionError>
    where
        Self: Sized,
    {
        if let SqlValue::Float32(f) = sql_value {
            Ok(f)
        } else {
            Err(FieldConversionError::IncorrectType)
        }
    }

    fn into_sql(self) -> SqlValue {
        SqlValue::Float32(self)
    }
}

#[cfg(feature = "sql-value-f64")]
impl RecordField for f64 {
    fn from_sql(sql_value: SqlValue) -> Result<Self, FieldConversionError>
    where
        Self: Sized,
    {
        if let SqlValue::Float64(f) = sql_value {
            Ok(f)
        } else {
            Err(FieldConversionError::IncorrectType)
        }
    }

    fn into_sql(self) -> SqlValue {
        SqlValue::Float64(self)
    }
}

#[cfg(feature = "sql-value-string")]
impl RecordField for String {
    fn from_sql(sql_value: SqlValue) -> Result<Self, FieldConversionError>
    where
        Self: Sized,
    {
        if let SqlValue::String(string) = sql_value {
            Ok(string)
        } else {
            Err(FieldConversionError::IncorrectType)
        }
    }

    fn into_sql(self) -> SqlValue {
        SqlValue::String(self)
    }
}

#[cfg(feature = "sql-value-i8")]
impl RecordField for i8 {
    fn from_sql(sql_value: SqlValue) -> Result<Self, FieldConversionError>
    where
        Self: Sized,
    {
        if let SqlValue::Signed8(i) = sql_value {
            Ok(i)
        } else {
            Err(FieldConversionError::IncorrectType)
        }
    }

    fn into_sql(self) -> SqlValue {
        SqlValue::Signed8(self)
    }
}

#[cfg(feature = "sql-value-i16")]
impl RecordField for i16 {
    fn from_sql(sql_value: SqlValue) -> Result<Self, FieldConversionError>
    where
        Self: Sized,
    {
        if let SqlValue::Signed16(i) = sql_value {
            Ok(i)
        } else {
            Err(FieldConversionError::IncorrectType)
        }
    }

    fn into_sql(self) -> SqlValue {
        SqlValue::Signed16(self)
    }
}

#[cfg(feature = "sql-value-i32")]
impl RecordField for i32 {
    fn from_sql(sql_value: SqlValue) -> Result<Self, FieldConversionError>
    where
        Self: Sized,
    {
        if let SqlValue::Signed32(i) = sql_value {
            Ok(i)
        } else {
            Err(FieldConversionError::IncorrectType)
        }
    }

    fn into_sql(self) -> SqlValue {
        SqlValue::Signed32(self)
    }
}

#[cfg(feature = "sql-value-i64")]
impl RecordField for i64 {
    fn from_sql(sql_value: SqlValue) -> Result<Self, FieldConversionError>
    where
        Self: Sized,
    {
        if let SqlValue::Signed64(i) = sql_value {
            Ok(i)
        } else {
            Err(FieldConversionError::IncorrectType)
        }
    }

    fn into_sql(self) -> SqlValue {
        SqlValue::Signed64(self)
    }
}

#[cfg(feature = "sql-value-i128")]
impl RecordField for i128 {
    fn from_sql(sql_value: SqlValue) -> Result<Self, FieldConversionError>
    where
        Self: Sized,
    {
        if let SqlValue::Signed128(i) = sql_value {
            Ok(i)
        } else {
            Err(FieldConversionError::IncorrectType)
        }
    }

    fn into_sql(self) -> SqlValue {
        SqlValue::Signed128(self)
    }
}

#[cfg(feature = "sql-value-u8")]
impl RecordField for u8 {
    fn from_sql(sql_value: SqlValue) -> Result<Self, FieldConversionError>
    where
        Self: Sized,
    {
        if let SqlValue::Unsigned8(u) = sql_value {
            Ok(u)
        } else {
            Err(FieldConversionError::IncorrectType)
        }
    }

    fn into_sql(self) -> SqlValue {
        SqlValue::Unsigned8(self)
    }
}

#[cfg(feature = "sql-value-u16")]
impl RecordField for u16 {
    fn from_sql(sql_value: SqlValue) -> Result<Self, FieldConversionError>
    where
        Self: Sized,
    {
        if let SqlValue::Unsigned16(u) = sql_value {
            Ok(u)
        } else {
            Err(FieldConversionError::IncorrectType)
        }
    }

    fn into_sql(self) -> SqlValue {
        SqlValue::Unsigned16(self)
    }
}

#[cfg(feature = "sql-value-u32")]
impl RecordField for u32 {
    fn from_sql(sql_value: SqlValue) -> Result<Self, FieldConversionError>
    where
        Self: Sized,
    {
        if let SqlValue::Unsigned32(u) = sql_value {
            Ok(u)
        } else {
            Err(FieldConversionError::IncorrectType)
        }
    }

    fn into_sql(self) -> SqlValue {
        SqlValue::Unsigned32(self)
    }
}

#[cfg(feature = "sql-value-u64")]
impl RecordField for u64 {
    fn from_sql(sql_value: SqlValue) -> Result<Self, FieldConversionError>
    where
        Self: Sized,
    {
        if let SqlValue::Unsigned64(u) = sql_value {
            Ok(u)
        } else {
            Err(FieldConversionError::IncorrectType)
        }
    }

    fn into_sql(self) -> SqlValue {
        SqlValue::Unsigned64(self)
    }
}

#[cfg(feature = "sql-value-u128")]
impl RecordField for u128 {
    fn from_sql(sql_value: SqlValue) -> Result<Self, FieldConversionError>
    where
        Self: Sized,
    {
        if let SqlValue::Unsigned128(u) = sql_value {
            Ok(u)
        } else {
            Err(FieldConversionError::IncorrectType)
        }
    }

    fn into_sql(self) -> SqlValue {
        SqlValue::Unsigned128(self)
    }
}
