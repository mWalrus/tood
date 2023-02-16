use tui::text::StyledGrapheme;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

const NBSP: &str = "\u{00a0}";

pub type Symbols<'a, 'b> = &'b mut dyn Iterator<Item = StyledGrapheme<'a>>;

pub trait LineComposer<'a> {
    fn next_line(&mut self) -> Option<(&[StyledGrapheme<'a>], u16)>;
}

pub struct WordWrapper<'a, 'b> {
    symbols: Symbols<'a, 'b>,
    max_line_width: u16,
    current_line: Vec<StyledGrapheme<'a>>,
    next_line: Vec<StyledGrapheme<'a>>,
    trim: bool,
}

impl<'a, 'b> WordWrapper<'a, 'b> {
    pub fn new(symbols: Symbols<'a, 'b>, max_line_width: u16, trim: bool) -> WordWrapper<'a, 'b> {
        Self {
            symbols,
            max_line_width,
            current_line: vec![],
            next_line: vec![],
            trim,
        }
    }
}

impl<'a, 'b> LineComposer<'a> for WordWrapper<'a, 'b> {
    fn next_line(&mut self) -> Option<(&[StyledGrapheme<'a>], u16)> {
        if self.max_line_width == 0 {
            return None;
        }
        std::mem::swap(&mut self.current_line, &mut self.next_line);
        self.next_line.truncate(0);

        let mut current_line_width = self
            .current_line
            .iter()
            .map(|StyledGrapheme { symbol, .. }| symbol.width() as u16)
            .sum();

        let mut symbols_to_last_word_end: usize = 0;
        let mut width_to_last_word_end: u16 = 0;
        let mut prev_whitespace = false;
        let mut symbols_exhausted = true;
        for StyledGrapheme { symbol, style } in &mut self.symbols {
            symbols_exhausted = false;
            let symbol_whitespace = symbol.chars().all(&char::is_whitespace) && symbol != NBSP;

            // Ignore characters wider that the total max width.
            if symbol.width() as u16 > self.max_line_width
                // Skip leading whitespace when trim is enabled.
                || self.trim && symbol_whitespace && symbol != "\n" && current_line_width == 0
            {
                continue;
            }

            // Break on newline and discard it.
            if symbol == "\n" {
                if prev_whitespace {
                    current_line_width = width_to_last_word_end;
                    self.current_line.truncate(symbols_to_last_word_end);
                }
                break;
            }

            // Mark the previous symbol as word end.
            if symbol_whitespace && !prev_whitespace {
                symbols_to_last_word_end = self.current_line.len();
                width_to_last_word_end = current_line_width;
            }

            self.current_line.push(StyledGrapheme { symbol, style });
            current_line_width += symbol.width() as u16;

            if current_line_width > self.max_line_width {
                // If there was no word break in the text, wrap at the end of the line.
                let (truncate_at, truncated_width) = if symbols_to_last_word_end != 0 {
                    (symbols_to_last_word_end, width_to_last_word_end)
                } else {
                    (self.current_line.len() - 1, self.max_line_width)
                };

                // Push the remainder to the next line but strip leading whitespace:
                {
                    let remainder = &self.current_line[truncate_at..];
                    if let Some(remainder_nonwhite) =
                        remainder.iter().position(|StyledGrapheme { symbol, .. }| {
                            !symbol.chars().all(&char::is_whitespace)
                        })
                    {
                        self.next_line
                            .extend_from_slice(&remainder[remainder_nonwhite..]);
                    }
                }
                self.current_line.truncate(truncate_at);
                current_line_width = truncated_width;
                break;
            }

            prev_whitespace = symbol_whitespace;
        }

        // Even if the iterator is exhausted, pass the previous remainder.
        if symbols_exhausted && self.current_line.is_empty() {
            None
        } else {
            Some((&self.current_line[..], current_line_width))
        }
    }
}

/// A state machine that truncates overhanging lines.
pub struct LineTruncator<'a, 'b> {
    symbols: &'b mut dyn Iterator<Item = StyledGrapheme<'a>>,
    max_line_width: u16,
    current_line: Vec<StyledGrapheme<'a>>,
    /// Record the offet to skip render
    horizontal_offset: u16,
}

impl<'a, 'b> LineTruncator<'a, 'b> {
    pub fn new(
        symbols: &'b mut dyn Iterator<Item = StyledGrapheme<'a>>,
        max_line_width: u16,
    ) -> LineTruncator<'a, 'b> {
        LineTruncator {
            symbols,
            max_line_width,
            horizontal_offset: 0,
            current_line: vec![],
        }
    }

    pub fn set_horizontal_offset(&mut self, horizontal_offset: u16) {
        self.horizontal_offset = horizontal_offset;
    }
}

impl<'a, 'b> LineComposer<'a> for LineTruncator<'a, 'b> {
    fn next_line(&mut self) -> Option<(&[StyledGrapheme<'a>], u16)> {
        if self.max_line_width == 0 {
            return None;
        }

        self.current_line.truncate(0);
        let mut current_line_width = 0;

        let mut skip_rest = false;
        let mut symbols_exhausted = true;
        let mut horizontal_offset = self.horizontal_offset as usize;
        for StyledGrapheme { symbol, style } in &mut self.symbols {
            symbols_exhausted = false;

            // Ignore characters wider that the total max width.
            if symbol.width() as u16 > self.max_line_width {
                continue;
            }

            // Break on newline and discard it.
            if symbol == "\n" {
                break;
            }

            if current_line_width + symbol.width() as u16 > self.max_line_width {
                // Exhaust the remainder of the line.
                skip_rest = true;
                break;
            }

            let symbol = if horizontal_offset == 0 {
                symbol
            } else {
                let w = symbol.width();
                if w > horizontal_offset {
                    let t = trim_offset(symbol, horizontal_offset);
                    horizontal_offset = 0;
                    t
                } else {
                    horizontal_offset -= w;
                    ""
                }
            };
            current_line_width += symbol.width() as u16;
            self.current_line.push(StyledGrapheme { symbol, style });
        }

        if skip_rest {
            for StyledGrapheme { symbol, .. } in &mut self.symbols {
                if symbol == "\n" {
                    break;
                }
            }
        }

        if symbols_exhausted && self.current_line.is_empty() {
            None
        } else {
            Some((&self.current_line[..], current_line_width))
        }
    }
}

/// This function will return a str slice which start at specified offset.
/// As src is a unicode str, start offset has to be calculated with each character.
fn trim_offset(src: &str, mut offset: usize) -> &str {
    let mut start = 0;
    for c in UnicodeSegmentation::graphemes(src, true) {
        let w = c.width();
        if w <= offset {
            offset -= w;
            start += c.len();
        } else {
            break;
        }
    }
    &src[start..]
}
