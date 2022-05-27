#![feature(drain_filter)]
use std::f32::consts::FRAC_1_SQRT_2;
use std::path::Path;

use rand::Rng;
use runty8::runtime::draw_context::DrawContext;
use runty8::runtime::state::{Button, State};
use runty8::App;

// TODO: Deduplicate this code.
fn assets_path() -> String {
    let buf = Path::new(file!()).with_extension("");
    let dir_name = buf.to_str().unwrap();

    dir_name.to_owned()
}

fn main() {
    runty8::run_app::<GameState>(assets_path())
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
    fn init(state: &State) -> Self {
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
                s: (rnd(5.) / 4.).floor() as i32,
                spd: 0.25 + rnd(5.),
                off: rnd(1.),
                c: 6 + (0.5 + rnd(1.)).floor() as i32,
            })
            .collect();

        let mut gs = Self {
            room: Vec2 { x: 0, y: 0 },
            objects: vec![],
            // types: vec![],
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

        title_screen(&mut gs, state);

        gs
    }

    fn update(&mut self, state: &State) {
        self.frames = (self.frames + 1) % 30;

        if self.frames == 0 && level_index(self.room) < 30 {
            self.seconds = (self.seconds + 1) % 60;
            if self.seconds == 0 {
                self.minutes += 1;
            }
        }

        // TODO: Implement `music` api
        // if self.music_timer > 0 {
        //     self.music_timer -= 1;

        //     if music_timer <= 0 {
        //         music(10, 0, 7);
        //     }
        // }
        // if self.sfx_timer > 0 {
        //     self.sfx_timer -= 1;
        // }

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
                load_room(self, state, self.room.x, self.room.y);
            }
        }

        let mut i = 0;
        while i < self.objects.len() {
            let (previous_objects, object, future_objects) =
                split_at_index(i, &mut self.objects).unwrap();

            let mut iter = previous_objects.iter_mut().chain(future_objects.iter_mut());

            // Apply velocity
            object.move_(state, &mut iter, self.room);
            let UpdateAction {
                should_destroy,
                next_level,
                mut new_objects,
            } = object.update(
                &mut iter,
                &mut self.effects,
                &mut self.got_fruit,
                self.room,
                state,
                self.max_djump,
            );

            if should_destroy {
                self.objects.remove(i);
            } else {
                i += 1;
            }
            self.objects.append(&mut new_objects);
        }

        if !is_title(self) {
            self.clouds.iter_mut().for_each(Cloud::update);
        }

        self.particles.iter_mut().for_each(Particle::update);

        // Update and remove dead dead_particles
        self.dead_particles.drain_filter(DeadParticle::update);

        // TODO: remove
        if is_title(self) {
            self.begin_game(state);
        }
        // // start game
        // if is_title(self) {
        //     if !self.start_game && (state.btn(K_JUMP) || state.btn(K_DASH)) {
        //         // music(-1);
        //         self.start_game_flash = 50;
        //         self.start_game = true;
        //         // sfx(38);
        //     }
        //     if self.start_game {
        //         self.start_game_flash -= 1;
        //         if self.start_game_flash <= -30 {
        //             self.begin_game(state);
        //         }
        //     }
        // }
    }

    fn draw(&mut self, draw: &mut DrawContext) {
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
        // TODO: Unify code somehow, loop below is identical, with a different if-check
        self.objects = self
            .objects
            .iter()
            .copied()
            .map(|mut object| {
                if object.object_type.kind() == ObjectKind::Platform {
                    //|| object.object_type == ObjectKind::BigChest {
                    object.draw(draw, self);
                }
                object
            })
            .collect();

        // Draw terrain
        let off = if is_title(self) { -4 } else { 0 };
        draw.map(self.room.x * 16, self.room.y * 16, off, 0, 16, 16, 2);

        // Draw objects
        self.objects = self
            .objects
            .iter()
            .copied()
            .map(|mut object| {
                if object.object_type.kind() != ObjectKind::Platform {
                    //&& object.object_type.kind() != ObjectKind::BigChest {
                    object.draw(draw, self);
                }
                object
            })
            .collect();

        // Draw fg terrain
        draw.map(self.room.x * 16, self.room.y * 16, 0, 0, 16, 16, 8);

        // Particles
        for p in &self.particles {
            // p.x += p.spd;

            // p.y += p.off.sin();
            // p.off += (0.05_f32).min(p.spd / 32.);
            draw.rectfill(
                p.x.floor() as i32,
                p.y.floor() as i32,
                (p.x + p.s as f32).floor() as i32,
                (p.y + p.s as f32).floor() as i32,
                p.c as u8,
            );
            // if p.x > 128. + 4. {
            //     p.x = -4.;
            //     p.y = rnd(128.);
            // }
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
            draw.print("MATT THORSON", 42, 96, 5);
            draw.print("NOEL BERRY", 46, 102, 5);
        }

        if level_index(self.room) == 30 {
            if let Some(p) = self
                .objects
                .iter()
                .find(|object| object.object_type.kind() == ObjectKind::Player)
            {
                let diff = i32::min(24, 40 - i32::abs(p.base_object.x + 4 - 64));
                draw.rectfill(0, 0, diff, 128, 0);
                draw.rectfill(128 - diff, 0, 128, 128, 0);
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

        self.y += self.off.sin();
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

        // Remove if
        self.t <= 0
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct Vec2<T> {
    x: T,
    y: T,
}

fn rnd(max: f32) -> f32 {
    rand::thread_rng().gen_range(0.0..max)
}

impl GameState {
    fn begin_game(&mut self, state: &State) {
        self.frames = 0;
        self.seconds = 0;
        self.minutes = 0;
        self.music_timer = 0;
        self.start_game = false;
        // music(0, 0, 7);
        load_room(self, state, 0, 0);
    }
}

const K_LEFT: Button = Button::Left;
const K_RIGHT: Button = Button::Right;
const K_UP: Button = Button::Up;
const K_DOWN: Button = Button::Down;
const K_JUMP: Button = Button::C;
const K_DASH: Button = Button::X;

fn title_screen(game_state: &mut GameState, state: &State) {
    game_state.got_fruit = vec![false; 30];
    game_state.frames = 0;
    game_state.deaths = 0;
    game_state.max_djump = 1;
    game_state.start_game = false;
    game_state.start_game_flash = 0;
    // music(40,0,7)
    load_room(game_state, state, 7, 3)
}

fn level_index(room: Vec2<i32>) -> i32 {
    room.x % 8 + room.y * 8
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

    fn update<'a>(
        &mut self,
        this: &mut BaseObject,
        state: &State,
        objects: &mut impl Iterator<Item = &'a mut Object>,
        got_fruit: &[bool],
        room: Vec2<i32>,
        max_djump: i32,
    ) -> UpdateAction {
        let mut update_action = UpdateAction::noop();

        // TODO:
        // if pause_player {
        //     return
        // }
        let input = horizontal_input(state);

        // -- spikes collide
        // if spikes_at(
        //     this.x + this.hitbox.x,
        //     this.y + this.hitbox.y,
        //     this.hitbox.w,
        //     this.hitbox.h,
        //     this.spd.x,
        //     this.spd.y,
        // ) {
        //     kill_player(self);
        // }

        // -- bottom death
        // if this.y > 128 {
        //     kill_player(self);
        // }

        // TODO: Check is_solid, is_ice
        // Check appr
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
                // has_dashed = true;
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
                // freeze = 2;
                // shake = 6;
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
        // if this.y < -4 && level_index(room) < 30 {
        //     next_room(game_state, state)
        // }

        self.was_on_ground = on_ground;
        update_action
    }

    fn draw(&mut self, base_object: &mut BaseObject, draw: &mut DrawContext, frames: i32) {
        if base_object.x < -1 || base_object.x >= 121 {
            base_object.x = clampi(base_object.x, -1, 121);
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

        // base_object.hitbox.draw(draw, base_object.x, base_object.y);
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

fn set_hair_color(draw: &mut DrawContext, frames: i32, djump: i32) {
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
fn unset_hair_color(draw: &mut DrawContext) {
    draw.pal(8, 8);
}

struct Spring {
    hide_in: i32,
    hide_for: i32,
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct BaseObject {
    x: i32,
    y: i32,
    hitbox: Hitbox,
    spr: f32, // hack they use
    spd: Vec2<f32>,
    rem: Vec2<f32>,
    last: i32,
    dir: i32, // not sure if all objects use this?
    // obj.solids in original source
    is_solid: bool,
    collideable: bool,
    flip: Vec2<bool>,
}

impl BaseObject {
    fn collide<'a>(
        &self,
        objects: &mut impl Iterator<Item = &'a mut Object>,
        kind: &ObjectKind,
        ox: i32,
        oy: i32,
    ) -> Option<(usize, &'a mut Object)> {
        for (index, other_object) in objects.enumerate() {
            let other_type = other_object.object_type;
            let other = other_object.base_object;

            if &other_type.kind() == kind && other.collideable // This kills rust-fmt?
                && other.x + other.hitbox.x + other.hitbox.w > self.x + self.hitbox.x + ox
                && other.y + other.hitbox.y + other.hitbox.h > self.y + self.hitbox.y + oy
                && other.x + other.hitbox.x < self.x + self.hitbox.x + self.hitbox.w + ox
                && other.y + other.hitbox.y < self.y + self.hitbox.y + self.hitbox.h + oy
            {
                return Some((index, other_object));
            }
        }
        None
    }

    fn is_solid<'a>(
        &self,
        state: &State,
        objects: &mut impl Iterator<Item = &'a mut Object>,
        room: Vec2<i32>,
        ox: i32,
        oy: i32,
    ) -> bool {
        if oy > 0
            && !self.check(objects, &ObjectKind::Platform, ox, 0)
            && self.check(objects, &ObjectKind::Platform, ox, oy)
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
        )
        //  || self.check(objects, &ObjectType::FallFloor, ox, oy)
            || self.check(objects, &ObjectKind::FakeWall, ox, oy)
    }

    fn check<'a>(
        &self,
        objects: &mut impl Iterator<Item = &'a mut Object>,
        kind: &ObjectKind,
        ox: i32,
        oy: i32,
    ) -> bool {
        self.collide(objects, kind, ox, oy).is_some()
    }

    fn is_ice(&self, state: &State, room: Vec2<i32>, ox: i32, oy: i32) -> bool {
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

// impl Spring {
//     fn init() -> Self {
//         Self {
//             hide_in: 0,
//             hide_for: 0,
//         }
//     }

//     fn update(&mut self, object: &mut BaseObject) {
//         if self.hide_for > 0 {
//             self.hide_for -= 1;

//             if self.hide_for <= 0 {
//                 object.spr = 18.;
//                 object.delay = 0;
//             }
//         } else if object.spr == 18. {
//             // TODO: Borrowchecker madness
//             // let hit = object.collide(player);
//             //
//             // if let Some(hit) = hit.and_then(|hit| hit.spd.y >= 0) {
//             //     object.spr = 19.;
//             //     hit.y = object.y - 4.;
//             //     hit.spd.x *= 0.2;
//             //     hit.spd.y = -3;
//             // hit.djump = game_state.max_djump;
//             // object.delay = 10;
//             //
//             // init_object(smoke,this.x,this.y)
//             //
//             // -- breakable below us
//             // local below=this.collide(fall_floor,0,1)
//             // if below~=nil then
//             //     break_fall_floor(below)
//             // end
//             //
//             // psfx(8)
//             // }
//         } else if object.delay > 0 {
//             object.delay -= 1;

//             if object.delay <= 0 {
//                 object.spr = 18.;
//             }
//         }

//         // begin hiding
//         if self.hide_in > 0 {
//             self.hide_in -= 1;

//             if self.hide_in <= 0 {
//                 self.hide_for = 60;
//                 object.spr = 0.;
//             }
//         }
//     }

//     fn break_spring(&mut self) {
//         self.hide_in = 15;
//     }
// }

// balloon = {
//     tile=22,
//     init=function(this)
//         this.offset=rnd(1)
//         this.start=this.y
//         this.timer=0
//         this.hitbox={x=-1,y=-1,w=10,h=10}
//     end,
//     update=function(this)
//         if this.spr==22 then
//             this.offset+=0.01
//             this.y=this.start+sin(this.offset)*2
//             local hit = this.collide(player,0,0)
//             if hit~=nil and hit.djump<max_djump then
//                 psfx(6)
//                 init_object(smoke,this.x,this.y)
//                 hit.djump=max_djump
//                 this.spr=0
//                 this.timer=60
//             end
//         elseif this.timer>0 then
//             this.timer-=1
//         else
//          psfx(7)
//          init_object(smoke,this.x,this.y)
//             this.spr=22
//         end
//     end,
//     draw=function(this)
//         if this.spr==22 then
//             spr(13+(this.offset*8)%3,this.x,this.y+6)
//             spr(this.spr,this.x,this.y)
//         end
//     end
// }
// add(types,balloon)

// fall_floor = {
//     tile=23,
//     init=function(this)
//         this.state=0
//         this.solid=true
//     end,
//     update=function(this)
//         -- idling
//         if this.state == 0 then
//             if this.check(player,0,-1) or this.check(player,-1,0) or this.check(player,1,0) then
//                 break_fall_floor(this)
//             end
//         -- shaking
//         elseif this.state==1 then
//             this.delay-=1
//             if this.delay<=0 then
//                 this.state=2
//                 this.delay=60--how long it hides for
//                 this.collideable=false
//             end
//         -- invisible, waiting to reset
//         elseif this.state==2 then
//             this.delay-=1
//             if this.delay<=0 and not this.check(player,0,0) then
//                 psfx(7)
//                 this.state=0
//                 this.collideable=true
//                 init_object(smoke,this.x,this.y)
//             end
//         end
//     end,
//     draw=function(this)
//         if this.state!=2 then
//             if this.state!=1 then
//                 spr(23,this.x,this.y)
//             else
//                 spr(23+(15-this.delay)/5,this.x,this.y)
//             end
//         end
//     end
// }
// add(types,fall_floor)

// function break_fall_floor(obj)
//  if obj.state==0 then
//      psfx(15)
//         obj.state=1
//         obj.delay=15--how long until it falls
//         init_object(smoke,obj.x,obj.y)
//         local hit=obj.collide(spring,0,-1)
//         if hit~=nil then
//             break_spring(hit)
//         end
//     end
// end

// fruit={
//     tile=26,
//     if_not_fruit=true,
//     init=function(this)
//         this.start=this.y
//         this.off=0
//     end,
//     update=function(this)
//      local hit=this.collide(player,0,0)
//         if hit~=nil then
//          hit.djump=max_djump
//             sfx_timer=20
//             sfx(13)
//             got_fruit[1+level_index()] = true
//             init_object(lifeup,this.x,this.y)
//             destroy_object(this)
//         end
//         this.off+=1
//         this.y=this.start+sin(this.off/40)*2.5
//     end
// }
// add(types,fruit)

// fly_fruit={
//     tile=28,
//     if_not_fruit=true,
//     init=function(this)
//         this.start=this.y
//         this.fly=false
//         this.step=0.5
//         this.solids=false
//         this.sfx_delay=8
//     end,
//     update=function(this)
//         --fly away
//         if this.fly then
//          if this.sfx_delay>0 then
//           this.sfx_delay-=1
//           if this.sfx_delay<=0 then
//            sfx_timer=20
//            sfx(14)
//           end
//          end
//             this.spd.y=appr(this.spd.y,-3.5,0.25)
//             if this.y<-16 then
//                 destroy_object(this)
//             end
//         -- wait
//         else
//             if has_dashed then
//                 this.fly=true
//             end
//             this.step+=0.05
//             this.spd.y=sin(this.step)*0.5
//         end
//         -- collect
//         local hit=this.collide(player,0,0)
//         if hit~=nil then
//          hit.djump=max_djump
//             sfx_timer=20
//             sfx(13)
//             got_fruit[1+level_index()] = true
//             init_object(lifeup,this.x,this.y)
//             destroy_object(this)
//         end
//     end,
//     draw=function(this)
//         local off=0
//         if not this.fly then
//             local dir=sin(this.step)
//             if dir<0 then
//                 off=1+max(0,sign(this.y-this.start))
//             end
//         else
//             off=(off+0.25)%3
//         end
//         spr(45+off,this.x-6,this.y-2,1,1,true,false)
//         spr(this.spr,this.x,this.y)
//         spr(45+off,this.x+6,this.y-2)
//     end
// }
// add(types,fly_fruit)

// lifeup = {
//     init=function(this)
//         this.spd.y=-0.25
//         this.duration=30
//         this.x-=2
//         this.y-=4
//         this.flash=0
//         this.solids=false
//     end,
//     update=function(this)
//         this.duration-=1
//         if this.duration<= 0 then
//             destroy_object(this)
//         end
//     end,
//     draw=function(this)
//         this.flash+=0.5

//         print("1000",this.x-2,this.y,7+this.flash%2)
//     end
// }

// key={
//     tile=8,
//     if_not_fruit=true,
//     update=function(this)
//         local was=flr(this.spr)
//         this.spr=9+(sin(frames/30)+0.5)*1
//         local is=flr(this.spr)
//         if is==10 and is!=was then
//             this.flip.x=not this.flip.x
//         end
//         if this.check(player,0,0) then
//             sfx(23)
//             sfx_timer=10
//             destroy_object(this)
//             has_key=true
//         end
//     end
// }
// add(types,key)

// chest={
//     tile=20,
//     if_not_fruit=true,
//     init=function(this)
//         this.x-=4
//         this.start=this.x
//         this.timer=20
//     end,
//     update=function(this)
//         if has_key then
//             this.timer-=1
//             this.x=this.start-1+rnd(3)
//             if this.timer<=0 then
//              sfx_timer=20
//              sfx(16)
//                 init_object(fruit,this.x,this.y-4)
//                 destroy_object(this)
//             end
//         end
//     end
// }
// add(types,chest)

// message={
//     tile=86,
//     last=0,
//     draw=function(this)
//         this.text="-- celeste mountain --#this memorial to those# perished on the climb"
//         if this.check(player,4,0) then
//             if this.index<#this.text then
//              this.index+=0.5
//                 if this.index>=this.last+1 then
//                  this.last+=1
//                  sfx(35)
//                 end
//             end
//             this.off={x=8,y=96}
//             for i=1,this.index do
//                 if sub(this.text,i,i)~="#" then
//                     rectfill(this.off.x-2,this.off.y-2,this.off.x+7,this.off.y+6 ,7)
//                     print(sub(this.text,i,i),this.off.x,this.off.y,0)
//                     this.off.x+=5
//                 else
//                     this.off.x=8
//                     this.off.y+=7
//                 end
//             end
//         else
//             this.index=0
//             this.last=0
//         end
//     end
// }
// add(types,message)

// big_chest={
//     tile=96,
//     init=function(this)
//         this.state=0
//         this.hitbox.w=16
//     end,
//     draw=function(this)
//         if this.state==0 then
//             local hit=this.collide(player,0,8)
//             if hit~=nil and hit.is_solid(0,1) then
//                 music(-1,500,7)
//                 sfx(37)
//                 pause_player=true
//                 hit.spd.x=0
//                 hit.spd.y=0
//                 this.state=1
//                 init_object(smoke,this.x,this.y)
//                 init_object(smoke,this.x+8,this.y)
//                 this.timer=60
//                 this.particles={}
//             end
//             spr(96,this.x,this.y)
//             spr(97,this.x+8,this.y)
//         elseif this.state==1 then
//             this.timer-=1
//          shake=5
//          flash_bg=true
//             if this.timer<=45 and count(this.particles)<50 then
//                 add(this.particles,{
//                     x=1+rnd(14),
//                     y=0,
//                     h=32+rnd(32),
//                     spd=8+rnd(8)
//                 })
//             end
//             if this.timer<0 then
//                 this.state=2
//                 this.particles={}
//                 flash_bg=false
//                 new_bg=true
//                 init_object(orb,this.x+4,this.y+4)
//                 pause_player=false
//             end
//             foreach(this.particles,function(p)
//                 p.y+=p.spd
//                 line(this.x+p.x,this.y+8-p.y,this.x+p.x,min(this.y+8-p.y+p.h,this.y+8),7)
//             end)
//         end
//         spr(112,this.x,this.y+8)
//         spr(113,this.x+8,this.y+8)
//     end
// }
// add(types,big_chest)

// orb={
//     init=function(this)
//         this.spd.y=-4
//         this.solids=false
//         this.particles={}
//     end,
//     draw=function(this)
//         this.spd.y=appr(this.spd.y,0,0.5)
//         local hit=this.collide(player,0,0)
//         if this.spd.y==0 and hit~=nil then
//          music_timer=45
//             sfx(51)
//             freeze=10
//             shake=10
//             destroy_object(this)
//             max_djump=2
//             hit.djump=2
//         end

//         spr(102,this.x,this.y)
//         local off=frames/30
//         for i=0,7 do
//             circfill(this.x+4+cos(off+i/8)*8,this.y+4+sin(off+i/8)*8,1,7)
//         end
//     end
// }

// flag = {
//     tile=118,
//     init=function(this)
//         this.x+=5
//         this.score=0
//         this.show=false
//         for i=1,count(got_fruit) do
//             if got_fruit[i] then
//                 this.score+=1
//             end
//         end
//     end,
//     draw=function(this)
//         this.spr=118+(frames/5)%3
//         spr(this.spr,this.x,this.y)
//         if this.show then
//             rectfill(32,2,96,31,0)
//             spr(26,55,6)
//             print("x"..this.score,64,9,7)
//             draw_time(49,16)
//             print("deaths:"..deaths,48,24,7)
//         elseif this.check(player,0,0) then
//             sfx(55)
//       sfx_timer=30
//             this.show=true
//         end
//     end
// }
// add(types,flag)

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

    fn draw(&self, draw: &mut DrawContext, room: Vec2<i32>) {
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

            draw_time(draw, 4, 4);
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
impl Hitbox {
    // TODO: Remove, for debugging purposes only
    fn draw(&self, draw: &mut DrawContext, x: i32, y: i32, color: u8) {
        draw.rect(
            x + self.x,
            y + self.y,
            x + self.x + self.w - 1,
            y + self.y + self.h - 1,
            8,
        );
    }
}

#[derive(Clone, Copy, Debug)]
struct Object {
    base_object: BaseObject,
    object_type: ObjectType,
}

fn got_fruit_for_room(got_fruit: &[bool], room: Vec2<i32>) -> bool {
    got_fruit[1 + level_index(room) as usize]
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
        println!("Created {:?}", kind);
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
            last: 0,
            dir: 0,
            flip: Vec2 { x: false, y: false },
            // TODO: figure out if we need an option here
            spr: kind.tile().map(|t| t as f32).unwrap_or(-42.),
        };
        let object_type = ObjectKind::create(&kind, &mut base_object, max_djump);

        Some(Self {
            base_object,
            object_type,
        })
    }

    fn update<'a>(
        &mut self,
        other_objects: &mut impl Iterator<Item = &'a mut Object>,
        effects: &mut GameEffects,
        got_fruit: &mut [bool],
        room: Vec2<i32>,
        state: &State,
        max_djump: i32,
    ) -> UpdateAction {
        self.object_type.update(
            &mut self.base_object,
            other_objects,
            effects,
            got_fruit,
            room,
            state,
            max_djump,
        )
    }

    fn draw(&mut self, draw: &mut DrawContext, game_state: &GameState) {
        match &mut self.object_type {
            ObjectType::PlayerSpawn(player_spawn) => {
                player_spawn.draw(&mut self.base_object, game_state, draw)
            }
            // ObjectType::BigChest => todo!(),
            ObjectType::Player(player) => {
                player.draw(&mut self.base_object, draw, game_state.frames)
            }
            // ObjectType::LifeUp => todo!(),
            // ObjectType::Fruit => todo!(),
            // ObjectType::Orb => todo!(),
            ObjectType::FakeWall => FakeWall::draw(&mut self.base_object, game_state, draw),
            // ObjectType::FallFloor => todo!(),
            // ObjectType::Key => todo!(),
            ObjectType::RoomTitle(room_title) => room_title.draw(draw, game_state.room),
            ObjectType::Platform => todo!("Platform draw"),
            ObjectType::Smoke => default_draw(&mut self.base_object, draw),
            ObjectType::Fruit(_) => default_draw(&mut self.base_object, draw),
        }
    }

    fn move_<'a>(
        &mut self,
        state: &State,
        objects: &mut impl Iterator<Item = &'a mut Object>,
        room: Vec2<i32>,
    ) {
        let ox = self.base_object.spd.x;
        let oy = self.base_object.spd.y;

        // [x] get move amount
        self.base_object.rem.x += ox;
        let amount_x = (self.base_object.rem.x as f32 + 0.5).floor();
        self.base_object.rem.x -= amount_x;
        self.move_x(state, objects, room, amount_x as i32, 0);

        // [y] get move amount
        self.base_object.rem.y += oy;
        let amount_y = (self.base_object.rem.y as f32 + 0.5).floor();
        self.base_object.rem.y -= amount_y;
        self.move_y(state, objects, room, amount_y as i32);
    }

    fn move_x<'a>(
        &mut self,
        state: &State,
        objects: &mut impl Iterator<Item = &'a mut Object>,
        room: Vec2<i32>,
        amount: i32,
        start: i32,
    ) {
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

    fn move_y<'a>(
        &mut self,
        state: &State,
        objects: &mut impl Iterator<Item = &'a mut Object>,
        room: Vec2<i32>,
        amount: i32,
    ) {
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

    fn is_solid<'a>(
        &self,
        state: &State,
        objects: &mut impl Iterator<Item = &'a mut Object>,
        room: Vec2<i32>,
        ox: i32,
        oy: i32,
    ) -> bool {
        self.base_object.is_solid(state, objects, room, ox, oy)
    }

    fn check<'a>(
        &self,
        objects: &mut impl Iterator<Item = &'a mut Object>,
        kind: &ObjectKind,
        ox: i32,
        oy: i32,
    ) -> bool {
        self.base_object.check(objects, kind, ox, oy)
    }

    fn to_player_mut(&mut self) -> Option<&mut Player> {
        match &mut self.object_type {
            ObjectType::Player(player) => Some(player),
            _ => None,
        }
    }
}

