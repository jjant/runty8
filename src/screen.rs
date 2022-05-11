use crate::editor::Editor;
use crate::graphics::{whole_screen_vertex_buffer, FRAGMENT_SHADER, VERTEX_SHADER};
use crate::runtime::draw_context::{DrawContext, DrawData};
use crate::runtime::map::Map;
use crate::runtime::sprite_sheet::SpriteSheet;
use crate::runtime::state::{Flags, InternalState, Scene};
use crate::ui::DispatchEvent;
use crate::State;

use crate::{Event, MouseButton, MouseEvent};
use glium::glutin::dpi::{LogicalPosition, LogicalSize};
use glium::glutin::event::{self, ElementState, KeyboardInput, VirtualKeyCode};
use glium::glutin::event_loop::ControlFlow;
use glium::texture::{RawImage2d, SrgbTexture2d};
use glium::uniform;
use glium::uniforms::{MagnifySamplerFilter, Sampler};
use glium::{glutin, Surface};

pub fn run_app(
    assets_path: &'static str,
    mut map: Map,
    mut sprite_flags: Flags,
    mut sprite_sheet: SpriteSheet,
    mut data: DrawData,
) {
    let mut app = Editor::init();
    let mut internal_state = InternalState::new();

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

    let mut keys = Keys::new();

    let fps = 30_u64;
    let nanoseconds_per_frame = 1_000_000_000 / fps;

    event_loop.run(move |event, _, control_flow| {
        let event: Option<Event> = handle_event(
            event,
            scale_factor,
            logical_size,
            control_flow,
            &mut internal_state,
            &mut keys,
        );

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(nanoseconds_per_frame);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        let mut target = display.draw();
        target.clear_color(1.0, 0.0, 0.0, 1.0);

        // let mut state = State::new(assets_path, &mut sprite_sheet, &mut sprite_flags, &mut map);
        let mut msg_queue = vec![];
        {
            internal_state.update_keys(&keys);

            match internal_state.scene {
                Scene::Editor => {}
                Scene::App => {
                    let mut view = app.view(&sprite_flags, &map, &sprite_sheet);

                    let dispatch_event = &mut DispatchEvent::new(&mut msg_queue);
                    if let Some(event) = event {
                        view.as_widget_mut().on_event(
                            event,
                            (internal_state.mouse_x, internal_state.mouse_y),
                            dispatch_event,
                        );
                    }

                    let mut state = State::new(
                        &internal_state,
                        assets_path,
                        &mut sprite_sheet,
                        &mut sprite_flags,
                        &mut map,
                    );
                    let mut draw_context = DrawContext::new(&mut state, &mut data);
                    view.as_widget().draw(&mut draw_context);
                    drop(view);

                    for msg in msg_queue.into_iter() {
                        app.update(&mut sprite_flags, &msg);
                    }
                }
            }
            if internal_state.escape.btnp() {
                internal_state.scene.flip();
            }

            keys.reset();
        }

        let image = RawImage2d::from_raw_rgb(data.buffer.to_vec(), (128, 128));
        let texture = SrgbTexture2d::new(&display, image).unwrap();
        let uniforms = uniform! {
            tex: Sampler::new(&texture).magnify_filter(MagnifySamplerFilter::Nearest)
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

fn handle_event(
    event: event::Event<()>,
    hidpi_factor: f64,
    window_size: LogicalSize<f64>,
    control_flow: &mut ControlFlow,
    state: &mut InternalState,
    keys: &mut Keys,
) -> Option<Event> {
    match event {
        event::Event::WindowEvent { event, .. } => match event {
            glutin::event::WindowEvent::CloseRequested => {
                *control_flow = glutin::event_loop::ControlFlow::Exit;

                None
            }
            // TODO: Handle resize events.
            glutin::event::WindowEvent::CursorMoved { position, .. } => {
                let logical_mouse: LogicalPosition<f64> = position.to_logical(hidpi_factor);

                state.mouse_x = (logical_mouse.x / window_size.width * 128.).floor() as i32;
                state.mouse_y = (logical_mouse.y / window_size.height * 128.).floor() as i32;

                Some(Event::Mouse(MouseEvent::Move {
                    x: state.mouse_x,
                    y: state.mouse_y,
                }))
            }
            glutin::event::WindowEvent::MouseInput {
                button: event::MouseButton::Left,
                state: input_state,
                ..
            } => {
                keys.mouse = Some(input_state == ElementState::Pressed);

                let mouse_event = match input_state {
                    ElementState::Pressed => MouseEvent::Down(MouseButton::Left),
                    ElementState::Released => MouseEvent::Up(MouseButton::Left),
                };

                Some(Event::Mouse(mouse_event))
            }
            glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                handle_key(input, keys);
                None
            }
            _ => None,
        },
        event::Event::NewEvents(cause) => match cause {
            glutin::event::StartCause::ResumeTimeReached { .. } => None, // todo!(),
            glutin::event::StartCause::Init => None,                     // todo!(),
            _ => None,
        },
        _ => None,
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
    pub(crate) fn new() -> Self {
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

    pub(crate) fn reset(&mut self) {
        *self = Self::new()
    }
}
