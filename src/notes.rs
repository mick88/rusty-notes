use crate::models::Note;
use rusqlite::{params, Connection, Error};
use std::path::Path;


pub struct NoteManager {
    connection: Connection,
}

impl NoteManager {
    fn get_database_connection() -> Result<Connection, Error> {
        let connection = Connection::open(Path::new("notes.db"))?;

        connection.execute("CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            contents TEXT NOT NULL
        )", ())?;

        Ok(connection)
    }

    pub fn new() -> Result<NoteManager, Error> {
        Ok(NoteManager {
            connection: NoteManager::get_database_connection()?,
        })
    }

    pub fn load_notes(&self) -> Result<Vec<Note>, Error> {
        let mut stmt = self.connection.prepare("SELECT id, name, contents FROM notes")?;

        let iterator = stmt.query_map([], |row| {
            Ok(Note {
                id: row.get(0)?,
                name: row.get(1)?,
                contents: row.get(2)?,
            })
        })?;

        Ok(Vec::from_iter(iterator.map(|x| x.unwrap())))
    }

    fn create_note(&self, note: &mut Note) -> Result<(), Error> {
        let result = self.connection.execute("INSERT INTO notes (name, contents) values (?1, ?2)", params![&note.name, &note.contents])?;
        note.id = Some(self.connection.last_insert_rowid() as i32);
        Ok(())
    }

    fn update_note(&self, note: &Note) -> Result<(), Error> {
        self.connection.execute("UPDATE notes SET name=?1, contents=?2 WHERE id=?3)", params![note.name, note.contents, note.id.unwrap()])?;
        Ok(())
    }

    pub fn save_note(&self, note: &mut Note) -> Result<(), Error> {
        if note.id == None {
            self.create_note(note)
        } else {
            self.update_note(note)
        }
    }
}
