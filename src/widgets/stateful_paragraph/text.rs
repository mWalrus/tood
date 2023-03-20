use ratatui::text::StyledGrapheme;
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
}

impl<'a, 'b> WordWrapper<'a, 'b> {
    pub fn new(symbols: Symbols<'a, 'b>, max_line_width: u16) -> WordWrapper<'a, 'b> {
        Self {
            symbols,
            max_line_width,
            current_line: vec![],
            next_line: vec![],
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
            .map(|StyledGrapheme { symbol, .. }| -> u16 { symbol.width() as u16 })
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
                || symbol_whitespace && symbol != "\n" && current_line_width == 0
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
                let (truncate_at, truncated_width) = if symbols_to_last_word_end == 0 {
                    (self.current_line.len() - 1, self.max_line_width)
                } else {
                    (symbols_to_last_word_end, width_to_last_word_end)
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
