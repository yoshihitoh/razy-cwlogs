pub mod debug;
pub mod groups;
mod helper;
pub mod presets;
pub mod profiles;
pub mod search;
pub mod stateful;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::{StatefulWidget, Widget};

pub trait CustomWidget {
    type Data;
    type State;

    fn render_app_widget(
        self,
        area: Rect,
        buf: &mut Buffer,
        data: &Self::Data,
        state: &mut Self::State,
    );
}

pub fn render_widget<W>(widget: W, area: Rect, buf: &mut Buffer)
where
    W: Widget,
{
    widget.render(area, buf)
}

pub fn render_stateful_widget<W, S>(widget: W, area: Rect, buf: &mut Buffer, state: &mut S)
where
    W: StatefulWidget<State = S>,
{
    widget.render(area, buf, state)
}
