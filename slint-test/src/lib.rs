slint::include_modules!();

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    // This runs when the wasm module is loaded
    console_error_panic_hook::set_once();

    let ui = AppWindow::new().unwrap();
    ui.set_status_message("Browser application initialized successfully!".into());
    ui.run().unwrap();
}
