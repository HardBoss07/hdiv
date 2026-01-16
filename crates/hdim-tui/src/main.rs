use color_eyre::eyre::{Ok, Result};
use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal, Frame,
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

fn main() -> Result<()> {
    color_eyre::install()?;

    let terminal = ratatui::init();
    let result = run(terminal);

    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        // Rendering
        terminal.draw(render)?;
        // Input handling
        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame) {
    // Vertical Chunks (Top, Middle, Bottom)
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header: fixed height
            Constraint::Min(0),    // Main Content: takes up remaining space
            Constraint::Length(3), // Footer: fixed height
        ])
        .split(frame.area());

    // Horizontal Chunks inside the "Middle" (Left, Middle, Right)
    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), // Left Sidebar
            Constraint::Percentage(60), // Main Window
            Constraint::Percentage(20), // Right Sidebar
        ])
        .split(vertical_chunks[1]);

    // Widgets
    // Top Navbar
    frame.render_widget(
        Paragraph::new(" Navigation Bar / Title ")
            .block(Block::default().borders(Borders::ALL).title("Top")),
        vertical_chunks[0],
    );

    // Left Sidebar
    frame.render_widget(
        Paragraph::new(" Left Banner Content ")
            .block(Block::default().borders(Borders::ALL).title("Left")),
        middle_chunks[0],
    );

    // Main Window (Center)
    frame.render_widget(
        Paragraph::new(" Main Application Area \n Press ESC to quit.")
            .block(Block::default().borders(Borders::ALL).title("Main")),
        middle_chunks[1],
    );

    // Right Sidebar
    frame.render_widget(
        Paragraph::new(" Right Banner Content ")
            .block(Block::default().borders(Borders::ALL).title("Right")),
        middle_chunks[2],
    );

    // Bottom Toolbar
    frame.render_widget(
        Paragraph::new(" Status Bar / Shortcuts ")
            .block(Block::default().borders(Borders::ALL).title("Bottom")),
        vertical_chunks[2],
    );
}
