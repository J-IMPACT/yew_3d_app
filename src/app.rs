use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::physics::simulate_n_body;
use crate::render::initialize_webgl;

#[function_component(App)]
pub fn app() -> Html {
    let on_simulate = Callback::from(|_| {
        spawn_local(async {
            simulate_n_body(300).await;
        });
    });

    let on_render = Callback::from(|_| {
        initialize_webgl();
    });

    html! {
        <div>
            <h1>{ "Structured 3D N-Body Simulation (WASM + WebGL)" }</h1>
            <button onclick={on_simulate}>{ "Run Simulation" }</button>
            <button onclick={on_render}>{ "Init WebGL" }</button>
            <canvas id="webgl-canvas" width="600" height="600"></canvas>
        </div>
    }
}