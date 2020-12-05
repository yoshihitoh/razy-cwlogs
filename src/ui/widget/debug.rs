use std::collections::VecDeque;

use chrono::{DateTime, Local};
use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::widgets::{List, ListItem};

use crate::event::Key;
use crate::ui::theme::WidgetStyle;
use crate::ui::widget::helper::{apply_item_style, default_block_with_style};
use crate::ui::widget::{render_widget, CustomWidget};

fn append_item<T>(dest: &mut VecDeque<T>, item: T) {
    if dest.len() == dest.capacity() {
        dest.pop_front();
    }
    dest.push_back(item);
}

pub struct DebugData {
    keys: VecDeque<(u64, Key)>,
    key_no: u64,
    logs: VecDeque<(DateTime<Local>, String)>,
}

impl DebugData {
    pub fn new(max_size: usize) -> DebugData {
        DebugData {
            keys: VecDeque::with_capacity(max_size),
            key_no: 0,
            logs: VecDeque::with_capacity(max_size),
        }
    }

    pub fn keys(&self) -> &VecDeque<(u64, Key)> {
        &self.keys
    }

    pub fn append_key(&mut self, key: Key) {
        self.key_no += 1;
        append_item(&mut self.keys, (self.key_no, key));
    }

    pub fn logs(&self) -> &VecDeque<(DateTime<Local>, String)> {
        &self.logs
    }

    pub fn append_log(&mut self, msg: String) {
        append_item(&mut self.logs, (Local::now(), msg));
    }
}

impl Default for DebugData {
    fn default() -> Self {
        DebugData::new(30)
    }
}

#[derive(Debug, Clone, Default)]
pub struct DebugState {}

#[derive(Debug, Copy, Clone)]
pub struct DebugWidget {
    style: WidgetStyle,
}

impl DebugWidget {
    pub fn with_style(style: WidgetStyle) -> Self {
        DebugWidget { style }
    }

    fn logs_from(&self, debug_data: &DebugData) -> List {
        let items = debug_data
            .logs()
            .iter()
            .rev()
            .map(|(dt, msg)| format!("{} {}", dt.time().format("%H:%M:%S"), msg))
            .map(ListItem::new)
            .map(|l| apply_item_style(l, &self.style.item))
            .collect::<Vec<_>>();

        List::new(items).block(default_block_with_style(&self.style.block, "Messages"))
    }

    fn keys_from(&self, debug_data: &DebugData) -> List {
        let items = debug_data
            .keys()
            .iter()
            .rev()
            .map(|(no, k)| format!("({:4}) {}", no, k))
            .map(|s| ListItem::new(s))
            .map(|l| apply_item_style(l, &self.style.item))
            .collect::<Vec<_>>();

        List::new(items).block(default_block_with_style(&self.style.block, "Keys"))
    }
}

impl CustomWidget for DebugWidget {
    type Data = DebugData;
    type State = DebugState;

    fn render_app_widget(
        self,
        area: Rect,
        buf: &mut Buffer,
        data: &Self::Data,
        _states: &mut Self::State,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(area);

        render_widget(self.logs_from(data), chunks[0], buf);
        render_widget(self.keys_from(data), chunks[1], buf);
    }
}
