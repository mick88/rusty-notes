#[derive(Debug)]
pub struct Note {
    pub id: Option<i32>,
    pub name: String,
    pub contents: String,
}

impl Note {
    pub fn new(name: String, contents: String) -> Note {
        Note {
            id: None,
            name,
            contents,
        }
    }
}