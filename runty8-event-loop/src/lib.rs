use glow::HasContext;
use runty8_runtime::{App, Key, KeyState, KeyboardEvent, Pico8, Resources};
use winit::{
    dpi::{LogicalPosition, LogicalSize},
    event::{ElementState, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod gl;

/// Keyboard events (key up, key down).
#[derive(Clone, Copy, Debug)]
pub enum Event {
    Mouse(MouseEvent),
    Keyboard(KeyboardEvent),
    Tick { delta_millis: f64 },
}

impl Event {
    /// Translates a winit::event::Event into a runty8 Event.
    fn translate_event(
        winit_event: &winit::event::Event<()>,
        hidpi_factor: f64,
        window_size: &mut LogicalSize<f64>,
        control_flow: &mut ControlFlow,
    ) -> Option<Event> {
        match winit_event {
            winit::event::Event::RedrawRequested(_) => Some(Self::Tick {
                delta_millis: 16.6666,
            }),
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;

                    None
                }
                // TODO: Force aspect ratio on resize.
                &winit::event::WindowEvent::Resized(new_size) => {
                    let new_size: LogicalSize<f64> = new_size.to_logical(hidpi_factor);

                    *window_size = new_size;

                    None
                }
                winit::event::WindowEvent::CursorMoved { position, .. } => {
                    let logical_mouse: LogicalPosition<f64> = position.to_logical(hidpi_factor);

                    Some(Event::Mouse(MouseEvent::Move {
                        x: (logical_mouse.x / window_size.width * 128.).floor() as i32,
                        y: (logical_mouse.y / window_size.height * 128.).floor() as i32,
                    }))
                }
                winit::event::WindowEvent::MouseInput {
                    button: winit::event::MouseButton::Left,
                    state: input_state,
                    ..
                } => {
                    let mouse_event = match input_state {
                        ElementState::Pressed => MouseEvent::Down(MouseButton::Left),
                        ElementState::Released => MouseEvent::Up(MouseButton::Left),
                    };

                    Some(Event::Mouse(mouse_event))
                }
                winit::event::WindowEvent::KeyboardInput { input, .. } => {
                    KeyboardEvent::from_winit(*input).map(Event::Keyboard)
                }
                _ => None,
            },
            winit::event::Event::NewEvents(cause) => match cause {
                winit::event::StartCause::ResumeTimeReached {
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
                winit::event::StartCause::Init => {
                    set_next_timer(control_flow);

                    None
                }
                winit::event::StartCause::Poll => Some(Event::Tick {
                    delta_millis: 16.6666,
                }),
                _ => None,
            },
            _ => None,
        }
    }
}
///
/// Mouse buttons.
#[derive(Clone, Copy, Debug)]
pub enum MouseButton {
    // TODO: Handle other mouse buttons?
    Left,
    Right,
    Middle,
}

/// Mouse events (mouse move, button presses).
#[derive(Clone, Copy, Debug)]
pub enum MouseEvent {
    /// Mouse move event.
    // Contains the current position of the mouse.
    Move {
        ///
        x: i32,
        ///
        y: i32,
    },
    // TODO: Refactor these two below to factor out the MouseButton
    /// Mouse button pressed.
    Down(MouseButton),
    /// Mouse button released.
    Up(MouseButton),
}

pub unsafe fn event_loop<Game: App + 'static>(resources: Resources) {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(640.0, 640.0))
        .with_title("My pico8 game");
    #[cfg(target_arch = "wasm32")]
    let window = window_builder.build(&event_loop).unwrap();
    #[cfg(not(target_arch = "wasm32"))]
    let (gl, shader_version, window, event_loop) = {
        let event_loop = glutin::event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_inner_size(LogicalSize::new(640.0, 640.0))
            .with_title("My pico8 game");
        let window: glutin::ContextWrapper<_, _> = glutin::ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(window_builder, &event_loop)
            .unwrap()
            .make_current()
            .unwrap();
        let gl = glow::Context::from_loader_function(|s| {
            glutin::ContextWrapper::get_proc_address(&window, s) as *const _
        });

        (gl, "#version 410", window, event_loop)
    };

    #[cfg(target_arch = "wasm32")]
    let (gl, shader_version) = wasm::insert_canvas_and_create_context(&window);

    // gl.debug_message_callback(|a, b, c, d, message| {
    //     println!("{}", message);
    //     println!("{} {} {} {}", a, b, c, d);
    // });

    let scale_factor = 1.0; // TODO
    let mut logical_size: LogicalSize<f64> = {
        #[cfg(not(target_arch = "wasm32"))]
        {
            window.window()
        }
        #[cfg(target_arch = "wasm32")]
        {
            window
        }
    }
    .inner_size()
    .to_logical(scale_factor);

    gl.clear_color(0.1, 0.2, 0.3, 1.0);
    let program = gl::make_program(&gl, shader_version);
    gl.use_program(Some(program));
    let texture = gl::make_texture(&gl);
    gl::use_texture(&gl, program);

    let mut pico8 = Pico8::new(resources);
    let mut game = Game::init(&mut pico8);
    gl::upload_pixels(&gl, texture, pico8.draw_data.buffer());

    #[cfg(not(target_arch = "wasm32"))]
    {
        event_loop.run(move |event, _, control_flow| {
            println!("{:?}", event);
            // *control_flow = ControlFlow::Wait;
            // let event = crate::Event::translate_event(
            //     &event,
            //     scale_factor,
            //     &mut logical_size,
            //     control_flow,
            // );
            // if let Some(event) = event {
            //     match event {
            //         crate::Event::Tick { .. } => {
            //             game.update(&mut pico8);
            //             game.draw(&mut pico8);
            //         }
            //         _ => {}
            //     }
            // };
            //
            // if let Some(_new_title) = pico8.take_new_title() {
            //     // display.gl_window().window().set_title(&new_title);
            // }

            match event {
                winit::event::Event::LoopDestroyed => {
                    return;
                }
                winit::event::Event::MainEventsCleared => {
                    window.window().request_redraw();
                }
                winit::event::Event::RedrawRequested(_) => {
                    // gl.clear(glow::COLOR_BUFFER_BIT);
                    // gl.draw_arrays(glow::TRIANGLES, 0, 3);
                    // window.swap_buffers().unwrap();
                    // // Actually draw stuff!
                    game.update(&mut pico8);
                    game.draw(&mut pico8);
                    draw_glutin(&window, &gl, texture, &pico8);
                }
                winit::event::Event::WindowEvent { ref event, .. } => match event {
                    winit::event::WindowEvent::Resized(physical_size) => {
                        window.resize(*physical_size);
                    }
                    winit::event::WindowEvent::CloseRequested => {
                        gl.delete_program(program);
                        // gl.delete_vertex_array(vertex_array);
                        *control_flow = ControlFlow::Exit
                    }
                    _ => (),
                },
                _ => (),
            }
        });
    }
    #[cfg(target_arch = "wasm32")]
    event_loop.run(move |winit_event, _, control_flow| {
        log::debug!("{:?}", winit_event);

        let event: Option<Event> =
            Event::translate_event(&winit_event, scale_factor, &mut logical_size, control_flow);

        if let Some(event) = event {
            match event {
                Event::Tick { .. } => {
                    game.update(&mut pico8);
                    game.draw(&mut pico8);
                }
                _ => {}
            }
        };

        if let Some(_new_title) = pico8.take_new_title() {
            // display.gl_window().window().set_title(&new_title);
        }

        // Actually draw stuff!
        draw(&gl, texture, &pico8);
    });
}