fn default_draw(base_object: &mut BaseObject, draw: &mut DrawContext) {
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
}

fn tile_flag_at(state: &State, room: Vec2<i32>, x: i32, y: i32, w: i32, h: i32, flag: u8) -> bool {
    let x_min = i32::max(0, (x as f32 / 8.0).floor() as i32);
    let x_max = i32::min(15, ((x as f32 + w as f32 - 1.0) / 8.0).floor() as i32);
    for i in x_min..=x_max {
        let y_min = i32::max(0, (y as f32 / 8.0).floor() as i32);
        let y_max = i32::min(15, ((y as f32 + h as f32 - 1.0) / 8.0).floor() as i32);
        for j in y_min..=y_max {
            if state.fget_n(tile_at(state, room, i, j), flag) {
                return true;
            }
        }
    }
    false
}

fn tile_at(state: &State, room: Vec2<i32>, x: i32, y: i32) -> usize {
    state.mget(room.x * 16 + x, room.y * 16 + y).into()
}

fn solid_at(state: &State, room: Vec2<i32>, x: i32, y: i32, w: i32, h: i32) -> bool {
    tile_flag_at(state, room, x, y, w, h, 0)
}

#[derive(Clone, Copy, Debug)]
enum ObjectType {
    Platform,
    // BigChest,
    Player(Player),
    PlayerSpawn(PlayerSpawn),
    Smoke,
    // LifeUp,
    Fruit(Fruit),
    // Orb,
    FakeWall,
    // FallFloor,
    // Key,
    RoomTitle(RoomTitle),
}

