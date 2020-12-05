use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::{List, ListItem, ListState};

use crate::aws::profile::{ProfileName, ProfileStore};
use crate::collection::AsStr;
use crate::ui::theme::WidgetStyle;
use crate::ui::widget::helper::default_block_with_style;
use crate::ui::widget::stateful::list::ListStateMut;
use crate::ui::widget::{render_stateful_widget, CustomWidget};

#[derive(Debug, Clone, Default)]
pub struct ProfilesState {
    list: ListState,
}

impl ProfilesState {
    pub fn selected_profile<'a>(&self, data: &'a ProfileStore) -> Option<&'a ProfileName> {
        if let Some(i) = self.list.selected() {
            data.iter().skip(i).next()
        } else {
            None
        }
    }
}

impl ListStateMut for ProfilesState {
    fn list_state_mut(&mut self) -> &mut ListState {
        &mut self.list
    }
}

pub struct ProfilesWidget {
    style: WidgetStyle,
}

impl ProfilesWidget {
    pub fn with_style(style: WidgetStyle) -> Self {
        ProfilesWidget { style }
    }

    fn profiles_from<'a>(&self, data: &'a ProfileStore) -> List<'a> {
        let items = data
            .iter()
            .map(|p| p.as_str())
            .map(|s| ListItem::new(s).style(self.style.item.normal))
            .collect::<Vec<_>>();

        List::new(items)
            .block(default_block_with_style(&self.style.block, data.label()))
            .highlight_style(self.style.item.highlight)
    }
}

impl CustomWidget for ProfilesWidget {
    type Data = ProfileStore;
    type State = ProfilesState;

    fn render_app_widget(
        self,
        area: Rect,
        buf: &mut Buffer,
        data: &Self::Data,
        state: &mut Self::State,
    ) {
        let profiles = self.profiles_from(data);
        render_stateful_widget(profiles, area, buf, &mut state.list);
    }
}
