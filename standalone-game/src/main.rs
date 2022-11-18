use glow::{HasContext, WebTextureKey};
use rand::Rng;
use runty8_runtime::{App, Flags, Map, Pico8, Resources, SpriteSheet};
use wasm_bindgen::JsCast;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    unsafe {
        let (gl, shader_version) = {
            let canvas = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("canvas")
                .unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .unwrap();

            let webgl2_context = canvas
                .get_context("webgl2")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::WebGl2RenderingContext>()
                .unwrap();
            let gl = glow::Context::from_webgl2_context(webgl2_context);
            (gl, "#version 300 es")
        };

        let vertex_array = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(vertex_array));
        log_error(&gl);

        let program = gl.create_program().expect("Cannot create program");
        log_error(&gl);

        let (vertex_shader_source, fragment_shader_source) = (
            r#"const vec2 verts[6] = vec2[6](
                    vec2(-1.0f, -1.0f),
                    vec2(1.0f, 1.0f),
                    vec2(-1.0f, 1.0f),
                    vec2(-1.0f, -1.0f),
                    vec2(1.0f, -1.0f),
                    vec2(1.0f, 1.0f)
                );
                const vec2 tex_coords[6] = vec2[6](
                    vec2(0.0f, 0.0f),
                    vec2(1.0f, 1.0f),
                    vec2(0.0f, 1.0f),
                    vec2(0.0f, 0.0f),
                    vec2(1.0f, 0.0f),
                    vec2(1.0f, 1.0f)
                );
                out vec2 vert;
                out vec2 v_tex_coords;

                void main() {
                    vert = verts[gl_VertexID];

                    v_tex_coords = tex_coords[gl_VertexID];
                    gl_Position = vec4(vert, 0.0, 1.0);
                }
            "#,
            r#"precision mediump float;
            in vec2 vert;
            in vec2 v_tex_coords;
            out vec4 color;

            uniform sampler2D tex;

            void main() {
                float y = 1.0 - v_tex_coords.y;
                color = texture(tex, vec2(v_tex_coords.x, y));
            }"#,
        );

        let shader_sources = [
            (glow::VERTEX_SHADER, vertex_shader_source),
            (glow::FRAGMENT_SHADER, fragment_shader_source),
        ];

        let mut shaders = Vec::with_capacity(shader_sources.len());

        for (shader_type, shader_source) in shader_sources.iter() {
            let shader = gl
                .create_shader(*shader_type)
                .expect("Cannot create shader");
            log_error(&gl);

            gl.shader_source(shader, &format!("{}\n{}", shader_version, shader_source));
            log_error(&gl);
            gl.compile_shader(shader);
            log_error(&gl);

            if !gl.get_shader_compile_status(shader) {
                panic!("{}", gl.get_shader_info_log(shader));
            }
            gl.attach_shader(program, shader);
            log_error(&gl);
            shaders.push(shader);
        }

        gl.link_program(program);
        log_error(&gl);
        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }

        for shader in shaders {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }

        gl.use_program(Some(program));
        log_error(&gl);
        gl.clear_color(0.1, 0.2, 0.3, 1.0);

        let texture = create_texture(&gl);
        log::info!("Texture {:?}", texture);

        gl.active_texture(glow::TEXTURE0);
        log_error(&gl);
        gl.bind_texture(glow::TEXTURE_2D, Some(texture));
        log_error(&gl);
        let tex_location = gl.get_uniform_location(program, "tex").unwrap();
        gl.uniform_1_i32(Some(&tex_location), 0);
        log_error(&gl);

        let mut pico8 = Pico8::new(Resources {
            assets_path: "standalone-game/assets".to_owned(),
            map: Map::new(),
            sprite_flags: Flags::new(),
            sprite_sheet: SpriteSheet::new(),
        });
        let game = Game::init(&mut pico8);

        gl.clear(glow::COLOR_BUFFER_BIT);
        gl.draw_arrays(glow::TRIANGLES, 0, 6);
        run_app(game, pico8);

        gl.delete_program(program);
        gl.delete_vertex_array(vertex_array);
    }
}

