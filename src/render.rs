use glam::{Mat4, Vec3};
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlCanvasElement, WebGlBuffer, WebGlProgram, 
    WebGlRenderingContext as GL, WebGlShader, window
};

pub struct Renderer {
    gl: GL,
    buffer: WebGlBuffer,
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
        // 頂点シェーダ
        let vert_shader_src = r#"
            attribute vec3 position;
            uniform mat4 u_mvp;

            void main() {
                vec4 p = u_mvp * vec4(position, 1.0);
                gl_Position = p;

                float size = 8.0 / (-p.z);
                gl_PointSize = clamp(size, 2.0, 8.0);
            }
        "#;
        // フラグメントシェーダ
        let frag_shader_src = r#"
            precision mediump float;
            void main() {
                gl_FragColor = vec4(1.0);
            }
        "#;

        // シェーダーとプログラムの作成
        let vert_shader = compile_shader(
            &gl, 
            GL::VERTEX_SHADER, 
            vert_shader_src
        ).unwrap();
        let frag_shader = compile_shader(
            &gl, 
            GL::FRAGMENT_SHADER, 
            frag_shader_src
        ).unwrap();
        let program = link_program(&gl, &vert_shader, &frag_shader).unwrap();

        // WebGLの初期設定
        gl.use_program(Some(&program));
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.enable(GL::DEPTH_TEST);

        // 頂点バッファの作成
        let buffer = gl.create_buffer().expect("failed to create buffer");
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
        
        // attribute (position) の設定
        let position_loc = gl.get_attrib_location(&program, "position") as u32;
        gl.enable_vertex_attrib_array(position_loc);
        gl.vertex_attrib_pointer_with_i32(
            position_loc,
            3, 
            GL::FLOAT, 
            false, 
            0, 
            0
        );

        // 画面アスペクト比
        let aspect = canvas.width() as f32 / canvas.height() as f32;

        // 透視投影行列
        let proj = Mat4::perspective_rh_gl(
            45.0_f32.to_radians(),
            aspect,
            0.1,
            100.0,
        );

        // 斜め上から原点を見る固定カメラ
        let view = Mat4::look_at_rh(
            Vec3::new(18.0, 12.0, 24.0),
            Vec3::ZERO,
            Vec3::Y,
        );

        // MVP行列の作成
        let mvp = proj * view;

        // uniformにMVP行列を送る
        let mvp_loc = gl.get_uniform_location(&program, "u_mvp").unwrap();
        gl.uniform_matrix4fv_with_f32_array(
            Some(&mvp_loc),
            false,
            mvp.as_ref(),
        );

        Self { gl, buffer }
    }

    /// xyz配列を描画
    pub fn render_xyz(&self, xyz: &[f32]) {
        // 前フレームをクリア
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        // 頂点バッファを再バインド
        self.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer));

        // WASM → JS → WebGLにデータ転送
        let arr = js_sys::Float32Array::from(xyz);
        self.gl.buffer_data_with_array_buffer_view(
            GL::ARRAY_BUFFER,
            &arr,
            GL::DYNAMIC_DRAW,
        );

        // POINTSとして描画
        let count = (xyz.len() / 3) as i32;
        self.gl.draw_arrays(GL::POINTS, 0, count);
    }
}

// シェーダーのコンパイル関数
fn compile_shader(gl: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader"))?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false) 
    {
        Ok(shader)
    } else {
        Err(
            gl.get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

// プログラムのリンク関数
fn link_program(
    gl: &GL, 
    vert_shader: &WebGlShader, 
    frag_shader: &WebGlShader
) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or_else(|| String::from("Unable to create program"))?;
    gl.attach_shader(&program, vert_shader);
    gl.attach_shader(&program, frag_shader);
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false) 
    {
        Ok(program)
    } else {
        Err(
            gl.get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object"))
        )
    }
}