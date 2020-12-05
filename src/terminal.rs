use std::io::{self, stdout, Stdout, Write};
use std::ops::{Deref, DerefMut};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use thiserror::Error;
use tui::backend::CrosstermBackend;
use tui::Terminal;

#[derive(Debug, Error)]
pub enum CrossTermError {
    #[error("io error")]
    Io(#[from] io::Error),

    #[error("crossterm error")]
    CrossTerm(#[from] crossterm::ErrorKind),
}

pub struct CrossTermTerminal {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl CrossTermTerminal {
    pub fn new() -> Result<CrossTermTerminal, CrossTermError> {
        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::new(backend)?;
        let mut term = CrossTermTerminal { terminal };

        term.initialize()?;
        Ok(term)
    }

    fn initialize(&mut self) -> Result<(), CrossTermError> {
        enable_raw_mode()?;
        execute!(self.terminal.backend_mut(), EnterAlternateScreen)?;
        Ok(())
    }

    fn terminate(&mut self) -> Result<(), CrossTermError> {
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}

impl Drop for CrossTermTerminal {
    fn drop(&mut self) {
        self.terminate().expect("could not terminate");
    }
}

impl Deref for CrossTermTerminal {
    type Target = Terminal<CrosstermBackend<Stdout>>;

    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}

impl DerefMut for CrossTermTerminal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terminal
    }
}
