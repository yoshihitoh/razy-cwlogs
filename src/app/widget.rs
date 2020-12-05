use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::widgets::Widget;

use crate::app::data::AppData;
use crate::app::header::state::HeaderState;
use crate::app::header::widget::HeaderWidgetSet;
use crate::app::shell::state::ShellState;
use crate::app::shell::widget::ShellWidgetSet;
use crate::ui::widget::debug::{DebugState, DebugWidget};
use crate::ui::widget::CustomWidget;

#[derive(Debug, Clone, Default)]
pub struct AppWidgetStates {
    pub debug: DebugState,
    pub shell: ShellState,
    pub header: HeaderState,
}

pub struct AppWidgetSet<'a> {
    pub data: &'a AppData,
    pub states: &'a mut AppWidgetStates,
    pub header: HeaderWidgetSet,
    pub shell: ShellWidgetSet,
    pub debug: Option<DebugWidget>,
}

impl<'a> AppWidgetSet<'a> {
    pub fn new(
        data: &'a AppData,
        states: &'a mut AppWidgetStates,
        header: HeaderWidgetSet,
        shell: ShellWidgetSet,
        debug: Option<DebugWidget>,
    ) -> AppWidgetSet<'a> {
        AppWidgetSet {
            data,
            states,
            header,
            shell,
            debug,
        }
    }
}

impl<'a> Widget for AppWidgetSet<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let constraints: &[Constraint] = if self.data.debug().is_some() {
            &[
                Constraint::Length(3),
                Constraint::Percentage(80),
                Constraint::Length(10),
            ]
        } else {
            &[Constraint::Length(3), Constraint::Percentage(100)]
        };

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(area);

        self.header
            .render_app_widget(chunks[0], buf, &self.data, &mut self.states.header);
        self.shell
            .render_app_widget(chunks[1], buf, self.data, &mut self.states.shell);
        if let Some(debug) = self.debug {
            debug.render_app_widget(
                chunks[2],
                buf,
                self.data.debug().unwrap(),
                &mut self.states.debug,
            );
        }
    }
}
