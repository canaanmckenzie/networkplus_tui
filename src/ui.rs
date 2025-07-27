use crate::app::App;
use ratatui::{
    Frame,
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

/// Draws the entire terminal user interface for one frame.
/// called ~5 times per second by main.rs
pub fn draw_ui(f: &mut Frame, app: &App) {
    // split the terminal screen vertically into two chunks
    // top: question prompts
    // bottom: answer options
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2) //adds padding around the edges
        .constraints([
            Constraint::Length(3), //question
            Constraint::Min(1),    //answers
            Constraint::Length(1), //score/status bar
        ])
        .split(f.area()); //apply this layout to the current terminal frame

    //format the question as styled text (bold, yellow)
    let question = Line::from(vec![Span::styled(
        &app.current_question().question,
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )]);
    //render the question inside a paragraph widget
    f.render_widget(Paragraph::new(question), chunks[0]);

    // Build the list of answer options with highlight on selected
    let items: Vec<ListItem> = app
        .current_question()
        .options
        .iter()
        .enumerate()
        .map(|(i, opt)| {
            let mut style = Style::default();

            if app.answered {
                if app.current_question().correct.contains(&i) {
                    style = style.fg(Color::Green);
                } else if app.selected == i {
                    style = style.fg(Color::Red).bg(Color::Cyan);
                }
            }
            ListItem::new(Line::from(Span::styled(opt.clone(), style)))
        })
        .collect();

    let list = List::new(items).block(Block::default().borders(Borders::ALL).title("Answers"));
    f.render_widget(list, chunks[1]);

    //render status bar with score
    let score_line = Line::from(vec![Span::raw(format!(
        " Score: {} / {}",
        app.score, app.total_attempted
    ))]);
    f.render_widget(Paragraph::new(score_line), chunks[2]);
}
