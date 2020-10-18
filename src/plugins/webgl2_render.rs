use bevy::asset::AssetLoader;
use bevy::prelude::*;
use bevy::render::texture::ImageTextureLoader;
use js_sys;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

#[derive(Default)]
pub struct WebGL2RenderPlugin;
type Gl = WebGl2RenderingContext;

#[derive(Debug)]
pub struct WebGL2Context {
    pub gl: web_sys::WebGl2RenderingContext,
}
unsafe impl Send for WebGL2Context {}
unsafe impl Sync for WebGL2Context {}

impl Plugin for WebGL2RenderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas = document
            .query_selector("#bevy-canvas")
            .expect("#bevy-canvas exists")
            .unwrap();
        let canvas: web_sys::HtmlCanvasElement =
            canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let context = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .unwrap();

        let ctx = WebGL2Context { gl: context };

        info!("ctx: {:?}", ctx);
        app.add_resource(ctx);
        app.add_startup_system(init_webgl.system());
        app.add_system_to_stage(stage::POST_UPDATE, draw.system());

        //        app.add_startup_system(webgl2_init.system());
    }
}

pub fn init_webgl(context: Res<WebGL2Context>) {
    let width: f32 = 31.0 * 32.0;
    let height: f32 = 18.0 * 32.0;

    let gl = &context.gl;
    // context.viewport(0, 0, 31 * 32, 18 * 32);
    let vert_shader = compile_shader(
        gl,
        WebGl2RenderingContext::VERTEX_SHADER,
        r#"#version 300 es
        // an attribute is an input (in) to a vertex shader.
        // It will receive data from a buffer
        in vec2 a_position;
        in vec2 a_texcoord;
        uniform vec2 u_resolution;

        // a varying to pass the texture coordinates to the fragment shader
        out vec2 v_texcoord;

        // all shaders have a main function
        void main() {
          gl_Position = vec4(a_position / u_resolution * 2.0 - 1.0, 0, 1);
          v_texcoord = a_texcoord;
        }
    "#,
    )
    .expect("compile vertex shader");

    let frag_shader = compile_shader(
        gl,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r#"#version 300 es
        // fragment shaders don't have a default precision so we need
        // to pick one. highp is a good default. It means "high precision"
        precision highp float;

        // Passed in from the vertex shader.
        in vec2 v_texcoord;

        // The texture.
        uniform sampler2D u_texture;

        // we need to declare an output for the fragment shader
        out vec4 outColor;

        void main() {
          // Just set the output to a constant reddish-purple
          //outColor = vec4(1, 0, 0.5, 1);
          outColor = texture(u_texture, v_texcoord);
        }
    "#,
    )
    .expect("compile fragment shader");
    let program = link_program(gl, &vert_shader, &frag_shader).expect("link program");
    gl.use_program(Some(&program));

    let tex = gl.create_texture().unwrap();
    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&tex));

    let loader = ImageTextureLoader;
    static IMAGE_DATA: &[u8] = include_bytes!("../../assets/icons32.png");
    let texture = loader
        .from_bytes(
            std::path::Path::new("bla.png"),
            IMAGE_DATA.iter().cloned().collect(),
        )
        .unwrap();
    gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        Gl::TEXTURE_2D,
        0,
        Gl::RGBA as i32,
        texture.size.x() as i32,
        texture.size.y() as i32,
        0,
        Gl::RGBA,
        Gl::UNSIGNED_BYTE,
        Some(&texture.data),
    )
    .expect("tex image");
    gl.generate_mipmap(Gl::TEXTURE_2D);

    let buffer = gl
        .create_buffer()
        .ok_or("failed to create buffer")
        .expect("create_buffer");
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    let position_attribute_location = gl.get_attrib_location(&program, "a_position");
    let texcoord_attribute_location = gl.get_attrib_location(&program, "a_texcoord");
    let resolution_uniform_location = gl
        .get_uniform_location(&program, "u_resolution")
        .expect("uniform");

    gl.vertex_attrib_pointer_with_i32(
        position_attribute_location as u32,
        2,
        WebGl2RenderingContext::FLOAT,
        false,
        16,
        0,
    );
    gl.enable_vertex_attrib_array(position_attribute_location as u32);

    gl.vertex_attrib_pointer_with_i32(
        texcoord_attribute_location as u32,
        2,
        WebGl2RenderingContext::FLOAT,
        true,
        16,
        8,
    );
    gl.enable_vertex_attrib_array(texcoord_attribute_location as u32);

    gl.uniform2f(
        Some(&resolution_uniform_location),
        width as f32,
        height as f32,
    );
    gl.enable(Gl::BLEND);
    gl.blend_func(Gl::ONE, Gl::ONE_MINUS_SRC_ALPHA);
}
fn append_tile(verticles: &mut Vec<f32>, x: f32, y: f32, tile_x: f32, tile_y: f32) {
    let tiles = &[
        x,
        y,
        tile_x / 12.0,
        1.0 - tile_y / 8.0,
        x,
        y + 32.0,
        tile_x / 12.0,
        1.0 - (tile_y + 1.0) / 8.0,
        x + 32.0,
        y,
        (tile_x + 1.0) / 12.0,
        1.0 - tile_y / 8.0,
        x + 32.0,
        y + 32.0,
        (tile_x + 1.0) / 12.0,
        1.0 - (tile_y + 1.0) / 8.0,
        x + 32.0,
        y,
        (tile_x + 1.0) / 12.0,
        1.0 - tile_y / 8.0,
        x,
        y + 32.0,
        tile_x / 12.0,
        1.0 - (tile_y + 1.0) / 8.0,
    ];
    verticles.extend(tiles);
}

pub fn draw(
    context: Res<WebGL2Context>,
    mut sprites: Query<(&Transform, &TextureAtlasSprite, &crate::components::Position)>,
) {
    let gl = &context.gl;
    gl.clear_color(0.3, 0.3, 0.3, 1.0);
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    let mut vertices = vec![];
    for (transform, sprite, _) in &mut sprites.iter() {
        let tile_x = (sprite.index % 12) as f32;
        let tile_y = (7-(sprite.index / 12)) as f32;
        let tr = transform.translation();
        append_tile(&mut vertices, tr.x(), tr.y(), tile_x, tile_y);
    }

    unsafe {
        let vert_array = js_sys::Float32Array::view(&vertices);

        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, (vertices.len() / 4) as i32);
}

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}
