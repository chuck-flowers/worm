use std::error::Error;
use worm::executors::SqlExecutor;
use worm::sql::SqlResult;
use worm::Script;
use worm_postgres::PostgresExecutor;

#[derive(Debug, SqlResult)]
struct DbAccount {
    handle: String,
    display_name: String,
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
