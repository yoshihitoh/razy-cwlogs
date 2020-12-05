use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};

use crate::app::data::AppData;
use crate::app::shell::state::{ShellSelection, ShellState};
use crate::ui::theme::Theme;
use crate::ui::widget::groups::GroupsWidget;
use crate::ui::widget::presets::PresetsWidget;
use crate::ui::widget::profiles::ProfilesWidget;
use crate::ui::widget::CustomWidget;

pub struct ShellWidgetSet {
    pub presets: PresetsWidget,
    pub profiles: ProfilesWidget,
    pub groups: GroupsWidget,
}

impl ShellWidgetSet {
    pub fn new(theme: Theme, state: &ShellState) -> Self {
        let selected = if state.has_focus() {
            theme.active_widget
        } else {
            theme.selecting_widget
        };
        let normal = theme.normal_widget;

        let (presets, profiles, groups) = match state.selection {
            Some(ShellSelection::Presets) => (selected, normal, normal),
            Some(ShellSelection::Profiles) => (normal, selected, normal),
            Some(ShellSelection::Groups) => (normal, normal, selected),
            None => (normal, normal, normal),
        };

        ShellWidgetSet {
            presets: PresetsWidget::with_style(presets),
            profiles: ProfilesWidget::with_style(profiles),
            groups: GroupsWidget::with_style(groups),
        }
    }
}

impl CustomWidget for ShellWidgetSet {
    type Data = AppData;
    type State = ShellState;

    fn render_app_widget(
        self,
        area: Rect,
        buf: &mut Buffer,
        data: &Self::Data,
        state: &mut Self::State,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
            .split(area);
        let (left, right) = (chunks[0], chunks[1]);

        // left widgets
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(left);
        let (left_top, left_bottom) = (chunks[0], chunks[1]);

        self.presets
            .render_app_widget(left_top, buf, &data.presets, &mut state.presets);
        self.profiles
            .render_app_widget(left_bottom, buf, &data.profiles, &mut state.profiles);

        // right widgets
        self.groups
            .render_app_widget(right, buf, &data.groups, &mut state.groups);
    }
}
