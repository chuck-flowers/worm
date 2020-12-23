use crate::sql::SqlResult;
use crate::sql::SqlRow;
use core::marker::PhantomData;

pub struct QueryResults<T>
where
    T: SqlResult,
{
    __: PhantomData<T>,
    row_iter: Box<dyn Iterator<Item = SqlRow>>,
}

impl<T> Iterator for QueryResults<T>
where
    T: SqlResult,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.row_iter.next().map(T::from_row)
    }
}
