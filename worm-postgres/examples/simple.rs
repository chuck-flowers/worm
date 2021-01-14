use std::error::Error;
use worm::executors::SqlExecutor;
use worm::sql::SqlResult;
use worm::sql::SqlRow;
use worm::Script;
use worm_postgres::PostgresExecutor;

#[derive(Debug)]
struct DbAccount {
    handle: String,
    display_name: String,
}

impl SqlResult for DbAccount {
    fn from_row(row: SqlRow) -> Self
    where
        Self: Sized,
    {
        use ::worm::sql::RecordField;
        let mut values = row.into_iter();
        let handle = String::from_sql(values.next().unwrap()).unwrap();
        let display_name = String::from_sql(values.next().unwrap()).unwrap();

        Self {
            handle,
            display_name,
        }
    }
}

#[derive(Clone, Script)]
#[worm(result = "DbAccount")]
struct GetAllAccounts {}

fn main() -> Result<(), Box<dyn Error>> {
    let script = GetAllAccounts {};

    let db_password = std::env::var("DB_PASSWORD")?;
    let db_name = std::env::var("DB_NAME")?;
    let connection_string = format!(
        "postgres://agora_admin:{}@localhost:5432/{}",
        db_password, db_name
    );
    let executor = PostgresExecutor::connect(&connection_string)?;
    let mut connection = worm::connections::Connection::new(executor);

    println!("Attempting to run: {}", script.clone().compile());
    let results = connection.execute(script)?;

    for account_result in results {
        match account_result {
            Ok(account) => println!("{:?}", account),
            Err(err) => eprintln!("{:?}", err),
        }
    }

    Ok(())
}
