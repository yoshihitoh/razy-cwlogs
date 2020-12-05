use tui::style::{Color, Modifier, Style};
use tui::widgets::{BorderType, Borders};

#[derive(Debug, Copy, Clone)]
pub struct ColorStyle {
    fg: Option<Color>,
    bg: Option<Color>,
}

impl ColorStyle {
    pub fn new(fg: Option<Color>, bg: Option<Color>) -> ColorStyle {
        ColorStyle { fg, bg }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BlockStyle {
    pub borders: Borders,
    pub border_type: BorderType,
    pub border_style: Style,
    pub title_style: Style,
}

impl BlockStyle {
    pub fn with_color(c: ColorStyle) -> BlockStyle {
        BlockStyle {
            borders: Borders::ALL,
            border_type: BorderType::Plain,
            border_style: style_with_colors(Style::default(), c),
            title_style: style_with_colors(Style::default().add_modifier(Modifier::BOLD), c),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ItemStyle {
    pub normal: Style,
    pub highlight: Style,
}

impl ItemStyle {
    pub fn with_colors(normal: ColorStyle, highlight: ColorStyle) -> ItemStyle {
        ItemStyle {
            normal: style_with_colors(Style::default(), normal),
            highlight: style_with_colors(Style::default(), highlight),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TableStyle {
    pub header: Style,
    pub normal: Style,
    pub highlight: Style,
}

impl TableStyle {
    pub fn with_colors(header: ColorStyle, normal: ColorStyle, highlight: ColorStyle) -> Self {
        TableStyle {
            header: style_with_colors(Style::default(), header),
            normal: style_with_colors(Style::default(), normal),
            highlight: style_with_colors(Style::default(), highlight),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct WidgetStyle {
    pub block: BlockStyle,
    pub item: ItemStyle,
    pub table: TableStyle,
}

impl WidgetStyle {
    pub fn with_color(
        block: ColorStyle,
        header: ColorStyle,
        normal: ColorStyle,
        highlight: ColorStyle,
    ) -> WidgetStyle {
        WidgetStyle {
            block: BlockStyle::with_color(block),
            item: ItemStyle::with_colors(normal, highlight),
            table: TableStyle::with_colors(header, normal, highlight),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Theme {
    pub active_widget: WidgetStyle,
    pub selecting_widget: WidgetStyle,
    pub normal_widget: WidgetStyle,
    pub debug_widget: WidgetStyle,
}

impl Default for Theme {
    fn default() -> Self {
        let header = ColorStyle::new(Some(Color::Yellow), None);
        let with_header =
            |block, normal, highlight| WidgetStyle::with_color(block, header, normal, highlight);

        let active_widget = with_header(
            ColorStyle::new(Some(Color::LightRed), None),
            ColorStyle::new(None, None),
            ColorStyle::new(Some(Color::LightRed), None),
        );

        let selecting_widget = with_header(
            ColorStyle::new(Some(Color::Yellow), None),
            ColorStyle::new(None, None),
            ColorStyle::new(Some(Color::Yellow), None),
        );

        let normal_widget = with_header(
            ColorStyle::new(None, None),
            ColorStyle::new(None, None),
            ColorStyle::new(Some(Color::Green), None),
        );

        let debug_widget = with_header(
            ColorStyle::new(Some(Color::DarkGray), None),
            ColorStyle::new(Some(Color::DarkGray), None),
            ColorStyle::new(None, None),
        );

        Theme {
            active_widget,
            selecting_widget,
            normal_widget,
            debug_widget,
        }
    }
}

fn style_with_colors(s: Style, c: ColorStyle) -> Style {
    Some(s)
        .map(|s| c.fg.map(|c| s.fg(c)).unwrap_or(s))
        .map(|s| c.bg.map(|c| s.bg(c)).unwrap_or(s))
        .unwrap()
}
