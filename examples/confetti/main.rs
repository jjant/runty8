use runty8::{App, Button, Pico8};

fn main() {
    runty8::run_app::<Confetti>("examples/confetti".to_owned()).unwrap();
}

struct Confetti {
    particles: Vec<Particle>,
    mouse_x: i32,
    mouse_y: i32,
}

impl App for Confetti {
    fn init(_: &mut Pico8) -> Self {
        Self {
            particles: vec![],
            mouse_x: 64,
            mouse_y: 64,
        }
    }

    fn update(&mut self, state: &mut Pico8) {
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

    fn draw(&mut self, draw_context: &mut Pico8) {
        draw_context.cls(0);

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
        let x = rand_between(-2.0, 2.0) + x;
        let y = rand_between(-2.0, 2.0) + y;
        let vx = rand_between(-15.0, 15.0) / 50.0;
        let vy = rand_between(-50.0, 10.0) / 50.0;
        let ttl = rand_between(10.0, 70.0) as i32;

        Self {
            x,
            y,
            vx,
            vy,
            ay: 0.05,
            ttl,
            color: rand_between(1.0, 16.0) as u8,
        }
    }

    fn update(&mut self) {
        self.vy += self.ay;
        self.x += self.vx;
        self.y += self.vy;
        self.ttl -= 1;
    }

    fn draw(&self, draw_context: &mut Pico8) {
        draw_context.pset(self.x as i32, self.y as i32, self.color);
    }
}

fn rand_between(min: f32, max: f32) -> f32 {
    min + runty8::rnd(max - min)
}
