use crossterm::event::{Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers, read};

mod terminal;
use terminal::Terminal;

use crate::editor::terminal::Position;
pub struct Editor {
    should_quit: bool,
    cursor_location: Position,
}

enum CursorDirecton {
    Left,
    Right,
    Up,
    Down,
}

impl Editor {
    pub const fn default() -> Self {
        Self {
            should_quit: false,
            cursor_location: Position { x: 10, y: 10 },
        }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn draw_raws() -> Result<(), std::io::Error> {
        let Ok(t_size) = Terminal::size() else {
            panic!("WHYYYYYYYYYYY!")
        };

        for i in 0..t_size.ys {
            Terminal::move_cursor_to(Position { x: 0, y: i })?;
            Terminal::print("~")?
        }
        Ok(())
    }

    pub fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn handle_cursor_location(&mut self, direction: CursorDirecton) {
        let Ok(size) = Terminal::size() else {
            panic!("WHYYYYYYYYYYY!")
        };
        match direction {
            CursorDirecton::Right => {
                if self.cursor_location.x < size.xs {
                    self.cursor_location.x += 1;
                }
            }
            CursorDirecton::Left => {
                if self.cursor_location.x != 0 {
                    self.cursor_location.x -= 1;
                }
            }
            CursorDirecton::Down => {
                if self.cursor_location.y < size.ys {
                    self.cursor_location.y += 1;
                }
            }
            CursorDirecton::Up => {
                if self.cursor_location.y == 0 {
                    self.cursor_location.y -= 1;
                }
            }
        }
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                Char('h') => {
                    self.handle_cursor_location(CursorDirecton::Left);
                }
                Char('j') => {
                    self.handle_cursor_location(CursorDirecton::Down);
                }
                Char('k') => {
                    self.handle_cursor_location(CursorDirecton::Up);
                }
                Char('l') => {
                    self.handle_cursor_location(CursorDirecton::Right);
                }
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
        } else {
            Self::draw_raws()?;
            Terminal::move_cursor_to(self.cursor_location)?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }
}
