use tui::text::Span;
use tui::widgets::{Block, ListItem};

use crate::ui::theme::{BlockStyle, ItemStyle};

pub fn apply_block_style<'a>(block: Block<'a>, styles: &BlockStyle, title: &'a str) -> Block<'a> {
    block
        .borders(styles.borders)
        .border_type(styles.border_type)
        .border_style(styles.border_style)
        .title(Span::styled(title, styles.title_style))
}

pub fn default_block_with_style<'a>(styles: &BlockStyle, title: &'a str) -> Block<'a> {
    apply_block_style(Block::default(), styles, title)
}

pub fn apply_item_style<'a>(item: ListItem<'a>, styles: &ItemStyle) -> ListItem<'a> {
    item.style(styles.normal)
}
