use postgres::{Client, NoTls, Error as PostgresError};

pub const DB_URL: &str = "postgres://postgres:PASSWORD@localhost:5432/postgres";

pub fn set_database() -> Result<(), PostgresError> {
    let mut client = Client::connect(DB_URL, NoTls)?;
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL
        )
    ",
    )?;
    Ok(())
}
