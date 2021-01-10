//! Types that represent the results of executed queries.

use crate::errors::RowConversionError;
use crate::executors::ResultIter;
use crate::sql::SqlResult;
use core::marker::PhantomData;

/// The results of a query.
pub struct QueryResults<'a, T>
where
    T: SqlResult,
{
    __: PhantomData<T>,
    row_iter: ResultIter<'a>,
}

impl<'a, T> QueryResults<'a, T>
where
    T: SqlResult,
{
    pub(crate) fn new(row_iter: ResultIter<'a>) -> Self {
        Self {
            __: PhantomData {},
            row_iter,
        }
    }
}

impl<'a, T> Iterator for QueryResults<'a, T>
where
    T: SqlResult,
{
    type Item = Result<T, RowConversionError>;
    fn next(&mut self) -> Option<Self::Item> {
        self.row_iter
            .next()
            .map(|res| res.map(T::from_row).map_err(RowConversionError::from))
    }
}
