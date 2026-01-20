mod app;
mod events;
mod ui;

use color_eyre::eyre::Result;
use ratatui::DefaultTerminal;
use std::path::PathBuf;

use crate::app::App;
use crate::events::handle_events;
use crate::ui::render;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    let path = PathBuf::from("./crates/hdim-render/tests/images/4k.jpg");
    let image = image::open(&path)
        .map_err(|e| color_eyre::eyre::eyre!("Could not find test image at {:?}: {}", path, e))?;

    // Start with a zoom level that fits the image width to a default 100-column view
    let initial_zoom = image.width() as f32 / 100.0;

    let app = App::new(image, initial_zoom)?;
    let result = run(terminal, app);

    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, mut app: App) -> Result<()> {
    loop {
        terminal.draw(|frame| render(frame, &mut app))?;

        if handle_events(&mut app)? {
            break;
        }
    }
    Ok(())
}
