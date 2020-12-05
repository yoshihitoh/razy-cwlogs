use async_trait::async_trait;

use crate::app::handler::{HandleAction, HandleKey};
use crate::app::shell::state::ShellSelection;
use crate::app::{App, AppFocus};
use crate::aws::cwlogs::group::{CwlGroup, CwlGroupCursor};
use crate::aws::profile::ProfileName;
use crate::event::{Action, Key};
use crate::preset::{Preset, PresetName};
use crate::query::{query_from, Query};

pub struct ShellHandler;

#[async_trait]
impl HandleKey for ShellHandler {
    async fn handle_key(&self, app: &mut App, key: Key) -> anyhow::Result<()> {
        match key {
            k if k == app.config.quit_key => on_quit(app).await?,
            Key::Up | Key::Char('k') => app.widget_states.shell.select_previous(&app.data),
            Key::Down | Key::Char('j') => app.widget_states.shell.select_next(&app.data),
            Key::Enter => on_enter(app).await?,
            Key::Esc => app.widget_states.shell.clear_focus(),
            Key::Char('/') => change_focus(app, AppFocus::Header),
            _ => (),
        }

        Ok(())
    }
}

async fn on_quit(app: &mut App) -> anyhow::Result<()> {
    app.shared_state().lock().await.stop_running();
    Ok(())
}

async fn on_enter(app: &mut App) -> anyhow::Result<()> {
    if app.widget_states.shell.has_focus() {
        let action = app.widget_states.shell.execute_item(&app.data);
        if let Some(action) = action {
            app.dispatch_action(action).await;
        }
    } else {
        app.widget_states.shell.set_focus();
    }

    Ok(())
}

fn change_focus(app: &mut App, focus: AppFocus) {
    app.focus = focus;
}

#[async_trait]
impl HandleAction for ShellHandler {
    async fn handle_action(&self, app: &mut App, action: Action) -> anyhow::Result<()> {
        match action {
            Action::Search(s) => on_query(app, query_from(s)).await,
            Action::RequestLogGroups(profile, preset) => {
                on_request_log_groups(app, profile, preset).await
            }
            Action::ReceiveLogGroups(groups) => on_receive_log_groups(app, groups).await,
            Action::Error(msg) => on_error(app, msg).await,
        }
    }
}

async fn on_query(app: &mut App, q: Option<Query>) -> anyhow::Result<()> {
    if let Some(selection) = app.widget_states.shell.selection {
        match selection {
            ShellSelection::Presets => app.data.presets.set_query(q),
            ShellSelection::Profiles => app.data.profiles.set_query(q),
            ShellSelection::Groups => {
                let profile = app.widget_states.shell.selected_profile(&app.data).cloned();
                if let Some(profile) = profile {
                    let preset = Preset::new(
                        PresetName::from("anonymous"),
                        q.map(|q| q.word().to_string()),
                    );
                    app.dispatch_action(Action::RequestLogGroups(profile, Some(preset)))
                        .await;
                }
            }
        }
    }

    Ok(())
}

async fn on_request_log_groups(
    app: &mut App,
    profile_name: ProfileName,
    preset: Option<Preset>,
) -> anyhow::Result<()> {
    app.data.debug_log(format!(
        "create new cursor with profile:{:?}, preset:{:?}",
        profile_name, preset
    ));

    let preset = preset.unwrap_or_else(Preset::default);
    let mut cursor = app.create_groups_cursor(profile_name, preset)?;
    app.data.set_groups_cursor(cursor.clone());

    let mut sender = app.action_sender();
    tokio::spawn(async move {
        let r = cursor
            .next()
            .await
            .map(|groups| groups.unwrap_or_else(Vec::new));

        let action = match r {
            Ok(groups) => Action::ReceiveLogGroups(groups),
            Err(e) => Action::Error(format!("{}", e)),
        };
        sender
            .send(action)
            .await
            .expect("could not send a log groups response")
    });

    Ok(())
}

async fn on_receive_log_groups(app: &mut App, groups: Vec<CwlGroup>) -> anyhow::Result<()> {
    app.data
        .debug_log(format!("receive {} log groups", groups.len()));

    app.data.groups.clear();
    app.data.groups.extend(groups.into_iter());
    Ok(())
}

async fn on_error(app: &mut App, msg: String) -> anyhow::Result<()> {
    app.data.debug_log(msg);
    Ok(())
}
