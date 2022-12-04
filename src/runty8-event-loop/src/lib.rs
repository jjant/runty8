#![deny(missing_docs)]

//! Winit/Glow/Glutin powered event loop for Runty8 applications.

use glow::HasContext;
use runty8_core::Event;
use runty8_winit::{Runty8EventExt as _, ScreenInfo};
use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod gl;

#[cfg(not(target_arch = "wasm32"))]
type Window = glutin::WindowedContext<glutin::PossiblyCurrent>;

#[cfg(target_arch = "wasm32")]
type Window = winit::window::Window;

/// Create a window (or canvas, in wasm) and respond to events on it.
pub fn event_loop(
    mut on_event: impl FnMut(Event, &mut ControlFlow, &dyn Fn(&[u8], &mut ControlFlow), &dyn Fn(&str))
        + 'static,
) {
    #[cfg(target_arch = "wasm32")]
    wasm::setup_console_log_panic_hook();

    let mut screen_info = ScreenInfo::new(640.0, 640.0);

    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_inner_size(screen_info.logical_size)
        .with_title("Runty8");

    let (window, gl, shader_version) = make_window_and_context(window_builder, &event_loop);
    screen_info.scale_factor = winit_window(&window).scale_factor();

    let texture = unsafe {
        let vertex_array = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(vertex_array));

        gl.clear_color(0.1, 0.2, 0.3, 1.0);

        let program = gl::make_program(&gl, shader_version);
        gl.use_program(Some(program));
        let texture = gl::make_texture(&gl);
        gl::use_texture(&gl, program);

        texture
    };

    let mut current_time = instant::now();
    // TODO: Initial render.
    // EDIT: Actually I think this handles itself through the Tick from Init? Maybe? Not sure.
    // => Test it
    // gl::upload_pixels(&gl, texture, pico8.draw_data.buffer());
    event_loop.run(move |winit_event, _, control_flow| {
        let event: Option<Event> =
            Event::from_winit(&winit_event, &mut current_time, &mut screen_info);

        if let Some(event) = event {
            let draw: &dyn Fn(&[u8], &mut ControlFlow) = &|pixels, _control_flow| {
                draw(&gl, texture, pixels);
                #[cfg(not(target_arch = "wasm32"))]
                window.swap_buffers().unwrap();
            };

            let set_title: &dyn Fn(&str) = &|title| set_title(&window, title);

            on_event(event, control_flow, draw, set_title);
        }
    })
}

fn draw(gl: &glow::Context, texture: glow::Texture, pixels: &[u8]) {
    unsafe {
        gl::upload_pixels(gl, texture, pixels);
        gl.clear(glow::COLOR_BUFFER_BIT);
        gl.draw_arrays(glow::TRIANGLES, 0, 6);
    }
}

fn make_window_and_context(
    window_builder: WindowBuilder,
    event_loop: &EventLoop<()>,
) -> (Window, glow::Context, &'static str) {
    #[cfg(not(target_arch = "wasm32"))]
    return native::make_window_and_context(window_builder, event_loop);

    #[cfg(target_arch = "wasm32")]
    return wasm::make_window_and_context(window_builder, event_loop);
}

fn winit_window(window: &Window) -> &winit::window::Window {
    #[cfg(not(target_arch = "wasm32"))]
    return window.window();

    #[cfg(target_arch = "wasm32")]
    return window;
}

fn set_title(window: &Window, title: &str) {
    winit_window(window).set_title(title);
    #[cfg(target_arch = "wasm32")]
    wasm::set_title(title);
}

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use glutin::{event_loop::EventLoop, window::WindowBuilder, ContextBuilder, ContextWrapper};

    pub(crate) fn make_window_and_context(
        window_builder: WindowBuilder,
        event_loop: &EventLoop<()>,
    ) -> (
        glutin::WindowedContext<glutin::PossiblyCurrent>,
        glow::Context,
        &'static str,
    ) {
        let window = unsafe {
            ContextBuilder::new()
                .with_vsync(true)
                .build_windowed(window_builder, event_loop)
                .unwrap()
                .make_current()
                .unwrap()
        };

        let gl = unsafe {
            glow::Context::from_loader_function(|s| {
                ContextWrapper::get_proc_address(&window, s) as *const _
            })
        };

        (window, gl, "#version 410")
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use wasm_bindgen::JsCast;
    use winit::event_loop::EventLoop;
    use winit::platform::web::WindowExtWebSys;
    use winit::window::{Window, WindowBuilder};

    pub(crate) fn setup_console_log_panic_hook() {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Debug).unwrap();
    }
    pub(crate) fn make_window_and_context(
        window_builder: WindowBuilder,
        event_loop: &EventLoop<()>,
    ) -> (Window, glow::Context, &'static str) {
        let window = window_builder.build(event_loop).unwrap();
        let gl = insert_canvas_and_create_context(&window);

        (window, gl, "#version 300 es")
    }

    pub(crate) fn insert_canvas_and_create_context(window: &Window) -> glow::Context {
        let canvas = window.canvas();

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        canvas.style().set_css_text(
            r#"
                border: 1px solid blue;
                image-rendering: pixelated;
                width: 100%;
                max-width: 600px;
            "#,
        );
        body.append_child(&canvas).unwrap();

        let webgl2_context = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::WebGl2RenderingContext>()
            .unwrap();

        glow::Context::from_webgl2_context(webgl2_context)
    }

    pub(crate) fn set_title(title: &str) {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        document.set_title(title);
    }
}

fn log_error(gl: &glow::Context) {
    let error = unsafe { gl.get_error() };

    if error != 0 {
        panic!("Error: {error}");
    }
}
