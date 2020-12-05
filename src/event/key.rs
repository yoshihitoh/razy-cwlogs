mod crossterm_key;

use std::fmt;

pub use crossterm_key::{crossterm_key_stream, CrossTermKeyStream};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Key {
    Enter,
    Tab,
    BackSpace,
    Esc,

    Up,
    Down,
    Left,
    Right,

    Char(char),
    Ctrl(char),
    Alt(char),

    Unknown,
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Key::*;
        match *self {
            Char(' ') => write!(f, "<Space>"),
            Ctrl(' ') => write!(f, "<Ctrl+Space>"),
            Alt(' ') => write!(f, "<Alt+Space>"),

            Char(c) => write!(f, "{}", c),
            Ctrl(c) => write!(f, "<Ctrl+{}>", c),
            Alt(c) => write!(f, "<Alt+{}>", c),

            Unknown => write!(f, "{:?}", self),
            _ => write!(f, "<{:?}>", self),
        }
    }
}
