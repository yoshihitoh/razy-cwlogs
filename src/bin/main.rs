use std::error::Error as StdError;
use std::panic;
use std::panic::PanicInfo;
use std::sync::Arc;
use std::time::Duration;

use futures::StreamExt;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Mutex;

use razy_cwlogs::app::config::AppConfig;
use razy_cwlogs::app::data::AppData;
use razy_cwlogs::app::handler::{handle_action, handle_key_input};
use razy_cwlogs::app::state::AppSharedState;
use razy_cwlogs::app::App;
use razy_cwlogs::event::{crossterm_key_stream, tick_stream, Action, Event};
use razy_cwlogs::terminal::CrossTermTerminal;
use razy_cwlogs::ui::widget::debug::DebugData;

const MAX_ACTIONS: usize = 100;
const MAX_EVENTS: usize = 100;

fn panic_hook(info: &PanicInfo<'_>) {
    if cfg!(debug_assertions) {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>",
            },
        };

        dbg!(msg, location);
        std::thread::sleep(Duration::from_secs(3));
    }
}

#[cfg(debug_assertions)]
fn app_data() -> AppData {
    AppData::with_debug(DebugData::default())
}

#[cfg(not(debug_assertions))]
fn app_data() -> AppData {
    AppData::default()
}

fn app(action_sender: Sender<Action>) -> App {
    let config = AppConfig::default();
    let data = app_data();
    App::new(config, data, action_sender)
}

fn terminal() -> anyhow::Result<CrossTermTerminal> {
    Ok(CrossTermTerminal::new()?)
}

fn app_is_running(state: &Arc<Mutex<AppSharedState>>) -> bool {
    if let Ok(st) = state.try_lock() {
        st.is_running()
    } else {
        true
    }
}

async fn enable_tick_events(
    mut sender: Sender<Event>,
    duration: Duration,
    state: Arc<Mutex<AppSharedState>>,
) {
    let mut ticker = tick_stream(duration);
    loop {
        if !app_is_running(&state) {
            break;
        }

        if let Some(_) = ticker.next().await {
            sender
                .send(Event::Tick)
                .await
                .expect("could not send a tick event.");
        }
    }
}

async fn enable_key_events(mut sender: Sender<Event>, state: Arc<Mutex<AppSharedState>>) {
    let mut keys = crossterm_key_stream();

    loop {
        if !app_is_running(&state) {
            break;
        }

        if let Some(key) = keys.next().await {
            sender
                .send(Event::Input(key))
                .await
                .expect("could not send a input event.");
        }
    }
}

async fn enable_action_events(
    mut sender: Sender<Event>,
    state: Arc<Mutex<AppSharedState>>,
    mut action_receiver: Receiver<Action>,
) {
    loop {
        if !app_is_running(&state) {
            break;
        }

        if let Some(action) = action_receiver.next().await {
            sender
                .send(Event::Action(action))
                .await
                .expect("could not send a action event");
        }
    }
}

fn event_receiver(app: &App, action_receiver: Receiver<Action>) -> Receiver<Event> {
    let (sender, receiver) = channel(MAX_EVENTS);
    tokio::spawn(enable_tick_events(
        sender.clone(),
        app.config.tick_rate,
        app.shared_state(),
    ));
    tokio::spawn(enable_key_events(sender.clone(), app.shared_state()));
    tokio::spawn(enable_action_events(
        sender.clone(),
        app.shared_state(),
        action_receiver,
    ));

    receiver
}

async fn run(
    mut app: App,
    mut terminal: CrossTermTerminal,
    mut receiver: Receiver<Event>,
) -> anyhow::Result<()> {
    terminal.clear()?;
    terminal.draw(|frame| frame.render_widget(app.widgets(), frame.size()))?;

    loop {
        if !app_is_running(&app.shared_state()) {
            break;
        }

        if let Some(event) = receiver.next().await {
            match event {
                Event::Input(key) => {
                    handle_key_input(&mut app, key).await?;
                }
                Event::Tick => {
                    terminal.draw(|frame| frame.render_widget(app.widgets(), frame.size()))?;
                }
                Event::Action(action) => {
                    handle_action(&mut app, action).await?;
                }
            }
        } else {
            break;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    panic::set_hook(Box::new(|info| {
        panic_hook(info);
    }));

    let (action_sender, action_receiver) = channel(MAX_ACTIONS);
    let app = app(action_sender);
    app.state().lock().await.start_running();

    let terminal = terminal()?;
    let receiver = event_receiver(&app, action_receiver);
    run(app, terminal, receiver).await?;
    Ok(())
}
