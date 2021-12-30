use crate::app::App;
use crate::{DrawContext, State};
use glium::glutin::dpi::LogicalSize;
use glium::glutin::event::{ElementState, Event, KeyboardInput, VirtualKeyCode};
use glium::glutin::event_loop::ControlFlow;
use glium::uniforms::MagnifySamplerFilter;
use glium::{glutin, Surface};
use glium::{implement_vertex, uniform};

const VERTEX_SHADER: &str = r#"
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

const FRAGMENT_SHADER: &str = r#"
#version 140

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D tex;

void main() {
    float y = 1.0 - v_tex_coords.y;
    color = texture(tex, vec2(v_tex_coords.x, y));
}
"#;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 4],
    tex_coords: [f32; 2], // <- this is new
}

implement_vertex!(Vertex, position, tex_coords); // don't forget to add `tex_coords` here

pub fn do_something<T: App + 'static>(mut state: State, mut draw_context: DrawContext) {
    let mut app = T::init();

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_inner_size(LogicalSize::new(600.0, 600.0));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    // let scale_factor = display.gl_window().window().scale_factor();

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

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program =
        glium::Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();

    let mut t = -0.5;

    event_loop.run(move |event, _, control_flow| {

        let should_return = handle_event(event, control_flow, &mut state);

        if let ShouldReturn::Yes = should_return {
            return;
        }

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        // we update `t`
        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        let mut target = display.draw();
        target.clear_color(1.0, 0.0, 0.0, 1.0);



        {
            app.draw(&mut draw_context);
            app.update(&state);
        }

        let image = glium::texture::RawImage2d::from_raw_rgb(draw_context.buffer.to_vec(), (128, 128));
        let texture = glium::texture::Texture2d::new(&display, image).unwrap();
        let uniforms = uniform! {
            tex: glium::uniforms::Sampler::new(&texture).magnify_filter(MagnifySamplerFilter::Nearest)
        };

        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    });
}

enum ShouldReturn {
    Yes,
    No,
}

fn handle_event(
    event: Event<()>,
    control_flow: &mut ControlFlow,
    state: &mut State,
) -> ShouldReturn {
    let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);

    *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

    match event {
        Event::WindowEvent { event, .. } => match event {
            glutin::event::WindowEvent::CloseRequested => {
                *control_flow = glutin::event_loop::ControlFlow::Exit;

                return ShouldReturn::Yes;
            }
            // glutin::event::WindowEvent::CursorMoved {position,..} => {
            //     on_mouse_move(position.to_logical(scale_factor), &mut data);
            //     println!("{:?}", position);
            // },
            glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                handle_key(input, state);
                return ShouldReturn::Yes;
            }
            _ => return ShouldReturn::Yes,
        },
        Event::NewEvents(cause) => match cause {
            glutin::event::StartCause::ResumeTimeReached { .. } => return ShouldReturn::No,
            glutin::event::StartCause::Init => return ShouldReturn::No,
            _ => return ShouldReturn::Yes,
        },
        _ => return ShouldReturn::Yes,
    }
}

fn handle_key(input: KeyboardInput, state: &mut State) {
    if let Some(key) = input.virtual_keycode {
        let key_ref = match key {
            VirtualKeyCode::X => &mut state.x,
            VirtualKeyCode::C => &mut state.c,
            VirtualKeyCode::Left => &mut state.left,
            VirtualKeyCode::Up => &mut state.up,
            VirtualKeyCode::Right => &mut state.right,
            VirtualKeyCode::Down => &mut state.down,
            _ => return,
        };
        *key_ref = input.state == ElementState::Pressed;
    }
}
