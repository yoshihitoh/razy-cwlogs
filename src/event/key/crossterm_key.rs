use std::pin::Pin;

use crossterm::event::EventStream;
use crossterm::event::KeyEvent;
use futures::{ready, StreamExt};
use tokio::stream::Stream;

use crate::event::Key;
use std::task::{Context, Poll};

impl From<crossterm::event::KeyEvent> for Key {
    fn from(event: KeyEvent) -> Self {
        use crossterm::event::{KeyCode, KeyModifiers};

        match event.code {
            KeyCode::Enter => Key::Enter,
            KeyCode::Tab => Key::Tab,
            KeyCode::Backspace => Key::BackSpace,
            KeyCode::Esc => Key::Esc,

            KeyCode::Up => Key::Up,
            KeyCode::Down => Key::Down,
            KeyCode::Left => Key::Left,
            KeyCode::Right => Key::Right,

            KeyCode::Char(c) => match event.modifiers {
                KeyModifiers::CONTROL => Key::Ctrl(c),
                KeyModifiers::ALT => Key::Alt(c),
                _ => Key::Char(c),
            },

            _ => Key::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct CrossTermKeyStream {
    inner: EventStream,
}

impl Stream for CrossTermKeyStream {
    type Item = Key;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        while let Some(Ok(event)) = ready!(self.inner.poll_next_unpin(cx)) {
            if let crossterm::event::Event::Key(key_event) = event {
                return Poll::Ready(Some(Key::from(key_event)));
            }
        }

        Poll::Ready(None)
    }
}

pub fn crossterm_key_stream() -> CrossTermKeyStream {
    let inner = EventStream::new();
    CrossTermKeyStream { inner }
}
