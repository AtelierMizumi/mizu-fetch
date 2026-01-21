use crossterm::{
    cursor, execute,
    style::{self, Color},
    terminal::{self, ClearType},
};
use std::io::{Write, stdout};

pub mod image;
pub mod logos;

pub struct TermBuffer {
    pub width: u16,
    pub height: u16,
}

impl Default for TermBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl TermBuffer {
    pub fn new() -> Self {
        let (width, height) = terminal::size().unwrap_or((80, 24));
        Self { width, height }
    }

    pub fn clear(&self) {
        let mut out = stdout();
        execute!(out, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
    }

    pub fn draw_text(&self, x: u16, y: u16, text: &str, color: Color) {
        let mut out = stdout();
        execute!(
            out,
            cursor::MoveTo(x, y),
            style::SetForegroundColor(color),
            style::Print(text),
            style::ResetColor
        )
        .unwrap();
        out.flush().unwrap();
    }
}
