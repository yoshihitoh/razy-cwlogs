pub mod list;
pub mod table;

pub trait SelectState {
    fn selected(&self) -> Option<usize>;
    fn select(&mut self, selected: Option<usize>);
}

pub fn select_next<S: SelectState>(state: &mut S, length: usize) {
    select(state, length, 1);
}

pub fn select_previous<S: SelectState>(state: &mut S, length: usize) {
    select(state, length, -1);
}

pub fn unselect<S: SelectState>(state: &mut S) {
    state.select(None);
}

fn select<S: SelectState>(state: &mut S, length: usize, direction: i32) {
    let length = length as i32;
    let selected = if length > 0 {
        let next_index = state.selected().map(|i| i as i32 + direction).unwrap_or(0);
        let selected = if next_index < 0 {
            length + next_index
        } else if next_index >= length {
            next_index % length
        } else {
            next_index
        };
        Some(selected as usize)
    } else {
        None
    };

    state.select(selected);
}
