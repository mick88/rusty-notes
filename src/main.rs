mod database;
mod models;

use crate::database::{get_database_connection, load_notes, save_note};
use rusqlite::{Connection, Error};

fn main() -> Result<(), Error> {
    let connection: Connection = get_database_connection()?;

    let notes = load_notes(&connection)?;
    notes.iter().for_each(|x| println!("{}: {}", x.name, x.contents));

    Ok(())
}
