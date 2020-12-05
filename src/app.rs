pub mod config;
pub mod data;
pub mod handler;
pub mod header;
pub mod shell;
pub mod state;
pub mod widget;

use std::sync::Arc;

use thiserror::Error;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;

use crate::app::config::AppConfig;
use crate::app::data::AppData;
use crate::app::header::widget::HeaderWidgetSet;
use crate::app::shell::widget::ShellWidgetSet;
use crate::app::state::AppSharedState;
use crate::app::widget::{AppWidgetSet, AppWidgetStates};
use crate::aws::cwlogs::client::{ClientFactory, ClientFactoryError};
use crate::aws::cwlogs::group::RusotoCwlGroupCursor;
use crate::aws::profile::ProfileName;
use crate::event::Action;
use crate::preset::Preset;
use crate::session::SessionId;
use crate::ui::theme::Theme;
use crate::ui::widget::debug::DebugWidget;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("client factory error")]
    ClientFactory(#[from] ClientFactoryError),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AppFocus {
    Shell,
    Header,
    Session(SessionId),
}

impl Default for AppFocus {
    fn default() -> Self {
        AppFocus::Shell
    }
}

pub struct App {
    pub config: AppConfig,
    pub data: AppData,
    pub widget_states: AppWidgetStates,
    pub theme: Theme,
    pub focus: AppFocus,
    action_sender: Sender<Action>,
    shared_state: Arc<Mutex<AppSharedState>>,
    client_factory: ClientFactory,
}

impl App {
    pub fn new(config: AppConfig, data: AppData, action_sender: Sender<Action>) -> App {
        App {
            config,
            data,
            widget_states: AppWidgetStates::default(),
            theme: Theme::default(),
            focus: AppFocus::default(),
            action_sender,
            shared_state: Arc::new(Mutex::new(AppSharedState::new())),
            client_factory: ClientFactory,
        }
    }

    pub fn create_groups_cursor(
        &self,
        profile_name: ProfileName,
        preset: Preset,
    ) -> Result<RusotoCwlGroupCursor, AppError> {
        let client = self.client_factory.new_client(profile_name)?;
        let cursor = RusotoCwlGroupCursor::new(client, preset.into());
        Ok(cursor)
    }

    pub fn state(&self) -> &Mutex<AppSharedState> {
        &self.shared_state
    }

    pub fn shared_state(&self) -> Arc<Mutex<AppSharedState>> {
        Arc::clone(&self.shared_state)
    }

    pub fn widgets(&mut self) -> AppWidgetSet {
        let header = HeaderWidgetSet::new(self.theme, self.focus);
        let shell = ShellWidgetSet::new(self.theme, &self.widget_states.shell);
        let debug = self
            .data
            .debug()
            .map(|_| DebugWidget::with_style(self.theme.debug_widget));

        AppWidgetSet {
            data: &self.data,
            states: &mut self.widget_states,
            header,
            shell,
            debug,
        }
    }

    pub fn action_sender(&self) -> Sender<Action> {
        self.action_sender.clone()
    }

    pub async fn dispatch_action(&mut self, action: Action) {
        self.action_sender
            .send(action)
            .await
            .expect("could not send an action");
    }
}