#[cfg(not(target_arch = "wasm32"))]
unsafe fn draw_glutin<B>(
    window: &glutin::ContextWrapper<glutin::PossiblyCurrent, B>,
    gl: &glow::Context,
    texture: glow::Texture,
    pico8: &Pico8,
) {
    draw(gl, texture, pico8);
    window.swap_buffers().unwrap();
}

unsafe fn draw(gl: &glow::Context, texture: glow::Texture, pico8: &Pico8) {
    gl::upload_pixels(gl, texture, pico8.draw_data.buffer());
    gl.clear(glow::COLOR_BUFFER_BIT);
    gl.draw_arrays(glow::TRIANGLES, 0, 6);
}

// fn create_gl_context_and_window(window: &winit::window::Window) -> glow::Context {
//     #[cfg(target_arch = "wasm32")]
//     {
//         let a = 42;
//         return wasm::insert_canvas_and_create_context(window);
//     }
//
//     // Glutin
//     {
//         unsafe { glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _) }
//     }
// }

#[cfg(target_arch = "wasm32")]
mod wasm {
    use winit::window::Window;

    pub(super) fn insert_canvas_and_create_context(
        window: &Window,
    ) -> (glow::Context, &'static str) {
        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowExtWebSys;

        let canvas = window.canvas();

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        canvas.style().set_css_text("border: 1px solid blue;");
        body.append_child(&canvas).unwrap();

        let webgl2_context = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::WebGl2RenderingContext>()
            .unwrap();
        let gl = glow::Context::from_webgl2_context(webgl2_context);

        (gl, "#version 300 es")
    }
}
//// Extension stuff for `winit` types

