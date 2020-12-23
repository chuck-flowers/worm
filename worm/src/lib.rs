pub mod errors;
pub mod results;
pub mod sql;

use self::errors::WormError;
use self::results::QueryResults;
use self::sql::SqlResult;

pub trait Script {
    type Output: SqlResult;

    fn execute(self) -> Result<QueryResults<Self::Output>, WormError>;
}
