use tui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Spans,
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

use crate::types::app::App;

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
        .map(|i| {
            let (finished, fg_style) = if i.finished {
                ("[x] ", Style::default().fg(Color::Green))
            } else {
                ("[ ] ", Style::default())
            };
            let line = finished.to_string() + i.name.as_ref();
            let line = vec![Spans::from(line)];
            ListItem::new(line).style(fg_style)
        })
        .collect();

    let is_selected_finished = if let Some(s) = app.todos.selected() {
        s.finished
    } else {
        false
    };

    let highlight_style = if is_selected_finished {
        Style::default().bg(Color::Green)
    } else {
        Style::default().bg(Color::White)
    };

    let items = List::new(list_items)
        .block(Block::default().borders(Borders::ALL).title("Todos"))
        .highlight_style(highlight_style.fg(Color::Black))
        .highlight_symbol(">> ");
    f.render_stateful_widget(items, chunks[0], &mut app.todos.state);

    let description = Paragraph::new(app.get_current_description())
        .wrap(tui::widgets::Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL).title("Description"));
    f.render_widget(description, chunks[1]);
}

pub fn edit_modal<B: Backend>(app: &App, f: &mut Frame<B>) {
    let size = f.size();
    let width = size.width / 2;
    let x = width / 2;

    let height = size.height.min(20);

    let y = if height == size.height { 0 } else { 20 };

    let rect = Rect {
        x,
        y,
        width,
        height,
    };

    let chunks = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Max(10)].as_ref())
        .split(rect);

    let name_input = &app.new_todo.name;
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
    let desc_input = Paragraph::new(&*app.new_todo.description)
        .wrap(tui::widgets::Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL).title("Description"));

    f.render_widget(Clear, chunks[0]);
    f.render_widget(Clear, chunks[1]);

    f.render_widget(name_input, chunks[0]);
    f.render_widget(desc_input, chunks[1]);
    f.set_cursor(
        chunks[0].x + (app.new_todo.name.cursor() as u16).min(width) + 1,
        chunks[0].y + 1,
    );
}
