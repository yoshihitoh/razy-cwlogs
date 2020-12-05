use std::convert::TryFrom;

use once_cell::sync::Lazy;
use tui::buffer::Buffer;
use tui::layout::{Constraint, Rect};
use tui::widgets::{Row, Table, TableState};

use crate::aws::cwlogs::group::{CwlGroup, CwlGroupStore};
use crate::size::HumanReadableSize;
use crate::ui::theme::WidgetStyle;
use crate::ui::widget::helper::default_block_with_style;
use crate::ui::widget::stateful::table::TableStateMut;
use crate::ui::widget::{render_stateful_widget, CustomWidget};

struct Column {
    name: &'static str,
    width: Constraint,
}

static COLUMNS: Lazy<Vec<Column>> = Lazy::new(|| {
    vec![
        Column {
            name: "Name",
            width: Constraint::Percentage(60),
        },
        Column {
            name: "Created at( Local)",
            width: Constraint::Length(20),
        },
        Column {
            name: "Stored Size",
            width: Constraint::Length(10),
        },
    ]
});

static COLUMN_WIDTH: Lazy<Vec<Constraint>> =
    Lazy::new(|| COLUMNS.iter().map(|c| c.width).collect());

#[derive(Debug, Clone, Default)]
pub struct GroupsStates {
    table: TableState,
}

impl TableStateMut for GroupsStates {
    fn table_state_mut(&mut self) -> &mut TableState {
        &mut self.table
    }
}

pub struct GroupsWidget {
    style: WidgetStyle,
}

impl GroupsWidget {
    pub fn with_style(style: WidgetStyle) -> Self {
        GroupsWidget { style }
    }
}

impl CustomWidget for GroupsWidget {
    type Data = CwlGroupStore;
    type State = GroupsStates;

    fn render_app_widget(
        self,
        area: Rect,
        buf: &mut Buffer,
        data: &Self::Data,
        state: &mut Self::State,
    ) {
        let table = Table::new(
            COLUMNS.iter().map(|c| c.name),
            data.order_by_name_asc()
                .map(|g| Row::Data(CwlGroupFormatter::new(g).format().into_iter())),
        )
        .block(default_block_with_style(&self.style.block, "Groups"))
        .header_style(self.style.table.header)
        .widths(&COLUMN_WIDTH)
        .style(self.style.table.normal)
        .highlight_style(self.style.table.highlight)
        .column_spacing(1);

        render_stateful_widget(table, area, buf, &mut state.table);
    }
}

struct CwlGroupFormatter<'a> {
    group: &'a CwlGroup,
}

impl<'a> CwlGroupFormatter<'a> {
    fn new(group: &'a CwlGroup) -> Self {
        CwlGroupFormatter { group }
    }

    fn format(&self) -> Vec<String> {
        let group_name = self.group_name();
        let creation_time = self.creation_time();
        let size = self.stored();
        vec![group_name, creation_time, size]
    }

    fn group_name(&self) -> String {
        self.group.group_name.clone()
    }

    fn creation_time(&self) -> String {
        self.group
            .creation_time_local()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
    }

    fn stored(&self) -> String {
        let size = HumanReadableSize::try_from(self.group.stored)
            .expect("cannot convert to human readable size.");
        format!("{:>4} {}", size.size, size.unit.short_name())
    }
}
