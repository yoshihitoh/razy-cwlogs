use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::{List, ListItem, ListState};

use crate::collection::AsStr;
use crate::preset::{Preset, PresetStore};
use crate::ui::theme::WidgetStyle;
use crate::ui::widget::helper::{apply_item_style, default_block_with_style};
use crate::ui::widget::stateful::list::ListStateMut;
use crate::ui::widget::{render_stateful_widget, CustomWidget};

#[derive(Debug, Clone, Default)]
pub struct PresetsState {
    list: ListState,
}

impl PresetsState {
    pub fn selected_preset<'a>(&self, data: &'a PresetStore) -> Option<&'a Preset> {
        if let Some(i) = self.list.selected() {
            data.iter().skip(i).next()
        } else {
            None
        }
    }
}

impl ListStateMut for PresetsState {
    fn list_state_mut(&mut self) -> &mut ListState {
        &mut self.list
    }
}

pub struct PresetsWidget {
    style: WidgetStyle,
}

impl PresetsWidget {
    pub fn with_style(style: WidgetStyle) -> Self {
        PresetsWidget { style }
    }

    fn presets_from<'a>(&self, data: &'a PresetStore) -> List<'a> {
        let items = data
            .iter()
            .map(|p| p.name.as_str())
            .map(ListItem::new)
            .map(|i| apply_item_style(i, &self.style.item))
            .collect::<Vec<_>>();

        List::new(items)
            .block(default_block_with_style(&self.style.block, data.label()))
            .highlight_style(self.style.item.highlight)
    }
}

impl CustomWidget for PresetsWidget {
    type Data = PresetStore;
    type State = PresetsState;

    fn render_app_widget(
        self,
        area: Rect,
        buf: &mut Buffer,
        data: &Self::Data,
        state: &mut Self::State,
    ) {
        let presets = self.presets_from(data);
        render_stateful_widget(presets, area, buf, &mut state.list);
    }
}
