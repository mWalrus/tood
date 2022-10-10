use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

use crate::types::app::App;

use super::utils;

pub fn todo_list<B: Backend>(app: &mut App, f: &mut Frame<B>) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(60),
                Constraint::Min(3),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(size);

    let list_items: Vec<ListItem> = app
        .todos
        .todos
        .iter()
        .map(|t| {
            let (finished, fg_style) = if t.metadata.recurring {
                ("[∞] ", Style::default().fg(Color::Blue))
            } else if t.finished {
                ("[x] ", Style::default().fg(Color::Green))
            } else {
                ("[ ] ", Style::default())
            };
            let line = finished.to_string() + t.name.as_ref();
            let line = vec![Spans::from(line)];
            ListItem::new(line).style(fg_style)
        })
        .collect();

    let items = List::new(list_items)
        .block(utils::default_block("Todos"))
        .highlight_style(
            Style::default()
                .bg(Color::Indexed(8))
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");
    f.render_stateful_widget(items, chunks[0], &mut app.todos.state);

    let data_chunks = Layout::default()
        .direction(tui::layout::Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Min(30)].as_ref())
        .split(chunks[1]);

    let (desc, metadata) = app.get_current_todo_data();

    let description = Paragraph::new(desc)
        .wrap(tui::widgets::Wrap { trim: true })
        .block(utils::default_block("Description"));
    f.render_widget(description, data_chunks[0]);

    if let Some(metadata) = metadata {
        let list = List::new(utils::metadata_to_list_item(metadata))
            .block(utils::default_block("Metadata"));
        f.render_widget(list, data_chunks[1]);
    } else {
        let placeholder_p = Paragraph::new("").block(utils::default_block("Metadata"));
        f.render_widget(placeholder_p, data_chunks[1]);
    };
}

pub fn edit_modal<B: Backend>(app: &App, f: &mut Frame<B>) {
    let rect = utils::centered_rect(f.size());

    let chunks = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Max(10)].as_ref())
        .split(rect);

    let name_input = &app.todos.new_todo.name;
    let width = chunks[0].width.max(3) - 3;
    let scroll = (name_input.cursor() as u16).max(width) - width;
    let name_input = Paragraph::new(name_input.value())
        .scroll((0, scroll))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue))
                .title("Name"),
        );

    let width = chunks[1].width.max(3) - 3;
    let desc_input = Paragraph::new(&*app.todos.new_todo.description)
        .wrap(tui::widgets::Wrap { trim: true })
        .block(utils::default_block("Description"));

    f.render_widget(Clear, chunks[0]);
    f.render_widget(Clear, chunks[1]);

    f.render_widget(name_input, chunks[0]);
    f.render_widget(desc_input, chunks[1]);
    f.set_cursor(
        chunks[0].x + (app.todos.new_todo.name.cursor() as u16).min(width) + 1,
        chunks[0].y + 1,
    );
}

pub fn fuzzy_matcher<B: Backend>(app: &mut App, f: &mut Frame<B>) {
    let rect = utils::centered_rect(f.size());

    let chunks = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Max(10)].as_ref())
        .split(rect);

    let skimmer_input = &app.skimmer.input;
    let width = chunks[0].width.max(3) - 3;
    let scroll = (skimmer_input.cursor() as u16).max(width) - width;
    let skimmer_input = Paragraph::new(skimmer_input.value())
        .scroll((0, scroll))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue))
                .title("Name"),
        );

    let width = chunks[1].width.max(3) - 3;

    let list_items: Vec<ListItem> = app
        .skimmer
        .matches
        .iter()
        .map(|m| {
            let mut spans: Vec<Span> = Vec::new();
            for (i, c) in m.text.chars().enumerate() {
                if m.indices.contains(&i) {
                    spans.push(Span::styled(
                        c.to_string(),
                        Style::default()
                            .fg(Color::Blue)
                            .add_modifier(Modifier::BOLD),
                    ));
                } else {
                    spans.push(Span::raw(c.to_string()));
                }
            }
            let spans = Spans::from(spans);
            ListItem::new(spans).style(Style::default())
        })
        .collect();

    let items = List::new(list_items)
        .block(utils::default_block("Todos"))
        .highlight_style(
            Style::default()
                .bg(Color::Indexed(8))
                .add_modifier(Modifier::BOLD),
        );
    f.render_widget(Clear, chunks[0]);
    f.render_widget(Clear, chunks[1]);

    f.render_widget(skimmer_input, chunks[0]);
    f.render_stateful_widget(items, chunks[1], &mut app.skimmer.state);
    f.set_cursor(
        chunks[0].x + (app.skimmer.input.cursor() as u16).min(width) + 1,
        chunks[0].y + 1,
    );
}
