use std::iter;
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{StyledGrapheme, Text},
    widgets::{Block, StatefulWidget, Widget, Wrap},
};
use unicode_width::UnicodeWidthStr;

use super::text::{LineComposer, WordWrapper};

pub struct StatefulParagraph<'p> {
    block: Option<Block<'p>>,
    style: Style,
    text: Text<'p>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ScrollPos {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ParagraphState {
    scroll: ScrollPos,
    lines: u16,
    height: u16,
}

impl ParagraphState {
    pub fn new(pos: ScrollPos, lines: u16, height: u16) -> Self {
        Self {
            scroll: pos,
            lines,
            height,
        }
    }

    pub const fn lines(self) -> u16 {
        self.lines
    }

    pub const fn height(self) -> u16 {
        self.height
    }

    pub const fn scroll(self) -> ScrollPos {
        self.scroll
    }

    pub fn set_scroll(&mut self, pos: ScrollPos) {
        self.scroll = pos;
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
        buf.set_style(area, self.style);
        let text_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        if text_area.height < 1 {
            return;
        }

        let style = self.style;
        let mut styled = self.text.lines.iter().flat_map(|spans| {
            spans
                .0
                .iter()
                .flat_map(|span| span.styled_graphemes(style))
                // Required given the way composers work but might be refactored out if we change
                // composers to operate on lines instead of a stream of graphemes.
                .chain(iter::once(StyledGrapheme {
                    symbol: "\n",
                    style: self.style,
                }))
        });

        let mut line_composer = Box::new(WordWrapper::new(&mut styled, text_area.width, true));
        let mut y = 0;
        let mut end_reached = false;
        while let Some((current_line, current_line_width)) = line_composer.next_line() {
            if end_reached && y >= state.scroll.y {
                let mut x = 0;
                for StyledGrapheme { symbol, style } in current_line {
                    println!("{symbol}");
                    buf.get_mut(text_area.left() + x, text_area.top() + y - state.scroll.y)
                        .set_symbol(if symbol.is_empty() {
                            // If the symbol is empty, the last char which rendered last time will
                            // leave on the line. It's a quick fix.
                            " "
                        } else {
                            symbol
                        })
                        .set_style(*style);
                    x += symbol.width() as u16;
                }
            }
            y += 1;
            if y >= text_area.height + state.scroll.x {
                end_reached = true;
            }
        }
    }
}
