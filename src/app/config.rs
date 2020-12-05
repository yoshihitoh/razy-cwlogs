use std::time::Duration;

use crate::event::Key;

#[derive(Debug, Copy, Clone)]
pub struct AppConfig {
    pub quit_key: Key,
    pub tick_rate: Duration,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            quit_key: Key::Char('q'),
            tick_rate: Duration::from_millis(100),
        }
    }
}
