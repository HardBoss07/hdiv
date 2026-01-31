use crate::app::{ActiveWidget, App, AppMode};
use crate::components::crop::handle_crop_events;
use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use hdim_core::state::Tool;
use std::time::{Duration, Instant};

const PAN_AMOUNT_CHARACTERS: u32 = 10; // Number of characters to pan per key press
const ZOOM_FACTOR: f32 = 1.2; // Zoom factor per key press

pub fn handle_events(app: &mut App) -> Result<bool> {
    if event::poll(Duration::from_millis(16))? {
        if app.last_input_time.elapsed() >= app.input_delay {
            let event = event::read()?;
            if let Event::Key(key) = event {
                if key.kind == KeyEventKind::Press {
                    app.last_input_time = Instant::now();
                    handle_key_press(app, key);
                }
            }
        } else {
            // If the input delay hasn't passed, drain the queue to prevent event buildup
            while event::poll(Duration::from_millis(0))? {
                let _ = event::read();
            }
        }
    }
    Ok(app.mode == AppMode::Normal
        && matches!(event::read(), Ok(Event::Key(key)) if key.code == KeyCode::Char('q')))
}

fn handle_key_press(app: &mut App, key: KeyEvent) {
    let pan_amount_pixels = (PAN_AMOUNT_CHARACTERS as f32 * app.zoom).round() as i32;

    match app.mode {
        AppMode::ExifView => match key.code {
            KeyCode::Up => {
                if let Some(exif_view) = app.exif_view.as_mut() {
                    exif_view.previous();
                }
            }
            KeyCode::Down => {
                if let Some(exif_view) = app.exif_view.as_mut() {
                    exif_view.next();
                }
            }
            KeyCode::Esc => {
                app.mode = AppMode::Normal;
                app.active_widget = ActiveWidget::Main;
            }
            _ => {}
        },
        AppMode::EditingCropValue => match key.code {
            KeyCode::Char(c) if c.is_ascii_digit() => {
                app.crop_input.push(c);
            }
            KeyCode::Backspace => {
                app.crop_input.pop();
            }
            KeyCode::Enter => {
                if let Ok(value) = app.crop_input.parse::<u32>() {
                    match app.selected_crop_option_index {
                        0 => app.crop_state.left = value,
                        1 => app.crop_state.right = value,
                        2 => app.crop_state.top = value,
                        3 => app.crop_state.bottom = value,
                        _ => {}
                    }
                }
                app.crop_input.clear();
                app.mode = AppMode::Normal;
            }
            KeyCode::Esc => {
                app.crop_input.clear();
                app.mode = AppMode::Normal;
            }
            _ => {}
        },
        AppMode::Normal => match key.code {
            KeyCode::Char('q') => {
                // This is now handled in the main loop for a more responsive exit.
            }
            KeyCode::Char('1') => {
                app.selected_tool = Some(Tool::Crop);
                app.active_widget = ActiveWidget::RightToolbar;
            }
            KeyCode::Char('2') => {
                app.selected_tool = Some(Tool::Exif);
                app.mode = AppMode::ExifView;
                app.active_widget = ActiveWidget::RightToolbar;
                if let Some(exif_view) = &mut app.exif_view {
                    exif_view.state.select(Some(0));
                }
            }
            KeyCode::Esc => {
                app.selected_tool = None;
                app.active_widget = ActiveWidget::Main;
            }
            _ => {
                if let Some(Tool::Crop) = app.selected_tool {
                    handle_crop_events(key, app);
                } else {
                    match app.active_widget {
                        ActiveWidget::Main => match key.code {
                            KeyCode::Up => app.scroll(0, -pan_amount_pixels),
                            KeyCode::Down => app.scroll(0, pan_amount_pixels),
                            KeyCode::Left => app.scroll(-pan_amount_pixels, 0),
                            KeyCode::Right => app.scroll(pan_amount_pixels, 0),
                            KeyCode::PageUp => app.zoom(1.0 / ZOOM_FACTOR),
                            KeyCode::PageDown => app.zoom(ZOOM_FACTOR),
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
        },
    }
}
