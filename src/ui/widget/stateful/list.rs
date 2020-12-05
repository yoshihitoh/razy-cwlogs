use tui::widgets::ListState;

use crate::collection::Length;
use crate::ui::widget::stateful::{select_next, select_previous, unselect, SelectState};

impl SelectState for ListState {
    fn selected(&self) -> Option<usize> {
        ListState::selected(self)
    }

    fn select(&mut self, selected: Option<usize>) {
        ListState::select(self, selected)
    }
}

pub trait ListStateMut {
    fn list_state_mut(&mut self) -> &mut ListState;
}

pub struct StatefulList<'a, D: Length> {
    state: &'a mut ListState,
    data: &'a D,
}

impl<'a, D: Length> StatefulList<'a, D> {
    pub fn new<S: ListStateMut>(state: &'a mut S, data: &'a D) -> Self {
        StatefulList {
            state: state.list_state_mut(),
            data,
        }
    }

    pub fn select_next(self) {
        select_next(self.state, self.data.len());
    }

    pub fn select_previous(self) {
        select_previous(self.state, self.data.len());
    }

    pub fn unselect(self) {
        unselect(self.state);
    }
}
