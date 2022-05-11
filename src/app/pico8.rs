// use crate::editor::SpriteEditor;
use crate::graphics::{whole_screen_vertex_buffer, FRAGMENT_SHADER, VERTEX_SHADER};
use crate::runtime::draw_context::{DrawContext, DrawData};
use crate::runtime::map::Map;
use crate::runtime::sprite_sheet::SpriteSheet;
use crate::runtime::state::{Flags, Scene};
use crate::screen::Keys;
use crate::{App, State};
use glium::glutin::dpi::{LogicalPosition, LogicalSize};
use glium::glutin::event::{ElementState, Event, KeyboardInput, VirtualKeyCode};
use glium::glutin::event_loop::ControlFlow;
use glium::texture::{RawImage2d, SrgbTexture2d};
use glium::uniform;
use glium::uniforms::{MagnifySamplerFilter, Sampler};
use glium::{glutin, Surface};

pub fn run_app<T: App + 'static>() {
    let mut app = T::init();
    let map: &'static Map = Box::leak(Box::new(Map::new()));
    let sprite_flags: &'static Flags = Box::leak(Box::new(Flags::new()));
    let sprite_sheet =
        SpriteSheet::deserialize(&std::fs::read_to_string("./sprite_sheet.txt").unwrap()).unwrap();

    let mut state = State::new(".", sprite_sheet, sprite_flags, map);
    let mut draw_data = DrawData::new();

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

    // let mut editor = SpriteEditor::init();

    let mut keys = Keys::new();

    let fps = 60_u64;
    let nanoseconds_per_frame = 1_000_000_000 / fps;

    event_loop.run(move |event, _, control_flow| {
        let should_return = handle_event(
            event,
            scale_factor,
            logical_size,
            control_flow,
            &mut state,
            &mut keys,
        );

        if let ShouldReturn::Yes = should_return {
            return;
        }

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(nanoseconds_per_frame);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        let mut target = display.draw();
        target.clear_color(1.0, 0.0, 0.0, 1.0);

        {
            state.update_keys(&keys);

            match state.scene {
                Scene::Editor => {
                    // let mut draw_context = DrawContext::new(&mut state, &mut draw_data);
                    // editor.draw(&mut draw_context);
                    // editor.update(&mut state);
                }
                Scene::App => {
                    let mut draw_context = DrawContext::new(&mut state, &mut draw_data);
                    app.draw(&mut draw_context);
                    app.update(&state);
                }
            }
            if state.escape.btnp() {
                state.scene.flip();
            }

            keys.reset();
        }

        let image = RawImage2d::from_raw_rgb(draw_data.buffer.to_vec(), (128, 128));
        let texture = SrgbTexture2d::new(&display, image).unwrap();

        let uniform = uniform! {
            tex: Sampler::new(&texture).magnify_filter(MagnifySamplerFilter::Nearest)
        };
        target
            .draw(
                &whole_screen_vertex_buffer(&display),
                &indices,
                &program,
                &uniform,
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

                ShouldReturn::Yes
            }
            // TODO: Handle resize events.
            glutin::event::WindowEvent::CursorMoved { position, .. } => {
                let logical_mouse: LogicalPosition<f64> = position.to_logical(hidpi_factor);

                state.mouse_x = (logical_mouse.x / window_size.width * 128.).floor() as i32;
                state.mouse_y = (logical_mouse.y / window_size.height * 128.).floor() as i32;

                ShouldReturn::Yes
            }
            glutin::event::WindowEvent::MouseInput {
                button: glutin::event::MouseButton::Left,
                state: input_state,
                ..
            } => {
                keys.mouse = Some(input_state == ElementState::Pressed);

                ShouldReturn::Yes
            }
            glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                handle_key(input, keys);
                ShouldReturn::Yes
            }
            _ => ShouldReturn::Yes,
        },
        Event::NewEvents(cause) => match cause {
            glutin::event::StartCause::ResumeTimeReached { .. } => ShouldReturn::No,
            glutin::event::StartCause::Init => ShouldReturn::No,
            _ => ShouldReturn::Yes,
        },
        _ => ShouldReturn::Yes,
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
