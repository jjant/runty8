mod event;
mod graphics;

use event::{Event, MouseButton, MouseEvent};
use glium::backend::Facade;
use glium::glutin::dpi::{LogicalPosition, LogicalSize};
use glium::glutin::event::{self as glutin_event, ElementState, KeyboardInput};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::index::NoIndices;
use glium::texture::{RawImage2d, SrgbTexture2d};
use glium::uniforms::{MagnifySamplerFilter, Sampler};
use glium::{glutin, Display, Program, Surface};
use glium::{uniform, Frame};
use graphics::{whole_screen_vertex_buffer, FRAGMENT_SHADER, VERTEX_SHADER};
use runty8_runtime::{App, Key, KeyState, KeyboardEvent, Pico8, Resources};

pub fn run_app<Game: App + 'static>(resources: Resources) {
    let event_loop = glutin::event_loop::EventLoop::new();
    let display = make_display(&event_loop, "Runty8");
    let scale_factor = display.gl_window().window().scale_factor();
    let mut logical_size = display
        .gl_window()
        .window()
        .inner_size()
        .to_logical(scale_factor);

    let (indices, program) = make_gl_program(&display);

    let mut pico8 = Pico8::new(resources);
    let mut game = Game::init(&mut pico8);
    event_loop.run(move |glutin_event, _, control_flow| {
        let event: Option<Event> =
            translate_event(&glutin_event, scale_factor, &mut logical_size, control_flow);

        if let Some(event) = event {
            match event {
                // game.step(event);
                Event::Tick { .. } => {
                    game.update(&mut pico8);
                    game.draw(&mut pico8);
                }
                _ => {}
            };
        };

        if let Some(new_title) = pico8.take_new_title() {
            display.gl_window().window().set_title(&new_title);
        }

        do_draw(
            &display,
            display.draw(),
            pico8.draw_data.buffer(),
            &indices,
            &program,
        );
    });
}

/// Translates a glutin::event::Event into a runty8 Event.
fn translate_event(
    event: &glutin_event::Event<()>,
    hidpi_factor: f64,
    window_size: &mut LogicalSize<f64>,
    control_flow: &mut ControlFlow,
) -> Option<Event> {
    match event {
        glutin_event::Event::WindowEvent { event, .. } => match event {
            glutin_event::WindowEvent::CloseRequested => {
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
                button: glutin_event::MouseButton::Left,
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
        glutin_event::Event::NewEvents(cause) => match cause {
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

fn make_gl_program(display: &impl Facade) -> (NoIndices, Program) {
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let program =
        glium::Program::from_source(display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();

    (indices, program)
}
