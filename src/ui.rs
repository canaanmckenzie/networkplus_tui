use crate::app::App;
use ratatui::{
    Frame,
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Colors, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

/// Draws the entire terminal user interface for one frame.
/// called ~5 times per second by main.rs
pub fn draw_ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    // split the terminal screen vertically into two chunks
    // top: question prompts
    // bottom: answer options
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2) //adds padding around the edges
        .constraints([
            Constraint::Length(3), //fixed height for the question
            Constraint::Min(1),    //Take remaining space for answers
        ])
        .split(f.size()); //apply this layout to the current terminal frame

    //format the question as styled text (bold, yellow)
    let question = Spans::from(vec![Span::styled(
        &app.current_question.question,
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )]);

    //render the question inside a paragraph widget
    f.render_widget(Paragraph::new(question), chunks[0]);

    // Build the list of answer options with highlight on selected
    let items: Vec<ListItem> = app
        .current_question
        .options
        .iter()
        .enumerate()
        .map(|(i, opt)| {
            let style = if app.selected == i {
                // highlight the selected item with inverted colors
                Style::default().fg(Color::Black).bg(Color::Cyan)
            } else {
                Style::default()
            };
            // each answer is wrapped as a styled text span in a list item
            ListItem::news(Spans::from(Span::styled(opt.clone(), style)))
        })
        .collect();

    let list = List::new(items).block(Block::default().borders(Borders::ALL).title("Answers"));

    f.render_widget(list, chunks[1]);
}
