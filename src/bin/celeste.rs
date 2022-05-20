#![feature(drain_filter)]
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
    // types: Vec<_>,
    freeze: i32,
    shake: i32,
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
            shake: 0,
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
        };

        title_screen(&mut gs, state);

        gs
    }

    fn update(&mut self, state: &State) {
        self.frames = (self.frames + 1) % 30;

        if self.frames == 0 && level_index(self) < 30 {
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
        if self.shake > 0 {
            self.shake -= 1;
        }

        // Restart (soon)
        if self.will_restart && self.delay_restart > 0 {
            self.delay_restart -= 1;
            if self.delay_restart <= 0 {
                self.will_restart = false;
                load_room(self, state, self.room.x, self.room.y);
            }
        }

        // Update each object?
        self.objects = self
            .objects
            .iter()
            .copied()
            .filter_map(|mut object| {
                object.move_(state, &self.objects, self.room);
                let should_destroy = object.object_type.update(&mut object.base_object, state);

                if should_destroy {
                    None
                } else {
                    Some(object)
                }
            })
            .collect();

        if !is_title(self) {
            self.clouds.iter_mut().for_each(Cloud::update);
        }

        self.particles.iter_mut().for_each(Particle::update);

        // Update and remove dead dead_particles
        self.dead_particles.drain_filter(DeadParticle::update);

        // start game
        if is_title(self) {
            if !self.start_game && (state.btn(K_JUMP) || state.btn(K_DASH)) {
                // music(-1);
                self.start_game_flash = 50;
                self.start_game = true;
                // sfx(38);
            }
            if self.start_game {
                self.start_game_flash -= 1;
                if self.start_game_flash <= -30 {
                    self.begin_game(state);
                }
            }
        }
    }

    fn draw(&self, draw: &mut DrawContext) {
        draw.camera(0, 0);
        if self.shake > 0 {
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
        for object in self.objects.iter() {
            if object.object_type == ObjectType::Platform {
                //|| object.object_type == ObjectType::BigChest {
                object.draw(draw, self);
            }
        }

        // Draw terrain
        let off = if is_title(self) { -4 } else { 0 };
        draw.map(self.room.x * 16, self.room.y * 16, off, 0, 16, 16, 2);

        // Draw objects
        for object in &self.objects {
            if object.object_type != ObjectType::Platform {
                //&& object.object_type != ObjectType::BigChest {
                object.draw(draw, self);
            }
        }

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

        if level_index(self) == 30 {
            if let Some(p) = self
                .objects
                .iter()
                .find(|object| object.object_type == ObjectType::Player)
            {
                let diff = f32::min(24., 40. - f32::abs(p.base_object.x + 4. - 64.)).floor() as i32;
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
// k_up=2
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

fn level_index(game_state: &GameState) -> i32 {
    game_state.room.x % 8 + game_state.room.y * 8
}

/// Starting title screen
fn is_title(game_state: &GameState) -> bool {
    level_index(game_state) == 31
}

// -- player entity --
// -------------------

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
    hitbox: Hitbox,
    spr_off: f32,
    was_on_ground: bool,
    hair: Hair,
}

impl Player {
    fn init(x: f32, y: f32, max_djump: i32) -> Self {
        let hair = Self::create_hair(x, y);
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
            hitbox: Hitbox {
                x: 1.,
                y: 3.,
                w: 6,
                h: 5,
            },
            spr_off: 0.,
            was_on_ground: false,
            hair,
        }
    }

    fn create_hair(x: f32, y: f32) -> Hair {
        Hair {
            segments: [0, 1, 2, 3, 4].map(|index| HairElement {
                x,
                y,
                size: i32::max(1, i32::min(2, 3 - index)),
            }),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct HairElement {
    x: f32,
    y: f32,
    size: i32,
}

// player =
//     update=function(this)
//         if (pause_player) return

//         local input = btn(k_right) and 1 or (btn(k_left) and -1 or 0)

//         -- spikes collide
//         if spikes_at(this.x+this.hitbox.x,this.y+this.hitbox.y,this.hitbox.w,this.hitbox.h,this.spd.x,this.spd.y) then
//          kill_player(this) end

//         -- bottom death
//         if this.y>128 then
//             kill_player(this) end

//         local on_ground=this.is_solid(0,1)
//         local on_ice=this.is_ice(0,1)

//         -- smoke particles
//         if on_ground and not this.was_on_ground then
//          init_object(smoke,this.x,this.y+4)
//         end

//         local jump = btn(k_jump) and not this.p_jump
//         this.p_jump = btn(k_jump)
//         if (jump) then
//             this.jbuffer=4
//         elseif this.jbuffer>0 then
//          this.jbuffer-=1
//         end

//         local dash = btn(k_dash) and not this.p_dash
//         this.p_dash = btn(k_dash)

//         if on_ground then
//             this.grace=6
//             if this.djump<max_djump then
//              psfx(54)
//              this.djump=max_djump
//             end
//         elseif this.grace > 0 then
//          this.grace-=1
//         end

//         this.dash_effect_time -=1
//   if this.dash_time > 0 then
//    init_object(smoke,this.x,this.y)
//       this.dash_time-=1
//       this.spd.x=appr(this.spd.x,this.dash_target.x,this.dash_accel.x)
//       this.spd.y=appr(this.spd.y,this.dash_target.y,this.dash_accel.y)
//   else

//             -- move
//             local maxrun=1
//             local accel=0.6
//             local deccel=0.15

//             if not on_ground then
//                 accel=0.4
//             elseif on_ice then
//                 accel=0.05
//                 if input==(this.flip.x and -1 or 1) then
//                     accel=0.05
//                 end
//             end

//             if abs(this.spd.x) > maxrun then
//              this.spd.x=appr(this.spd.x,sign(this.spd.x)*maxrun,deccel)
//             else
//                 this.spd.x=appr(this.spd.x,input*maxrun,accel)
//             end

//             --facing
//             if this.spd.x!=0 then
//                 this.flip.x=(this.spd.x<0)
//             end

//             -- gravity
//             local maxfall=2
//             local gravity=0.21

//       if abs(this.spd.y) <= 0.15 then
//        gravity*=0.5
//             end

//             -- wall slide
//             if input!=0 and this.is_solid(input,0) and not this.is_ice(input,0) then
//              maxfall=0.4
//              if rnd(10)<2 then
//                  init_object(smoke,this.x+input*6,this.y)
//                 end
//             end

//             if not on_ground then
//                 this.spd.y=appr(this.spd.y,maxfall,gravity)
//             end

//             -- jump
//             if this.jbuffer>0 then
//              if this.grace>0 then
//               -- normal jump
//               psfx(1)
//               this.jbuffer=0
//               this.grace=0
//                     this.spd.y=-2
//                     init_object(smoke,this.x,this.y+4)
//                 else
//                     -- wall jump
//                     local wall_dir=(this.is_solid(-3,0) and -1 or this.is_solid(3,0) and 1 or 0)
//                     if wall_dir!=0 then
//                      psfx(2)
//                      this.jbuffer=0
//                      this.spd.y=-2
//                      this.spd.x=-wall_dir*(maxrun+1)
//                      if not this.is_ice(wall_dir*3,0) then
//                          init_object(smoke,this.x+wall_dir*6,this.y)
//                         end
//                     end
//                 end
//             end

//             -- dash
//             local d_full=5
//             local d_half=d_full*0.70710678118

//             if this.djump>0 and dash then
//              init_object(smoke,this.x,this.y)
//              this.djump-=1
//              this.dash_time=4
//              has_dashed=true
//              this.dash_effect_time=10
//              local v_input=(btn(k_up) and -1 or (btn(k_down) and 1 or 0))
//              if input!=0 then
//               if v_input!=0 then
//                this.spd.x=input*d_half
//                this.spd.y=v_input*d_half
//               else
//                this.spd.x=input*d_full
//                this.spd.y=0
//               end
//              elseif v_input!=0 then
//                  this.spd.x=0
//                  this.spd.y=v_input*d_full
//              else
//                  this.spd.x=(this.flip.x and -1 or 1)
//               this.spd.y=0
//              end

//              psfx(3)
//              freeze=2
//              shake=6
//              this.dash_target.x=2*sign(this.spd.x)
//              this.dash_target.y=2*sign(this.spd.y)
//              this.dash_accel.x=1.5
//              this.dash_accel.y=1.5

//              if this.spd.y<0 then
//               this.dash_target.y*=.75
//              end

//              if this.spd.y!=0 then
//               this.dash_accel.x*=0.70710678118
//              end
//              if this.spd.x!=0 then
//               this.dash_accel.y*=0.70710678118
//              end
//             elseif dash and this.djump<=0 then
//              psfx(9)
//              init_object(smoke,this.x,this.y)
//             end

//         end

//         -- animation
//         this.spr_off+=0.25
//         if not on_ground then
//             if this.is_solid(input,0) then
//                 this.spr=5
//             else
//                 this.spr=3
//             end
//         elseif btn(k_down) then
//             this.spr=6
//         elseif btn(k_up) then
//             this.spr=7
//         elseif (this.spd.x==0) or (not btn(k_left) and not btn(k_right)) then
//             this.spr=1
//         else
//             this.spr=1+this.spr_off%4
//         end

//         -- next level
//         if this.y<-4 and level_index()<30 then next_room() end

//         -- was on the ground
//         this.was_on_ground=on_ground

//     end, --<end update loop

//     draw=function(this)

//         -- clamp in screen
//         if this.x<-1 or this.x>121 then
//             this.x=clamp(this.x,-1,121)
//             this.spd.x=0
//         end

//         set_hair_color(this.djump)
//         draw_hair(this,this.flip.x and -1 or 1)
//         spr(this.spr,this.x,this.y,1,1,this.flip.x,this.flip.y)
//         unset_hair_color()
//     end
// }

#[allow(dead_code, unused_variables)]
fn psfx(game_state: &GameState, num: i32) {
    if game_state.sfx_timer <= 0 {
        // TODO: Implement
        // sfx(num)
    }
}

#[allow(dead_code)]
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
fn draw_hair(
    state: &State,
    draw: &mut DrawContext,
    x: f32,
    y: f32,
    hair: &mut [HairElement],
    facing: i32,
) {
    let mut last = Vec2 {
        x: x + (4 - facing * 2) as f32,
        y: y + (if state.btn(K_DOWN) { 4. } else { 3. }),
    };

    for hair_element in hair {
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

#[allow(dead_code)]
fn unset_hair_color(draw: &mut DrawContext) {
    draw.pal(8, 8);
}

// player_spawn = {
//     tile=1,
//     init=function(this)
//      sfx(4)
//         this.spr=3
//         this.target= {x=this.x,y=this.y}
//         this.y=128
//         this.spd.y=-4
//         this.state=0
//         this.delay=0
//         this.solids=false
//         create_hair(this)
//     end,
//     update=function(this)
//         -- jumping up
//         if this.state==0 then
//             if this.y < this.target.y+16 then
//                 this.state=1
//                 this.delay=3
//             end
//         -- falling
//         elseif this.state==1 then
//             this.spd.y+=0.5
//             if this.spd.y>0 and this.delay>0 then
//                 this.spd.y=0
//                 this.delay-=1
//             end
//             if this.spd.y>0 and this.y > this.target.y then
//                 this.y=this.target.y
//                 this.spd = {x=0,y=0}
//                 this.state=2
//                 this.delay=5
//                 shake=5
//                 init_object(smoke,this.x,this.y+4)
//                 sfx(5)
//             end
//         -- landing
//         elseif this.state==2 then
//             this.delay-=1
//             this.spr=6
//             if this.delay<0 then
//                 destroy_object(this)
//                 init_object(player,this.x,this.y)
//             end
//         end
//     end,
//     draw=function(this)
//         set_hair_color(max_djump)
//         draw_hair(this,1)
//         spr(this.spr,this.x,this.y,1,1,this.flip.x,this.flip.y)
//         unset_hair_color()
//     end
// }
// add(types,player_spawn)

struct Spring {
    hide_in: i32,
    hide_for: i32,
}

#[derive(PartialEq, Clone, Copy)]
struct BaseObject {
    x: f32,
    y: f32,
    hitbox: Hitbox,
    spr: f32, // hack they use
    spd: Vec2<f32>,
    rem: Vec2<f32>,
    last: f32,
    dir: i32, // not sure if all objects use this?
    // obj.solids in original source
    is_solid: bool,
    collideable: bool,
    flip: Vec2<bool>,
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

// fake_wall = {
//     tile=64,
//     if_not_fruit=true,
//     update=function(this)
//         this.hitbox={x=-1,y=-1,w=18,h=18}
//         local hit = this.collide(player,0,0)
//         if hit~=nil and hit.dash_effect_time>0 then
//             hit.spd.x=-sign(hit.spd.x)*1.5
//             hit.spd.y=-1.5
//             hit.dash_time=-1
//             sfx_timer=20
//             sfx(16)
//             destroy_object(this)
//             init_object(smoke,this.x,this.y)
//             init_object(smoke,this.x+8,this.y)
//             init_object(smoke,this.x,this.y+8)
//             init_object(smoke,this.x+8,this.y+8)
//             init_object(fruit,this.x+4,this.y+4)
//         end
//         this.hitbox={x=0,y=0,w=16,h=16}
//     end,
//     draw=function(this)
//         spr(64,this.x,this.y)
//         spr(65,this.x+8,this.y)
//         spr(80,this.x,this.y+8)
//         spr(81,this.x+8,this.y+8)
//     end
// }
// add(types,fake_wall)

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

#[derive(PartialEq, Clone, Copy)]
struct RoomTitle {
    delay: i32,
}

impl RoomTitle {
    fn init() -> Self {
        Self { delay: 5 }
    }

    fn update(&mut self) -> bool {
        self.delay -= 1;

        // Destroy if
        self.delay < -30
    }

    fn draw(&self, draw: &mut DrawContext, game_state: &GameState) {
        if self.delay < 0 {
            draw.rectfill(24, 58, 104, 70, 0);

            if game_state.room.x == 3 && game_state.room.y == 1 {
                draw.print("OLD SITE", 48, 62, 7);
            } else if level_index(game_state) == 30 {
                draw.print("SUMMIT", 52, 62, 7);
            } else {
                let level = (1 + level_index(game_state)) * 100;
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
    x: f32,
    y: f32,
    w: i32,
    h: i32,
}

#[derive(PartialEq, Clone, Copy)]
struct Object {
    base_object: BaseObject,
    object_type: ObjectType,
}

impl Object {
    fn init(game_state: &GameState, kind: ObjectKind, x: f32, y: f32) -> Option<Self> {
        // What this means: If the fruit has been already
        // picked up, don't instantiate this (fake wall containing, flying fruits, chests, etc)
        if dbg!(&kind).if_not_fruit() && game_state.got_fruit[1 + level_index(game_state) as usize]
        {
            return None;
        }

        let mut base_object = BaseObject {
            x,
            y,
            collideable: true,
            is_solid: true,
            hitbox: Hitbox {
                x: 0.,
                y: 0.,
                w: 8,
                h: 8,
            },
            spd: Vec2 { x: 0., y: 0. },
            rem: Vec2 { x: 0., y: 0. },
            last: 0.,
            dir: 0,
            flip: Vec2 { x: false, y: false },
            // TODO: figure out if we need an option here
            spr: kind.tile().map(|t| t as f32).unwrap_or(-42.),
        };
        let object_type = ObjectKind::create(&kind, &mut base_object);

        // kind.init(&mut object);

        Some(Self {
            base_object,
            object_type,
        })
    }

    fn draw(&self, draw: &mut DrawContext, game_state: &GameState) {
        match self.object_type {
            // ObjectType::Platform => todo!(),
            // ObjectType::BigChest => todo!(),
            ObjectType::Player => {
                // if this.x<-1 or this.x>121 then
                //     this.x=clamp(this.x,-1,121)
                //     this.spd.x=0
                // end

                // set_hair_color(this.djump)
                // draw_hair(this,this.flip.x and -1 or 1)
                draw.spr(
                    self.base_object.spr.floor() as usize,
                    self.base_object.x.floor() as i32,
                    self.base_object.y.floor() as i32,
                    // 1,
                    // 1,
                    // self.base_object.flip.x,
                    // self.base_object.flip.y,
                )
                // unset_hair_color()
            }
            // ObjectType::Smoke => todo!(),
            // ObjectType::LifeUp => todo!(),
            // ObjectType::Fruit => todo!(),
            // ObjectType::Orb => todo!(),
            // ObjectType::FakeWall => todo!(),
            // ObjectType::FallFloor => todo!(),
            // ObjectType::Key => todo!(),
            ObjectType::RoomTitle(room_title) => room_title.draw(draw, game_state),
            _ => {
                if self.base_object.spr > 0. {
                    // TODO: Implement version with many arguments
                    // draw.spr(self.base_object.spr, self.base_object.x, self.base_object.y, 1, 1, self.base_object.flip.x, self.base_object.flip.y);
                    draw.spr(
                        self.base_object.spr.floor() as usize,
                        self.base_object.x.floor() as i32,
                        self.base_object.y.floor() as i32,
                    );
                }
            }
        }
    }

    fn move_(&mut self, state: &State, objects: &[Object], room: Vec2<i32>) {
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

    fn move_x(
        &mut self,
        state: &State,
        objects: &[Object],
        room: Vec2<i32>,
        amount: i32,
        start: i32,
    ) {
        if self.base_object.is_solid {
            let step = amount.signum();

            for _ in start..=amount.abs() {
                if !self.is_solid(state, objects, room, step, 0) {
                    self.base_object.x += step as f32;
                } else {
                    self.base_object.spd.x = 0.;
                    self.base_object.rem.x = 0.;
                    break;
                }
            }
        } else {
            self.base_object.x += amount as f32;
        }
    }

    fn move_y(&mut self, state: &State, objects: &[Object], room: Vec2<i32>, amount: i32) {
        if self.base_object.is_solid {
            let step = amount.signum();

            for _ in 0..=amount.abs() {
                if !self.is_solid(state, objects, room, 0, step) {
                    self.base_object.y += step as f32;
                } else {
                    self.base_object.spd.y = 0.;
                    self.base_object.rem.y = 0.;
                    break;
                }
            }
        } else {
            self.base_object.y += amount as f32;
        }
    }

    fn is_solid(
        &self,
        state: &State,
        objects: &[Object],
        room: Vec2<i32>,
        ox: i32,
        oy: i32,
    ) -> bool {
        if oy > 0
            && !self.check(objects, &ObjectType::Platform, ox, 0)
            && self.check(objects, &ObjectType::Platform, ox, oy)
        {
            return true;
        }

        solid_at(
            state,
            room,
            (self.base_object.x + self.base_object.hitbox.x + ox as f32).floor() as i32,
            (self.base_object.y + self.base_object.hitbox.y + oy as f32).floor() as i32,
            self.base_object.hitbox.w,
            self.base_object.hitbox.h,
        )
        // || self.check(objects, &ObjectType::FallFloor, ox, oy)
        //     || self.check(objects, &ObjectType::FakeWall, ox, oy)
    }

    fn check(&self, objects: &[Object], type_: &ObjectType, ox: i32, oy: i32) -> bool {
        self.collide(objects, type_, ox, oy).is_some()
    }

    fn collide<'a>(
        &self,
        objects: &'a [Object],
        type_: &ObjectType,
        ox: i32,
        oy: i32,
    ) -> Option<(usize, &'a Object)> {
        for (index, other) in objects.iter().enumerate() {
            if !std::ptr::eq(other, self)
                && other.object_type == self.object_type
                && other.base_object.collideable
                && other.base_object.x
                    + other.base_object.hitbox.x
                    + other.base_object.hitbox.w as f32
                    > self.base_object.x + self.base_object.hitbox.x + ox as f32
                && other.base_object.y
                    + other.base_object.hitbox.y
                    + other.base_object.hitbox.h as f32
                    > self.base_object.y + self.base_object.hitbox.y + oy as f32
                && other.base_object.x + other.base_object.hitbox.x
                    < self.base_object.x
                        + self.base_object.hitbox.x
                        + self.base_object.hitbox.w as f32
                        + ox as f32
                && other.base_object.y + other.base_object.hitbox.y
                    < self.base_object.y
                        + self.base_object.hitbox.y
                        + self.base_object.hitbox.h as f32
                        + oy as f32
            {
                return Some((index, other));
            }
        }
        None
    }
}

fn tile_flag_at(state: &State, room: Vec2<i32>, x: i32, y: i32, w: i32, h: i32, flag: i32) -> bool {
    for i in 0.max(x / 8)..=(15.min((x + w - 1) / 8)) {
        for j in 0.max(y / 8)..=(15.min((y + h - 1) / 8)) {
            if state.fget_n(tile_at(state, room, i, j) as usize, flag as u8) {
                return true;
            }
        }
    }
    false
}

fn tile_at(state: &State, room: Vec2<i32>, x: i32, y: i32) -> i32 {
    state.mget(room.x * 16 + x, room.y * 16 + y).into()
}

fn solid_at(state: &State, room: Vec2<i32>, x: i32, y: i32, w: i32, h: i32) -> bool {
    tile_flag_at(state, room, x, y, w, h, 0)
}

#[derive(PartialEq, Clone, Copy)]
enum ObjectType {
    Platform,
    // BigChest,
    Player,
    PlayerSpawn(PlayerSpawn),
    // Smoke,
    // LifeUp,
    // Fruit,
    // Orb,
    // FakeWall,
    // FallFloor,
    // Key,
    RoomTitle(RoomTitle),
}

impl ObjectType {
    // TODO: Figure out what exactly needs to go here
    // const TYPES: &'static [ObjectType] = &[Self::BigChest];

    fn update(&mut self, base_object: &mut BaseObject, state: &State) -> bool {
        match self {
            ObjectType::PlayerSpawn(player_spawn) => player_spawn.update(base_object, state),
            ObjectType::Platform => todo!(),
            // ObjectType::BigChest => todo!(),
            ObjectType::Player => {
                let input = if state.btn(K_RIGHT) {
                    1
                } else if state.btn(K_LEFT) {
                    -1
                } else {
                    0
                } as f32;

                // -- move
                // part
                let mut maxrun = 1.0;
                let mut accel = 0.6;
                let mut deccel = 0.15;

                let on_ground = true;
                let on_ice = false;
                if !on_ground {
                    accel = 0.4;
                } else if on_ice {
                    accel = 0.05;

                    if input == (if base_object.flip.x { -1.0 } else { 1.0 }) {
                        accel = 0.05;
                    }
                }

                if base_object.spd.x.abs() > maxrun {
                    base_object.spd.x = appr(
                        base_object.spd.x,
                        base_object.spd.x.signum() * maxrun,
                        deccel,
                    );
                } else {
                    base_object.spd.x = appr(base_object.spd.x, input * maxrun, accel);
                }

                // TODO: Update

                // dont destroy
                false
            }
            // ObjectType::Smoke => todo!(),
            // ObjectType::LifeUp => todo!(),
            // ObjectType::Fruit => todo!(),
            // ObjectType::Orb => todo!(),
            // ObjectType::FakeWall => todo!(),
            // ObjectType::FallFloor => todo!(),
            // ObjectType::Key => todo!(),
            ObjectType::RoomTitle(rt) => rt.update(),
        }
    }
}

impl Object {
    fn is_ice(&self, state: &State, room: Vec2<i32>, ox: f32, oy: f32) -> bool {
        ice_at(
            state,
            room,
            (self.base_object.x + self.base_object.hitbox.x + ox).floor() as i32,
            (self.base_object.y + self.base_object.hitbox.y + oy).floor() as i32,
            self.base_object.hitbox.w,
            self.base_object.hitbox.h,
        )
    }
}

fn kill_player(obj: &Object, game_state: &mut GameState) {
    game_state.sfx_timer = 12;
    // sfx(0);
    game_state.deaths += 1;
    game_state.shake = 10;
    destroy_object(game_state, obj);

    game_state.dead_particles.clear();
    for dir in 0..=7 {
        let dir = dir as f32;
        let angle = dir / 8.;

        game_state.dead_particles.push(DeadParticle {
            x: obj.base_object.x + 4.,
            y: obj.base_object.y + 4.,
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
    game_state.objects.retain(|o| o != object);
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

    println!("Begin game {} {}", x, y);
    for tx in 0..=15 {
        for ty in 0..=15 {
            // entities
            let ftx = tx as f32;
            let fty = ty as f32;
            let tile = state.mget(game_state.room.x * 16 + tx, game_state.room.y * 16 + ty);
            if tile == 11 {
                let mut platform =
                    Object::init(game_state, ObjectKind::Platform, ftx * 8., fty * 8.).unwrap();
                platform.base_object.dir = -1;
                game_state.objects.push(platform);
            } else if tile == 12 {
                let mut platform =
                    Object::init(game_state, ObjectKind::Platform, ftx * 8., fty * 8.).unwrap();
                platform.base_object.dir = 1;
                game_state.objects.push(platform);
            } else {
                for kind in ObjectKind::TYPES.iter().copied() {
                    if kind.tile() == Some(tile.into()) {
                        if let Some(object) = Object::init(game_state, kind, ftx * 8., fty * 8.) {
                            game_state.objects.push(object);
                        }
                    }
                }
            }
        }
    }

    if !is_title(game_state) {
        if let Some(object) = Object::init(&game_state, ObjectKind::RoomTitle, 0., 0.) {
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

    fn update(
        self_: &mut Object,
        state: &State,
        objects: &[Object],
        room: Vec2<i32>,
    ) -> Option<(usize, Object)> {
        self_.base_object.spd.x = self_.base_object.dir as f32 * 0.65;
        if self_.base_object.x < -16. {
            self_.base_object.x = 128.;
        } else if self_.base_object.x > 128. {
            self_.base_object.x = -16.;
        }
        self_.base_object.last = self_.base_object.x;

        let ret = if !self_.check(objects, &ObjectType::Player, 0, 0) {
            let (index, hit) = self_.collide(objects, &ObjectType::Player, 0, -1)?;
            let mut hit = *hit;
            hit.move_x(
                state,
                objects,
                room,
                (self_.base_object.x - self_.base_object.last).floor() as i32,
                1,
            );

            Some((index, hit))
        } else {
            None
        };

        ret
    }

    fn draw(self_: &Object, draw: &mut DrawContext) {
        draw.spr(
            11,
            self_.base_object.x.floor() as i32,
            self_.base_object.y.floor() as i32 - 1,
        );
        draw.spr(
            12,
            self_.base_object.x.floor() as i32 + 8,
            self_.base_object.y.floor() as i32 - 1,
        )
    }
}
// struct Smoke;

// impl Smoke {
//     fn init(this: &mut Object) {
//         this.spr = 29.;
//         this.spd.y = -0.1;
//         this.spd.x = 0.3 + rnd(0.2);
//         this.x += -1. + rnd(2.);
//         this.y += -1. + rnd(2.);
//         this.flip.x = maybe();
//         this.flip.y = maybe();
//         this.is_solid = false;
//     }

//     fn update(self_: &mut Object) -> bool {
//         self_.spr += 0.2;

//         // destroy if
//         self_.spr >= 32.
//     }
// }

struct Fruit;

impl Fruit {
    fn init(object: &mut Object) {
        // object.start
    }
}

#[derive(Clone, Copy, Debug)]
enum ObjectKind {
    PlayerSpawn,
    // Spring,
    // Balloon,
    // FallFloor,
    // Fruit,
    // FlyFruit,
    // FakeWall,
    // Key,
    // Chest,
    // Message,
    // BigChest,
    // Flag,

    // Non-tile-instantiable
    RoomTitle,
    Platform,
}

impl ObjectKind {
    // I think these are the "instantiable" objects
    // (you put a "marker" tile in the map and this creates the object for it)
    // see line 1135 of source.p8
    const TYPES: &'static [Self] = &[
        ObjectKind::PlayerSpawn,
        // ObjectKind::Spring,
        // ObjectKind::Balloon,
        // ObjectKind::FallFloor,
        // ObjectKind::Fruit,
        // ObjectKind::FlyFruit,
        // ObjectKind::FakeWall,
        // ObjectKind::Key,
        // ObjectKind::Chest,
        // ObjectKind::Message,
        // ObjectKind::BigChest,
        // ObjectKind::Flag,
    ];

    fn init(&self, object: &mut Object) {
        match self {
            ObjectKind::PlayerSpawn => todo!(),
            // ObjectKind::Spring => todo!(),
            // ObjectKind::Balloon => todo!(),
            // ObjectKind::FallFloor => todo!(),
            // ObjectKind::Fruit => todo!(),
            // ObjectKind::FlyFruit => todo!(),
            // ObjectKind::FakeWall => todo!(),
            // ObjectKind::Key => todo!(),
            // ObjectKind::Chest => todo!(),
            // ObjectKind::Message => todo!(),
            // ObjectKind::BigChest => todo!(),
            // ObjectKind::Flag => todo!(),
            ObjectKind::RoomTitle => todo!(),
            ObjectKind::Platform => todo!(),
        }
    }

    fn create(&self, object: &mut BaseObject) -> ObjectType {
        match self {
            ObjectKind::PlayerSpawn => ObjectType::PlayerSpawn(PlayerSpawn::init(object)),
            // ObjectKind::Spring => todo!(),
            // ObjectKind::Balloon => todo!(),
            // ObjectKind::FallFloor => todo!(),
            // ObjectKind::Fruit => todo!(),
            // ObjectKind::FlyFruit => todo!(),
            // ObjectKind::FakeWall => todo!(),
            // ObjectKind::Key => todo!(),
            // ObjectKind::Chest => todo!(),
            // ObjectKind::Message => todo!(),
            // ObjectKind::BigChest => todo!(),
            // ObjectKind::Flag => todo!(),
            ObjectKind::RoomTitle => ObjectType::RoomTitle(RoomTitle { delay: 5 }),
            ObjectKind::Platform => todo!(),
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
            // ObjectKind::FakeWall => Some(64),
            // ObjectKind::Key => Some(8),
            // ObjectKind::Chest => Some(20),
            // ObjectKind::Message => Some(86),
            // ObjectKind::BigChest => Some(96),
            // ObjectKind::Flag => Some(118),
            _ => None,
        }
    }

    fn if_not_fruit(&self) -> bool {
        #[allow(clippy::match_single_binding)]
        match self {
            // ObjectKind::Fruit => true,
            // ObjectKind::FlyFruit => true,
            // ObjectKind::FakeWall => true,
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

#[derive(PartialEq, Clone, Copy, Debug)]
struct PlayerSpawn {
    hair: Hair,
    delay: i32,
    target: Vec2<f32>,
    state: PlayerSpawnState,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum PlayerSpawnState {
    Jumping,
    Falling,
    Landing,
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
        base_object.y = 128.0;
        base_object.spd.y = -4.0;
        base_object.is_solid = false;

        Self {
            hair: Player::create_hair(base_object.x, base_object.y),
            delay: 0,
            target,
            state: Jumping,
        }
    }

    fn update(&mut self, base_object: &mut BaseObject, state: &State) -> bool {
        match self.state {
            PlayerSpawnState::Jumping => {
                if base_object.y < self.target.y + 16.0 {
                    self.state = PlayerSpawnState::Falling;
                    self.delay = 3;
                }

                false
            }
            PlayerSpawnState::Falling => {
                base_object.spd.y += 0.5;

                if base_object.spd.y > 0.0 && self.delay > 0 {
                    base_object.spd.y = 0.0;
                    self.delay -= 1;
                }
                if base_object.spd.y > 0.0 && base_object.y > self.target.y {
                    base_object.y = self.target.y;
                    base_object.spd = Vec2 { x: 0.0, y: 0.0 };
                    self.state = PlayerSpawnState::Landing;
                    self.delay = 5;
                    // TODO: Integrate this screen shake
                    // shake = 5;
                    // init_object(smoke, this.x, base_object.y + 4);
                    // sfx(5);
                }
                false
            }
            PlayerSpawnState::Landing => {
                self.delay -= 1;
                base_object.spr = 6.0;
                //     if this.delay<0 then
                //         destroy_object(this)
                //         init_object(player,this.x,this.y)
                //     end
                false
            }
        }
    }
}
