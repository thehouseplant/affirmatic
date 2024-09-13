use crate::models::Affirmation;
use rusqlite::{params, Connection, Result};

pub fn add_affirmation(conn: &Connection, title: &str, description: &str) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS affirmations (
            id          INTEGER PRIMARY KEY,
            title       TEXT NOT NULL,
            description TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "INSERT INTO affirmations (title, description) VALUES (?1, ?2)",
        params![title, description],
    )?;

    Ok(())
}
