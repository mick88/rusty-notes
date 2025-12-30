mod models;
mod notes;
mod ui;

use crate::notes::NoteManager;
use rusqlite::Error;

fn main() -> Result<(), Error> {
    let note_manager = NoteManager::new().expect("Unable to open notes database");

    let notes = note_manager.load_notes()?;
    notes
        .iter()
        .for_each(|x| println!("{}: {}", x.name, x.contents));

    Ok(())
}
