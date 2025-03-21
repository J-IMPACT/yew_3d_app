use yew::Renderer;

mod app;
mod physics;
mod render;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"Rust main() started".into());

    Renderer::<App>::new().render();
}
