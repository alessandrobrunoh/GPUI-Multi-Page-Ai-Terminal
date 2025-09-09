use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Frame,
};
use crate::app::App;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tab bar
            Constraint::Min(0),    // Terminal content
            Constraint::Length(3), // Input area
        ])
        .split(f.size());

    // Draw tab bar
    draw_tab_bar(f, app, chunks[0]);
    
    // Draw terminal content
    draw_terminal_content(f, app, chunks[1]);
    
    // Draw input area
    draw_input_area(f, app, chunks[2]);
}

fn draw_tab_bar<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let tabs: Vec<String> = app
        .get_tabs()
        .iter()
        .map(|(_, title, is_active)| {
            if *is_active {
                format!("● {}", title)
            } else {
                format!("  {}", title)
            }
        })
        .collect();

    let active_tab_index = app
        .get_tabs()
        .iter()
        .position(|(_, _, is_active)| *is_active)
        .unwrap_or(0);

    let tabs_widget = Tabs::new(tabs)
        .block(Block::default().borders(Borders::ALL).title("Terminals"))
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .select(active_tab_index);

    f.render_widget(tabs_widget, area);
}

fn draw_terminal_content<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let content = app.get_current_terminal_content();
    
    let mut lines = Vec::new();
    for line in content.iter() {
        if line.starts_with("AI:") {
            lines.push(Line::from(Span::styled(
                line.clone(),
                Style::default().fg(Color::Cyan),
            )));
        } else if line.starts_with("$") {
            lines.push(Line::from(Span::styled(
                line.clone(),
                Style::default().fg(Color::Green),
            )));
        } else {
            lines.push(Line::from(line.clone()));
        }
    }
    
    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Terminal Output"),
        )
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .wrap(ratatui::widgets::Wrap { trim: false });

    f.render_widget(paragraph, area);
}

fn draw_input_area<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),      // Input field
            Constraint::Length(20),  // Completions
        ])
        .split(area);

    // Draw input field
    let input = Paragraph::new(format!("$ {}", app.get_current_input()))
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[0]);

    // Draw completions if available
    if app.show_completions() && !app.get_completions().is_empty() {
        let completions: Vec<ListItem> = app
            .get_completions()
            .iter()
            .map(|completion| ListItem::new(completion.clone()))
            .collect();

        let completions_list = List::new(completions)
            .block(Block::default().borders(Borders::ALL).title("Completions"))
            .style(Style::default().fg(Color::Green));

        f.render_widget(completions_list, chunks[1]);
    }
}