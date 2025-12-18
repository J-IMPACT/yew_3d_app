use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlBuffer, WebGlProgram, WebGlRenderingContext as GL, WebGlShader, window};

pub struct Renderer {
    gl: GL,
    program: WebGlProgram,
    buffer: WebGlBuffer,
    position_loc: u32,
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
            attribute vec2 position;
            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
                gl_PointSize = 2.0;
            }
        "#;

        let frag_shader_src = r#"
            precision mediump float;
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

        let buffer = gl.create_buffer().expect("failed to create buffer");
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
        
        let position_loc = gl.get_attrib_location(&program, "position") as u32;
        gl.enable_vertex_attrib_array(position_loc);
        gl.vertex_attrib_pointer_with_i32(position_loc, 2, GL::FLOAT, false, 0, 0);

        Self { gl, program, buffer, position_loc }
    }

    pub fn render_xy(&self, xy: &[f32]) {
        self.gl.clear(GL::COLOR_BUFFER_BIT);

        self.gl.use_program(Some(&self.program));
        self.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer));

        let arr = js_sys::Float32Array::from(xy);
        self.gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &arr, GL::DYNAMIC_DRAW);

        self.gl.enable_vertex_attrib_array(self.position_loc);
        self.gl.vertex_attrib_pointer_with_i32(self.position_loc, 2, GL::FLOAT, false, 0, 0);

        let count = (xy.len() / 2) as i32;
        self.gl.draw_arrays(GL::POINTS, 0, count);
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