impl ObjectType {
    fn kind(&self) -> ObjectKind {
        match self {
            ObjectType::Platform => ObjectKind::Platform,
            ObjectType::Player(_) => ObjectKind::Player,
            ObjectType::PlayerSpawn(_) => ObjectKind::PlayerSpawn,
            ObjectType::RoomTitle(_) => ObjectKind::RoomTitle,
            ObjectType::Smoke => ObjectKind::Smoke,
            ObjectType::FakeWall => ObjectKind::FakeWall,
            ObjectType::Fruit(_) => ObjectKind::Fruit,
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn update<'a>(
        &mut self,
        base_object: &mut BaseObject,
        other_objects: &mut impl Iterator<Item = &'a mut Object>,
        effects: &mut GameEffects,
        got_fruit: &mut [bool],
        room: Vec2<i32>,
        state: &State,
        max_djump: i32,
    ) -> UpdateAction {
        match self {
            ObjectType::PlayerSpawn(player_spawn) => {
                player_spawn.update(base_object, effects, got_fruit, room, max_djump, state)
            }
            ObjectType::Smoke => Smoke::update(base_object),
            ObjectType::Platform => todo!(),
            // ObjectType::BigChest => todo!(),
            ObjectType::Player(player) => player.update(
                base_object,
                state,
                other_objects,
                got_fruit,
                room,
                max_djump,
            ),
            // ObjectType::LifeUp => todo!(),
            ObjectType::Fruit(fruit) => {
                fruit.update(base_object, other_objects, got_fruit, room, max_djump)
            }
            // ObjectType::Orb => todo!(),
            ObjectType::FakeWall => FakeWall::update(
                base_object,
                other_objects,
                got_fruit,
                room,
                max_djump,
                state,
            ),
            // ObjectType::FallFloor => todo!(),
            // ObjectType::Key => todo!(),
            ObjectType::RoomTitle(rt) => rt.update(),
        }
    }
}

fn kill_player(obj: &Object, game_state: &mut GameState) {
    game_state.sfx_timer = 12;
    // sfx(0);
    game_state.deaths += 1;
    game_state.effects.shake = 10;
    destroy_object(game_state, obj);

    game_state.dead_particles.clear();
    for dir in 0..=7 {
        let dir = dir as f32;
        let angle = dir / 8.;

        game_state.dead_particles.push(DeadParticle {
            x: (obj.base_object.x + 4) as f32,
            y: (obj.base_object.y + 4) as f32,
            t: 10,
            spd: Vec2 {
                x: (angle).sin() * 3.,
                y: (angle).cos() * 3.,
            },
        });
    }
    restart_room(game_state)
}

fn destroy_object(game_state: &mut GameState, object: &Object) {
    game_state.objects.retain(|o| std::ptr::eq(o, object));
}
// -- room functions --
// --------------------

fn restart_room(game_state: &mut GameState) {
    game_state.will_restart = true;
    game_state.delay_restart = 15;
}

fn next_room(game_state: &mut GameState, state: &State) {
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

fn load_room(game_state: &mut GameState, state: &State, x: i32, y: i32) {
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

fn draw_time(draw: &mut DrawContext, x: i32, y: i32) {
    // TODO
    //  local s=seconds
    //  local m=minutes%60
    //  local h=flr(minutes/60)
    let s = 42;
    let m = 1;
    let h = 0;

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

fn clampi(val: i32, a: i32, b: i32) -> i32 {
    a.max(b.min(val))
}

fn clamp(val: f32, a: f32, b: f32) -> f32 {
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
    rand::thread_rng().gen()
}
fn ice_at(state: &State, room: Vec2<i32>, x: i32, y: i32, w: i32, h: i32) -> bool {
    tile_flag_at(state, room, x, y, w, h, 4)
}

// function spikes_at(x,y,w,h,xspd,yspd)
//  for i=max(0,flr(x/8)),min(15,(x+w-1)/8) do
//      for j=max(0,flr(y/8)),min(15,(y+h-1)/8) do
//       local tile=tile_at(i,j)
//       if tile==17 and ((y+h-1)%8>=6 or y+h==j*8+8) and yspd>=0 then
//        return true
//       elseif tile==27 and y%8<=2 and yspd<=0 then
//        return true
//          elseif tile==43 and x%8<=2 and xspd<=0 then
//           return true
//          elseif tile==59 and ((x+w-1)%8>=6 or x+w==i*8+8) and xspd>=0 then
//           return true
//          end
//      end
//  end
//     return false
// end

struct Platform {}

impl Platform {
    // fn init(this: &mut Object) {
    //     this.x -= 4.;
    //     this.is_solid = false;
    //     this.hitbox.w = 16;
    //     this.last = this.x;
    // }

    fn update<'a>(
        self_: &mut Object,
        state: &State,
        objects: &mut impl Iterator<Item = &'a mut Object>,
        room: Vec2<i32>,
    ) -> Option<(usize, Object)> {
        self_.base_object.spd.x = self_.base_object.dir as f32 * 0.65;
        if self_.base_object.x < -16 {
            self_.base_object.x = 128;
        } else if self_.base_object.x > 128 {
            self_.base_object.x = -16;
        }
        self_.base_object.last = self_.base_object.x;

        let ret = if !self_.check(objects, &ObjectKind::Player, 0, 0) {
            let (index, hit) = self_
                .base_object
                .collide(objects, &ObjectKind::Player, 0, -1)?;
            let mut hit = *hit;
            hit.move_x(
                state,
                objects,
                room,
                self_.base_object.x - self_.base_object.last,
                1,
            );

            Some((index, hit))
        } else {
            None
        };

        ret
    }

    fn draw(self_: &Object, draw: &mut DrawContext) {
        draw.spr(11, self_.base_object.x, self_.base_object.y - 1);
        draw.spr(12, self_.base_object.x + 8, self_.base_object.y - 1)
    }
}
#[derive(Clone, Copy)]
struct Smoke;

impl Smoke {
    fn init(base_object: &mut BaseObject) {
        base_object.spr = 29.;
        base_object.spd.y = -0.1;
        base_object.spd.x = 0.3 + rnd(0.2);
        base_object.x += -1 + rnd(2.).floor() as i32;
        base_object.y += -1 + rnd(2.).floor() as i32;
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

    fn update<'a>(
        &mut self,
        base_object: &mut BaseObject,
        other_objects: &mut impl Iterator<Item = &'a mut Object>,
        got_fruit: &mut [bool],
        room: Vec2<i32>,
        max_djump: i32,
    ) -> UpdateAction {
        let update_action =
            if let Some((_, hit)) = base_object.collide(other_objects, &ObjectKind::Player, 0, 0) {
                let player = hit.to_player_mut().unwrap();

                player.djump = max_djump;
                // sfx_timer=20
                // sfx(13)
                got_fruit[1 + level_index(room) as usize] = true;

                UpdateAction::noop().destroy()
                // TODO: Implement LifeUp
                // .push(Object::init(
                //     got_fruit,
                //     room,
                //     ObjectKind::LifeUp,
                //     base_object.x,
                //     base_object.y + 4.0,
                // ))
            } else {
                UpdateAction::noop()
            };

        self.off += 1;
        base_object.y = self.start + ((self.off as f32 / 40.0).sin() * 2.5) as i32;

        update_action
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ObjectKind {
    PlayerSpawn,
    Player,
    // Spring,
    // Balloon,
    // FallFloor,
    Fruit,
    // FlyFruit,
    FakeWall,
    // Key,
    // Chest,
    // Message,
    // BigChest,
    // Flag,

    // Non-tile-instantiable
    RoomTitle,
    Platform,
    Smoke,
}

impl ObjectKind {
    // I think these are the "instantiable" objects
    // (you put a "marker" tile in the map and this creates the object for it)
    // see line 1135 of source.p8
    //
    // TODO: Make this just an array of all the variants with a macro or the strum crate.
    // and depend on the tile function for tile-instantiability
    const TYPES: &'static [Self] = &[
        ObjectKind::PlayerSpawn,
        // ObjectKind::Spring,
        // ObjectKind::Balloon,
        // ObjectKind::FallFloor,
        // ObjectKind::Fruit,
        // ObjectKind::FlyFruit,
        ObjectKind::FakeWall,
        // ObjectKind::Key,
        // ObjectKind::Chest,
        // ObjectKind::Message,
        // ObjectKind::BigChest,
        // ObjectKind::Flag,
    ];

    fn create(&self, base_object: &mut BaseObject, max_djump: i32) -> ObjectType {
        match self {
            ObjectKind::PlayerSpawn => ObjectType::PlayerSpawn(PlayerSpawn::init(base_object)),
            ObjectKind::Player => ObjectType::Player(Player::init(base_object, max_djump)),

            // ObjectKind::Spring => todo!(),
            // ObjectKind::Balloon => todo!(),
            // ObjectKind::FallFloor => todo!(),
            ObjectKind::Fruit => ObjectType::Fruit(Fruit::init(base_object)),
            // ObjectKind::FlyFruit => todo!(),

            // ObjectKind::Key => todo!(),
            // ObjectKind::Chest => todo!(),
            // ObjectKind::Message => todo!(),
            // ObjectKind::BigChest => todo!(),
            // ObjectKind::Flag => todo!(),
            ObjectKind::RoomTitle => ObjectType::RoomTitle(RoomTitle { delay: 5 }),
            ObjectKind::Platform => todo!(),
            ObjectKind::Smoke => {
                Smoke::init(base_object);
                ObjectType::Smoke
            }
            ObjectKind::FakeWall => ObjectType::FakeWall,
        }
    }
    fn tile(&self) -> Option<i32> {
        match self {
            ObjectKind::PlayerSpawn => Some(1),
            // ObjectKind::Spring => Some(18),
            // ObjectKind::Balloon => Some(22),
            // ObjectKind::FallFloor => Some(23),
            // ObjectKind::Fruit => Some(26),
            // ObjectKind::FlyFruit => Some(28),
            ObjectKind::FakeWall => Some(64),
            // ObjectKind::Key => Some(8),
            // ObjectKind::Chest => Some(20),
            // ObjectKind::Message => Some(86),
            // ObjectKind::BigChest => Some(96),
            // ObjectKind::Flag => Some(118),
            _ => None,
        }
    }

    fn if_not_fruit(&self) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match self {
            // ObjectKind::Fruit => true,
            // ObjectKind::FlyFruit => true,
            ObjectKind::FakeWall => true,
            // ObjectKind::Key => true,
            // ObjectKind::Chest => true,
            _ => false,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct Hair {
    segments: [HairElement; 5],
}

impl Hair {
    fn draw(&mut self, draw: &mut DrawContext, x: i32, y: i32, facing: i32) {
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

    fn destroy_if(mut self, should_destroy: bool) -> Self {
        self.should_destroy = should_destroy;

        self
    }

    fn destroy(mut self) -> Self {
        self.destroy_if(true)
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
        _: &State,
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
        game_state: &GameState,
        draw: &mut DrawContext,
    ) {
        set_hair_color(draw, game_state.frames, game_state.max_djump);

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
    fn update<'a>(
        base_object: &mut BaseObject,
        other_objects: &mut impl Iterator<Item = &'a mut Object>,
        got_fruit: &[bool],
        room: Vec2<i32>,
        max_djump: i32,
        _: &State,
    ) -> UpdateAction {
        base_object.hitbox = Hitbox {
            x: -1,
            y: -1,
            w: 18,
            h: 18,
        };

        let mut update_action = UpdateAction::noop();

        // TODO: This doesn't work as well as in the original.
        let hit: Option<(usize, &mut Object)> =
            base_object.collide(other_objects, &ObjectKind::Player, 0, 0);
        if let Some((_, hit_object)) = hit {
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
        // base_object.hitbox = Hitbox {
        //     x: 0,
        //     y: 0,
        //     w: 16,
        //     h: 16,
        // };

        update_action
    }

    fn draw(base_object: &mut BaseObject, _: &GameState, draw: &mut DrawContext) {
        let x = base_object.x;
        let y = base_object.y;
        draw.spr(64, x, y);
        draw.spr(65, x + 8, y);
        draw.spr(80, x, y + 8);
        draw.spr(81, x + 8, y + 8);
    }
}

fn split_at_index<T>(index: usize, elements: &mut [T]) -> Option<(&mut [T], &mut T, &mut [T])> {
    let (a, bc) = elements.split_at_mut(index);
    let (b, c) = bc.split_first_mut()?;

    Some((a, b, c))
}

fn horizontal_input(state: &State) -> i32 {
    if state.btn(K_RIGHT) {
        1
    } else if state.btn(K_LEFT) {
        -1
    } else {
        0
    }
}

fn vertical_input(state: &State) -> i32 {
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
    match f.total_cmp(&0.0) {
        std::cmp::Ordering::Less => -1.0,
        std::cmp::Ordering::Equal => 0.0,
        std::cmp::Ordering::Greater => 1.0,
    }
}
