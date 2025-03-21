use wasm_bindgen::prelude::*;
use yew::Renderer;

mod app;
mod physics;
mod render;

use app::App;

#[wasm_bindgen(start)]
fn main() {
    console_error_panic_hook::set_once();
    Renderer::<App>::new().render();
}
