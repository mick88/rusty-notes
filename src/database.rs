use crate::models::Note;
use rusqlite::{params, Connection, Error};
use std::path::Path;

pub fn get_database_connection() -> Result<Connection, Error> {
    let connection = Connection::open(Path::new("notes.db"))?;

    connection.execute("CREATE TABLE IF NOT EXISTS notes (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        contents TEXT NOT NULL
    )", ())?;

    Ok(connection)
}

pub fn load_notes(conn: &Connection) -> Result<Vec<Note>, Error> {
    let mut stmt = conn.prepare("SELECT id, name, contents FROM notes")?;

    let iterator = stmt.query_map([], |row| {
        Ok(Note {
            id: row.get(0)?,
            name: row.get(1)?,
            contents: row.get(2)?,
        })
    })?;

    Ok(Vec::from_iter(iterator.map(|x| x.unwrap())))
}

fn create_note(connection: &Connection, note: &mut Note) -> Result<(), Error> {
    let result = connection.execute("INSERT INTO notes (name, contents) values (?1, ?2)", params![&note.name, &note.contents])?;
    note.id = Some(connection.last_insert_rowid() as i32);
    Ok(())
}

fn update_note(connection: &Connection, note: &Note) -> Result<(), Error> {
    connection.execute("UPDATE notes SET name=?1, contents=?2 WHERE id=?3)", params![note.name, note.contents, note.id.unwrap()])?;
    Ok(())
}

pub fn save_note(conn: &Connection, note: &mut Note) -> Result<(), Error> {
    if note.id == None {
        create_note(conn, note)
    } else {
        update_note(conn, note)
    }
}