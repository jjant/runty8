use glium::{
    backend::Facade,
    glutin,
    index::NoIndices,
    texture::{RawImage2d, SrgbTexture2d},
    uniform,
    uniforms::{MagnifySamplerFilter, Sampler},
    Display, Program, Surface, VertexBuffer,
};
use runty8_core::{App, Event, Flags, Keys, Map, Pico8, Resources, SpriteSheet};
use runty8_winit::Runty8EventExt as _;
use winit::{
    dpi::LogicalSize,
    event_loop::{ControlFlow, EventLoop},
};

fn create_directory(_assets_path: &str) -> std::io::Result<()> {
    Ok(())
}

fn create_map(_assets_path: &str) -> Map {
    Map::new()
}

fn create_sprite_flags(_assets_path: &str) -> Flags {
    Flags::new()
}

fn create_sprite_sheet(_assets_path: &str) -> SpriteSheet {
    SpriteSheet::new()
}

/// Runs a standalone Runty8 game.
pub fn run<Game: App + 'static>(assets_path: String) -> std::io::Result<()> {
    create_directory(&assets_path)?;

    let map: Map = create_map(&assets_path);
    let sprite_flags: Flags = create_sprite_flags(&assets_path);
    let sprite_sheet = create_sprite_sheet(&assets_path);

    let resources = Resources {
        assets_path,
        sprite_sheet,
        sprite_flags,
        map,
    };
    let mut pico8 = Pico8::new(resources);
    let mut game = Game::init(&mut pico8);
    let mut keys = Keys::new();

    let on_event = move |event, control_flow: &mut ControlFlow, draw: &dyn Fn(&[u8])| match event {
        Event::Tick { .. } => {
            pico8.state.update_keys(&keys);

            game.update(&mut pico8);
            game.draw(&mut pico8);

            draw(pico8.draw_data.buffer());
        }
        Event::Keyboard(event) => {
            keys.on_event(event);
        }
        // Event::Mouse
        Event::WindowClosed => {
            *control_flow = ControlFlow::Exit;
        }
        _ => {
            println!("TODO: Handle event {:?}", event);
        }
    };

    event_loop(on_event);
    Ok(())
}

pub fn event_loop(mut on_event: impl FnMut(Event, &mut ControlFlow, &dyn Fn(&[u8])) + 'static) {
    let event_loop = EventLoop::new();
    let display = make_display(&event_loop, "Runty8");
    let (scale_factor, mut logical_size) = {
        let gl_window = display.gl_window();
        let window = gl_window.window();
        let scale_factor = window.scale_factor();

        (
            scale_factor,
            window.inner_size().to_logical::<f64>(scale_factor),
        )
    };
    let (indices, program) = make_gl_program(&display);
    let vertex_buffer = gl_boilerplate::whole_screen_vertex_buffer(&display);

    event_loop.run(move |winit_event, _, control_flow| {
        let event: Option<Event> =
            Event::from_winit(&winit_event, scale_factor, &mut logical_size, &mut || {
                set_next_timer(control_flow)
            });

        if let Some(event) = event {
            let draw: &dyn Fn(&[u8]) =
                &|pixels| do_draw(&display, &indices, &program, &vertex_buffer, pixels);

            on_event(event, control_flow, draw);
        }
    })
}

fn do_draw(
    display: &Display,
    indices: &NoIndices,
    program: &Program,
    vertex_buffer: &VertexBuffer<gl_boilerplate::Vertex>,
    pixels: &[u8],
) {
    let mut target = display.draw();
    target.clear_color(1.0, 0.0, 0.0, 1.0);
    let image = RawImage2d::from_raw_rgb(pixels.to_vec(), (128, 128));
    let texture = SrgbTexture2d::new(display, image).unwrap();
    let uniforms = uniform! {
        tex: Sampler::new(&texture).magnify_filter(MagnifySamplerFilter::Nearest)
    };
    target
        .draw(
            vertex_buffer,
            indices,
            program,
            &uniforms,
            &Default::default(),
        )
        .unwrap();
    target.finish().unwrap();
}

fn set_next_timer(control_flow: &mut ControlFlow) {
    let fps = 30_u64;
    let nanoseconds_per_frame = 1_000_000_000 / fps;

    let next_frame_time =
        std::time::Instant::now() + std::time::Duration::from_nanos(nanoseconds_per_frame);
    *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
}
fn make_gl_program(display: &impl Facade) -> (NoIndices, Program) {
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let program = glium::Program::from_source(
        display,
        gl_boilerplate::VERTEX_SHADER,
        gl_boilerplate::FRAGMENT_SHADER,
        None,
    )
    .unwrap();

    (indices, program)
}

fn make_display(event_loop: &EventLoop<()>, title: &str) -> Display {
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(LogicalSize::new(640.0, 640.0))
        .with_title(title);
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, event_loop).unwrap();
    {
        display.gl_window().window().set_cursor_visible(false);
    }

    display
}

mod gl_boilerplate {
    use glium::backend::Facade;
    use glium::implement_vertex;
    use glium::VertexBuffer;
    // Rendering boilerplate

    #[derive(Copy, Clone)]
    pub(crate) struct Vertex {
        position: [f32; 4],
        tex_coords: [f32; 2],
    }

    implement_vertex!(Vertex, position, tex_coords); // don't forget to add `tex_coords` here

    pub(crate) fn whole_screen_vertex_buffer(display: &impl Facade) -> VertexBuffer<Vertex> {
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

    pub(crate) const VERTEX_SHADER: &str = r#"
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

    pub(crate) const FRAGMENT_SHADER: &str = r#"
#version 140

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D tex;

void main() {
    float y = 1.0 - v_tex_coords.y;
    color = texture(tex, vec2(v_tex_coords.x, y));
}
"#;
}
