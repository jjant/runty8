use crate::app::AppCompat;
use crate::controller::Controller;
use crate::graphics::{whole_screen_vertex_buffer, FRAGMENT_SHADER, VERTEX_SHADER};
use crate::runtime::draw_context::DrawData;
use crate::runtime::flags::Flags;
use crate::runtime::map::Map;
use crate::runtime::sprite_sheet::SpriteSheet;
use crate::{Event, KeyState, MouseButton, MouseEvent, Resources};
use crate::{Key, KeyboardEvent};
use glium::backend::Facade;
use glium::glutin::dpi::{LogicalPosition, LogicalSize};
use glium::glutin::event::{self, ElementState, KeyboardInput};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::index::NoIndices;
use glium::texture::{RawImage2d, SrgbTexture2d};
use glium::uniforms::{MagnifySamplerFilter, Sampler};
use glium::{glutin, Display, Program, Surface};
use glium::{uniform, Frame};

pub(crate) fn run_app<Game: AppCompat + 'static>(
    assets_path: String,
    map: Map,
    sprite_flags: Flags,
    sprite_sheet: SpriteSheet,
    mut draw_data: DrawData,
) {
    let mut resources = Resources {
        assets_path,
        sprite_flags,
        sprite_sheet,
        map,
    };
    let mut controller = Controller::<Game>::init(&mut resources);
    let event_loop = glutin::event_loop::EventLoop::new();
    let display = make_display(&event_loop);
    let scale_factor = display.gl_window().window().scale_factor();
    let mut logical_size = display
        .gl_window()
        .window()
        .inner_size()
        .to_logical(scale_factor);

    let (indices, program) = make_gl_program(&display);

    event_loop.run(move |glutin_event, _, control_flow| {
        let event: Option<Event> =
            translate_event(&glutin_event, scale_factor, &mut logical_size, control_flow);

        controller.step(&mut resources, &mut draw_data, event);

        do_draw(
            &display,
            display.draw(),
            &draw_data.buffer,
            &indices,
            &program,
        );
    });
}

/// Translates a glutin::event::Event into a runty8 Event.
fn translate_event(
    event: &glutin::event::Event<()>,
    hidpi_factor: f64,
    window_size: &mut LogicalSize<f64>,
    control_flow: &mut ControlFlow,
) -> Option<Event> {
    match event {
        event::Event::WindowEvent { event, .. } => match event {
            glutin::event::WindowEvent::CloseRequested => {
                *control_flow = glutin::event_loop::ControlFlow::Exit;

                None
            }
            // TODO: Force aspect ratio on resize.
            &glutin::event::WindowEvent::Resized(new_size) => {
                let new_size: LogicalSize<f64> = new_size.to_logical(hidpi_factor);

                *window_size = new_size;

                None
            }
            glutin::event::WindowEvent::CursorMoved { position, .. } => {
                let logical_mouse: LogicalPosition<f64> = position.to_logical(hidpi_factor);

                Some(Event::Mouse(MouseEvent::Move {
                    x: (logical_mouse.x / window_size.width * 128.).floor() as i32,
                    y: (logical_mouse.y / window_size.height * 128.).floor() as i32,
                }))
            }
            glutin::event::WindowEvent::MouseInput {
                button: event::MouseButton::Left,
                state: input_state,
                ..
            } => {
                let mouse_event = match input_state {
                    ElementState::Pressed => MouseEvent::Down(MouseButton::Left),
                    ElementState::Released => MouseEvent::Up(MouseButton::Left),
                };

                Some(Event::Mouse(mouse_event))
            }
            glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                handle_keyboard_event(input).map(Event::Keyboard)
            }
            _ => None,
        },
        event::Event::NewEvents(cause) => match cause {
            glutin::event::StartCause::ResumeTimeReached {
                start,
                requested_resume,
            } => {
                set_next_timer(control_flow);

                let delta: Result<i32, _> = requested_resume
                    .duration_since(*start)
                    .as_millis()
                    .try_into();

                Some(Event::Tick {
                    delta_millis: delta.unwrap().try_into().unwrap(),
                })
            }
            glutin::event::StartCause::Init => {
                set_next_timer(control_flow);

                None
            }
            _ => None,
        },
        _ => None,
    }
}

fn handle_keyboard_event(input: &KeyboardInput) -> Option<KeyboardEvent> {
    let key = input.virtual_keycode?;
    let runty8_key = Key::from_virtual_keycode(key)?;
    let state = KeyState::from_state(input.state);

    Some(KeyboardEvent {
        key: runty8_key,
        state,
    })
}

fn set_next_timer(control_flow: &mut ControlFlow) {
    let fps = 30_u64;
    let nanoseconds_per_frame = 1_000_000_000 / fps;

    let next_frame_time =
        std::time::Instant::now() + std::time::Duration::from_nanos(nanoseconds_per_frame);
    *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
}

fn do_draw(
    display: &impl Facade,
    mut target: Frame,
    buffer: &[u8],
    indices: &NoIndices,
    program: &Program,
) {
    target.clear_color(1.0, 0.0, 0.0, 1.0);
    let image = RawImage2d::from_raw_rgb(buffer.to_vec(), (128, 128));
    let texture = SrgbTexture2d::new(display, image).unwrap();
    let uniforms = uniform! {
        tex: Sampler::new(&texture).magnify_filter(MagnifySamplerFilter::Nearest)
    };
    target
        .draw(
            &whole_screen_vertex_buffer(display),
            indices,
            program,
            &uniforms,
            &Default::default(),
        )
        .unwrap();
    target.finish().unwrap();
}

fn make_display(event_loop: &EventLoop<()>) -> Display {
    let wb = glutin::window::WindowBuilder::new().with_inner_size(LogicalSize::new(640.0, 640.0));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, event_loop).unwrap();
    {
        display.gl_window().window().set_cursor_visible(false);
    }

    display
}

fn make_gl_program(display: &impl Facade) -> (NoIndices, Program) {
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let program =
        glium::Program::from_source(display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();

    (indices, program)
}
