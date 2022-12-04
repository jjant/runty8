use std::f32::consts::{FRAC_1_SQRT_2, PI};

use runty8::{rnd, App, Button, Pico8};

use std::iter::{Chain, Map};
use std::slice;

fn main() {
    let resources = runty8::load_assets!("celeste").unwrap();

    runty8::debug_run::<GameState>(resources).unwrap();
}

struct GameState {
    room: Vec2<i32>,
    frames: i32,
    deaths: i32,
    max_djump: i32,
    start_game: bool,
    start_game_flash: i32,
    objects: Vec<Object>,
    freeze: i32,
    will_restart: bool,
    delay_restart: i32,
    got_fruit: Vec<bool>,
    #[allow(dead_code)]
    sfx_timer: i32,
    has_key: bool,
    #[allow(dead_code)]
    has_dashed: bool,
    #[allow(dead_code)]
    pause_player: bool,
    flash_bg: bool,
    new_bg: bool,
    music_timer: i32,
    clouds: Vec<Cloud>,
    seconds: i32,
    minutes: i32,
    particles: Vec<Particle>,
    // Particles created when the player dies
    dead_particles: Vec<DeadParticle>,
    effects: GameEffects,
}

struct GameEffects {
    shake: i32,
}

impl App for GameState {
    fn init(pico8: &mut Pico8) -> Self {
        pico8.set_title("Celeste".to_owned());

        let clouds = (0..=16)
            .into_iter()
            .map(|_| Cloud {
                x: rnd(128.),
                y: rnd(128.),
                spd: 1. + rnd(4.),
                w: 32. + rnd(32.),
            })
            .collect();

        let particles = (0..=24)
            .into_iter()
            .map(|_| Particle {
                x: rnd(128.),
                y: rnd(128.),
                s: flr(rnd(5.) / 4.),
                spd: 0.25 + rnd(5.),
                off: rnd(1.),
                c: 6 + flr(0.5 + rnd(1.)),
            })
            .collect();

        let mut gs = Self {
            room: Vec2 { x: 0, y: 0 },
            objects: vec![],
            freeze: 0,
            will_restart: false,
            delay_restart: 0,
            got_fruit: vec![],
            has_dashed: false,
            sfx_timer: 0,
            has_key: false,
            pause_player: false,
            frames: 0,
            deaths: 0,
            max_djump: 1,
            start_game: false,
            start_game_flash: 0,
            flash_bg: false,
            new_bg: false,
            music_timer: 0,
            clouds,
            seconds: 0,
            minutes: 0,
            particles,
            dead_particles: vec![],
            effects: GameEffects { shake: 0 },
        };

        title_screen(&mut gs, pico8);

        gs
    }

    fn update(&mut self, pico8: &mut Pico8) {
        self.frames = (self.frames + 1) % 30;

        if self.frames == 0 && level_index(self.room) < 30 {
            self.seconds = (self.seconds + 1) % 60;
            if self.seconds == 0 {
                self.minutes += 1;
            }
        }

        if self.music_timer > 0 {
            self.music_timer -= 1;

            if self.music_timer <= 0 {
                // TODO: Implement `music` api
                // music(10, 0, 7);
            }
        }
        if self.sfx_timer > 0 {
            self.sfx_timer -= 1;
        }

        if self.freeze > 0 {
            self.freeze -= 1;
            return;
        }

        // screenshake
        if self.effects.shake > 0 {
            self.effects.shake -= 1;
        }

        // Restart (soon)
        if self.will_restart && self.delay_restart > 0 {
            self.delay_restart -= 1;
            if self.delay_restart <= 0 {
                self.will_restart = false;
                load_room(self, pico8, self.room.x, self.room.y);
            }
        }

        let mut player_dead = false;
        let mut do_next_level = false;
        let mut i = 0;
        while i < self.objects.len() {
            let (object, mut other_objects) =
                OtherObjects::split_slice(i, &mut self.objects).unwrap();

            // Apply velocity
            object.move_(pico8, &mut other_objects, self.room);
            let UpdateAction {
                should_destroy,
                next_level,
                mut new_objects,
            } = object.update(
                &mut other_objects,
                &mut self.effects,
                &mut self.got_fruit,
                self.room,
                pico8,
                self.max_djump,
                &mut self.has_dashed,
                self.frames,
                &mut self.has_key,
                self.pause_player,
                &mut self.freeze,
            );

            player_dead = object.kind() == ObjectKind::Player && should_destroy;
            if player_dead {
                break;
            }

            if next_level {
                do_next_level = true;
                break;
            }
            if should_destroy {
                self.objects.remove(i);
            } else {
                i += 1;
            }
            self.objects.append(&mut new_objects);
        }

        if player_dead {
            kill_player(self)
        } else if do_next_level {
            next_room(self, pico8);
        }

        if !is_title(self) {
            self.clouds.iter_mut().for_each(Cloud::update);
        }

        self.particles.iter_mut().for_each(Particle::update);

        // Update and remove dead dead_particles
        self.dead_particles.retain_mut(DeadParticle::update);

        // self.begin_game(pico8);
        // start game
        if is_title(self) {
            if !self.start_game && (pico8.btn(K_JUMP) || pico8.btn(K_DASH)) {
                // music(-1);
                self.start_game_flash = 50;
                self.start_game = true;
                // sfx(38);
            }
            if self.start_game {
                self.start_game_flash -= 1;
                if self.start_game_flash <= -30 {
                    self.begin_game(pico8);
                }
            }
        }
    }

    fn draw(&mut self, draw: &mut Pico8) {
        draw.camera(0, 0);
        if self.effects.shake > 0 {
            draw.camera(
                (-2. + rnd(5.)).floor() as i32,
                (-2. + rnd(5.)).floor() as i32,
            );
        }

        if self.freeze > 0 {
            return;
        }

        // Reset all palette values
        draw.reset_pal();

        // Start game flash
        if self.start_game {
            let mut c = 10;

            if self.start_game_flash > 10 {
                if self.frames % 10 < 5 {
                    c = 7;
                }
            } else if self.start_game_flash > 5 {
                c = 2;
            } else if self.start_game_flash > 0 {
                c = 1;
            } else {
                c = 0;
            }

            if c < 10 {
                draw.pal(6, c);
                draw.pal(12, c);
                draw.pal(13, c);
                draw.pal(5, c);
                draw.pal(1, c);
                draw.pal(7, c);
            }
        }

        // Clear screen
        let mut bg_col = 0;
        if self.flash_bg {
            bg_col = (self.frames / 5) as u8;
        } else if self.new_bg {
            bg_col = 2;
        }

        draw.rectfill(0, 0, 128, 128, bg_col);

        // Clouds
        if !is_title(self) {
            for cloud in self.clouds.iter() {
                draw.rectfill(
                    cloud.x.floor() as i32,
                    cloud.y.floor() as i32,
                    (cloud.x + cloud.w).floor() as i32,
                    (cloud.y + 4. + (1. - cloud.w / 64.) * 12.).floor() as i32,
                    if self.new_bg { 14 } else { 1 },
                );
            }
        }

        // Draw bg terrain
        draw.map(self.room.x * 16, self.room.y * 16, 0, 0, 16, 16, 4);

        // Platforms/big chest
        self.draw_objects(draw, |kind| {
            [ObjectKind::Platform, ObjectKind::BigChest].contains(&kind)
        });

        // Draw terrain
        let off = if is_title(self) { -4 } else { 0 };
        draw.map(self.room.x * 16, self.room.y * 16, off, 0, 16, 16, 2);

        // Draw objects
        self.draw_objects(draw, |kind| {
            ![ObjectKind::Platform, ObjectKind::BigChest].contains(&kind)
        });

        // Draw fg terrain
        draw.map(self.room.x * 16, self.room.y * 16, 0, 0, 16, 16, 8);

        // Particles
        for p in &self.particles {
            draw.rectfill(
                flr(p.x),
                flr(p.y),
                flr(p.x + p.s as f32),
                flr(p.y + p.s as f32),
                p.c as u8,
            );
        }

        // Dead particles
        for p in &self.dead_particles {
            let t = p.t as f32;
            draw.rectfill(
                (p.x - t / 5.) as i32,
                (p.y - t / 5.) as i32,
                (p.x + t / 5.) as i32,
                (p.y + t / 5.) as i32,
                (14 + p.t % 2) as u8,
            );
        }

        // Draw outside of the screen for screenshake
        draw.rectfill(-5, -5, -1, 133, 0);
        draw.rectfill(-5, -5, 133, -1, 0);
        draw.rectfill(-5, 128, 133, 133, 0);
        draw.rectfill(128, -5, 133, 133, 0);

        // Credits
        if is_title(self) {
            draw.print("X+C", 58, 80, 5);
            draw.print("MADDY THORSON", 41, 96, 5);
            draw.print("NOEL BERRY", 46, 102, 5);
            draw.print("PORTED BY JJANT", 34, 114, 5);
        }

        if level_index(self.room) == 30 {
            if let Some(p) = self
                .objects
                .iter()
                .find(|object| object.kind() == ObjectKind::Player)
            {
                let diff = i32::min(24, 40 - i32::abs(p.base_object.x + 4 - 64));
                draw.rectfill(0, 0, diff, 128, 0);
                draw.rectfill(128 - diff, 0, 128, 128, 0);
            }
        }
    }
}

impl GameState {
    fn draw_objects(&mut self, draw: &mut Pico8, predicate: impl Fn(ObjectKind) -> bool) {
        {
            let mut i = 0;
            while i < self.objects.len() {
                let (object, mut other_objects) =
                    OtherObjects::split_slice(i, &mut self.objects).unwrap();

                if predicate(object.kind()) {
                    let UpdateAction {
                        should_destroy,
                        mut new_objects,
                        ..
                    } = object.draw(
                        draw,
                        &mut other_objects,
                        self.room,
                        &self.got_fruit,
                        self.frames,
                        &mut self.max_djump,
                        &mut self.effects.shake,
                        &mut self.freeze,
                        &mut self.flash_bg,
                        &mut self.new_bg,
                        &mut self.pause_player,
                        self.seconds,
                        self.minutes,
                        self.deaths,
                    );

                    if should_destroy {
                        self.objects.remove(i);
                    } else {
                        i += 1;
                    }

                    self.objects.append(&mut new_objects);
                } else {
                    i += 1;
                }
            }
        }
    }
}

struct Cloud {
    x: f32,
    y: f32,
    spd: f32,
    w: f32,
}

