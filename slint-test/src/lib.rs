slint::include_modules!();

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    // This runs when the wasm module is loaded
    console_error_panic_hook::set_once();

    let ui = AppWindow::new().unwrap();

    // Get the browser window size and set the Slint window to match
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            let width = window.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(1920.0);
            let height = window.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(1080.0);
            ui.window().set_size(slint::LogicalSize::new(width as f32, height as f32));
        }
    }

    ui.run().unwrap();
}
