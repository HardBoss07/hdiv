use crate::app::App;
use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::time::{Duration, Instant};

const PAN_AMOUNT_CHARACTERS: u32 = 10; // Number of characters to pan per key press
const ZOOM_FACTOR: f32 = 1.2; // Zoom factor per key press

pub fn handle_events(app: &mut App) -> Result<bool> {
    if event::poll(Duration::from_millis(16))? {
        if app.last_input_time.elapsed() >= app.input_delay {
            let mut last_key_event = None;
            // Drain the event queue, only keeping the last key press event
            while event::poll(Duration::from_millis(0))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        last_key_event = Some(key);
                    }
                }
            }

            if let Some(key) = last_key_event {
                app.last_input_time = Instant::now();
                // Pan amount is scaled by zoom to maintain consistent screen-space speed
                let pan_amount_pixels = (PAN_AMOUNT_CHARACTERS as f32 * app.zoom).round() as i32;

                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => return Ok(true),
                    // Panning keys
                    KeyCode::Up => app.scroll(0, -pan_amount_pixels),
                    KeyCode::Down => app.scroll(0, pan_amount_pixels),
                    KeyCode::Left => app.scroll(-pan_amount_pixels, 0),
                    KeyCode::Right => app.scroll(pan_amount_pixels, 0),
                    // Zoom keys
                    KeyCode::PageUp => app.zoom(1.0 / ZOOM_FACTOR), // Zoom In
                    KeyCode::PageDown => app.zoom(ZOOM_FACTOR),     // Zoom Out
                    _ => {}
                }
            }
        } else {
            // If the input delay hasn't passed, drain the queue to prevent event buildup
            while event::poll(Duration::from_millis(0))? {
                let _ = event::read();
            }
        }
    }
    Ok(false)
}
