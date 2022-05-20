use rand::Rng;
use runty8::runtime::draw_context::DrawContext;
use runty8::runtime::state::{Button, State};
use runty8::App;

fn main() {
    runty8::run_app::<Confetti>("".to_owned());
}

struct Confetti {
    particles: Vec<Particle>,
    mouse_x: i32,
    mouse_y: i32,
}

impl App for Confetti {
    fn init(_: &State) -> Self {
        Self {
            particles: vec![],
            mouse_x: 64,
            mouse_y: 64,
        }
    }

    fn update(&mut self, state: &State) {
        (self.mouse_x, self.mouse_y) = state.mouse();

        if state.btn(Button::Mouse) {
            for _ in 0..10 {
                self.particles
                    .push(Particle::new(self.mouse_x as f32, self.mouse_y as f32));
            }
        }

        let mut i = 0;
        while i < self.particles.len() {
            if self.particles[i].ttl == 0 {
                self.particles.remove(i);
            } else {
                self.particles[i].update();

                i += 1
            }
        }
    }

    fn draw(&mut self, draw_context: &mut DrawContext) {
        draw_context.cls();

        let text_x = 3;
        let text_y = 3;
        draw_context.print(
            &"click and drag to ".to_ascii_uppercase(),
            text_x,
            text_y,
            7,
        );
        draw_context.print(
            &"throw confetti".to_ascii_uppercase(),
            text_x,
            text_y + 7,
            7,
        );

        for particle in self.particles.iter() {
            particle.draw(draw_context)
        }

        draw_context.spr(8, self.mouse_x - 4, self.mouse_y - 3);
    }
}

struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    ay: f32,
    ttl: i32,
    color: u8,
}

impl Particle {
    fn new(x: f32, y: f32) -> Self {
        let x = rand::thread_rng().gen_range(-2.0..2.0) + x;
        let y = rand::thread_rng().gen_range(-2.0..2.0) + y;
        let vx = rand::thread_rng().gen_range(-15.0..15.0) / 50.0;
        let vy = rand::thread_rng().gen_range(-50.0..10.0) / 50.0;
        let ttl = rand::thread_rng().gen_range(10..70);

        Self {
            x,
            y,
            vx,
            vy,
            ay: 0.05,
            ttl,
            color: rand::thread_rng().gen_range(1..16),
        }
    }

    fn update(&mut self) {
        self.vy += self.ay;
        self.x += self.vx;
        self.y += self.vy;
        self.ttl -= 1;
    }

    fn draw(&self, draw_context: &mut DrawContext) {
        draw_context.pset(self.x as i32, self.y as i32, self.color);
    }
}
