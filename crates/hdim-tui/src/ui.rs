use crate::app::{App, AppMode};
use crate::components::crop::render_crop_options;
use ansi_to_tui::IntoText;
use color_eyre::eyre::Result;
use hdim_core::state::Tool;
use hdim_render::view::View;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

pub fn render(frame: &mut Frame, app: &mut App) {
    let main_layout = if app.show_right_toolbar {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
            .split(frame.area())
    } else {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .split(frame.area())
    };

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(main_layout[0]);

    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(vertical_chunks[1]);

    let main_area = middle_chunks[1];

    // RENDER THE VIEWPORT
    let image_width = app.hdim_image.width;
    let image_height = app.hdim_image.height;

    let source_width = (main_area.width as f32 * app.zoom).round() as u32;
    let source_height = (main_area.height as f32 * app.zoom * 2.0).round() as u32;

    app.source_pos.0 = app
        .source_pos
        .0
        .min(image_width.saturating_sub(source_width));
    app.source_pos.1 = app
        .source_pos
        .1
        .min(image_height.saturating_sub(source_height));

    let view = View {
        source_x: app.source_pos.0,
        source_y: app.source_pos.1,
        source_width,
        source_height,
        target_width: main_area.width as u32,
        target_height: main_area.height as u32,
    };

    let image_text = match hdim_render::render(&app.hdim_image.data, &view) {
        Result::Ok(ansi_string) => ansi_string.into_text().unwrap_or_default(),
        Err(_) => "Error rendering image".into_text().unwrap(),
    };

    let magnification = 1.0 / app.zoom;
    let main_title = format!(
        "Main Window - Pos [Y: {}, X: {}] - Zoom: {:.2}x",
        app.source_pos.1, app.source_pos.0, magnification
    );

    frame.render_widget(
        Paragraph::new(" Navigation Bar / Title ")
            .block(Block::default().borders(Borders::ALL).title("Top")),
        vertical_chunks[0],
    );

    let tools = List::new([ListItem::new("1. Crop"), ListItem::new("2. Exif")])
        .block(Block::default().borders(Borders::ALL).title("Tools"));
    frame.render_widget(tools, middle_chunks[0]);

    frame.render_widget(
        Paragraph::new(image_text).block(Block::default().borders(Borders::ALL).title(main_title)),
        main_area,
    );

    if app.show_right_toolbar {
        match app.mode {
            AppMode::ExifView => {
                if let Some(exif_view) = &mut app.exif_view {
                    let mut list = exif_view.widget();
                    if app.active_widget == crate::app::ActiveWidget::RightToolbar {
                        list =
                            list.highlight_style(Style::default().add_modifier(Modifier::REVERSED));
                    }
                    frame.render_stateful_widget(list, main_layout[1], &mut exif_view.state);
                } else {
                    // Fallback if exif_view is None in ExifView mode
                    frame.render_widget(
                        List::new(vec![ListItem::new("No EXIF data available.")])
                            .block(Block::default().borders(Borders::ALL).title("EXIF Data")),
                        main_layout[1],
                    );
                }
            }
            AppMode::Normal | AppMode::EditingCropValue => {
                if let Some(Tool::Crop) = app.selected_tool {
                    frame.render_widget(render_crop_options(app), main_layout[1]);
                } else {
                    // Default content for the right toolbar when no tool is selected
                    frame.render_widget(
                        List::new(vec![ListItem::new("Right Toolbar Content")])
                            .block(Block::default().borders(Borders::ALL).title("Right")),
                        main_layout[1],
                    );
                }
            }
        };
    }

    let bottom_text = match app.mode {
        AppMode::Normal if app.selected_tool.is_some() => {
            "Tab to switch | Enter to edit/select | Esc to deselect"
        }
        AppMode::ExifView => "Up/Down to scroll | Esc to deselect",
        _ => " Arrows to Pan | PgUp/PgDn to Zoom | 'q' to Quit ",
    };

    frame.render_widget(
        Paragraph::new(bottom_text).block(Block::default().borders(Borders::ALL).title("Bottom")),
        vertical_chunks[2],
    );
}
