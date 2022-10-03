use crate::types::{App, Field, InputMode};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::io;
use tui::backend::Backend;
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
use tui::{Frame, Terminal};

pub fn run<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        return Ok(());
                    }
                    KeyCode::Up | KeyCode::Char('k') => app.todos.previous(),
                    KeyCode::Down | KeyCode::Char('j') => app.todos.next(),
                    KeyCode::Char('a') => app.mode = InputMode::Editing,
                    KeyCode::Char(' ') => app.todos.toggle_completed(),
                    KeyCode::Char('d') => app.remove_current_todo(),
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Esc => app.mode = InputMode::Normal,
                    KeyCode::Char('s')
                        if key.modifiers == KeyModifiers::CONTROL
                            && app.field == Field::Description =>
                    {
                        match app.field {
                            Field::Name => app.field = Field::Description,
                            Field::Description => app.add_todo(),
                        }
                    }
                    KeyCode::Enter => match app.field {
                        Field::Name => app.field = Field::Description,
                        Field::Description => {} // do nothing since we want new lines
                    },
                    _ => app.handle_input_event(key),
                },
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // TODO: draw
    match app.mode {
        InputMode::Normal => {
            // TODO: normal todo view
            let chunks = Layout::default()
                .direction(tui::layout::Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.size());

            let list_items: Vec<ListItem> = app
                .todos
                .todos
                .iter()
                .map(|i| {
                    let finished = if i.finished { "[x] " } else { "[ ] " };
                    let line = finished.to_string() + i.name.as_ref();
                    let line = vec![Spans::from(line)];
                    ListItem::new(line).style(Style::default())
                })
                .collect();

            let items = List::new(list_items)
                .block(Block::default().borders(Borders::ALL).title("Todos"))
                .highlight_style(Style::default().bg(Color::White).fg(Color::Black))
                .highlight_symbol(">> ");
            f.render_stateful_widget(items, chunks[0], &mut app.todos.state);

            let description = Paragraph::new(app.get_current_description())
                .wrap(tui::widgets::Wrap { trim: true })
                .block(Block::default().borders(Borders::ALL).title("Description"));
            f.render_widget(description, chunks[1]);
        }
        InputMode::Editing => {
            // TODO: edit todo view
            let chunks = Layout::default()
                .direction(tui::layout::Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(20),
                        Constraint::Length(3),
                        Constraint::Min(10),
                        Constraint::Percentage(20),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let name_input = &app.new_todo.name;
            let width = chunks[1].width.max(3) - 3;
            let scroll = (name_input.cursor() as u16).max(width) - width;
            let name_input = Paragraph::new(name_input.value())
                .style(match app.field {
                    Field::Name => Style::default().fg(Color::Blue),
                    Field::Description => Style::default(),
                })
                .scroll((0, scroll))
                .block(Block::default().borders(Borders::ALL).title("Name"));

            let desc_input = &app.new_todo.description;
            let width = chunks[2].width.max(3) - 3;
            let scroll = (desc_input.cursor() as u16).max(width) - width;
            let desc_input = Paragraph::new(desc_input.value())
                .style(match app.field {
                    Field::Description => Style::default().fg(Color::Blue),
                    Field::Name => Style::default(),
                })
                .scroll((0, scroll))
                .block(Block::default().borders(Borders::ALL).title("Description"));

            f.render_widget(name_input, chunks[1]);
            f.render_widget(desc_input, chunks[2]);

            match app.field {
                Field::Name => f.set_cursor(
                    chunks[1].x + (app.new_todo.name.cursor() as u16).min(width) + 1,
                    chunks[1].y + 1,
                ),
                Field::Description => f.set_cursor(
                    chunks[2].x + (app.new_todo.description.cursor() as u16).min(width) + 1,
                    chunks[2].y + 1,
                ),
            }
        }
    }
}
