use async_trait::async_trait;

use crate::app::header::handler::HeaderHandler;
use crate::app::shell::handler::ShellHandler;
use crate::app::{App, AppFocus};
use crate::event::{Action, Key};

#[async_trait]
pub trait HandleKey {
    async fn handle_key(&self, app: &mut App, key: Key) -> anyhow::Result<()>;
}

pub async fn handle_key_input(app: &mut App, key: Key) -> anyhow::Result<()> {
    app.data.debug_key(key);

    let focus = app.focus;
    match focus {
        AppFocus::Shell => ShellHandler.handle_key(app, key).await,
        AppFocus::Header => HeaderHandler.handle_key(app, key).await,
        AppFocus::Session(_session_id) => Ok(()),
    }
}

#[async_trait]
pub trait HandleAction {
    async fn handle_action(&self, app: &mut App, action: Action) -> anyhow::Result<()>;
}

pub async fn handle_action(app: &mut App, action: Action) -> anyhow::Result<()> {
    let focus = app.focus;
    match focus {
        AppFocus::Shell => ShellHandler.handle_action(app, action).await,
        AppFocus::Header => Ok(()),
        AppFocus::Session(_session_id) => Ok(()),
    }
}
