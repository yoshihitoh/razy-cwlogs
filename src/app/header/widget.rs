use crate::app::data::AppData;
use crate::app::header::state::HeaderState;
use crate::app::AppFocus;
use crate::ui::theme::Theme;
use crate::ui::widget::search::SearchWidget;
use crate::ui::widget::CustomWidget;
use tui::buffer::Buffer;
use tui::layout::Rect;

pub struct HeaderWidgetSet {
    pub search: SearchWidget,
}

impl HeaderWidgetSet {
    pub fn new(theme: Theme, app_focus: AppFocus) -> Self {
        let style = if app_focus == AppFocus::Header {
            theme.active_widget
        } else {
            theme.normal_widget
        };

        HeaderWidgetSet {
            search: SearchWidget::with_style(style),
        }
    }
}

impl CustomWidget for HeaderWidgetSet {
    type Data = AppData;
    type State = HeaderState;

    fn render_app_widget(
        self,
        area: Rect,
        buf: &mut Buffer,
        data: &Self::Data,
        state: &mut Self::State,
    ) {
        self.search
            .render_app_widget(area, buf, &data.search, &mut state.search);
    }
}
