use crate::keymap::key_match;
use crate::types::app::{App, InputMode};
use crate::types::notification::ToodMsgType;
use crossterm::event::{self, Event};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::error::Error;
use std::io;
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
use tui::{Frame, Terminal};

pub fn run(mut app: App) -> io::Result<()> {
    let mut terminal = init_terminal().unwrap();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        // clear the current flashed notification from the screen
        if app.notification.rx.try_recv().is_ok() {
            app.notification.clear();
        }

        if let Event::Key(key) = event::read()? {
            match app.mode {
                InputMode::Normal => {
                    if key_match(&key, &app.keys.quit) {
                        reset_terminal().unwrap();
                        return Ok(());
                    } else if key_match(&key, &app.keys.move_up) {
                        app.todos.previous();
                    } else if key_match(&key, &app.keys.move_down) {
                        app.todos.next();
                    } else if key_match(&key, &app.keys.add_todo) {
                        app.mode = InputMode::Editing;
                    } else if key_match(&key, &app.keys.edit_todo) {
                        app.edit_todo();
                    } else if key_match(&key, &app.keys.toggle_completed) {
                        app.toggle_todo_completed();
                    } else if key_match(&key, &app.keys.remove_todo) {
                        app.remove_current_todo();
                    }
                }
                InputMode::Editing => {
                    if key_match(&key, &app.keys.back) {
                        app.reset_state();
                    } else if key_match(&key, &app.keys.save_new_todo) {
                        app.add_todo();
                    } else if key_match(&key, &app.keys.add_description) {
                        reset_terminal().unwrap();
                        app.edit_description();
                        terminal = init_terminal().unwrap();
                    } else {
                        app.handle_input_event(key);
                    }
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    match app.mode {
        InputMode::Normal => {
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

            let binds = [
                ("Up", app.keys.move_up.to_string()),
                ("Down", app.keys.move_down.to_string()),
                ("Add", app.keys.add_todo.to_string()),
                ("Toggle", app.keys.toggle_completed.to_string()),
                ("Edit", app.keys.edit_todo.to_string()),
                ("Delete", app.keys.remove_todo.to_string()),
                ("Quit", app.keys.quit.to_string()),
            ];
            let mut spans: Vec<Span> = Vec::new();
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
                .wrap(tui::widgets::Wrap { trim: true })
                .block(Block::default().borders(Borders::NONE));
            f.render_widget(bind_bar, chunks[2]);

            if let Some(notif) = &app.notification.msg {
                let notif_span = match notif.level {
                    ToodMsgType::Error => {
                        Span::styled(&notif.message, Style::default().bg(Color::LightRed))
                    }
                    ToodMsgType::Warn => Span::styled(
                        &notif.message,
                        Style::default().bg(Color::Yellow).fg(Color::Black),
                    ),
                    ToodMsgType::Info => {
                        Span::styled(&notif.message, Style::default().bg(Color::Green))
                    }
                };
                let notif_paragraph =
                    Paragraph::new(notif_span).block(Block::default().borders(Borders::NONE));
                let width = notif.message.len() as u16;
                // 2 extra to move it inside the borders
                let x = size.width - width - 2;

                let rect = Rect {
                    x,
                    y: 1,
                    width,
                    height: 1,
                };

                f.render_widget(notif_paragraph, rect);
            }
        }
        InputMode::Editing => {
            // TODO: edit todo view
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
                .style(Style::default().fg(Color::Blue))
                .scroll((0, scroll))
                .block(Block::default().borders(Borders::ALL).title("Name"));

            let width = chunks[1].width.max(3) - 3;
            let desc_input = Paragraph::new(&*app.new_todo.description)
                .wrap(tui::widgets::Wrap { trim: true })
                .block(Block::default().borders(Borders::ALL).title("Description"));

            f.render_widget(name_input, chunks[0]);
            f.render_widget(desc_input, chunks[1]);

            let binds = [
                ("Back", app.keys.back.to_string()),
                ("Next Field", app.keys.next_input.to_string()),
                ("Prev Field", app.keys.prev_input.to_string()),
                ("Add desc", app.keys.add_description.to_string()),
                ("Save", app.keys.save_new_todo.to_string()),
            ];
            let mut spans: Vec<Span> = Vec::new();
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
                .wrap(tui::widgets::Wrap { trim: true })
                .block(Block::default().borders(Borders::NONE));
            let bind_bar_rect = Rect {
                x: 0,
                y: size.height - 1,
                width: size.width,
                height: 1,
            };
            f.render_widget(bind_bar, bind_bar_rect);

            f.set_cursor(
                chunks[0].x + (app.new_todo.name.cursor() as u16).min(width) + 1,
                chunks[0].y + 1,
            );
        }
    }
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

// Inits the terminal.
pub fn init_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    crossterm::execute!(io::stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(io::stdout());

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    Ok(terminal)
}

// Resets the terminal.
pub fn reset_terminal() -> Result<()> {
    disable_raw_mode()?;
    crossterm::execute!(io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}
