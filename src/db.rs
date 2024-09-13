use crate::models::Affirmation;
use rusqlite::{params, Connection, Result};

pub fn initialize(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS affirmations (
            id          INTEGER PRIMARY KEY,
            title       TEXT NOT NULL,
            description TEXT NOT NULL
        )",
        [],
    )?;

    Ok(())
}

pub fn add_affirmation(conn: &Connection, title: &str, description: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO affirmations (title, description) VALUES (?1, ?2)",
        params![title, description],
    )?;

    Ok(())
}

pub fn list_affirmations(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, title, description FROM affirmations")?;
    let affirmation_iter = stmt.query_map([], |row| {
        Ok(Affirmation {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
        })
    })?;

    println!("ID | Title | Description");
    for affirmation in affirmation_iter {
        let affirmation = affirmation?;
        println!(
            "{} | {} | {}",
            affirmation.id, affirmation.title, affirmation.description
        );
    }

    Ok(())
}

pub fn delete_affirmation(conn: &Connection, id: i32) -> Result<()> {
    let affected_rows = conn.execute("DELETE FROM affirmations WHERE id = ?1", params![id])?;

    if affected_rows == 0 {
        println!("No record found with ID {}", id);
    } else {
        println!("Affirmation with ID {} deleted successfully.", id);
    }

    Ok(())
}

pub fn clear_affirmations(conn: &Connection) -> Result<()> {
    conn.execute("DELETE FROM affirmations", params![])?;

    Ok(())
}