impl Cloud {
    fn update(&mut self) {
        self.x += self.spd;

        if self.x > 128. {
            self.x = -self.w;
            self.y = rnd(128. - 8.);
        }
    }
}

struct Particle {
    x: f32,
    y: f32,
    s: i32,
    spd: f32,
    off: f32,
    c: i32,
}

impl Particle {
    fn update(&mut self) {
        self.x += self.spd;

        self.y += sin(self.off);
        self.off += (0.05_f32).min(self.spd / 32.);
        if self.x > 128. + 4. {
            self.x = -4.;
            self.y = rnd(128.);
        }
    }
}
struct DeadParticle {
    x: f32,
    y: f32,
    spd: Vec2<f32>,
    t: i32,
}

impl DeadParticle {
    fn update(&mut self) -> bool {
        self.x += self.spd.x;
        self.y += self.spd.y;
        self.t -= 1;

        // Keep if
        self.t > 0
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct Vec2<T> {
    x: T,
    y: T,
}

impl GameState {
    fn begin_game(&mut self, pico8: &Pico8) {
        self.frames = 0;
        self.seconds = 0;
        self.minutes = 0;
        self.music_timer = 0;
        self.start_game = false;
        // music(0, 0, 7);
        load_room(self, pico8, 0, 0);
    }
}

const K_LEFT: Button = Button::Left;
const K_RIGHT: Button = Button::Right;
const K_UP: Button = Button::Up;
const K_DOWN: Button = Button::Down;
const K_JUMP: Button = Button::C;
const K_DASH: Button = Button::X;

fn title_screen(game_state: &mut GameState, pico8: &Pico8) {
    game_state.got_fruit = vec![false; 30];
    game_state.frames = 0;
    game_state.deaths = 0;
    game_state.max_djump = 1;
    game_state.start_game = false;
    game_state.start_game_flash = 0;
    // music(40,0,7)
    load_room(game_state, pico8, 7, 3)
}

fn level_index(room: Vec2<i32>) -> usize {
    (room.x % 8 + room.y * 8) as usize
}

/// Starting title screen
fn is_title(game_state: &GameState) -> bool {
    level_index(game_state.room) == 31
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
struct Player {
    p_jump: bool,
    p_dash: bool,
    grace: i32,
    jbuffer: i32,
    djump: i32,
    dash_time: i32,
    dash_effect_time: i32,
    dash_target: Vec2<f32>,
    dash_accel: Vec2<f32>,

    spr_off: f32,
    was_on_ground: bool,
    hair: Hair,
}

impl Player {
    fn init(base_object: &mut BaseObject, max_djump: i32) -> Self {
        base_object.hitbox = Hitbox {
            x: 1,
            y: 3,
            w: 6,
            h: 5,
        };
        Self {
            p_jump: false,
            p_dash: false,
            grace: 0,
            jbuffer: 0,
            djump: max_djump,
            dash_time: 0,
            dash_effect_time: 0,
            dash_target: Vec2 { x: 0., y: 0. },
            dash_accel: Vec2 { x: 0., y: 0. },
            spr_off: 0.,
            was_on_ground: false,
            hair: Self::create_hair(base_object.x, base_object.y),
        }
    }

    fn create_hair(x: i32, y: i32) -> Hair {
        Hair {
            segments: [0, 1, 2, 3, 4].map(|index| HairElement {
                x: x as f32,
                y: y as f32,
                size: i32::max(1, i32::min(2, 3 - index)),
            }),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn update<T>(
        &mut self,
        this: &mut BaseObject,
        state: &Pico8,
        objects: &mut T,
        got_fruit: &[bool],
        room: Vec2<i32>,
        max_djump: i32,
        has_dashed: &mut bool,
        pause_player: bool,
        shake: &mut i32,
        // TODO:
        #[allow(unused_variables)] freeze: &mut i32,
    ) -> UpdateAction
    where
        for<'b> &'b mut T: IntoIterator<Item = &'b mut Object>,
    {
        let mut update_action = UpdateAction::noop();

        if pause_player {
            return update_action;
        }

        let input = horizontal_input(state);

        // -- spikes collide
        if spikes_at(
            state,
            room,
            this.x + this.hitbox.x,
            this.y + this.hitbox.y,
            this.hitbox.w,
            this.hitbox.h,
            this.spd.x,
            this.spd.y,
        ) {
            update_action.destroy_if_mut(true);
        }

        // -- bottom death
        if this.y > 128 {
            update_action.destroy_if_mut(true);
        }

        let on_ground = this.is_solid(state, objects, room, 0, 1);
        let on_ice = this.is_ice(state, room, 0, 1);

        // -- smoke particles
        if on_ground && !self.was_on_ground {
            update_action.push_mut(Object::init(
                got_fruit,
                room,
                ObjectKind::Smoke,
                this.x,
                this.y,
                max_djump,
            ));
        }

        let jump = state.btn(K_JUMP) && !self.p_jump;
        self.p_jump = state.btn(K_JUMP);

        if jump {
            self.jbuffer = 4;
        } else if self.jbuffer > 0 {
            self.jbuffer -= 1;
        }

        let dash = state.btn(K_DASH) && !self.p_dash;
        self.p_dash = state.btn(K_DASH);

        if on_ground {
            self.grace = 6;

            if self.djump < max_djump {
                // psfx(54)
                self.djump = max_djump
            }
        } else if self.grace > 0 {
            self.grace -= 1;
        }

        self.dash_effect_time -= 1;

        if self.dash_time > 0 {
            update_action.push_mut(Object::init(
                got_fruit,
                room,
                ObjectKind::Smoke,
                this.x,
                this.y,
                max_djump,
            ));
            self.dash_time -= 1;
            this.spd.x = appr(this.spd.x, self.dash_target.x, self.dash_accel.x);
            this.spd.y = appr(this.spd.y, self.dash_target.y, self.dash_accel.y);
        } else {
            // -- move
            let maxrun = 1.0;
            let mut accel = 0.6;
            let deccel = 0.15;

            if !on_ground {
                accel = 0.4;
            } else if on_ice {
                accel = 0.05;

                if input == (if this.flip.x { -1 } else { 1 }) {
                    accel = 0.05;
                }
            }

            if this.spd.x.abs() > maxrun {
                this.spd.x = appr(this.spd.x, sign(this.spd.x) * maxrun, deccel);
            } else {
                this.spd.x = appr(this.spd.x, input as f32 * maxrun, accel);
            }

            // -- facing
            if this.spd.x != 0.0 {
                this.flip.x = this.spd.x < 0.0;
            }

            // -- gravity
            let mut maxfall = 2.0;
            let mut gravity = 0.21;

            if this.spd.y.abs() <= 0.15 {
                gravity *= 0.5;
            }

            // -- wall slide
            if input != 0
                && this.is_solid(state, objects, room, input, 0)
                && !this.is_ice(state, room, input, 0)
            {
                maxfall = 0.4;

                if rnd(10.) < 2. {
                    update_action.push_mut(Object::init(
                        got_fruit,
                        room,
                        ObjectKind::Smoke,
                        this.x + input * 6,
                        this.y,
                        max_djump,
                    ));
                }
            }

            if !on_ground {
                this.spd.y = appr(this.spd.y, maxfall, gravity);
            }

            // -- jump
            if self.jbuffer > 0 {
                if self.grace > 0 {
                    // -- normal jump
                    // psfx(1)
                    self.jbuffer = 0;
                    self.grace = 0;
                    this.spd.y = -2.0;
                    update_action.push_mut(Object::init(
                        got_fruit,
                        room,
                        ObjectKind::Smoke,
                        this.x,
                        this.y + 4,
                        max_djump,
                    ));
                } else {
                    // -- wall jump
                    #[allow(clippy::bool_to_int_with_if)]
                    let wall_dir = if this.is_solid(state, objects, room, -3, 0) {
                        -1
                    } else if this.is_solid(state, objects, room, 3, 0) {
                        1
                    } else {
                        0
                    };
                    if wall_dir != 0 {
                        // psfx(2)

                        self.jbuffer = 0;
                        this.spd.y = -2.0;
                        this.spd.x = -wall_dir as f32 * (maxrun + 1.0);
                        if !(this.is_ice(state, room, wall_dir * 3, 0)) {
                            update_action.push_mut(Object::init(
                                got_fruit,
                                room,
                                ObjectKind::Smoke,
                                this.x + wall_dir * 6,
                                this.y,
                                max_djump,
                            ));
                        }
                    }
                }
            }

            // -- dash

            let d_full = 5.0;
            let d_half = d_full * FRAC_1_SQRT_2;

            if self.djump > 0 && dash {
                update_action.push_mut(Object::init(
                    got_fruit,
                    room,
                    ObjectKind::Smoke,
                    this.x,
                    this.y,
                    max_djump,
                ));

                self.djump -= 1;
                self.dash_time = 4;
                // below: used for flying fruits to leave
                *has_dashed = true;
                self.dash_effect_time = 10;
                let v_input = vertical_input(state);

                if input != 0 {
                    if v_input != 0 {
                        this.spd.x = input as f32 * d_half;
                        this.spd.y = v_input as f32 * d_half;
                    } else {
                        this.spd.x = input as f32 * d_full;
                        this.spd.y = 0.0;
                    }
                } else if v_input != 0 {
                    this.spd.x = 0.0;
                    this.spd.y = v_input as f32 * d_full;
                } else {
                    this.spd.x = if this.flip.x { -1.0 } else { 1.0 };
                    this.spd.y = 0.0;
                }

                // psfx(3);
                // *freeze = 2;
                *shake = 6;
                self.dash_target.x = 2.0 * sign(this.spd.x);
                self.dash_target.y = 2.0 * sign(this.spd.y);
                self.dash_accel.x = 1.5;
                self.dash_accel.y = 1.5;

                if this.spd.y < 0.0 {
                    self.dash_target.y *= 0.75;
                }

                if this.spd.y != 0.0 {
                    self.dash_accel.x *= FRAC_1_SQRT_2;
                }
                if this.spd.x != 0.0 {
                    self.dash_accel.y *= FRAC_1_SQRT_2;
                }
            } else if dash && self.djump <= 0 {
                // psfx(9)
                update_action.push_mut(Object::init(
                    got_fruit,
                    room,
                    ObjectKind::Smoke,
                    this.x,
                    this.y,
                    max_djump,
                ));
            }
        }

        // -- animation
        self.spr_off += 0.25;
        if !on_ground {
            if this.is_solid(state, objects, room, input, 0) {
                this.spr = 5.0;
            } else {
                this.spr = 3.0;
            }
        } else if state.btn(K_DOWN) {
            this.spr = 6.0;
        } else if state.btn(K_UP) {
            this.spr = 7.0;
        } else if this.spd.x == 0.0 && (!state.btn(K_LEFT) && !state.btn(K_RIGHT)) {
            this.spr = 1.0;
        } else {
            this.spr = 1.0 + self.spr_off % 4.0;
        }

        // -- next level
        if this.y < -4 && level_index(room) < 30 {
            update_action.next_level();
        }

        self.was_on_ground = on_ground;
        update_action
    }

    fn draw(&mut self, base_object: &mut BaseObject, draw: &mut Pico8, frames: i32) {
        if base_object.x < -1 || base_object.x > 121 {
            base_object.x = clamp(base_object.x, -1, 121);
            base_object.spd.x = 0.0;
        }

        set_hair_color(draw, frames, self.djump);

        let facing = if base_object.flip.x { -1 } else { 1 };
        self.hair.draw(draw, base_object.x, base_object.y, facing);

        draw.spr_(
            base_object.spr.floor() as usize,
            base_object.x,
            base_object.y,
            1.0,
            1.0,
            base_object.flip.x,
            base_object.flip.y,
        );
        unset_hair_color(draw);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct HairElement {
    x: f32,
    y: f32,
    size: i32,
}

#[allow(dead_code, unused_variables)]
fn psfx(game_state: &GameState, num: i32) {
    if game_state.sfx_timer <= 0 {
        // TODO: Implement
        // sfx(num)
    }
}

fn set_hair_color(draw: &mut Pico8, frames: i32, djump: i32) {
    let c = if djump == 1 {
        8
    } else if djump == 2 {
        7 + ((frames / 3) % 2) * 4
    } else {
        12
    };

    draw.pal(8, c as u8);
}

#[allow(dead_code)]
fn unset_hair_color(draw: &mut Pico8) {
    draw.pal(8, 8);
}

#[derive(Clone, Copy, Debug)]
struct Spring {
    hide_in: i32,
    hide_for: i32,
    delay: i32,
}

impl Spring {
    fn init() -> Self {
        Self {
            hide_in: 0,
            hide_for: 0,
            delay: 0,
        }
    }

    fn update<T>(
        &mut self,
        this: &mut BaseObject,
        objects: &mut T,
        got_fruit: &mut [bool],
        room: Vec2<i32>,
        max_djump: i32,
    ) -> UpdateAction
    where
        for<'b> &'b mut T: IntoIterator<Item = &'b mut Object>,
    {
        let mut update_action = UpdateAction::noop();
        if self.hide_for > 0 {
            self.hide_for -= 1;

            if self.hide_for <= 0 {
                this.spr = 18.0;
                self.delay = 0;
            }
        } else if this.spr == 18.0 {
            if let Some((_, hit)) = this.collide(objects.into_iter(), &ObjectKind::Player, 0, 0) {
                let (player_base, player) = hit.to_player_mut().expect("Got wrong object back");

                if player_base.spd.y >= 0.0 {
                    this.spr = 19.0;

                    player_base.y = this.y - 4;
                    player_base.spd.x *= 0.2;
                    player_base.spd.y = -3.0;
                    player.djump = max_djump;
                    self.delay = 10;

                    update_action.push_mut(Object::init(
                        got_fruit,
                        room,
                        ObjectKind::Smoke,
                        this.x,
                        this.y,
                        max_djump,
                    ));

                    // psfx(8)
                }
                // -- breakable below us
                let below = this.collide(objects.into_iter(), &ObjectKind::FallFloor, 0, 1);
                if let Some((others, below)) = below {
                    let (fall_floor_object, fall_floor) = below.to_fall_floor_mut().unwrap();

                    // Have to break it here because others doesn't include this spring.
                    self.break_spring();
                    fall_floor.break_fall_floor::<VecObjects>(
                        fall_floor_object,
                        &mut VecObjects { vec: others },
                        got_fruit,
                        room,
                        max_djump,
                        &mut update_action,
                    )
                }
            }
        } else if self.delay > 0 {
            self.delay -= 1;
            if self.delay <= 0 {
                this.spr = 18.0;
            }
        }

        if self.hide_in > 0 {
            self.hide_in -= 1;

            if self.hide_in <= 0 {
                self.hide_for = 60;
                this.spr = 0.0;
            }
        }
        update_action
    }

    fn break_spring(&mut self) {
        self.hide_in = 15;
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct BaseObject {
    x: i32,
    y: i32,
    hitbox: Hitbox,
    spr: f32, // hack they use
    spd: Vec2<f32>,
    rem: Vec2<f32>,
    dir: i32, // not sure if all objects use this?
    // obj.solids in original source
    is_solid: bool,
    collideable: bool,
    flip: Vec2<bool>,
}

impl BaseObject {
    fn collide<'a>(
        &self,
        objects: impl Iterator<Item = &'a mut Object>,
        kind: &ObjectKind,
        ox: i32,
        oy: i32,
    ) -> Option<(Vec<&'a mut Object>, &'a mut Object)> {
        let mut others = vec![];

        let mut result = None;
        for other_object in objects {
            let other_type = &other_object.object_type;
            let other = other_object.base_object;

            if &other_type.kind() == kind && other.collideable // This kills rust-fmt?
                && other.x + other.hitbox.x + other.hitbox.w > self.x + self.hitbox.x + ox
                && other.y + other.hitbox.y + other.hitbox.h > self.y + self.hitbox.y + oy
                && other.x + other.hitbox.x < self.x + self.hitbox.x + self.hitbox.w + ox
                && other.y + other.hitbox.y < self.y + self.hitbox.y + self.hitbox.h + oy
            {
                result = Some(other_object);
                break;
            } else {
                others.push(other_object)
            }
        }
        result.map(|object| (others, object))
    }

    fn is_solid<T>(&self, state: &Pico8, objects: &mut T, room: Vec2<i32>, ox: i32, oy: i32) -> bool
    where
        for<'b> &'b mut T: IntoIterator<Item = &'b mut Object>,
    {
        if oy > 0
            && !self.check(objects.into_iter(), &ObjectKind::Platform, ox, 0)
            && self.check(objects.into_iter(), &ObjectKind::Platform, ox, oy)
        {
            return true;
        }

        solid_at(
            state,
            room,
            self.x + self.hitbox.x + ox,
            self.y + self.hitbox.y + oy,
            self.hitbox.w,
            self.hitbox.h,
        ) || self.check(objects.into_iter(), &ObjectKind::FallFloor, ox, oy)
            || self.check(objects.into_iter(), &ObjectKind::FakeWall, ox, oy)
    }

    fn check<'a>(
        &self,
        objects: impl Iterator<Item = &'a mut Object>,
        kind: &ObjectKind,
        ox: i32,
        oy: i32,
    ) -> bool {
        self.collide(objects, kind, ox, oy).is_some()
    }

    fn is_ice(&self, state: &Pico8, room: Vec2<i32>, ox: i32, oy: i32) -> bool {
        ice_at(
            state,
            room,
            self.x + self.hitbox.x + ox,
            self.y + self.hitbox.y + oy,
            self.hitbox.w,
            self.hitbox.h,
        )
    }
}

#[derive(Clone, Copy, Debug)]
struct LifeUp {
    duration: i32,
    flash: f32,
}

impl LifeUp {
    fn init(this: &mut BaseObject) -> Self {
        this.spd.y = -0.25;
        this.x -= 2;
        this.y -= 4;
        this.is_solid = false;

        Self {
            duration: 30,
            flash: 0.0,
        }
    }

    fn update(&mut self) -> UpdateAction {
        self.duration -= 1;

        UpdateAction::noop().destroy_if(self.duration <= 0)
    }

    fn draw(&mut self, this: &BaseObject, draw: &mut Pico8) {
        self.flash += 0.5;

        draw.print("1000", this.x - 2, this.y, 7 + (self.flash % 2.0) as u8);
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct RoomTitle {
    delay: i32,
}

impl RoomTitle {
    fn init() -> Self {
        Self { delay: 5 }
    }

    fn update(&mut self) -> UpdateAction {
        self.delay -= 1;

        UpdateAction::noop().destroy_if(self.delay < -30)
    }

    fn draw(&self, draw: &mut Pico8, room: Vec2<i32>, seconds: i32, minutes: i32) {
        if self.delay < 0 {
            draw.rectfill(24, 58, 104, 70, 0);

            if room.x == 3 && room.y == 1 {
                draw.print("OLD SITE", 48, 62, 7);
            } else if level_index(room) == 30 {
                draw.print("SUMMIT", 52, 62, 7);
            } else {
                let level = (1 + level_index(room)) * 100;
                let x = 52 + (if level < 1000 { 2 } else { 0 });
                draw.print(&format!("{} M", level), x, 62, 7);
            }

            draw_time(seconds, minutes, draw, 4, 4);
        }
    }
}

// -- object functions --

// -----------------------

#[derive(PartialEq, Debug, Clone, Copy)]
struct Hitbox {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

#[derive(Clone, Debug)]
struct Object {
    base_object: BaseObject,
    object_type: ObjectType,
}

fn got_fruit_for_room(got_fruit: &[bool], room: Vec2<i32>) -> bool {
    *got_fruit.get(1 + level_index(room)).unwrap_or(&false)
}

impl Object {
    #[must_use]
    fn init(
        got_fruit: &[bool],
        room: Vec2<i32>,
        kind: ObjectKind,
        x: i32,
        y: i32,
        max_djump: i32,
    ) -> Option<Self> {
        // What this means: If the fruit has been already
        // picked up, don't instantiate this (fake wall containing, flying fruits, chests, etc)
        if kind.if_not_fruit() && got_fruit_for_room(got_fruit, room) {
            return None;
        }

        let mut base_object = BaseObject {
            x,
            y,
            collideable: true,
            is_solid: true,
            hitbox: Hitbox {
                x: 0,
                y: 0,
                w: 8,
                h: 8,
            },
            spd: Vec2 { x: 0., y: 0. },
            rem: Vec2 { x: 0., y: 0. },
            dir: 0,
            flip: Vec2 { x: false, y: false },
            // TODO: figure out if we need an option here
            spr: kind.tile().map(|t| t as f32).unwrap_or(-42.),
        };
        let object_type = ObjectKind::create(&kind, &mut base_object, got_fruit, max_djump);

        Some(Self {
            base_object,
            object_type,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn update<T>(
        &mut self,
        other_objects: &mut T,
        effects: &mut GameEffects,
        got_fruit: &mut [bool],
        room: Vec2<i32>,
        state: &Pico8,
        max_djump: i32,
        has_dashed: &mut bool,
        frames: i32,
        has_key: &mut bool,
        pause_player: bool,
        freeze: &mut i32,
    ) -> UpdateAction
    where
        for<'b> &'b mut T: IntoIterator<Item = &'b mut Object>,
    {
        self.object_type.update(
            &mut self.base_object,
            other_objects,
            effects,
            got_fruit,
            room,
            state,
            max_djump,
            has_dashed,
            frames,
            has_key,
            pause_player,
            freeze,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn draw<T>(
        &mut self,
        draw: &mut Pico8,
        objects: &mut T,
        room: Vec2<i32>,
        got_fruit: &[bool],
        frames: i32,
        max_djump: &mut i32,
        shake: &mut i32,
        freeze: &mut i32,
        flash_bg: &mut bool,
        new_bg: &mut bool,
        pause_player: &mut bool,
        seconds: i32,
        minutes: i32,
        deaths: i32,
    ) -> UpdateAction
    where
        for<'b> &'b mut T: IntoIterator<Item = &'b mut Object>,
    {
        match &mut self.object_type {
            ObjectType::PlayerSpawn(player_spawn) => {
                player_spawn.draw(&mut self.base_object, draw, frames, *max_djump)
            }
            ObjectType::BigChest(big_chest) => {
                return big_chest.draw(
                    &self.base_object,
                    draw,
                    objects,
                    room,
                    got_fruit,
                    *max_djump,
                    shake,
                    flash_bg,
                    new_bg,
                    pause_player,
                )
            }
            ObjectType::Chest(_) => default_draw(&mut self.base_object, draw),
            ObjectType::Player(player) => player.draw(&mut self.base_object, draw, frames),
            ObjectType::FakeWall => FakeWall::draw(&mut self.base_object, draw),
            ObjectType::FallFloor(fall_floor) => fall_floor.draw(&mut self.base_object, draw),
            ObjectType::RoomTitle(room_title) => room_title.draw(draw, room, seconds, minutes),
            ObjectType::Platform(platform) => platform.draw(&self.base_object, draw),
            ObjectType::Smoke => default_draw(&mut self.base_object, draw),
            ObjectType::Fruit(_) => default_draw(&mut self.base_object, draw),
            ObjectType::LifeUp(life_up) => life_up.draw(&self.base_object, draw),
            ObjectType::Spring(_) => default_draw(&mut self.base_object, draw),
            ObjectType::FlyFruit(fly_fruit) => fly_fruit.draw(&mut self.base_object, draw),
            ObjectType::Key => default_draw(&mut self.base_object, draw),
            ObjectType::Balloon(balloon) => balloon.draw(&self.base_object, draw),
            ObjectType::Orb(orb) => {
                return orb.draw(
                    &mut self.base_object,
                    draw,
                    objects,
                    max_djump,
                    freeze,
                    shake,
                    frames,
                )
            }
            ObjectType::Message(message) => message.draw(&mut self.base_object, draw, objects),
            ObjectType::Flag(flag) => flag.draw(
                &mut self.base_object,
                draw,
                objects,
                frames,
                seconds,
                minutes,
                deaths,
            ),
        }

        UpdateAction::noop()
    }

    fn move_<T>(&mut self, state: &Pico8, objects: &mut T, room: Vec2<i32>)
    where
        for<'b> &'b mut T: IntoIterator<Item = &'b mut Object>,
    {
        let ox = self.base_object.spd.x;
        let oy = self.base_object.spd.y;

        // [x] get move amount
        self.base_object.rem.x += ox;
        let amount_x = (self.base_object.rem.x + 0.5).floor();
        self.base_object.rem.x -= amount_x;
        self.move_x(state, objects, room, amount_x as i32, 0);

        // [y] get move amount
        self.base_object.rem.y += oy;
        let amount_y = (self.base_object.rem.y + 0.5).floor();
        self.base_object.rem.y -= amount_y;
        self.move_y(state, objects, room, amount_y as i32);
    }

    fn move_x<T>(
        &mut self,
        state: &Pico8,
        objects: &mut T,
        room: Vec2<i32>,
        amount: i32,
        start: i32,
    ) where
        for<'b> &'b mut T: IntoIterator<Item = &'b mut Object>,
    {
        if self.base_object.is_solid {
            let step = signi(amount);

            for _ in start..=amount.abs() {
                if !self.is_solid(state, objects, room, step, 0) {
                    self.base_object.x += step;
                } else {
                    self.base_object.spd.x = 0.;
                    self.base_object.rem.x = 0.;
                    break;
                }
            }
        } else {
            self.base_object.x += amount;
        }
    }

    fn move_y<T>(&mut self, state: &Pico8, objects: &mut T, room: Vec2<i32>, amount: i32)
    where
        for<'b> &'b mut T: IntoIterator<Item = &'b mut Object>,
    {
        if self.base_object.is_solid {
            let step = signi(amount);

            for _ in 0..=amount.abs() {
                if !self.is_solid(state, objects, room, 0, step) {
                    self.base_object.y += step;
                } else {
                    self.base_object.spd.y = 0.;
                    self.base_object.rem.y = 0.;
                    break;
                }
            }
        } else {
            self.base_object.y += amount;
        }
    }

    fn is_solid<T>(&self, state: &Pico8, objects: &mut T, room: Vec2<i32>, ox: i32, oy: i32) -> bool
    where
        for<'b> &'b mut T: IntoIterator<Item = &'b mut Object>,
    {
        self.base_object.is_solid(state, objects, room, ox, oy)
    }

    fn to_player_mut(&mut self) -> Option<(&mut BaseObject, &mut Player)> {
        match &mut self.object_type {
            ObjectType::Player(player) => Some((&mut self.base_object, player)),
            _ => None,
        }
    }

    fn to_spring_mut(&mut self) -> Option<(&mut BaseObject, &mut Spring)> {
        match &mut self.object_type {
            ObjectType::Spring(spring) => Some((&mut self.base_object, spring)),
            _ => None,
        }
    }

    fn to_fall_floor_mut(&mut self) -> Option<(&mut BaseObject, &mut FallFloor)> {
        match &mut self.object_type {
            ObjectType::FallFloor(fall_floor) => Some((&mut self.base_object, fall_floor)),
            _ => None,
        }
    }

    fn kind(&self) -> ObjectKind {
        self.object_type.kind()
    }
}

fn default_draw(base_object: &mut BaseObject, draw: &mut Pico8) {
    if base_object.spr > 0. {
        draw.spr_(
            base_object.spr.floor() as usize,
            base_object.x,
            base_object.y,
            1.0,
            1.0,
            base_object.flip.x,
            base_object.flip.y,
        );
    }

    // base_object
    //     .hitbox
    //     .draw(draw, base_object.x, base_object.y, 3);
}

fn tile_flag_at(state: &Pico8, room: Vec2<i32>, x: i32, y: i32, w: i32, h: i32, flag: u8) -> bool {
    let x_min = i32::max(0, flr(x as f32 / 8.0));
    let x_max = i32::min(15, flr((x as f32 + w as f32 - 1.0) / 8.0));
    for i in x_min..=x_max {
        let y_min = i32::max(0, flr(y as f32 / 8.0));
        let y_max = i32::min(15, flr((y as f32 + h as f32 - 1.0) / 8.0));
        for j in y_min..=y_max {
            if state.fget_n(tile_at(state, room, i, j), flag) {
                return true;
            }
        }
    }
    false
}

fn tile_at(state: &Pico8, room: Vec2<i32>, x: i32, y: i32) -> usize {
    state.mget(room.x * 16 + x, room.y * 16 + y).into()
}

fn solid_at(state: &Pico8, room: Vec2<i32>, x: i32, y: i32, w: i32, h: i32) -> bool {
    tile_flag_at(state, room, x, y, w, h, 0)
}

#[derive(Clone, Debug)]
enum ObjectType {
    Platform(Platform),
    Chest(Chest),
    Balloon(Balloon),
    BigChest(BigChest),
    Player(Player),
    PlayerSpawn(PlayerSpawn),
    Smoke,
    LifeUp(LifeUp),
    Fruit(Fruit),
    FlyFruit(FlyFruit),
    Orb(Orb),
    FakeWall,
    FallFloor(FallFloor),
    Key,
    RoomTitle(RoomTitle),
    Spring(Spring),
    Message(Message),
    Flag(Flag),
}

impl ObjectType {
    fn kind(&self) -> ObjectKind {
        match self {
            ObjectType::Platform(_) => ObjectKind::Platform,
            ObjectType::Player(_) => ObjectKind::Player,
            ObjectType::PlayerSpawn(_) => ObjectKind::PlayerSpawn,
            ObjectType::RoomTitle(_) => ObjectKind::RoomTitle,
            ObjectType::Smoke => ObjectKind::Smoke,
            ObjectType::FakeWall => ObjectKind::FakeWall,
            ObjectType::Fruit(_) => ObjectKind::Fruit,
            ObjectType::LifeUp(_) => ObjectKind::LifeUp,
            ObjectType::Spring(_) => ObjectKind::Spring,
            ObjectType::FlyFruit(_) => ObjectKind::FlyFruit,
            ObjectType::FallFloor(_) => ObjectKind::FallFloor,
            ObjectType::Key => ObjectKind::Key,
            ObjectType::Chest(_) => ObjectKind::Chest,
            ObjectType::Balloon(_) => ObjectKind::Balloon,
            ObjectType::BigChest(_) => ObjectKind::BigChest,
            ObjectType::Orb(_) => ObjectKind::Orb,
            ObjectType::Message(_) => ObjectKind::Message,
            ObjectType::Flag(_) => ObjectKind::Flag,
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn update<T>(
        &mut self,
        base_object: &mut BaseObject,
        other_objects: &mut T,
        effects: &mut GameEffects,
        got_fruit: &mut [bool],
        room: Vec2<i32>,
        state: &Pico8,
        max_djump: i32,
        has_dashed: &mut bool,
        frames: i32,
        has_key: &mut bool,
        pause_player: bool,
        freeze: &mut i32,
    ) -> UpdateAction
    where
        for<'b> &'b mut T: IntoIterator<Item = &'b mut Object>,
    {
        match self {
            ObjectType::PlayerSpawn(player_spawn) => {
                player_spawn.update(base_object, effects, got_fruit, room, max_djump)
            }
            ObjectType::Smoke => Smoke::update(base_object),
            ObjectType::Platform(platform) => {
                platform.update(base_object, state, other_objects, room)
            }
            ObjectType::BigChest(_) => UpdateAction::noop(),
            ObjectType::Player(player) => player.update(
                base_object,
                state,
                other_objects,
                got_fruit,
                room,
                max_djump,
                has_dashed,
                pause_player,
                &mut effects.shake,
                freeze,
            ),
            ObjectType::LifeUp(life_up) => life_up.update(),
            ObjectType::Fruit(fruit) => {
                fruit.update(base_object, other_objects, got_fruit, room, max_djump)
            }
            ObjectType::FakeWall => {
                FakeWall::update(base_object, other_objects, got_fruit, room, max_djump)
            }
            ObjectType::FallFloor(fall_floor) => {
                fall_floor.update(base_object, other_objects, got_fruit, room, max_djump)
            }
            ObjectType::Key => Key::update(base_object, other_objects, frames, has_key),
            ObjectType::RoomTitle(rt) => rt.update(),
            ObjectType::Spring(spring) => {
                spring.update(base_object, other_objects, got_fruit, room, max_djump)
            }
            ObjectType::FlyFruit(fly_fruit) => fly_fruit.update(
                base_object,
                other_objects,
                got_fruit,
                room,
                max_djump,
                *has_dashed,
            ),
            ObjectType::Chest(chest) => {
                chest.update(base_object, got_fruit, room, *has_key, max_djump)
            }
            ObjectType::Balloon(balloon) => {
                balloon.update(base_object, other_objects, got_fruit, room, max_djump)
            }
            ObjectType::Orb(_) => UpdateAction::noop(),
            ObjectType::Message(_) => UpdateAction::noop(),
            ObjectType::Flag(_) => UpdateAction::noop(),
        }
    }
}

fn kill_player(game_state: &mut GameState) {
    game_state.sfx_timer = 12;
    // sfx(0);
    game_state.deaths += 1;
    game_state.effects.shake = 10;

    game_state.dead_particles.clear();
    game_state.objects.retain(|obj| {
        let is_player = obj.kind() == ObjectKind::Player;

        if is_player {
            for dir in 0..=7 {
                let dir = dir as f32;
                let angle = dir / 8.;

                game_state.dead_particles.push(DeadParticle {
                    x: (obj.base_object.x + 4) as f32,
                    y: (obj.base_object.y + 4) as f32,
                    t: 10,
                    spd: Vec2 {
                        x: sin(angle) * 3.,
                        y: cos(angle) * 3.,
                    },
                });
            }
        }

        !is_player
    });

    restart_room(game_state)
}

// -- room functions --
// --------------------

fn restart_room(game_state: &mut GameState) {
    game_state.will_restart = true;
    game_state.delay_restart = 15;
}

fn next_room(game_state: &mut GameState, state: &Pico8) {
    let room = game_state.room;

    #[allow(clippy::if_same_then_else)]
    if room.x == 2 && room.y == 1 {
        // music(30, 500, 7)
    } else if room.x == 3 && room.y == 1 {
        // music(20, 500, 7)
    } else if room.x == 4 && room.y == 2 {
        // music(30, 500, 7)
    } else if room.x == 5 && room.y == 3 {
        // music(30, 500, 7)
    }
    if room.x == 7 {
        load_room(game_state, state, 0, room.y + 1);
    } else {
        load_room(game_state, state, room.x + 1, room.y);
    }
}

fn load_room(game_state: &mut GameState, state: &Pico8, x: i32, y: i32) {
    game_state.has_dashed = false;
    game_state.has_key = false;

    // Remove existing objects
    game_state.objects.clear();

    // Current room
    game_state.room.x = x;
    game_state.room.y = y;

    for tx in 0..=15 {
        for ty in 0..=15 {
            // entities
            let tile = state.mget(game_state.room.x * 16 + tx, game_state.room.y * 16 + ty);
            if tile == 11 {
                let mut platform = Object::init(
                    &game_state.got_fruit,
                    game_state.room,
                    ObjectKind::Platform,
                    tx * 8,
                    ty * 8,
                    game_state.max_djump,
                )
                .unwrap();
                platform.base_object.dir = -1;
                game_state.objects.push(platform);
            } else if tile == 12 {
                let mut platform = Object::init(
                    &game_state.got_fruit,
                    game_state.room,
                    ObjectKind::Platform,
                    tx * 8,
                    ty * 8,
                    game_state.max_djump,
                )
                .unwrap();
                platform.base_object.dir = 1;
                game_state.objects.push(platform);
            } else {
                for kind in ObjectKind::TYPES.iter().copied() {
                    if kind.tile() == Some(tile.into()) {
                        if let Some(object) = Object::init(
                            &game_state.got_fruit,
                            game_state.room,
                            kind,
                            tx * 8,
                            ty * 8,
                            game_state.max_djump,
                        ) {
                            game_state.objects.push(object);
                        }
                    }
                }
            }
        }
    }

    if !is_title(game_state) {
        if let Some(object) = Object::init(
            &game_state.got_fruit,
            game_state.room,
            ObjectKind::RoomTitle,
            0,
            0,
            game_state.max_djump,
        ) {
            game_state.objects.push(object);
        }
    }
}

fn draw_time(seconds: i32, minutes: i32, draw: &mut Pico8, x: i32, y: i32) {
    let s = seconds;
    let m = minutes % 60;
    let h = minutes / 60;

    let h_str = if h < 10 {
        format!("0{}", h)
    } else {
        h.to_string()
    };
    let m_str = if m < 10 {
        format!("0{}", m)
    } else {
        m.to_string()
    };
    let s_str = if s < 10 {
        format!("0{}", s)
    } else {
        s.to_string()
    };
    let time_str = format!("{}:{}:{}", h_str, m_str, s_str);

    draw.rectfill(x, y, x + 32, y + 6, 0);
    draw.print(&time_str, x + 1, y + 1, 7);
}

// -- helper functions --
// ----------------------

fn clamp(val: i32, a: i32, b: i32) -> i32 {
    a.max(b.min(val))
}

fn appr(val: f32, target: f32, amount: f32) -> f32 {
    if val > target {
        f32::max(val - amount, target)
    } else {
        f32::min(val + amount, target)
    }
}

fn maybe() -> bool {
    rnd(1.0) > 0.5
}

fn ice_at(state: &Pico8, room: Vec2<i32>, x: i32, y: i32, w: i32, h: i32) -> bool {
    tile_flag_at(state, room, x, y, w, h, 4)
}

#[allow(clippy::too_many_arguments)]
fn spikes_at(
    state: &Pico8,
    room: Vec2<i32>,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    xspd: f32,
    yspd: f32,
) -> bool {
    for i in i32::max(0, flr(x as f32 / 8.0))..=i32::min(15, flr((x + w - 1) as f32 / 8.0)) {
        for j in i32::max(0, flr(y as f32 / 8.))..=i32::min(15, flr((y + h - 1) as f32 / 8.0)) {
            let tile = tile_at(state, room, i, j);

            let spikes_up =
                tile == 17 && ((y + h - 1) % 8 >= 6 || y + h == j * 8 + 8) && yspd >= 0.0;
            let spikes_down = tile == 27 && y % 8 <= 2 && yspd <= 0.0;
            let spikes_right = tile == 43 && x % 8 <= 2 && xspd <= 0.0;
            let spikes_left =
                tile == 59 && ((x + w - 1) % 8 >= 6 || x + w == i * 8 + 8) && xspd >= 0.0;

            if spikes_up || spikes_down || spikes_right || spikes_left {
                return true;
            }
        }
    }

    false
}

#[derive(Clone, Debug)]
struct Platform {
    last: i32,
}

impl Platform {
    fn init(this: &mut BaseObject) -> Self {
        this.x -= 4;
        this.is_solid = false;
        this.hitbox.w = 16;

        Self { last: this.x }
    }

    fn update<T>(
        &mut self,
        this: &mut BaseObject,
        state: &Pico8,
        objects: &mut T,
        room: Vec2<i32>,
    ) -> UpdateAction
    where
        for<'b> &'b mut T: IntoIterator<Item = &'b mut Object>,
    {
        this.spd.x = this.dir as f32 * 0.65;
        if this.x < -16 {
            this.x = 128;
        } else if this.x > 128 {
            this.x = -16;
        }

        // Platforms drag you
        if !this.check(objects.into_iter(), &ObjectKind::Player, 0, 0) {
            if let Some((objects, hit)) =
                this.collide(objects.into_iter(), &ObjectKind::Player, 0, -1)
            {
                // TODO: other_objects here won't include the current platform. Is that a problem?
                let mut other_objects = VecObjects { vec: objects };
                hit.move_x::<VecObjects<'_>>(
                    state,
                    &mut other_objects,
                    room,
                    this.x - self.last,
                    1,
                );
            }
        }

        self.last = this.x;

        UpdateAction::noop()
    }

    fn draw(&self, this: &BaseObject, draw: &mut Pico8) {
        draw.spr(11, this.x, this.y - 1);
        draw.spr(12, this.x + 8, this.y - 1)
    }
}

#[derive(Clone, Copy)]
struct Smoke;

impl Smoke {
    fn init(base_object: &mut BaseObject) {
        base_object.spr = 29.;
        base_object.spd.y = -0.1;
        base_object.spd.x = 0.3 + rnd(0.2);
        base_object.x += -1 + flr(rnd(2.));
        base_object.y += -1 + flr(rnd(2.));
        base_object.flip.x = maybe();
        base_object.flip.y = maybe();
        base_object.is_solid = false;
    }

    fn update(base_object: &mut BaseObject) -> UpdateAction {
        base_object.spr += 0.2;

        UpdateAction::noop().destroy_if(base_object.spr >= 32.0)
    }
}

#[derive(Clone, Copy, Debug)]
struct Fruit {
    start: i32,
    off: i32,
}

impl Fruit {
    fn init(base_object: &mut BaseObject) -> Self {
        Self {
            start: base_object.y,
            off: 0,
        }
    }

    fn update<T>(
        &mut self,
        base_object: &mut BaseObject,
        other_objects: &mut T,
        got_fruit: &mut [bool],
        room: Vec2<i32>,
        max_djump: i32,
    ) -> UpdateAction
    where
        for<'b> &'b mut T: IntoIterator<Item = &'b mut Object>,
    {
        let update_action = if let Some((_, hit)) =
            base_object.collide(other_objects.into_iter(), &ObjectKind::Player, 0, 0)
        {
            let (_, player) = hit.to_player_mut().unwrap();

            player.djump = max_djump;
            // sfx_timer=20
            // sfx(13)
            got_fruit[level_index(room)] = true;

            UpdateAction::noop().destroy().push(Object::init(
                got_fruit,
                room,
                ObjectKind::LifeUp,
                base_object.x,
                base_object.y + 4,
                max_djump,
            ))
        } else {
            UpdateAction::noop()
        };

        self.off += 1;
        base_object.y = self.start + (sin(self.off as f32 / 40.0) * 2.5) as i32;

        update_action
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ObjectKind {
    PlayerSpawn,
    Player,
    Spring,
    Balloon,
    FallFloor,
    Fruit,
    FlyFruit,
    FakeWall,
    Key,
    Chest,
    Message,
    BigChest,
    Flag,

    // Non-tile-instantiable
    RoomTitle,
    Platform,
    Smoke,
    LifeUp,
    Orb,
}

impl ObjectKind {
    // These are the "instantiable" objects
    // (you put a "marker" tile in the map and this creates the object for it)
    // see line 1135 of source.p8
    const TYPES: &'static [Self] = &[
        ObjectKind::PlayerSpawn,
        ObjectKind::Spring,
        ObjectKind::Balloon,
        ObjectKind::FallFloor,
        ObjectKind::Fruit,
        ObjectKind::FlyFruit,
        ObjectKind::FakeWall,
        ObjectKind::Key,
        ObjectKind::Chest,
        ObjectKind::Message,
        ObjectKind::BigChest,
        ObjectKind::Flag,
    ];

    fn create(
        &self,
        base_object: &mut BaseObject,
        got_fruit: &[bool],
        max_djump: i32,
    ) -> ObjectType {
        match self {
            ObjectKind::PlayerSpawn => ObjectType::PlayerSpawn(PlayerSpawn::init(base_object)),
            ObjectKind::Player => ObjectType::Player(Player::init(base_object, max_djump)),

            ObjectKind::Spring => ObjectType::Spring(Spring::init()),
            ObjectKind::Balloon => ObjectType::Balloon(Balloon::init(base_object)),
            ObjectKind::FallFloor => ObjectType::FallFloor(FallFloor::init()),
            ObjectKind::Fruit => ObjectType::Fruit(Fruit::init(base_object)),
            ObjectKind::FlyFruit => ObjectType::FlyFruit(FlyFruit::init(base_object)),

            ObjectKind::Key => ObjectType::Key,
            ObjectKind::Chest => ObjectType::Chest(Chest::init(base_object)),
            ObjectKind::Message => ObjectType::Message(Message::init()),
            ObjectKind::BigChest => ObjectType::BigChest(BigChest::init(base_object)),
            ObjectKind::Flag => ObjectType::Flag(Flag::init(base_object, got_fruit)),
            ObjectKind::RoomTitle => ObjectType::RoomTitle(RoomTitle::init()),
            ObjectKind::Platform => ObjectType::Platform(Platform::init(base_object)),
            ObjectKind::Smoke => {
                Smoke::init(base_object);
                ObjectType::Smoke
            }
            ObjectKind::FakeWall => ObjectType::FakeWall,
            ObjectKind::LifeUp => ObjectType::LifeUp(LifeUp::init(base_object)),
            ObjectKind::Orb => ObjectType::Orb(Orb::init(base_object)),
        }
    }
    fn tile(&self) -> Option<i32> {
        match self {
            ObjectKind::PlayerSpawn => Some(1),
            ObjectKind::Spring => Some(18),
            ObjectKind::Balloon => Some(22),
            ObjectKind::FallFloor => Some(23),
            ObjectKind::Fruit => Some(26),
            ObjectKind::FlyFruit => Some(28),
            ObjectKind::FakeWall => Some(64),
            ObjectKind::Key => Some(8),
            ObjectKind::Chest => Some(20),
            ObjectKind::Message => Some(86),
            ObjectKind::BigChest => Some(96),
            ObjectKind::Flag => Some(118),
            _ => None,
        }
    }

    fn if_not_fruit(&self) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match self {
            ObjectKind::Fruit => true,
            ObjectKind::FlyFruit => true,
            ObjectKind::FakeWall => true,
            ObjectKind::Key => true,
            ObjectKind::Chest => true,
            _ => false,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct Hair {
    segments: [HairElement; 5],
}

impl Hair {
    fn draw(&mut self, draw: &mut Pico8, x: i32, y: i32, facing: i32) {
        let mut last = Vec2 {
            x: (x + (4 - facing * 2)) as f32,
            y: (y + (if draw.btn(K_DOWN) { 4 } else { 3 })) as f32,
        };

        for hair_element in self.segments.iter_mut() {
            hair_element.x += (last.x - hair_element.x) / 1.5;
            hair_element.y += (last.y + 0.5 - hair_element.y) / 1.5;

            draw.circfill(
                hair_element.x.floor() as i32,
                hair_element.y.floor() as i32,
                hair_element.size,
                8,
            );

            last = Vec2 {
                x: hair_element.x,
                y: hair_element.y,
            };
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct PlayerSpawn {
    hair: Hair,
    delay: i32,
    target: Vec2<i32>,
    state: PlayerSpawnState,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum PlayerSpawnState {
    Jumping,
    Falling,
    Landing,
}

struct UpdateAction {
    should_destroy: bool,
    next_level: bool,
    new_objects: Vec<Object>,
}

impl UpdateAction {
    fn noop() -> Self {
        Self {
            should_destroy: false,
            next_level: false,
            new_objects: vec![],
        }
    }

    fn destroy_if_mut(&mut self, should_destroy: bool) {
        self.should_destroy = should_destroy;
    }

    fn destroy_if(mut self, should_destroy: bool) -> Self {
        self.destroy_if_mut(should_destroy);

        self
    }

    fn destroy(self) -> Self {
        self.destroy_if(true)
    }

    fn next_level(&mut self) {
        self.next_level = true;
    }

    fn push_mut(&mut self, object: Option<Object>) {
        if let Some(object) = object {
            self.new_objects.push(object);
        }
    }

    fn push(mut self, object: Option<Object>) -> Self {
        self.push_mut(object);

        self
    }
}

impl PlayerSpawn {
    fn init(base_object: &mut BaseObject) -> Self {
        use PlayerSpawnState::*;

        // TODO: Implement sound api
        // sfx(4)
        base_object.spr = 3.0;
        let target = Vec2 {
            x: base_object.x,
            y: base_object.y,
        };
        base_object.y = 128;
        base_object.spd.y = -4.0;
        base_object.is_solid = false;

        Self {
            hair: Player::create_hair(base_object.x, base_object.y),
            delay: 0,
            target,
            state: Jumping,
        }
    }

    fn update(
        &mut self,
        base_object: &mut BaseObject,
        effects: &mut GameEffects,
        got_fruit: &[bool],
        room: Vec2<i32>,
        max_djump: i32,
    ) -> UpdateAction {
        match self.state {
            PlayerSpawnState::Jumping => {
                if base_object.y < self.target.y + 16 {
                    self.state = PlayerSpawnState::Falling;
                    self.delay = 3;
                }

                UpdateAction::noop()
            }
            PlayerSpawnState::Falling => {
                base_object.spd.y += 0.5;

                if base_object.spd.y > 0.0 && self.delay > 0 {
                    base_object.spd.y = 0.0;
                    self.delay -= 1;
                }

                let mut update_action = UpdateAction::noop();
                if base_object.spd.y > 0.0 && base_object.y > self.target.y {
                    base_object.y = self.target.y;
                    base_object.spd = Vec2 { x: 0.0, y: 0.0 };
                    self.state = PlayerSpawnState::Landing;
                    self.delay = 5;

                    effects.shake = 5;
                    update_action.push_mut(Object::init(
                        got_fruit,
                        room,
                        ObjectKind::Smoke,
                        base_object.x,
                        base_object.y + 4,
                        max_djump,
                    ));

                    // sfx(5);
                };

                update_action
            }
            PlayerSpawnState::Landing => {
                self.delay -= 1;
                base_object.spr = 6.0;

                let should_destroy = self.delay < 0;

                if should_destroy {
                    let player = Object::init(
                        got_fruit,
                        room,
                        ObjectKind::Player,
                        base_object.x,
                        base_object.y,
                        max_djump,
                    );

                    UpdateAction::noop().destroy().push(player)
                } else {
                    UpdateAction::noop()
                }
            }
        }
    }

    fn draw(
        &mut self,
        base_object: &mut BaseObject,
        draw: &mut Pico8,
        frames: i32,
        max_djump: i32,
    ) {
        set_hair_color(draw, frames, max_djump);

        self.hair.draw(draw, base_object.x, base_object.y, 1);
        draw.spr_(
            base_object.spr.floor() as usize,
            base_object.x,
            base_object.y,
            1.0,
            1.0,
            base_object.flip.x,
            base_object.flip.y,
        );
        unset_hair_color(draw);
    }
}

struct FakeWall;

impl FakeWall {
    fn update<T>(
        base_object: &mut BaseObject,
        other_objects: &mut T,
        got_fruit: &[bool],
        room: Vec2<i32>,
        max_djump: i32,
    ) -> UpdateAction
    where
        for<'b> &'b mut T: IntoIterator<Item = &'b mut Object>,
    {
        base_object.hitbox = Hitbox {
            x: -1,
            y: -1,
            w: 18,
            h: 18,
        };

        let mut update_action = UpdateAction::noop();

        if let Some((_, hit_object)) =
            base_object.collide(other_objects.into_iter(), &ObjectKind::Player, 0, 0)
        {
            if let ObjectType::Player(player) = &mut hit_object.object_type {
                if player.dash_effect_time > 0 {
                    hit_object.base_object.spd.x = -sign(hit_object.base_object.spd.x) * 1.5;
                    hit_object.base_object.spd.y = -1.5;
                    player.dash_time = -1;
                    // TODO:
                    //     sfx_timer=20
                    //     sfx(16)

                    update_action = update_action
                        .destroy()
                        .push(Object::init(
                            got_fruit,
                            room,
                            ObjectKind::Smoke,
                            base_object.x,
                            base_object.y,
                            max_djump,
                        ))
                        .push(Object::init(
                            got_fruit,
                            room,
                            ObjectKind::Smoke,
                            base_object.x + 8,
                            base_object.y,
                            max_djump,
                        ))
                        .push(Object::init(
                            got_fruit,
                            room,
                            ObjectKind::Smoke,
                            base_object.x,
                            base_object.y + 8,
                            max_djump,
                        ))
                        .push(Object::init(
                            got_fruit,
                            room,
                            ObjectKind::Smoke,
                            base_object.x + 8,
                            base_object.y + 8,
                            max_djump,
                        ))
                        .push(Object::init(
                            got_fruit,
                            room,
                            ObjectKind::Fruit,
                            base_object.x + 4,
                            base_object.y + 4,
                            max_djump,
                        ));
                };
            } else {
                panic!("Got a different object than a player on collide(player)")
            }
        }

        // If I add this this the wall stops breaking, probably something to do with
        // the order of updates?
        base_object.hitbox = Hitbox {
            x: 0,
            y: 0,
            w: 16,
            h: 16,
        };

        update_action
    }

    fn draw(base_object: &mut BaseObject, draw: &mut Pico8) {
        let x = base_object.x;
        let y = base_object.y;
        draw.spr(64, x, y);
        draw.spr(65, x + 8, y);
        draw.spr(80, x, y + 8);
        draw.spr(81, x + 8, y + 8);
    }
}

fn horizontal_input(state: &Pico8) -> i32 {
    if state.btn(K_RIGHT) {
        1
    } else if state.btn(K_LEFT) {
        -1
    } else {
        0
    }
}

fn vertical_input(state: &Pico8) -> i32 {
    #[allow(clippy::bool_to_int_with_if)]
    if state.btn(K_UP) {
        -1
    } else if state.btn(K_DOWN) {
        1
    } else {
        0
    }
}
fn signi(i: i32) -> i32 {
    match i.cmp(&0) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

fn sign(f: f32) -> f32 {
    match f.partial_cmp(&0.0).unwrap() {
        std::cmp::Ordering::Less => -1.0,
        std::cmp::Ordering::Equal => 0.0,
        std::cmp::Ordering::Greater => 1.0,
    }
}

fn turn_to_rad(turns: f32) -> f32 {
    turns * 2.0 * PI
}

fn sin(turns: f32) -> f32 {
    turn_to_rad(turns).sin()
}

fn cos(turns: f32) -> f32 {
    turn_to_rad(turns).cos()
}

fn flr(f: f32) -> i32 {
    f.floor() as i32
}

#[derive(Clone, Copy, Debug)]
struct FlyFruit {
    start: i32,
    fly: bool,
    step: f32,
    sfx_delay: i32,
}

impl FlyFruit {
    fn init(this: &mut BaseObject) -> Self {
        this.is_solid = false;

        Self {
            start: this.y,
            fly: false,
            step: 0.5,
            sfx_delay: 8,
        }
    }

    fn update<T>(
        &mut self,
        this: &mut BaseObject,
        objects: &mut T,
        got_fruit: &mut [bool],
        room: Vec2<i32>,
        max_djump: i32,
        has_dashed: bool,
    ) -> UpdateAction
    where
        for<'b> &'b mut T: IntoIterator<Item = &'b mut Object>,
    {
        let mut update_action = UpdateAction::noop();
        // -- fly away
        if self.fly {
            if self.sfx_delay > 0 {
                self.sfx_delay -= 1;

                if self.sfx_delay <= 0 {
                    // sfx_timer = 20;
                    // sfx(14);
                }
            }
            this.spd.y = appr(this.spd.y, -3.5, 0.25);

            update_action.destroy_if_mut(this.y < -16);
        }
        // -- wait
        else {
            if has_dashed {
                self.fly = true;
            }
            self.step += 0.05;
            this.spd.y = sin(self.step) * 0.5;
        }
        // -- collect
        if let Some((_, hit)) = this.collide(objects.into_iter(), &ObjectKind::Player, 0, 0) {
            let (_, player) = hit.to_player_mut().unwrap();
            player.djump = max_djump;
            // sfx_timer = 20;
            // sfx(13);
            got_fruit[1 + level_index(room)] = true;
            update_action.push_mut(Object::init(
                got_fruit,
                room,
                ObjectKind::LifeUp,
                this.x,
                this.y,
                max_djump,
            ));
            update_action.destroy_if_mut(true);
        }

        update_action
    }

    fn draw(&mut self, this: &mut BaseObject, draw: &mut Pico8) {
        let mut off = 0.0;

        if !self.fly {
            let dir = sin(self.step);

            if dir < 0.0 {
                off = 1.0 + i32::max(0, signi(this.y - self.start)) as f32;
            }
        } else {
            off = (off + 0.25) % 3.0;
        }

        draw.spr_(
            flr(45.0 + off) as usize,
            this.x - 6,
            this.y - 2,
            1.0,
            1.0,
            true,
            false,
        );
        draw.spr(flr(this.spr) as usize, this.x, this.y);
        draw.spr(flr(45.0 + off) as usize, this.x + 6, this.y - 2);
    }
}

#[derive(Clone, Copy, Debug)]
enum FallFloorState {
    Idling,
    Shaking,
    Invisible,
}

#[derive(Clone, Copy, Debug)]
struct FallFloor {
    state: FallFloorState,
    delay: i32,
}

impl FallFloor {
    fn init() -> Self {
        Self {
            state: FallFloorState::Idling,
            delay: 0,
        }
    }

    fn update<T>(
        &mut self,
        this: &mut BaseObject,
        objects: &mut T,
        got_fruit: &[bool],
        room: Vec2<i32>,
        max_djump: i32,
    ) -> UpdateAction
    where
        for<'b> &'b mut T: IntoIterator<Item = &'b mut Object>,
    {
        let mut update_action = UpdateAction::noop();
        match self.state {
            FallFloorState::Idling => {
                if this.check(objects.into_iter(), &ObjectKind::Player, 0, -1)
                    || this.check(objects.into_iter(), &ObjectKind::Player, -1, 0)
                    || this.check(objects.into_iter(), &ObjectKind::Player, 1, 0)
                {
                    self.break_fall_floor(
                        this,
                        objects,
                        got_fruit,
                        room,
                        max_djump,
                        &mut update_action,
                    );
                }
            }
            FallFloorState::Shaking => {
                self.delay -= 1;

                if self.delay <= 0 {
                    self.state = FallFloorState::Invisible;
                    self.delay = 60;
                    this.collideable = false;
                }
            }
            FallFloorState::Invisible => {
                self.delay -= 1;

                if self.delay <= 0 && !this.check(objects.into_iter(), &ObjectKind::Player, 0, 0) {
                    // psfx(7)
                    self.state = FallFloorState::Idling;
                    this.collideable = true;

                    update_action.push_mut(Object::init(
                        got_fruit,
                        room,
                        ObjectKind::Smoke,
                        this.x,
                        this.y,
                        max_djump,
                    ));
                }
            }
        }
        update_action
    }

    fn draw(&self, this: &mut BaseObject, draw: &mut Pico8) {
        match self.state {
            FallFloorState::Idling => draw.spr(23, this.x, this.y),
            FallFloorState::Shaking => {
                draw.spr((23 + (15 - self.delay) / 5) as usize, this.x, this.y)
            }
            FallFloorState::Invisible => {}
        }
    }

    fn break_fall_floor<T>(
        &mut self,
        this: &mut BaseObject,
        objects: &mut T,
        got_fruit: &[bool],
        room: Vec2<i32>,
        max_djump: i32,
        update_action: &mut UpdateAction,
    ) where
        for<'b> &'b mut T: IntoIterator<Item = &'b mut Object>,
    {
        match self.state {
            FallFloorState::Idling => {
                // psfx(15);
                self.state = FallFloorState::Shaking;
                self.delay = 15; // --how long until it falls

                update_action.push_mut(Object::init(
                    got_fruit,
                    room,
                    ObjectKind::Smoke,
                    this.x,
                    this.y,
                    max_djump,
                ));

                if let Some((_, hit)) =
                    this.collide(objects.into_iter(), &ObjectKind::Spring, 0, -1)
                {
                    let (_, spring) = hit.to_spring_mut().unwrap();

                    spring.break_spring();
                }
            }
            FallFloorState::Shaking => {}
            FallFloorState::Invisible => {}
        }
    }
}

struct Key;

impl Key {
    fn update<T>(
        this: &mut BaseObject,
        objects: &mut T,
        frames: i32,
        has_key: &mut bool,
    ) -> UpdateAction
    where
        for<'b> &'b mut T: IntoIterator<Item = &'b mut Object>,
    {
        let mut update_action = UpdateAction::noop();

        let was = flr(this.spr);
        this.spr = 9.0 + (sin(frames as f32 / 30.0) + 0.5) * 1.0;
        let is = flr(this.spr);
        if is == 10 && is != was {
            this.flip.x = !this.flip.x;
        }
        if this.check(objects.into_iter(), &ObjectKind::Player, 0, 0) {
            // sfx(23);
            // sfx_timer = 10;
            update_action.destroy_if_mut(true);
            *has_key = true;
        }

        update_action
    }
}

#[derive(Clone, Copy, Debug)]
struct Chest {
    start: i32,
    timer: i32,
}

impl Chest {
    fn init(this: &mut BaseObject) -> Self {
        this.x -= 4;
        Self {
            start: this.x,
            timer: 20,
        }
    }

    fn update(
        &mut self,
        this: &mut BaseObject,
        got_fruit: &[bool],
        room: Vec2<i32>,
        has_key: bool,
        max_djump: i32,
    ) -> UpdateAction {
        let mut update_action = UpdateAction::noop();
        if has_key {
            self.timer -= 1;
            this.x = self.start - 1 + flr(rnd(3.0));
            if self.timer <= 0 {
                // sfx_timer = 20;
                // sfx(16);
                update_action.push_mut(Object::init(
                    got_fruit,
                    room,
                    ObjectKind::Fruit,
                    this.x,
                    this.y - 4,
                    max_djump,
                ));
                update_action.destroy_if_mut(true);
            }
        }

        update_action
    }
}

#[derive(Clone, Copy, Debug)]
struct Balloon {
    offset: f32,
    start: i32,
    timer: i32,
}

impl Balloon {
    fn init(this: &mut BaseObject) -> Self {
        this.hitbox = Hitbox {
            x: -1,
            y: -1,
            w: 10,
            h: 10,
        };

        Self {
            offset: rnd(1.0),
            timer: 0,
            start: this.y,
        }
    }

    fn update<T>(
        &mut self,
        this: &mut BaseObject,
        objects: &mut T,
        got_fruit: &[bool],
        room: Vec2<i32>,
        max_djump: i32,
    ) -> UpdateAction
    where
        for<'b> &'b mut T: IntoIterator<Item = &'b mut Object>,
    {
        let mut update_action = UpdateAction::noop();

        if this.spr == 22.0 {
            self.offset += 0.01;
            this.y = self.start + flr(sin(self.offset) * 2.0);

            if let Some((_, hit)) = this.collide(objects.into_iter(), &ObjectKind::Player, 0, 0) {
                let (_, player) = hit.to_player_mut().unwrap();
                if player.djump < max_djump {
                    // psfx(6);
                    update_action.push_mut(Object::init(
                        got_fruit,
                        room,
                        ObjectKind::Smoke,
                        this.x,
                        this.y,
                        max_djump,
                    ));
                    player.djump = max_djump;
                    this.spr = 0.0;
                    self.timer = 60;
                }
            }
        } else if self.timer > 0 {
            self.timer -= 1;
        } else {
            // psfx(7);
            update_action.push_mut(Object::init(
                got_fruit,
                room,
                ObjectKind::Smoke,
                this.x,
                this.y,
                max_djump,
            ));
            this.spr = 22.0;
        }

        update_action
    }

    fn draw(&self, this: &BaseObject, draw: &mut Pico8) {
        if this.spr == 22.0 {
            draw.spr(
                flr(13.0 + (self.offset * 8.0) % 3.0) as usize,
                this.x,
                this.y + 6,
            );
            draw.spr(flr(this.spr) as usize, this.x, this.y);
        }
    }
}

#[derive(Clone, Debug)]
enum BigChest {
    Idle,
    PickedUp {
        timer: i32,
        particles: Vec<BigChestParticle>,
    },
}

#[derive(Clone, Debug)]
struct BigChestParticle {
    x: i32,
    y: i32,
    h: i32,
    spd: i32,
}

impl BigChest {
    fn init(this: &mut BaseObject) -> Self {
        this.hitbox.w = 16;

        Self::Idle
    }

    #[allow(clippy::too_many_arguments)]
    fn draw<T>(
        &mut self,
        this: &BaseObject,
        draw: &mut Pico8,
        objects: &mut T,
        room: Vec2<i32>,
        got_fruit: &[bool],
        max_djump: i32,
        shake: &mut i32,
        flash_bg: &mut bool,
        new_bg: &mut bool,
        pause_player: &mut bool,
    ) -> UpdateAction
    where
        for<'b> &'b mut T: IntoIterator<Item = &'b mut Object>,
    {
        let mut update_action = UpdateAction::noop();

        match self {
            Self::Idle => {
                if let Some((objects, hit)) =
                    this.collide(objects.into_iter(), &ObjectKind::Player, 0, 8)
                {
                    let (base, _) = hit.to_player_mut().unwrap();
                    let is_solid = base.is_solid::<VecObjects<'_>>(
                        draw,
                        &mut VecObjects { vec: objects },
                        room,
                        0,
                        1,
                    );

                    if is_solid {
                        // music(-1, 500, 7);
                        // sfx(37);
                        *pause_player = true;
                        base.spd.x = 0.0;
                        base.spd.y = 0.0;

                        update_action.push_mut(Object::init(
                            got_fruit,
                            room,
                            ObjectKind::Smoke,
                            this.x,
                            this.y,
                            max_djump,
                        ));
                        update_action.push_mut(Object::init(
                            got_fruit,
                            room,
                            ObjectKind::Smoke,
                            this.x + 8,
                            this.y,
                            max_djump,
                        ));

                        *self = Self::PickedUp {
                            particles: vec![],
                            timer: 60,
                        };
                    }
                }
                draw.spr(96, this.x, this.y);
                draw.spr(97, this.x + 8, this.y);
            }
            Self::PickedUp { timer, particles } => {
                *timer -= 1;
                *shake = 5;
                *flash_bg = true;
                if *timer <= 45 && particles.len() < 50 {
                    particles.push(BigChestParticle {
                        x: 1 + flr(rnd(14.0)),
                        y: 0,
                        h: 32 + flr(rnd(32.0)),
                        spd: 8 + flr(rnd(8.0)),
                    })
                }

                particles.iter_mut().for_each(|particle| {
                    particle.y += particle.spd;
                    draw.line(
                        this.x + particle.x,
                        this.y + 8 - particle.y,
                        this.x + particle.x,
                        i32::min(this.y + 8 - particle.y + particle.h, this.y + 8),
                        7,
                    );
                });

                if *timer < 0 {
                    *flash_bg = false;
                    *new_bg = true;

                    update_action.push_mut(Object::init(
                        got_fruit,
                        room,
                        ObjectKind::Orb,
                        this.x + 4,
                        this.y + 4,
                        max_djump,
                    ));
                    update_action.destroy_if_mut(true);
                    *pause_player = false;
                }
            }
        }

        draw.spr(112, this.x, this.y + 8);
        draw.spr(113, this.x + 8, this.y + 8);

        update_action
    }
}

struct OtherObjects<'a> {
    prev: &'a mut [Object],
    next: &'a mut [Object],
}

fn split_at_index<T>(index: usize, elements: &mut [T]) -> Option<(&mut [T], &mut T, &mut [T])> {
    let (a, bc) = elements.split_at_mut(index);
    let (b, c) = bc.split_first_mut()?;

    Some((a, b, c))
}

impl<'a> OtherObjects<'a> {
    fn split_slice(
        index: usize,
        elements: &mut [Object],
    ) -> Option<(&mut Object, OtherObjects<'_>)> {
        let (previous_objects, object, future_objects) = split_at_index(index, elements)?;

        Some((
            object,
            OtherObjects {
                prev: previous_objects,
                next: future_objects,
            },
        ))
    }
}

impl<'a, 'objects> IntoIterator for &'a mut OtherObjects<'objects> {
    type Item = &'a mut Object;
    type IntoIter = Chain<std::slice::IterMut<'a, Object>, std::slice::IterMut<'a, Object>>;

    fn into_iter(self) -> Self::IntoIter {
        self.prev.iter_mut().chain(self.next.iter_mut())
    }
}

struct VecObjects<'a> {
    vec: Vec<&'a mut Object>,
}

impl<'a, 'objects> IntoIterator for &'a mut VecObjects<'objects> {
    type Item = &'a mut Object;
    type IntoIter = Map<
        slice::IterMut<'a, &'objects mut Object>,
        fn(&'a mut &'objects mut Object) -> &'a mut Object,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.iter_mut().map(|r| &mut **r)
    }
}

#[derive(Clone, Debug)]
struct Orb {}

impl Orb {
    fn init(this: &mut BaseObject) -> Self {
        this.spd.y = -4.0;
        this.is_solid = false;

        Self {}
    }

    #[allow(clippy::too_many_arguments)]
    fn draw<T>(
        &mut self,
        this: &mut BaseObject,
        draw: &mut Pico8,
        objects: &mut T,
        max_djump: &mut i32,
        freeze: &mut i32,
        shake: &mut i32,
        frames: i32,
    ) -> UpdateAction
    where
        for<'b> &'b mut T: IntoIterator<Item = &'b mut Object>,
    {
        let mut update_action = UpdateAction::noop();
        this.spd.y = appr(this.spd.y, 0.0, 0.5);

        if this.spd.y == 0.0 {
            if let Some((_, hit)) = this.collide(objects.into_iter(), &ObjectKind::Player, 0, 0) {
                let (_, player) = hit.to_player_mut().unwrap();
                // music_timer = 45;
                // sfx(51);
                *freeze = 10;
                *shake = 10;
                update_action.destroy_if_mut(true);
                *max_djump = 2;
                player.djump = 2;
            }
        }

        draw.spr(102, this.x, this.y);
        let off = frames as f32 / 30.0;

        for i in (0..=7).map(|i| i as f32) {
            draw.circfill(
                this.x + 4 + flr(cos(off + i / 8.0) * 8.0),
                this.y + 4 + flr(sin(off + i / 8.0) * 8.0),
                1,
                7,
            )
        }

        update_action
    }
}

#[derive(Debug, Clone)]
struct Message {
    index: f32,
    last: i32,
    off: Vec2<i32>,
}

impl Message {
    fn init() -> Self {
        Self {
            last: 0,
            index: 0.0,
            off: Vec2 { x: 8, y: 96 },
        }
    }

    fn draw<T>(&mut self, this: &mut BaseObject, draw: &mut Pico8, objects: &mut T)
    where
        for<'b> &'b mut T: IntoIterator<Item = &'b mut Object>,
    {
        // TODO: Fix font in runty8, this shouldn't need to be uppercase here.
        const TEXT: &str = "-- CELESTE MOUNTAIN --#THIS MEMORIAL TO THOSE# PERISHED ON THE CLIMB";

        if this.check(objects.into_iter(), &ObjectKind::Player, 4, 0) {
            if self.index < TEXT.len() as f32 {
                self.index += 0.5;
                if self.index >= (self.last + 1) as f32 {
                    self.last += 1;
                    //  sfx(35)
                }
            }

            self.off = Vec2 { x: 8, y: 96 };
            for i in 0..(flr(self.index) as usize) {
                if sub(TEXT, i, i) != "#" {
                    draw.rectfill(
                        self.off.x - 2,
                        self.off.y - 2,
                        self.off.x + 7,
                        self.off.y + 6,
                        7,
                    );
                    draw.print(sub(TEXT, i, i), self.off.x, self.off.y, 0);
                    self.off.x += 5
                } else {
                    self.off.x = 8;
                    self.off.y += 7;
                }
            }
        } else {
            self.index = 0.0;
            self.last = 0;
        }
    }
}

fn sub(str: &str, min: usize, max: usize) -> &str {
    &str[min..=max]
}

#[derive(Clone, Debug)]
struct Flag {
    score: i32,
    show: bool,
}

impl Flag {
    fn init(this: &mut BaseObject, got_fruit: &[bool]) -> Self {
        this.x += 5;

        Self {
            score: got_fruit.iter().map(|b| *b as i32).sum(),
            show: false,
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn draw<T>(
        &mut self,
        this: &mut BaseObject,
        draw: &mut Pico8,
        objects: &mut T,
        frames: i32,
        seconds: i32,
        minutes: i32,
        deaths: i32,
    ) where
        for<'b> &'b mut T: IntoIterator<Item = &'b mut Object>,
    {
        this.spr = 118.0 + ((frames as f32) / 5.0) % 3.0;
        draw.spr(flr(this.spr) as usize, this.x, this.y);

        if self.show {
            draw.rectfill(32, 2, 96, 31, 0);
            draw.spr(26, 55, 6);
            draw.print(&format!("X{}", self.score), 64, 9, 7);
            draw_time(seconds, minutes, draw, 49, 16);
            draw.print(&format!("DEATHS:{}", deaths), 48, 24, 7);
        } else if this.check(objects.into_iter(), &ObjectKind::Player, 0, 0) {
            // sfx(55);
            // sfx_timer = 30;
            self.show = true;
        }
    }
}
