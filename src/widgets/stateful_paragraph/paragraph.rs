use tui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::Text,
    widgets::{Block, StatefulWidget},
};

pub struct StatefulParagraph<'p> {
    block: Option<Block<'p>>,
    style: Style,
    text: Text<'p>,
}

#[derive(Debug, Clone, Copy)]
pub struct ScrollPos {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct ParagraphState {
    pos: ScrollPos,
    lines: u16,
    height: u16,
}

impl ParagraphState {
    pub fn new(pos: ScrollPos, lines: u16, height: u16) -> Self {
        Self { pos, lines, height }
    }

    pub fn set_pos(&mut self, pos: ScrollPos) {
        self.pos = pos;
    }
}

impl<'p> StatefulParagraph<'p> {
    pub fn new<T>(text: T) -> Self
    where
        T: Into<Text<'p>>,
    {
        Self {
            block: None,
            style: Style::default(),
            text: text.into(),
        }
    }

    pub fn block(mut self, block: Block<'p>) -> Self {
        self.block = Some(block);
        self
    }
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl<'a> StatefulWidget for StatefulParagraph<'a> {
    type State = ParagraphState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Something
    }
}