trait Runty8KeyboardEventExt: Sized {
    fn from_winit(input: winit::event::KeyboardInput) -> Option<Self>;
}

impl Runty8KeyboardEventExt for KeyboardEvent {
    fn from_winit(input: winit::event::KeyboardInput) -> Option<KeyboardEvent> {
        let key = input.virtual_keycode?;
        let runty8_key = Key::from_virtual_keycode(key)?;
        let state = KeyState::from_state(input.state);

        Some(KeyboardEvent {
            key: runty8_key,
            state,
        })
    }
}
trait Runty8KeyExt: Sized {
    fn from_virtual_keycode(key: VirtualKeyCode) -> Option<Self>;
}

impl Runty8KeyExt for Key {
    fn from_virtual_keycode(key: VirtualKeyCode) -> Option<Self> {
        match key {
            VirtualKeyCode::A => Some(Self::A),
            VirtualKeyCode::B => Some(Self::B),
            VirtualKeyCode::C => Some(Self::C),
            VirtualKeyCode::D => Some(Self::D),
            VirtualKeyCode::E => Some(Self::E),
            VirtualKeyCode::F => Some(Self::F),
            VirtualKeyCode::G => Some(Self::G),
            VirtualKeyCode::H => Some(Self::H),
            VirtualKeyCode::I => Some(Self::I),
            VirtualKeyCode::J => Some(Self::J),
            VirtualKeyCode::K => Some(Self::K),
            VirtualKeyCode::L => Some(Self::L),
            VirtualKeyCode::M => Some(Self::M),
            VirtualKeyCode::N => Some(Self::N),
            VirtualKeyCode::O => Some(Self::O),
            VirtualKeyCode::P => Some(Self::P),
            VirtualKeyCode::Q => Some(Self::Q),
            VirtualKeyCode::R => Some(Self::R),
            VirtualKeyCode::S => Some(Self::S),
            VirtualKeyCode::T => Some(Self::T),
            VirtualKeyCode::U => Some(Self::U),
            VirtualKeyCode::V => Some(Self::V),
            VirtualKeyCode::W => Some(Self::W),
            VirtualKeyCode::X => Some(Self::X),
            VirtualKeyCode::Y => Some(Self::Y),
            VirtualKeyCode::Z => Some(Self::Z),
            VirtualKeyCode::LControl => Some(Self::Control),
            VirtualKeyCode::Left => Some(Self::LeftArrow),
            VirtualKeyCode::Right => Some(Self::RightArrow),
            VirtualKeyCode::Up => Some(Self::UpArrow),
            VirtualKeyCode::Down => Some(Self::DownArrow),
            VirtualKeyCode::Escape => Some(Self::Escape),
            VirtualKeyCode::LAlt => Some(Self::Alt),
            VirtualKeyCode::Space => Some(Self::Space),
            _ => None,
        }
    }
}

trait Runty8KeyStateExt: Sized {
    fn from_state(state: ElementState) -> Self;
}

impl Runty8KeyStateExt for KeyState {
    fn from_state(state: ElementState) -> Self {
        match state {
            ElementState::Pressed => Self::Down,
            ElementState::Released => Self::Up,
        }
    }
}

////// Utils
fn set_next_timer(control_flow: &mut ControlFlow) {
    let fps = 30_u64;
    let nanoseconds_per_frame = 1_000_000_000 / fps;

    let next_frame_time =
        instant::Instant::now() + std::time::Duration::from_nanos(nanoseconds_per_frame);
    *control_flow = ControlFlow::WaitUntil(next_frame_time);
    // *control_flow = ControlFlow::Poll;
}
