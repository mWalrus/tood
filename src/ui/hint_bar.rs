use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, binds: &[(&'static str, String)]) {
    let size = f.size();
    let rect = Rect {
        x: 0,
        y: size.height - 1,
        width: size.width,
        height: 1,
    };
    let mut spans: Vec<Span> = Vec::new();
    spans.push(Span::raw(" "));
    for bind in binds {
        spans.push(Span::styled(
            format!("{} [{}]", bind.0, bind.1),
            Style::default()
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        ));
        // space between hints
        spans.push(Span::raw(" "));
    }
    let bind_bar = Paragraph::new(Spans::from(spans))
        .wrap(tui::widgets::Wrap { trim: false })
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(bind_bar, rect);
}
