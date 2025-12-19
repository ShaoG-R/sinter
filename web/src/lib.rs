mod app;
mod components;
mod pages;

use app::App;
use leptos::mount::mount_to_body;

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
