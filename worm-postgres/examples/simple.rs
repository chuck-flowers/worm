use std::error::Error;
use worm::executors::SqlExecutor;
use worm::Script;
use worm_postgres::PostgresExecutor;

#[derive(Clone, Script)]
struct InsertUser {
    name: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let script = InsertUser {
        name: "John".into(),
    };

    let db_password = std::env::var("DB_PASSWORD")?;
    let conn_string = format!("host=localhost user=tflowers password={}", db_password);
    let executor = PostgresExecutor::new(&conn_string)?;
    let mut connection = worm::connections::Connection::new(executor);

    println!("Attempting to run: {}", script.clone().compile());
    connection.execute(script)?;

    Ok(())
}
