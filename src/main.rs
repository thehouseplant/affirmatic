mod cli;
mod db;
mod models;

use cli::build_cli;
use rusqlite::Connection;

fn main() -> rusqlite::Result<()> {
    let matches = build_cli().get_matches();

    // Connect to the SQLite database
    let conn = Connection::open("affirmations.db")?;

    match matches.subcommand() {
        Some(("add", sub_m)) => {
            let title = sub_m.get_one::<String>("title").expect("Title is required");
            let description = sub_m
                .get_one::<String>("description")
                .expect("Description is required");
            db::add_affirmation(&conn, title, description)?;
            println!("Added {}, {} to the database.", title, description);
        }
        Some(("list", _)) => {
            db::list_affirmations(&conn)?;
        }
        _ => unreachable!(),
    }

    Ok(())
}
