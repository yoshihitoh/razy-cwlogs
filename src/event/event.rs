use crate::event::{Action, Key};

#[derive(Debug, Clone)]
pub enum Event {
    Tick,
    Input(Key),
    Action(Action),
}
