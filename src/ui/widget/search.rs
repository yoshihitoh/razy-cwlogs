use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Paragraph;

use crate::ui::theme::WidgetStyle;
use crate::ui::widget::helper::default_block_with_style;
use crate::ui::widget::{render_widget, CustomWidget};

fn clamp<T: Ord>(x: T, min: T, max: T) -> T {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

#[derive(Debug, Clone, Default)]
pub struct SearchData {
    chars: Vec<char>,
    input_pos: usize,
}

impl SearchData {
    pub fn append_char(&mut self, c: char) {
        self.chars.insert(self.input_pos, c);
        self.input_pos += 1;
    }

    pub fn delete_char(&mut self) {
        if !self.chars.is_empty() && self.input_pos >= 1 {
            self.chars.remove(self.input_pos - 1);
            self.input_pos -= 1;
        }
    }

    pub fn move_position(&mut self, direction: i32) {
        if !self.chars.is_empty() {
            let next_pos = self.input_pos as i32 + direction;
            self.input_pos = clamp(next_pos, 0, self.chars.len() as i32) as usize;
        }
    }

    pub fn query(&self) -> String {
        self.chars.iter().collect()
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct SearchState {
    cursor_pos: usize,
}

pub struct SearchWidget {
    style: WidgetStyle,
}

impl SearchWidget {
    pub fn with_style(style: WidgetStyle) -> Self {
        SearchWidget { style }
    }

    fn search_text_from(&self, data: &SearchData) -> Paragraph {
        let text = data.query();
        Paragraph::new(text).block(default_block_with_style(&self.style.block, "Search"))
    }
}

impl CustomWidget for SearchWidget {
    type Data = SearchData;
    type State = SearchState;

    fn render_app_widget(
        self,
        area: Rect,
        buf: &mut Buffer,
        data: &Self::Data,
        _state: &mut Self::State,
    ) {
        let search_text = self.search_text_from(data);
        render_widget(search_text, area, buf);
    }
}
