use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Padding},
    Frame,
    symbols::border,
};
use crate::App;

pub fn ui(frame: &mut Frame, app: &App) {

    

    
    
    let title = Line::from(Span::styled(
        format!("Wish #{}", app.get_current_wishes()),
        Style::default()
            .fg(Color::White)
            .bg(Color::Black)
    )).centered().bold();

    let instructions = Line::from(vec![
        " Next ".into(),
        "<Enter>".blue().bold(),
        " Quit ".into(),
        "<Q> ".blue().bold(),
    ]).centered();

    let container = Block::bordered()
        .border_set(border::THICK)
        .border_style(Style::default().fg(Color::White))
        .title_top(title)
        .title_bottom(instructions);

    let outer_area = frame.area();
    let inner_area = container.inner(outer_area);
    
    // Create the layout sections.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(75),
            Constraint::Percentage(25),
        ])
        .split(inner_area);

    let art_container = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::default().fg(Color::White))
        .title_style(Style::default().fg(Color::White).bg(Color::Black))
        .padding(Padding {left: inner_area.width/2 - app.get_art_length() as u16/2, right: 0, top: 0, bottom: 0});

    let ascii_art = Paragraph::new(Text::raw(
        format!("

{}", app.get_art())
    )).block(art_container);

    let wishes = Paragraph::new(Text::styled(
        app.get_current_dialog().to_string(),
        Style::default().fg(Color::White),
    )).centered();

    frame.render_widget(container, outer_area);

    frame.render_widget(ascii_art, chunks[0]);
    frame.render_widget(wishes, chunks[1]);
    // frame.render_widget(wishes, centered_rect(70, 70, chunks[1]));
}