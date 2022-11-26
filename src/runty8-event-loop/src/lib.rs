use glow::HasContext;
use runty8_core::{App, Event, Keys, MouseButton, MouseEvent, Pico8, Resources};
use runty8_winit::Runty8EventExt;
use std::sync::{Arc, Mutex};
use winit::{
    dpi::LogicalSize,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
mod gl;

// impl Event {
//     /// Translates a winit::event::Event into a runty8 Event.
//     fn translate_event(
//         winit_event: &winit::event::Event<()>,
//         hidpi_factor: f64,
//         window_size: &mut LogicalSize<f64>,
//         control_flow: &mut ControlFlow,
//     ) -> Option<Event> {
//         match winit_event {
//             winit::event::Event::RedrawRequested(_) =>
//             /* Some(Self::Tick {
//                 delta_millis: 16.6666,
//             }) */
//             {
//                 None
//             }
//             winit::event::Event::WindowEvent { event, .. } => match event {
//                 winit::event::WindowEvent::CloseRequested => {
//                     *control_flow = ControlFlow::Exit;
//
//                     None
//                 }
//                 // TODO: Force aspect ratio on resize.
//                 &winit::event::WindowEvent::Resized(new_size) => {
//                     let new_size: LogicalSize<f64> = new_size.to_logical(hidpi_factor);
//
//                     *window_size = new_size;
//
//                     None
//                 }
//                 winit::event::WindowEvent::CursorMoved { position, .. } => {
//                     let logical_mouse: LogicalPosition<f64> = position.to_logical(hidpi_factor);
//
//                     Some(Event::Mouse(MouseEvent::Move {
//                         x: (logical_mouse.x / window_size.width * 128.).floor() as i32,
//                         y: (logical_mouse.y / window_size.height * 128.).floor() as i32,
//                     }))
//                 }
//                 winit::event::WindowEvent::MouseInput {
//                     button: winit::event::MouseButton::Left,
//                     state: input_state,
//                     ..
//                 } => {
//                     let mouse_event = match input_state {
//                         ElementState::Pressed => MouseEvent::Down(MouseButton::Left),
//                         ElementState::Released => MouseEvent::Up(MouseButton::Left),
//                     };
//
//                     Some(Event::Mouse(mouse_event))
//                 }
//                 winit::event::WindowEvent::KeyboardInput { input, .. } => {
//                     KeyboardEvent::from_winit(*input).map(Event::Keyboard)
//                 }
//                 _ => None,
//             },
//             winit::event::Event::NewEvents(cause) => match cause {
//                 winit::event::StartCause::ResumeTimeReached {
//                     start,
//                     requested_resume,
//                 } => {
//                     // set_next_timer(control_flow);
//
//                     log::debug!("{:?} {:?}", start, requested_resume);
//                     // let delta: Result<i32, _> = requested_resume
//                     //     .duration_since(*start)
//                     //     .as_millis()
//                     //     .try_into();
//                     //
//                     *control_flow = ControlFlow::Poll;
//                     Some(Event::Tick {
//                         delta_millis: 16.666,
//                     })
//                 }
//                 winit::event::StartCause::Init => {
//                     // set_next_timer(control_flow);
//
//                     None
//                 }
//                 winit::event::StartCause::Poll => Some(Event::Tick {
//                     delta_millis: 16.6666,
//                 }),
//                 _ => None,
//             },
//             _ => None,
//         }
//     }
// }

pub fn event_loop<Game: App + 'static>(resources: Resources) {
    unsafe {
        #[cfg(target_arch = "wasm32")]
        {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Debug).unwrap();
        }

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

        log_error(&gl);

        let scale_factor = 1.0; // TODO
        let mut logical_size: LogicalSize<f64> = {
            #[cfg(not(target_arch = "wasm32"))]
            {
                window.window()
            }
            #[cfg(target_arch = "wasm32")]
            {
                &window
            }
        }
        .inner_size()
        .to_logical(scale_factor);

        let vertex_array = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        log_error(&gl);

        gl.bind_vertex_array(Some(vertex_array));
        log_error(&gl);

        gl.clear_color(0.1, 0.2, 0.3, 1.0);
        log_error(&gl);
        let program = gl::make_program(&gl, shader_version);
        gl.use_program(Some(program));
        log_error(&gl);
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
                            log_error(&gl);
                            // gl.delete_vertex_array(vertex_array);
                            log_error(&gl);
                            *control_flow = ControlFlow::Exit
                        }
                        _ => (),
                    },
                    _ => (),
                }
            });
        }

        let keys = Arc::new(Mutex::new(Keys::new()));
        #[cfg(target_arch = "wasm32")]
        let (_log_list, _runty8_log_list) = wasm::create_log_list();
        #[cfg(target_arch = "wasm32")]
        wasm::create_touch_controls(
            &web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .body()
                .unwrap(),
            Arc::clone(&keys),
        );
        let keys = Arc::clone(&keys);
        #[cfg(target_arch = "wasm32")]
        event_loop.run(move |winit_event, _, control_flow| {
            // log::debug!("Winit event {:?}", winit_event);
            // wasm::log_event(&log_list, &winit_event);

            let event: Option<Event> =
                Event::from_winit(&winit_event, scale_factor, &mut logical_size, &mut || {
                    set_next_timer(control_flow)
                });

            if let Some(event) = event {
                // wasm::log_runty8_event(&runty8_log_list, &event);

                // log::debug!("{:?}", pico8.state);

                match event {
                    Event::Tick { .. } => {
                        let keys = keys.lock().unwrap();
                        pico8.state.update_keys(&keys);
                        game.update(&mut pico8);
                        game.draw(&mut pico8);
                    }

                    Event::Keyboard(event) => {
                        let mut keys = keys.lock().unwrap();
                        keys.on_event(event);
                    }
                    Event::Mouse(MouseEvent::Move { x, y }) => pico8.state.on_mouse_move(x, y),
                    Event::Mouse(MouseEvent::Down(MouseButton::Left)) => {
                        let mut keys = keys.lock().unwrap();
                        keys.mouse = Some(true);
                    }
                    Event::Mouse(MouseEvent::Up(MouseButton::Left)) => {
                        let mut keys = keys.lock().unwrap();
                        keys.mouse = Some(false);
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
    println!("Buf len {}", pico8.draw_data.buffer().len());
    gl::upload_pixels(gl, texture, pico8.draw_data.buffer());
    log_error(gl);

    gl.clear(glow::COLOR_BUFFER_BIT);
    log_error(gl);

    gl.draw_arrays(glow::TRIANGLES, 0, 6);
    log_error(gl);
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
    use runty8_core::Keys;
    use runty8_core::{Key, KeyState};
    use std::sync::{Arc, Mutex};
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;
    use winit::window::Window;

    pub(super) fn create_touch_controls(container: &web_sys::HtmlElement, keys: Arc<Mutex<Keys>>) {
        for key in [
            Key::RightArrow,
            Key::LeftArrow,
            Key::UpArrow,
            Key::DownArrow,
        ]
        .into_iter()
        {
            let key_button = make_input_button(key, Arc::clone(&keys));
            container.append_child(&key_button).unwrap();
        }
    }

    fn make_input_button(key: Key, keys: Arc<Mutex<Keys>>) -> web_sys::HtmlElement {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let right_button = document
            .create_element("button")
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();
        right_button.style().set_css_text(
            r#"
            width: 100px;
            height: 100px;
            background-color: green;
            "#,
        );
        let right_button_closure_down = make_key_closure(key, KeyState::Down, Arc::clone(&keys));
        let right_button_closure_up = make_key_closure(key, KeyState::Up, keys);
        right_button.set_ontouchstart(Some(right_button_closure_down.as_ref().unchecked_ref()));
        right_button.set_ontouchend(Some(right_button_closure_up.as_ref().unchecked_ref()));
        right_button_closure_down.forget();
        right_button_closure_up.forget();

        right_button.set_inner_text(&format!("{:?}", key));

        right_button
    }

    fn make_key_closure(
        key: Key,
        key_state: KeyState,
        keys: Arc<Mutex<Keys>>,
    ) -> Closure<dyn FnMut()> {
        Closure::<dyn FnMut()>::new(move || {
            let mut keys = keys.lock().unwrap();

            let event = runty8_core::KeyboardEvent {
                key,
                state: key_state,
            };
            log::debug!("{:?}", event);

            keys.on_event(event);
        })
    }

    pub(super) fn insert_canvas_and_create_context(
        window: &Window,
    ) -> (glow::Context, &'static str) {
        use winit::platform::web::WindowExtWebSys;

        let canvas = window.canvas();

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let container = document
            .create_element("div")
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();
        body.append_child(&container).unwrap();

        canvas.style().set_css_text(
            r#"
            border: 1px solid blue;
            image-rendering: pixelated;
            width: 100%;
            max-width: 600px;
        "#,
        );
        container.append_child(&canvas).unwrap();

        let webgl2_context = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::WebGl2RenderingContext>()
            .unwrap();
        let gl = glow::Context::from_webgl2_context(webgl2_context);

        (gl, "#version 300 es")
    }

    pub(super) fn create_log_list() -> (web_sys::Element, web_sys::Element) {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let container = document.create_element("div").unwrap();
        let container = container.dyn_into::<web_sys::HtmlElement>().unwrap();
        container.style().set_css_text(
            r#"
            display: flex;
            align-content: center;
            justify-content: space-evenly;
        "#,
        );
        body.append_child(&container).unwrap();

        let log_header = document.create_element("h2").unwrap();
        log_header.set_text_content(Some("Event log"));
        // body.append_child(&log_header).unwrap();

        let log_list = document.create_element("ul").unwrap();
        container.append_child(&log_list).unwrap();
        let runty8_log_list = document.create_element("ul").unwrap();
        container.append_child(&runty8_log_list).unwrap();

        (log_list, runty8_log_list)
    }

    #[allow(dead_code)]
    pub(super) fn log_event(log_list: &web_sys::Element, event: &winit::event::Event<()>) {
        if let winit::event::Event::WindowEvent { event, .. } = &event {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let log = document.create_element("li").unwrap();
            log.set_text_content(Some(&format!("{:?}", event)));
            log_list
                .insert_before(&log, log_list.first_child().as_ref())
                .unwrap();
        }
    }

    #[allow(dead_code)]
    pub(super) fn log_runty8_event(log_list: &web_sys::Element, event: &crate::Event) {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let log = document.create_element("li").unwrap();
        if let crate::Event::Tick { .. } = event {
            return;
        }
        log.set_text_content(Some(&format!("{:?}", event)));
        log_list
            .insert_before(&log, log_list.first_child().as_ref())
            .unwrap();
    }
}

////// Utils
fn set_next_timer(control_flow: &mut ControlFlow) {
    let fps = 30_u64;
    let nanoseconds_per_frame = 1_000_000_000 / fps;

    let next_frame_time =
        instant::Instant::now() + std::time::Duration::from_nanos(nanoseconds_per_frame);
    log::debug!("{:?}", next_frame_time);
    *control_flow = ControlFlow::WaitUntil(next_frame_time);
    *control_flow = ControlFlow::Poll;
}

fn log_error(gl: &glow::Context) {
    let error = unsafe { gl.get_error() };

    if error != 0 {
        panic!("Error: {}", error);
    }
}
