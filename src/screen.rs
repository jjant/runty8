use crate::app::{DevApp, ElmApp};
use crate::editor::SpriteEditor;
use crate::{DrawContext, Scene, State};
use glium::backend::Facade;
use glium::glutin::dpi::{LogicalPosition, LogicalSize};
use glium::glutin::event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode};
use glium::glutin::event_loop::ControlFlow;
use glium::uniforms::MagnifySamplerFilter;
use glium::{glutin, Surface, VertexBuffer};
use glium::{implement_vertex, uniform};

pub fn do_something<T: ElmApp + 'static>(mut draw_context: DrawContext) {
    let mut app = T::init();

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_inner_size(LogicalSize::new(640.0, 640.0));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    {
        display.gl_window().window().set_cursor_visible(false);
    }
    let scale_factor = display.gl_window().window().scale_factor();
    let logical_size = display
        .gl_window()
        .window()
        .inner_size()
        .to_logical(scale_factor);

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program =
        glium::Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();

    let mut editor = SpriteEditor::init();

    let mut keys = Keys::new();

    let fps = 30_u64;
    let nanoseconds_per_frame = 1_000_000_000 / 60_u64;

    event_loop.run(move |event, _, control_flow| {
        let should_return = handle_event(event, scale_factor, logical_size, control_flow, &mut draw_context.state, &mut keys);

        if let ShouldReturn::Yes = should_return {
            return;
        }

        let next_frame_time = std::time::Instant::now()
        + std::time::Duration::from_nanos(nanoseconds_per_frame);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        let mut target = display.draw();
        target.clear_color(1.0, 0.0, 0.0, 1.0);



        {
            draw_context.state.update_keys(&keys);
            match draw_context.state.scene {
                Scene::Editor => {
                    editor.draw(&mut draw_context);
                    editor.update(&mut draw_context.state);
                },
                Scene::App => {
                    let actions = app.draw(&mut draw_context);
                    app.update(&draw_context.state, &actions);
                }
            }
            if draw_context.state.escape.btnp()  {
                draw_context.state.scene.flip();
            }

            keys.reset();
        }

        let image = glium::texture::RawImage2d::from_raw_rgb(draw_context.buffer.to_vec(), (128, 128));
        let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();
        let uniforms = uniform! {
            tex: glium::uniforms::Sampler::new(&texture).magnify_filter(MagnifySamplerFilter::Nearest)
        };

        target
            .draw(
                &whole_screen_vertex_buffer(&display),
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

// TODO: (IMPORTANT)
// Apparently draw() stops working if you move the mouse outside the window???
fn handle_event(
    event: Event<()>,
    hidpi_factor: f64,
    window_size: LogicalSize<f64>,
    control_flow: &mut ControlFlow,
    state: &mut State,
    keys: &mut Keys,
) -> ShouldReturn {
    match event {
        Event::WindowEvent { event, .. } => match event {
            glutin::event::WindowEvent::CloseRequested => {
                *control_flow = glutin::event_loop::ControlFlow::Exit;

                return ShouldReturn::Yes;
            }
            // TODO: Handle resize events.
            glutin::event::WindowEvent::CursorMoved { position, .. } => {
                let logical_mouse: LogicalPosition<f64> = position.to_logical(hidpi_factor);

                state.mouse_x = (logical_mouse.x / window_size.width * 128.).floor() as i32;
                state.mouse_y = (logical_mouse.y / window_size.height * 128.).floor() as i32;

                return ShouldReturn::Yes;
            }
            glutin::event::WindowEvent::MouseInput {
                button: MouseButton::Left,
                state: input_state,
                ..
            } => {
                keys.mouse = Some(input_state == ElementState::Pressed);

                return ShouldReturn::Yes;
            }
            glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                handle_key(input, keys);
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

fn handle_key(input: KeyboardInput, keys: &mut Keys) {
    if let Some(key) = input.virtual_keycode {
        let key_ref = match key {
            VirtualKeyCode::X => &mut keys.x,
            VirtualKeyCode::C => &mut keys.c,
            VirtualKeyCode::Left => &mut keys.left,
            VirtualKeyCode::Up => &mut keys.up,
            VirtualKeyCode::Right => &mut keys.right,
            VirtualKeyCode::Down => &mut keys.down,
            VirtualKeyCode::Escape => &mut keys.escape,

            _ => return,
        };

        *key_ref = Some(input.state == ElementState::Pressed);
    }
}

// Rendering boilerplate

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 4],
    tex_coords: [f32; 2], // <- this is new
}

implement_vertex!(Vertex, position, tex_coords); // don't forget to add `tex_coords` here

fn whole_screen_vertex_buffer(display: &impl Facade) -> VertexBuffer<Vertex> {
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

pub(crate) struct Keys {
    pub(crate) left: Option<bool>,
    pub(crate) right: Option<bool>,
    pub(crate) up: Option<bool>,
    pub(crate) down: Option<bool>,
    pub(crate) x: Option<bool>,
    pub(crate) c: Option<bool>,
    pub(crate) escape: Option<bool>,
    pub(crate) mouse: Option<bool>,
}

impl Keys {
    fn new() -> Self {
        Self {
            left: None,
            right: None,
            up: None,
            down: None,
            x: None,
            c: None,
            escape: None,
            mouse: None,
        }
    }

    fn reset(&mut self) {
        *self = Self::new()
    }
}
