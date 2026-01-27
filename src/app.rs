use yew::prelude::*;
use gloo::timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use crate::physics::{fill_xy_f32, init_simulation, step_simulation};
use crate::render::Renderer;

#[function_component(App)]
pub fn app() -> Html {
    let running = use_mut_ref(|| false);
    let renderer = use_state(|| None::<Renderer>);

    // 初回のみ: canvasがDOMに入ったあとにWebGLを初期化
    {
        let renderer = renderer.clone();
        use_effect_with((), move |_| { // DOMが確実に存在してから Renderer::new()
            let new_renderer = Renderer::new();
            renderer.set(Some(new_renderer));
            || ()
        });
    }

    let start_loop = {
        let running = running.clone();
        let renderer = renderer.clone();

        Callback::from(move |_| {
            if *running.borrow() {
                return;
            }

            *running.borrow_mut() = true;
            init_simulation(200);

            let renderer = renderer.clone();
            let running = running.clone();

            spawn_local(async move {
                let mut xy: Vec<f32> = Vec::with_capacity(200 * 2);
                web_sys::console::log_1(&"Started".into());

                loop {
                    if !*running.borrow() {
                        web_sys::console::log_1(&"Stopped".into());
                        break;
                    }

                    step_simulation();

                    fill_xy_f32(&mut xy, 0.01);

                    if let Some(r) = &*renderer {
                        r.render_xy(&xy);
                    }

                    TimeoutFuture::new(16).await;
                }
            });
        })
    };

    let stop_loop = {
        let running = running.clone();
        Callback::from(move |_| {
            *running.borrow_mut() = false;
        })
    };

    html! {
        <>
            <h1>{ "Structured 3D N-Body Simulation (WASM + WebGL)" }</h1>
            <div>
                <canvas id="webgl-canvas" width="600" height="600"></canvas>
            </div>
            <div>
                <button onclick={start_loop}>{ "Start" }</button>
                <button onclick={stop_loop}>{ "Stop" }</button>
            </div>
        </>
    }
}