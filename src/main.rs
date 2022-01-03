use runty8::{self, App};
mod examples;

use rand::{self, Rng};
fn main() {
    runty8::run_editor();

    // runty8::run_app::<Game>();
}

struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    ttl: i32,
    color: u8,
}

struct Game {
    particles: Vec<Particle>,
}

impl App for Game {
    fn init() -> Self {
        Self {
            particles: vec![Particle {
                x: 64.0,
                y: 64.0,
                vx: 0.0,
                vy: 1.0,
                ttl: 20,
                color: 7,
            }],
        }
    }

    fn update(&mut self, state: &runty8::State, draw_context: &mut runty8::DrawContext) {
        if state.mouse_pressed {
            for _ in 0..10 {
                let vx = rand::thread_rng().gen_range(-15.0..15.0) / 50.0;
                let vy = rand::thread_rng().gen_range(-15.0..15.0) / 50.0;
                let p = Particle {
                    x: state.mouse_x as f32,
                    y: state.mouse_y as f32,
                    vx,
                    vy,
                    ttl: rand::thread_rng().gen_range(10..240),
                    color: rand::thread_rng().gen_range(1..16),
                };

                self.particles.push(p);
            }
        }

        let mut i = 0;
        while i < self.particles.len() {
            if self.particles[i].ttl == 0 {
                self.particles.remove(i);
            } else {
                self.particles[i].x += self.particles[i].vx;
                self.particles[i].y += self.particles[i].vy;
                self.particles[i].ttl -= 1;
                i += 1
            }
        }
    }

    fn draw(&self, draw_context: &mut runty8::DrawContext) {
        draw_context.cls();
        // for _ in 0..20 {
        //     let x: i32 = rand::thread_rng().gen_range(0..128);
        //     let y: i32 = rand::thread_rng().gen_range(0..128);
        //     let c: u8 = rand::thread_rng().gen_range(0..16);

        //     draw_context.pset(x, y, c);
        // }

        for particle in self.particles.iter() {
            draw_context.pset(particle.x as i32, particle.y as i32, particle.color);
        }
    }
}
