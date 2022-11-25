use crate::app::AppCompat;
use crate::controller::{Controller, Scene};
use crate::graphics::{whole_screen_vertex_buffer, FRAGMENT_SHADER, VERTEX_SHADER};
use crate::Resources;
use glium::backend::Facade;
use glium::glutin::dpi::LogicalSize;
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::index::NoIndices;
use glium::texture::{RawImage2d, SrgbTexture2d};
use glium::uniforms::{MagnifySamplerFilter, Sampler};
use glium::{glutin, Display, Program, Surface};
use glium::{uniform, Frame};
use runty8_core::Event;
use runty8_winit::Runty8EventExt;

pub(crate) fn run_app<Game: AppCompat + 'static>(scene: Scene, resources: Resources) {
    let event_loop = glutin::event_loop::EventLoop::new();
    let display = make_display(&event_loop, "Runty8");
    let scale_factor = display.gl_window().window().scale_factor();
    let mut logical_size = display
        .gl_window()
        .window()
        .inner_size()
        .to_logical(scale_factor);

    let (indices, program) = make_gl_program(&display);

    let mut controller = Controller::<Game>::init(scene, resources);
    event_loop.run(move |winit_event, _, control_flow| {
        let event: Option<Event> =
            Event::from_winit(&winit_event, scale_factor, &mut logical_size, &mut || {
                set_next_timer(control_flow)
            });

        controller.step(event);

        if let Some(new_title) = controller.take_new_title() {
            display.gl_window().window().set_title(&new_title);
        }

        do_draw(
            &display,
            display.draw(),
            controller.screen_buffer(),
            &indices,
            &program,
        );
    });
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
