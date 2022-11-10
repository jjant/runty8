
use glium::backend::Facade;
use glium::implement_vertex;
use glium::VertexBuffer;
// Rendering boilerplate

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 4],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords); // don't forget to add `tex_coords` here

pub fn whole_screen_vertex_buffer(display: &impl Facade) -> VertexBuffer<Vertex> {
    let vertex1 = Vertex {
        position: [-1.0, -1.0, 0.0, 1.0],
        tex_coords: [0.0, 0.0],
    };
    let vertex2 = Vertex {
        position: [1.0, 1.0, 0.0, 1.0],
        tex_coords: [1.0, 1.0],
    };
    let vertex3 = Vertex {
        position: [-1.0, 1.0, 0.0, 1.0],
        tex_coords: [0.0, 1.0],
    };

    let vertex4 = Vertex {
        position: [-1.0, -1.0, 0.0, 1.0],
        tex_coords: [0.0, 0.0],
    };
    let vertex5 = Vertex {
        position: [1.0, -1.0, 0.0, 1.0],
        tex_coords: [1.0, 0.0],
    };
    let vertex6 = Vertex {
        position: [1.0, 1.0, 0.0, 1.0],
        tex_coords: [1.0, 1.0],
    };

    let shape = vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];

    glium::VertexBuffer::new(display, &shape).unwrap()
}

pub const VERTEX_SHADER: &str = r#"
#version 140

in vec4 position;
in vec2 tex_coords;
out vec2 v_tex_coords;

uniform vec2 wanted_resolution;

void main() {
    v_tex_coords = tex_coords;
    gl_Position = position;
}
"#;

pub const FRAGMENT_SHADER: &str = r#"
#version 140

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D tex;

void main() {
    float y = 1.0 - v_tex_coords.y;
    color = texture(tex, vec2(v_tex_coords.x, y));
}
"#;
