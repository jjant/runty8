use crate::editor::Editor;
use crate::graphics::{whole_screen_vertex_buffer, FRAGMENT_SHADER, VERTEX_SHADER};
use crate::runtime::draw_context::{DrawContext, DrawData};
use crate::runtime::flags::Flags;
use crate::runtime::map::Map;
use crate::runtime::sprite_sheet::SpriteSheet;
use crate::runtime::state::{InternalState, Scene};
use crate::ui::DispatchEvent;
use crate::{App, Key, KeyboardEvent, State};

use crate::{Event, MouseButton, MouseEvent};
use glium::glutin::dpi::{LogicalPosition, LogicalSize};
use glium::glutin::event::{self, ElementState, KeyboardInput, VirtualKeyCode};
use glium::glutin::event_loop::ControlFlow;
use glium::texture::{RawImage2d, SrgbTexture2d};
use glium::uniform;
use glium::uniforms::{MagnifySamplerFilter, Sampler};
use glium::{glutin, Surface};

pub(crate) fn run_app<T: App + 'static>(
    assets_path: String,
    map: Map,
    sprite_flags: Flags,
    sprite_sheet: SpriteSheet,
    mut draw_data: DrawData,
) {
    let mut app = T::init();
    let mut editor = Editor::init();
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

    let fps = 60_u64;
    let nanoseconds_per_frame = 1_000_000_000 / fps;

    let mut resources = Resources {
        assets_path,
        sprite_flags,
        sprite_sheet,
        map,
    };
    event_loop.run(move |glutin_event, _, control_flow| {
        let event: Option<Event> = handle_event(
            &glutin_event,
            scale_factor,
            logical_size,
            control_flow,
            &mut internal_state,
            &mut keys,
        );

        let should_update_pico8_app = matches!(
            glutin_event,
            event::Event::NewEvents(glutin::event::StartCause::ResumeTimeReached { .. })
                | event::Event::NewEvents(glutin::event::StartCause::Init { .. })
        );
        let should_update = (should_update_pico8_app && matches!(internal_state.scene, Scene::App))
            || matches!(internal_state.scene, Scene::Editor);

        if !should_update {
            return;
        }

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(nanoseconds_per_frame);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        let mut target = display.draw();
        target.clear_color(1.0, 0.0, 0.0, 1.0);

        internal_state.update_keys(&keys);

        update_app(
            &mut app,
            &mut editor,
            &internal_state,
            event,
            &mut resources,
            &mut draw_data,
        );

        if internal_state.escape.btnp() {
            internal_state.scene.flip();
        }
        keys.reset();

        let image = RawImage2d::from_raw_rgb(draw_data.buffer.to_vec(), (128, 128));
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
    event: &event::Event<()>,
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
                keys.mouse = Some(input_state == &ElementState::Pressed);

                let mouse_event = match input_state {
                    ElementState::Pressed => MouseEvent::Down(MouseButton::Left),
                    ElementState::Released => MouseEvent::Up(MouseButton::Left),
                };

                Some(Event::Mouse(mouse_event))
            }
            glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                handle_key(input, keys).map(Event::Keyboard)
            }
            _ => None,
        },
        event::Event::NewEvents(cause) => match cause {
            glutin::event::StartCause::ResumeTimeReached { .. } => {
                // Tick
                None
            } // todo!(),
            glutin::event::StartCause::Init => None, // todo!(),
            _ => None,
        },
        _ => None,
    }
}

fn handle_key(input: &KeyboardInput, keys: &mut Keys) -> Option<KeyboardEvent> {
    let key = input.virtual_keycode?;

    let mut other = None;
    let key_ref = match key {
        VirtualKeyCode::X => &mut keys.x,
        VirtualKeyCode::C => &mut keys.c,
        VirtualKeyCode::Left => &mut keys.left,
        VirtualKeyCode::Up => &mut keys.up,
        VirtualKeyCode::Right => &mut keys.right,
        VirtualKeyCode::Down => &mut keys.down,
        VirtualKeyCode::Escape => &mut keys.escape,
        _ => &mut other,
    };
    *key_ref = Some(input.state == ElementState::Pressed);

    let runty8_key = Key::from_virtual_keycode(key)?;

    Some(match input.state {
        ElementState::Pressed => KeyboardEvent::Down(runty8_key),
        ElementState::Released => KeyboardEvent::Up(runty8_key),
    })
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

pub(crate) struct Resources {
    pub(crate) assets_path: String,
    pub(crate) sprite_sheet: SpriteSheet,
    pub(crate) sprite_flags: Flags,
    pub(crate) map: Map,
}

fn update_app<'a>(
    app: &'a mut (impl App + 'static),
    editor: &'a mut Editor,
    internal_state: &'a InternalState,
    event: Option<Event>,
    resources: &'a mut Resources,
    draw_data: &'a mut DrawData,
) {
    match internal_state.scene {
        // TODO: App is refreshed too much (check celeste)
        Scene::App => {
            let mut state = State::new(internal_state, resources);
            let mut draw_context = DrawContext::new(&mut state, draw_data);
            app.draw(&mut draw_context);
            app.update(&state);
        }
        Scene::Editor => {
            let mut view = editor.view(
                &resources.sprite_flags,
                &resources.map,
                &resources.sprite_sheet,
            );

            let mut msg_queue = vec![];
            let dispatch_event = &mut DispatchEvent::new(&mut msg_queue);

            if let Some(event) = event {
                view.as_widget_mut().on_event(
                    event,
                    (internal_state.mouse_x, internal_state.mouse_y),
                    dispatch_event,
                );
            }

            let mut state = State::new(internal_state, resources);
            let mut draw_context = DrawContext::new(&mut state, draw_data);
            view.as_widget().draw(&mut draw_context);
            drop(view);

            if let Some(sub_msg) = event.and_then(|e| editor.subscriptions(&e)) {
                msg_queue.push(sub_msg);
            }
            for msg in msg_queue.into_iter() {
                editor.update(
                    &resources.assets_path,
                    &mut resources.sprite_flags,
                    &mut resources.sprite_sheet,
                    &mut resources.map,
                    &msg,
                );
            }
        }
    }
}
