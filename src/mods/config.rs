use crossterm::style::Color;

use super::terminal::CharStyle;

#[derive(Clone, Copy)]
pub struct Config {
    pub styles: Styles,
}

impl Config {
    pub fn new() -> Self {
        Self {
            styles: Styles::basic_theme(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Styles {
    pub num_col: CharStyle,
    pub buffer: CharStyle,
    pub normal_mode: CharStyle,
    pub inser_mode: CharStyle,
    pub filename: CharStyle,
    pub cursor: CharStyle,
    pub cursor_pos: CharStyle,
    pub status_line: CharStyle,
}

impl Styles {
    pub fn basic_theme() -> Self {
        Self {
            num_col: CharStyle::new(Color::Grey, Color::DarkGrey),
            buffer: CharStyle::new(Color::White, Color::Black),
            normal_mode: CharStyle::new(Color::White, Color::Blue),
            inser_mode: CharStyle::new(Color::Black, Color::Yellow),
            filename: CharStyle::new(Color::White, Color::DarkGrey),
            cursor: CharStyle::new(Color::White, Color::DarkGrey),
            cursor_pos: CharStyle::new(Color::White, Color::DarkRed),
            status_line: CharStyle::new(Color::White, Color::Grey),
        }
    }
}
