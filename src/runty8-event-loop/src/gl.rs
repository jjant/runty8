use glow::{Context, HasContext};

const VERTEX_SHADER: &str = r#"precision highp float;

const vec2 verts[6] = vec2[6](
    vec2(-1.0f, -1.0f),
    vec2(1.0f, 1.0f),
    vec2(-1.0f, 1.0f),
    vec2(-1.0f, -1.0f),
    vec2(1.0f, -1.0f),
    vec2(1.0f, 1.0f)
);
const vec2 all_tex_coords[6] = vec2[6](
    vec2(0.0f, 0.0f),
    vec2(1.0f, 1.0f),
    vec2(0.0f, 1.0f),
    vec2(0.0f, 0.0f),
    vec2(1.0f, 0.0f),
    vec2(1.0f, 1.0f)
);

out vec2 v_tex_coords;

void main() {
    vec2 position = verts[gl_VertexID];
    vec2 tex_coords = all_tex_coords[gl_VertexID];

    v_tex_coords = tex_coords;
    gl_Position = vec4(position, 0.0, 1.0);
}
"#;

const FRAGMENT_SHADER: &str = r#"precision highp float;

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D tex;

void main() {
    float x = v_tex_coords.x;
    float y = 1.0 - v_tex_coords.y;

    color = texture(tex, vec2(x, y));
}
"#;

pub(crate) unsafe fn make_program(gl: &Context, shader_version: &str) -> glow::Program {
    let program = gl.create_program().expect("Cannot create program");

    let shader_sources = [
        (glow::VERTEX_SHADER, VERTEX_SHADER),
        (glow::FRAGMENT_SHADER, FRAGMENT_SHADER),
    ];

    for (shader_type, shader_source) in shader_sources.into_iter() {
        let shader = gl.create_shader(shader_type).expect("Cannot create shader");
        gl.shader_source(shader, &format!("{shader_version}\n{shader_source}"));
        gl.compile_shader(shader);

        if !gl.get_shader_compile_status(shader) {
            panic!("{}", gl.get_shader_info_log(shader));
        }
        gl.attach_shader(program, shader);
    }

    gl.link_program(program);

    if !gl.get_program_link_status(program) {
        panic!("{}", gl.get_program_info_log(program));
    }
    program
}

pub(crate) unsafe fn make_texture(gl: &Context) -> glow::Texture {
    let texture = gl.create_texture().unwrap();

    gl.bind_texture(glow::TEXTURE_2D, Some(texture));

    gl.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_MAG_FILTER,
        glow::NEAREST as i32,
    );

    gl.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_MIN_FILTER,
        glow::NEAREST as i32,
    );

    texture
}

pub(crate) unsafe fn upload_pixels(gl: &Context, texture: glow::Texture, pixels: &[u8]) {
    gl.active_texture(glow::TEXTURE0);

    gl.bind_texture(glow::TEXTURE_2D, Some(texture));

    gl.tex_image_2d(
        glow::TEXTURE_2D,
        0,
        glow::RGB8 as i32,
        128,
        128,
        0,
        glow::RGB,
        glow::UNSIGNED_BYTE,
        Some(pixels),
    );
}

pub(crate) unsafe fn use_texture(gl: &Context, program: glow::Program) {
    let tex_location = gl.get_uniform_location(program, "tex").unwrap();
    gl.uniform_1_i32(Some(&tex_location), 0);
}
