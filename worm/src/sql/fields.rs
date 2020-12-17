use crate::sql::SqlValue;
use metafor::metafor;

#[derive(Debug, Eq, PartialEq)]
pub enum FieldConversionError {
    IncorrectType,
    ValueOutOfBounds,
}

/// A type which can be converted from/to a raw SQL literal.
pub trait RecordField {
    /// Creates a new instance of the type from a provided [SqlValue]
    fn from_sql(sql_value: SqlValue) -> Result<Self, FieldConversionError>
    where
        Self: Sized;

    /// Creates a [SqlValue] from an instance of the type.
    fn into_sql(self) -> SqlValue;
}

#[metafor(
    variant = [
        { ty: bool, name: Boolean },
        { ty: f64, name: Float },
        { ty: String, name: String },
        { ty: i128, name: SignedInteger },
        { ty: u128, name: UnsignedInteger }
    ]
)]
impl RecordField for __variant__ty__ {
    fn from_sql(sql_value: SqlValue) -> Result<Self, FieldConversionError>
    where
        Self: Sized,
    {
        if let SqlValue::__variant__name__(value) = sql_value {
            Ok(value)
        } else {
            Err(FieldConversionError::IncorrectType)
        }
    }

    fn into_sql(self) -> SqlValue {
        SqlValue::__variant__name__(self)
    }
}

#[metafor(
    u_type = [
        u8,
        u16,
        u32,
        u64
    ]
)]
impl RecordField for __u_type__ {
    fn from_sql(sql_value: SqlValue) -> Result<Self, FieldConversionError>
    where
        Self: Sized,
    {
        match sql_value {
            SqlValue::UnsignedInteger(u) => {
                if ((__u_type__::MIN as u128)..=(__u_type__::MAX as u128)).contains(&u) {
                    Ok(u as __u_type__)
                } else {
                    Err(FieldConversionError::ValueOutOfBounds)
                }
            }
            SqlValue::SignedInteger(i) => {
                if i >= 0 && (__u_type__::MIN..=__u_type__::MAX).contains(&(i as __u_type__)) {
                    Ok(i as __u_type__)
                } else {
                    Err(FieldConversionError::ValueOutOfBounds)
                }
            }
            _ => Err(FieldConversionError::IncorrectType),
        }
    }

    fn into_sql(self) -> SqlValue {
        SqlValue::UnsignedInteger(self as u128)
    }
}

#[metafor(
    i_type = [
        i8,
        i16,
        i32,
        i64
    ]
)]
impl RecordField for __i_type__ {
    fn from_sql(sql_value: SqlValue) -> Result<Self, FieldConversionError>
    where
        Self: Sized,
    {
        match sql_value {
            SqlValue::SignedInteger(i) => {
                if ((__i_type__::MIN as i128)..=(__i_type__::MAX as i128)).contains(&i) {
                    Ok(i as __i_type__)
                } else {
                    Err(FieldConversionError::ValueOutOfBounds)
                }
            }
            SqlValue::UnsignedInteger(u) => {
                if (0..=(__i_type__::MAX as u128)).contains(&u) {
                    Ok(u as __i_type__)
                } else {
                    Err(FieldConversionError::ValueOutOfBounds)
                }
            }
            _ => Err(FieldConversionError::IncorrectType),
        }
    }

    fn into_sql(self) -> SqlValue {
        SqlValue::SignedInteger(self as i128)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signed_typical_from_sql() {
        let result = i8::from_sql(SqlValue::SignedInteger(127));
        assert_eq!(result, Ok(127))
    }

    #[test]
    fn signed_atypical_from_sql() {
        let result = i8::from_sql(SqlValue::UnsignedInteger(127));
        assert_eq!(result, Ok(127))
    }

    #[test]
    fn signed_out_of_bounds_from_sql() {
        let result = i8::from_sql(SqlValue::SignedInteger(128));
        assert_eq!(result, Err(FieldConversionError::ValueOutOfBounds))
    }

    #[test]
    fn unsigned_typical_from_sql() {
        let result = u8::from_sql(SqlValue::UnsignedInteger(255));
        assert_eq!(result, Ok(255))
    }

    #[test]
    fn unsigned_atypical_from_sql() {
        let result = u8::from_sql(SqlValue::SignedInteger(255));
        assert_eq!(result, Ok(255))
    }

    #[test]
    fn unsigned_out_of_bounds_from_sql() {
        let result = u8::from_sql(SqlValue::UnsignedInteger(256));
        assert_eq!(result, Err(FieldConversionError::ValueOutOfBounds))
    }
}
