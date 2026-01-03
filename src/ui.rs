use std::cmp::{max, min};
use crate::models::Note;
use crate::notes::NoteManager;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState};
use ratatui::backend::Backend;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use ratatui::{Frame, Terminal};
use std::error::Error;
use std::io;
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::{Direction, Line};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;
use tui_textarea::TextArea;

enum CurrentScreen {
    NoteList,
    NoteEditor,
}

pub struct NotesApp<'a, 'b> {
    screen: CurrentScreen,
    state: ListState,
    notes: Vec<Note>,
    exit: bool,
    // Note editor
    editor: TextArea<'a>,
    editing_note: Option<usize>,
    manager: &'b NoteManager,
}

impl<'a, 'b> NotesApp<'a, 'b> {
    pub fn new(manager: &'b NoteManager) -> Result<Self, Box<dyn Error>> {
        let mut app = NotesApp {
            screen: CurrentScreen::NoteList,
            state: ListState::default(),
            notes: manager.load_notes()?,
            exit: false,
            editor: TextArea::default(),
            editing_note: None,
            manager,
        };
        app.jump_list(1);
        Ok(app)
    }

    fn render(&self, frame: &mut Frame) {
        match self.screen {
            CurrentScreen::NoteList => {
                let chunks = Layout::default().direction(Direction::Vertical).constraints([Constraint::Min(0)].as_ref()).split(frame.area());

                let items: Vec<ListItem> = self.notes.iter().map(|note| {
                    let lines = vec![Line::from(Span::raw(&note.name))];
                    ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
                }).collect();

                let list = List::new(items).block(Block::default().borders(Borders::ALL).title("Notes")).highlight_style(
                    Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD),
                ).highlight_symbol("👉 ");

                frame.render_stateful_widget(list, chunks[0], &mut self.state.clone());
            }
            CurrentScreen::NoteEditor => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .split(frame.area());

                frame.render_widget(self.editor.widget(), chunks[0]);
            }
        }
    }

    fn jump_list(&mut self, stride: isize) {
        let size: isize = self.notes.len() as isize;
        if size == 0 {
            return;
        }
        let mut i: isize = match self.state.selected() {
            Some(i) => {
                i as isize + stride
            }
            None => {
                if stride > 0 {
                    0
                } else {
                    size - 1
                }
            }
        };
        i = max(0, i);
        self.state.select(Some(min(i.try_into().unwrap(), size as usize)));
    }

    fn on_event(&mut self, event: Event) {
        match self.screen {
            CurrentScreen::NoteList => {},
            CurrentScreen::NoteEditor => {

                self.editor.input(event);
            }
        }
    }

    fn on_key_press(&mut self, key: KeyEvent) {
        match self.screen {
            CurrentScreen::NoteList => {
                match key.code {
                    KeyCode::Down => {
                        self.jump_list(1);
                    }
                    KeyCode::Up => {
                        self.jump_list(-1);
                    }
                    KeyCode::Enter => {
                        if let Some(i) = self.state.selected() {
                            self.editing_note = Some(i);
                            let note = &self.notes[i];

                            self.editor = TextArea::from(note.contents.lines());
                            self.editor.set_block(Block::default().borders(Borders::ALL).title(note.name.clone()));
                            self.editor.set_style(Style::default().fg(Color::Black).bg(Color::White));

                            self.screen = CurrentScreen::NoteEditor;
                        }
                    }
                    KeyCode::Esc => {
                        self.exit = true;
                    },
                    _ => {},
                }
            }
            CurrentScreen::NoteEditor => {
                match key.code {
                    KeyCode::Esc => {
                        self.save_current_note().expect("Failed to save the note")
                    },
                    _ => {},
                }
            }
        }
    }

    fn save_current_note(&mut self) -> Result<(), rusqlite::Error> {
        let i = self.editing_note.expect("No note selected");
        let note = &mut self.notes[i];
        let editor_lines = self.editor.lines();
        note.name = editor_lines[0].clone();
        note.contents = editor_lines.join("\n");
        self.manager.save_note(note)
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut NotesApp) -> io::Result<()> {
    loop {
        terminal.draw(|frame| app.render(frame)).expect("Failed to render screen");
        let result = event::read()?;
        if let Event::Key(key) = result {
            if key.code != KeyCode::Esc {
                app.on_event(result.clone());
            }
            if key.kind == KeyEventKind::Press {
                app.on_key_press(key);
            }
        }

        if app.exit {
            break;
        }
    }
    Ok(())
}