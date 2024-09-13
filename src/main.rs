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
        Some(("init", _)) => {
            db::initialize(&conn)?;
            println!("New affirmations database initialized");
        }
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
        Some(("delete", sub_m)) => {
            let id = sub_m
                .get_one::<String>("id")
                .expect("ID is required")
                .parse::<i32>()
                .expect("ID must be an integer");
            db::delete_affirmation(&conn, id)?;
            println!("Deleted affirmation with ID {}.", id);
        }
        Some(("clear", _)) => {
            db::clear_affirmations(&conn)?;
        }
        _ => unreachable!(),
    }

    Ok(())
}
