use crate::sql::SqlValue;
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
