use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
};

pub struct CyberpunkBlock {
    title: String,
    border_color: Color,
}

impl CyberpunkBlock {
    pub fn new(title: impl Into<String>, border_color: Color) -> Self {
        Self {
            title: title.into(),
            border_color,
        }
    }
}

impl Widget for CyberpunkBlock {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Draw standard border first
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.border_color))
            .title(format!(" {} ", self.title));
        block.render(area, buf);

        // Add "glitch" corners or decorative elements
        // Top-left corner decoration
        if area.width > 2 && area.height > 2 {
            buf.set_string(area.x, area.y, "▛", Style::default().fg(self.border_color));
            buf.set_string(
                area.x + area.width - 1,
                area.y,
                "▜",
                Style::default().fg(self.border_color),
            );
            buf.set_string(
                area.x,
                area.y + area.height - 1,
                "▙",
                Style::default().fg(self.border_color),
            );
            buf.set_string(
                area.x + area.width - 1,
                area.y + area.height - 1,
                "▟",
                Style::default().fg(self.border_color),
            );
        }
    }
}
