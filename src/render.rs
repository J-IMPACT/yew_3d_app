use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

pub fn initialize_webgl() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id("webgl-canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    let gl = canvas
        .get_context("webgl")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()
        .unwrap();

    gl.clear_color(0.1, 0.1, 0.2, 1.0);
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
}