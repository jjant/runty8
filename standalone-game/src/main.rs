use glow::HasContext;
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

        let program = gl.create_program().expect("Cannot create program");

        let (vertex_shader_source, fragment_shader_source) = (
            r#"const vec2 verts[3] = vec2[3](
                vec2(0.5f, 1.0f),
                vec2(0.0f, 0.0f),
                vec2(1.0f, 0.0f)
                );
                out vec2 vert;

                void main() {
                    vert = verts[gl_VertexID];
                    gl_Position = vec4(vert - 0.5, 0.0, 1.0);
                }
            "#,
            r#"precision mediump float;
            in vec2 vert;
            out vec4 color;

            void main() {
                color = vec4(vert, 0.5, 1.0);
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

            gl.shader_source(shader, &format!("{}\n{}", shader_version, shader_source));
            gl.compile_shader(shader);

            if !gl.get_shader_compile_status(shader) {
                panic!("{}", gl.get_shader_info_log(shader));
            }
            gl.attach_shader(program, shader);
            shaders.push(shader);
        }

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }

        for shader in shaders {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }

        gl.use_program(Some(program));
        gl.clear_color(0.1, 0.2, 0.3, 1.0);

        gl.clear(glow::COLOR_BUFFER_BIT);
        gl.draw_arrays(glow::TRIANGLES, 0, 3);
        gl.delete_program(program);
        gl.delete_vertex_array(vertex_array);

        let mut pico8 = Pico8::new(Resources {
            assets_path: "standalone-game/assets".to_owned(),
            map: Map::new(),
            sprite_flags: Flags::new(),
            sprite_sheet: SpriteSheet::new(),
        });
        let game = Game::init(&mut pico8);

        run_app(game, &mut pico8);
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

fn run_app(game: Game, pico8: &mut Pico8) {
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
            _ => (),
        }
    });
}

mod wasm {
    use wasm_bindgen::prelude::*;
    use winit::{event::Event, window::Window};

    #[wasm_bindgen(start)]
    pub fn run() {
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
        body.append_child(&canvas).unwrap();

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
