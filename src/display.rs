#![cfg(feature = "interactive")]

use minifb::{Key, Window, WindowOptions};

use crate::plot::Plot;

/// Display the plot in a window
pub fn plot_interactive(plot: &Plot) -> Option<()> {
    let width: usize = plot.canvas.width() as usize;
    let height: usize = plot.canvas.height() as usize;

    let mut window: Window = Window::new("Chartrs", width, height, WindowOptions::default())
        .map_err(|err| println!("[ERROR] Failed to open window for interactive plot: {err}"))
        .ok()?;

    window.set_target_fps(30);

    window
        .update_with_buffer(plot.canvas.get_data(), width, height)
        .map_err(|err| println!("[ERROR] Failed to draw interactive plot: {err}"))
        .ok()?;

    while window.is_open() && !(window.is_key_down(Key::Q) || window.is_key_down(Key::Escape)) {
        window.update();
    }

    Some(())
}
