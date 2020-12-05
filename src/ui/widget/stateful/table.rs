use tui::widgets::TableState;

use crate::collection::Length;
use crate::ui::widget::stateful::{select_next, select_previous, unselect, SelectState};

impl SelectState for TableState {
    fn selected(&self) -> Option<usize> {
        TableState::selected(self)
    }

    fn select(&mut self, selected: Option<usize>) {
        TableState::select(self, selected)
    }
}

pub trait TableStateMut {
    fn table_state_mut(&mut self) -> &mut TableState;
}

pub struct StatefulTable<'a, D: Length> {
    state: &'a mut TableState,
    data: &'a D,
}

impl<'a, D: Length> StatefulTable<'a, D> {
    pub fn new<S: TableStateMut>(state: &'a mut S, data: &'a D) -> Self {
        StatefulTable {
            state: state.table_state_mut(),
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
