mod models;
mod notes;
mod ui;

use crate::notes::NoteManager;
use crate::ui::{run_app, NotesApp};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let tui_backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(tui_backend)?;

    let note_manager = NoteManager::new().expect("Unable to open notes database");
    let mut app = NotesApp::new(&note_manager)?;

    let result = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    result.expect("Application error");
    Ok(())
}
