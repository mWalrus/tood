use ratatui::{
    buffer::Buffer as TUIBuffer,
    layout::{Margin, Rect},
    style::{Color, Style},
    symbols::{block::FULL, line::DOUBLE_VERTICAL},
    widgets::Widget,
};

#[derive(Clone, Copy, Debug)]
pub struct Scrollbar {
    max: u16,
    pos: u16,
    color: Color,
}

impl Scrollbar {
    pub fn new(max: u16, pos: u16, color: Color) -> Self {
        Self { max, pos, color }
    }
}

impl Widget for Scrollbar {
    fn render(self, area: Rect, buf: &mut TUIBuffer) {
        if area.height < 2 {
            return;
        }

        if self.max == 0 {
            return;
        }

        let right = area.right().saturating_sub(1);
        if right <= area.left() {
            return;
        }

        let (bar_top, bar_height) = {
            let scrollbar_area = area.inner(&Margin {
                horizontal: 0,
                vertical: 1,
            });
            (scrollbar_area.top(), scrollbar_area.height)
        };

        // the max should be the entire height of the content within the area.
        //

        for y in bar_top..(bar_top + bar_height) {
            buf.set_string(right, y, DOUBLE_VERTICAL, Style::default());
        }
        let progress = f32::from(self.pos) / f32::from(self.max);
        let progress = if progress > 1.0 { 1.0 } else { progress };
        let pos = f32::from(bar_height) * progress;

        let pos = (pos as u16).saturating_sub(1);

        buf.set_string(right, bar_top + pos, FULL, Style::default().fg(self.color))
    }
}
