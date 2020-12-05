use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::app::data::AppData;
use crate::aws::profile::ProfileName;
use crate::event::Action;
use crate::preset::Preset;
use crate::ui::widget::groups::GroupsStates;
use crate::ui::widget::presets::PresetsState;
use crate::ui::widget::profiles::ProfilesState;
use crate::ui::widget::stateful::list::StatefulList;
use crate::ui::widget::stateful::table::StatefulTable;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ShellSelection {
    Presets,
    Profiles,
    Groups,
}

#[derive(Debug, Copy, Clone)]
struct WidgetOrder {
    prev: ShellSelection,
    next: ShellSelection,
}

impl WidgetOrder {
    fn new(prev: ShellSelection, next: ShellSelection) -> Self {
        WidgetOrder { prev, next }
    }
}

impl Default for WidgetOrder {
    fn default() -> Self {
        WidgetOrder::new(ShellSelection::Groups, ShellSelection::Presets)
    }
}

static SHELL_WIDGET_ORDER: Lazy<HashMap<ShellSelection, WidgetOrder>> = Lazy::new(|| {
    use ShellSelection::*;

    let mut h = HashMap::new();
    h.insert(Presets, WidgetOrder::new(Groups, Profiles));
    h.insert(Profiles, WidgetOrder::new(Presets, Groups));
    h.insert(Groups, WidgetOrder::new(Profiles, Presets));

    h
});

impl Default for ShellSelection {
    fn default() -> Self {
        ShellSelection::Presets
    }
}

#[derive(Debug, Clone, Default)]
pub struct ShellState {
    pub selection: Option<ShellSelection>,
    focus: bool,
    pub presets: PresetsState,
    pub profiles: ProfilesState,
    pub groups: GroupsStates,
    pub query: String,
}

impl ShellState {
    pub fn has_focus(&self) -> bool {
        self.focus
    }

    pub fn set_focus(&mut self) {
        if let Some(_) = self.selection {
            self.focus = true;
        }
    }

    pub fn clear_focus(&mut self) {
        self.focus = false;
    }

    pub fn select_next(&mut self, data: &AppData) {
        if self.focus {
            self.next_item(data);
        } else {
            self.next_widget();
        }
    }

    pub fn select_previous(&mut self, data: &AppData) {
        if self.focus {
            self.previous_item(data);
        } else {
            self.previous_widget();
        }
    }

    pub fn execute_item(&mut self, data: &AppData) -> Option<Action> {
        match self.selection {
            Some(ShellSelection::Presets) => self.load_log_groups_action(data),
            Some(ShellSelection::Profiles) => self.load_log_groups_action(data),
            Some(ShellSelection::Groups) => self.load_log_streams_action(data),
            None => None,
        }
    }

    pub fn selected_profile<'a>(&self, data: &'a AppData) -> Option<&'a ProfileName> {
        self.profiles.selected_profile(&data.profiles)
    }

    pub fn selected_preset<'a>(&self, data: &'a AppData) -> Option<&'a Preset> {
        self.presets.selected_preset(&data.presets)
    }

    fn widget_order(&self) -> WidgetOrder {
        self.selection
            .and_then(|s| SHELL_WIDGET_ORDER.get(&s))
            .copied()
            .unwrap_or_else(WidgetOrder::default)
    }

    fn next_widget(&mut self) {
        let widget_order = self.widget_order();
        self.selection = Some(widget_order.next);
    }

    fn previous_widget(&mut self) {
        let widget_order = self.widget_order();
        self.selection = Some(widget_order.prev);
    }

    fn next_item(&mut self, data: &AppData) {
        match self.selection {
            Some(ShellSelection::Presets) => {
                StatefulList::new(&mut self.presets, &data.presets).select_next();
            }
            Some(ShellSelection::Profiles) => {
                StatefulList::new(&mut self.profiles, &data.profiles).select_next();
            }
            Some(ShellSelection::Groups) => {
                StatefulTable::new(&mut self.groups, &data.groups).select_next();
            }
            None => (),
        }
    }

    fn previous_item(&mut self, data: &AppData) {
        match self.selection {
            Some(ShellSelection::Presets) => {
                StatefulList::new(&mut self.presets, &data.presets).select_previous();
            }
            Some(ShellSelection::Profiles) => {
                StatefulList::new(&mut self.profiles, &data.profiles).select_previous();
            }
            Some(ShellSelection::Groups) => {
                StatefulTable::new(&mut self.groups, &data.groups).select_previous();
            }
            None => (),
        }
    }

    fn load_log_groups_action(&mut self, data: &AppData) -> Option<Action> {
        if let Some(profile) = Some(self.focus).and_then(|_| self.selected_profile(&data)) {
            self.selection = Some(ShellSelection::Groups);
            let selected_preset = Some(self.focus).and_then(|_| self.selected_preset(&data));
            Some(Action::RequestLogGroups(
                profile.clone(),
                selected_preset.cloned(),
            ))
        } else {
            None
        }
    }

    fn load_log_streams_action(&mut self, data: &AppData) -> Option<Action> {
        None
    }
}