fn do_draw(gl: glow::Context, buffer: &[u8]) {
    unsafe {
        gl.draw_arrays(glow::TRIANGLES, 0, 3);
    }
}

struct Game;

impl App for Game {
    fn init(pico8: &mut Pico8) -> Self {
        pico8.rect(15, 15, 30, 30, 8);
        pico8.set_title("nice".to_string());
        Self
    }

    fn update(&mut self, pico8: &mut Pico8) {
        //   pico8.cls(0);
        pico8.print("SOMETHING", 8, 8, 7);
    }

    fn draw(&mut self, pico8: &mut Pico8) {
        pico8.rect(
            rand::thread_rng().gen_range(0..128),
            rand::thread_rng().gen_range(0..128),
            rand::thread_rng().gen_range(0..128),
            rand::thread_rng().gen_range(0..128),
            8,
        );
    }
}

fn run_app(mut game: Game, mut pico8: Pico8) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("My pico8 game")
        .build(&event_loop)
        .unwrap();

    let log_list = wasm::insert_canvas_and_create_log_list(&window);
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        wasm::log_event(&log_list, &event);

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => control_flow.set_exit(),
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::NewEvents(cause) => match cause {
                winit::event::StartCause::ResumeTimeReached { .. } => {
                    game.update(&mut pico8);
                    game.draw(&mut pico8);
                }
                winit::event::StartCause::WaitCancelled { .. } => {}
                winit::event::StartCause::Poll => {}
                winit::event::StartCause::Init => {}
            },
            _ => (),
        }

        // do_draw(
        //     &display,
        //     display.draw(),
        //     pico8.draw_data.buffer(),
        //     &indices,
        //     &program,
        // );
    });
}

mod wasm {
    use wasm_bindgen::prelude::*;
    use winit::{event::Event, window::Window};

    #[wasm_bindgen(start)]
    pub fn run() {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Debug).expect("error in logger");

        super::main();
    }
    pub fn insert_canvas_and_create_log_list(window: &Window) -> web_sys::Element {
        use winit::platform::web::WindowExtWebSys;

        let canvas = window.canvas();

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        canvas.style().set_css_text("background-color: crimson;");
        // body.append_child(&canvas).unwrap();

        let log_header = document.create_element("h2").unwrap();
        log_header.set_text_content(Some("Event Log"));
        body.append_child(&log_header).unwrap();

        let log_list = document.create_element("ul").unwrap();
        body.append_child(&log_list).unwrap();

        log_list
    }

    pub fn log_event(log_list: &web_sys::Element, event: &Event<()>) {
        log::debug!("{:?}", event);

        if let Event::WindowEvent { event, .. } = &event {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let log = document.create_element("li").unwrap();
            log.set_text_content(Some(&format!("{:?}", event)));
            log_list
                .insert_before(&log, log_list.first_child().as_ref())
                .unwrap();
        }
    }
}

unsafe fn create_texture(gl: &glow::Context) -> WebTextureKey {
    let texture = gl.create_texture().unwrap();
    gl.bind_texture(glow::TEXTURE_2D, Some(texture));
    log_error(&gl);

    let red = vec![255, 0, 0, 255];
    let green = vec![0, 255, 0, 255];
    let blue = vec![0, 0, 255, 255];
    let white = vec![255, 255, 255, 255];

    let mut pixels = vec![];
    pixels.append(&mut red.clone());
    pixels.append(&mut green.clone());
    pixels.append(&mut blue.clone());
    pixels.append(&mut white.clone());

    gl.tex_image_2d(
        glow::TEXTURE_2D,
        0,
        glow::SRGB8_ALPHA8 as i32,
        2,
        2,
        0,
        glow::RGBA,
        glow::UNSIGNED_BYTE,
        Some(&pixels),
    );
    log_error(&gl);

    gl.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_MAG_FILTER,
        glow::NEAREST as i32,
    );
    log_error(&gl);

    gl.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_MIN_FILTER,
        glow::NEAREST as i32,
    );
    log_error(&gl);

    texture
}

fn log_error(gl: &glow::Context) {
    let err = unsafe { gl.get_error() };

    if err == 0 {
        log::info!("No error")
    } else {
        log::error!("Error: {}", err);
    }
}
