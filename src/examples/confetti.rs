use rand::Rng;
use runty8::{App, DrawContext, State};

pub struct Confetti {
    particles: Vec<Particle>,
}

impl App for Confetti {
    fn init() -> Self {
        Self { particles: vec![] }
    }

    fn update(&mut self, state: &State) {
        if state.mouse_pressed {
            for _ in 0..10 {
                self.particles
                    .push(Particle::new(state.mouse_x as f32, state.mouse_y as f32));
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

    fn draw(&self, draw_context: &mut runty8::DrawContext) {
        draw_context.cls();

        for particle in self.particles.iter() {
            particle.draw(draw_context)
        }
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
        let x = self.x as i32;
        let y = self.y as i32;
        draw_context.pset(x, y, self.color);
    }
}
