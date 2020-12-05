use async_trait::async_trait;

use crate::app::handler::HandleKey;
use crate::app::{App, AppFocus};
use crate::event::{Action, Key};

pub struct HeaderHandler;

#[async_trait]
impl HandleKey for HeaderHandler {
    async fn handle_key(&self, app: &mut App, key: Key) -> anyhow::Result<()> {
        match key {
            Key::Char(c) => app.data.search.append_char(c),
            Key::BackSpace => app.data.search.delete_char(),
            Key::Enter => on_enter(app).await,
            Key::Left => app.data.search.move_position(-1),
            Key::Right => app.data.search.move_position(1),
            _ => (),
        }

        Ok(())
    }
}

async fn on_enter(app: &mut App) {
    let action = Action::Search(app.data.search.query());
    app.dispatch_action(action).await;

    app.focus = AppFocus::Shell;
}
