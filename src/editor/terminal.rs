use crossterm::{
    cursor::{Hide, MoveTo, Show},
    queue,
    style::Print,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size},
};
use std::io::{Error, Write, stdout};

#[derive(Copy, Clone)]
pub struct Size {
    pub xs: u16,
    pub ys: u16,
}
#[derive(Copy, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::welcome()?;
        Self::move_cursor_to(Position { x: 0, y: 0 })?;
        Self::execute()?;
        Ok(())
    }

    fn welcome() -> Result<(), Error> {
        let Ok(t_size) = Terminal::size() else {
            panic!("WHYYYYYYYYYYY!")
        };
        let welcome_message = format!("{NAME} editor -- version {VERSION}");
        Self::move_cursor_to(Position {
            x: ((t_size.xs - welcome_message.len() as u16) / 2) + 2,
            y: t_size.ys / 2,
        })?;
        Self::print(&welcome_message)?;
        Ok(())
    }
    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        Self::show_cursor()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
        queue!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }

    //   pub fn clear_line() -> Result<(), Error> {
    //        queue!(stdout(), Clear(ClearType::CurrentLine))?;
    //        Ok(())
    //    }

    pub fn print(s: &str) -> Result<(), Error> {
        queue!(stdout(), Print(s))?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), Error> {
        queue!(stdout(), Show)?;
        Ok(())
    }

    pub fn hide_cursor() -> Result<(), Error> {
        queue!(stdout(), Hide)?;
        Ok(())
    }

    pub fn move_cursor_to(position: Position) -> Result<(), Error> {
        queue!(stdout(), MoveTo(position.x, position.y))?;
        Ok(())
    }

    pub fn size() -> Result<Size, Error> {
        let (xs, ys) = size()?;
        Ok(Size { xs, ys })
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }
}
