use crate::physics::Vec3;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, WebGlRenderingContext as GL, WebGlProgram, WebGlShader};

pub struct Renderer {
    gl: GL,
    program: WebGlProgram,
}

impl Renderer {
    pub fn new() -> Self {
        // ドキュメントとキャンバスの取得
        let window = window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let canvas = document
            .get_element_by_id("webgl-canvas")
            .expect("document should have a canvas element with id `webgl-canvas`")
            .dyn_into::<HtmlCanvasElement>()
            .expect("canvas should be an `HtmlCanvasElement`");

        // WebGLコンテキストの取得
        let gl: GL = canvas
            .get_context("webgl")
            .expect("should be able to get WebGL context")
            .unwrap()
            .dyn_into()
            .expect("context should be a `WebGlRenderingContext`");

        // シェーダーのソースコード
        let vert_shader_src = r#"
            attribute vec4 position;
            void main() {
                gl_Position = position;
                gl_PointSize = 2.0;
            }
        "#;

        let frag_shader_src = r#"
            void main() {
                gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
            }
        "#;

        // シェーダーとプログラムの作成
        let vert_shader = compile_shader(&gl, GL::VERTEX_SHADER, vert_shader_src).unwrap();
        let frag_shader = compile_shader(&gl, GL::FRAGMENT_SHADER, frag_shader_src).unwrap();
        let program = link_program(&gl, &vert_shader, &frag_shader).unwrap();

        // WebGLの初期設定
        gl.use_program(Some(&program));
        gl.clear_color(0.0, 0.0, 0.0, 1.0);

        Self { gl, program }
    }

    pub fn render(&self, positions: &[Vec3]) {
        self.gl.clear(GL::COLOR_BUFFER_BIT);

        for pos in positions {
            let x = (pos.x * 0.01) as f32;
            let y = (pos.y * 0.01) as f32;
            self.draw_point(x, y);
        }
    }

    fn draw_point(&self, x: f32, y: f32) {
        let vertices: [f32; 2] = [x, y];
        let buffer = self.gl.create_buffer().expect("failed to create buffer");
        self.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));

        // 安全なバッファデータの設定
        unsafe {
            let vert_array = js_sys::Float32Array::view(&vertices);
            self.gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);
        }

        let position_attr_location = self.gl.get_attrib_location(&self.program, "position") as u32;
        self.gl.enable_vertex_attrib_array(position_attr_location);
        self.gl.vertex_attrib_pointer_with_i32(position_attr_location, 2, GL::FLOAT, false, 0, 0);
        self.gl.draw_arrays(GL::POINTS, 0, 1);
    }
}

// シェーダーのコンパイル関数
fn compile_shader(gl: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = gl.create_shader(shader_type).ok_or_else(|| String::from("Unable to create shader"))?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl.get_shader_parameter(&shader, GL::COMPILE_STATUS).as_bool().unwrap_or(false) {
        Ok(shader)
    } else {
        Err(gl.get_shader_info_log(&shader).unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

// プログラムのリンク関数
fn link_program(gl: &GL, vert_shader: &WebGlShader, frag_shader: &WebGlShader) -> Result<WebGlProgram, String> {
    let program = gl.create_program().ok_or_else(|| String::from("Unable to create program"))?;
    gl.attach_shader(&program, vert_shader);
    gl.attach_shader(&program, frag_shader);
    gl.link_program(&program);

    if gl.get_program_parameter(&program, GL::LINK_STATUS).as_bool().unwrap_or(false) {
        Ok(program)
    } else {
        Err(gl.get_program_info_log(&program).unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}
