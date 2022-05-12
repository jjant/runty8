#![feature(drain_filter)]
use std::path::Path;

use rand::Rng;
use runty8::runtime::draw_context::DrawContext;
use runty8::runtime::state::{Button, State};
use runty8::App;

// Deduplicate this code.
fn create_directory() -> String {
    let buf = Path::new(file!()).with_extension("");
    let dir_name = buf.to_str().unwrap();

    if let Err(e) = std::fs::create_dir(dir_name) {
        println!("Couldn't create directory, error: {:?}", e);
    };

    dir_name.to_owned()
}

fn main() {
    let assets_path = create_directory();

    runty8::run_app::<GameState>(assets_path)
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

#[derive(PartialEq, Clone, Copy)]
struct Vec2<T> {
    x: T,
    y: T,
}

fn rnd(max: f32) -> f32 {
    rand::thread_rng().gen_range(0.0..max)
}

impl App for GameState {
    fn init() -> Self {
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

        title_screen(&mut gs);

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
                load_room(self, self.room.x, self.room.y);
            }
        }

        // Update each object?
        self.objects = self
            .objects
            .iter()
            .copied()
            .filter_map(|mut object| {
                object.move_(state, &self.objects, self.room);
                let should_destroy = object.type_.update();

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
                    self.begin_game();
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
                c = 0
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
        // TODO: Implement `map` api
        // draw.map(self.room.x * 16, self.room.y * 16, 0, 0, 16, 16, 4);

        // Platforms/big chest
        for object in self.objects.iter() {
            if object.type_ == ObjectType::Platform || object.type_ == ObjectType::BigChest {
                object.draw(draw, self);
            }
        }

        // draw terrain
        // TODO: Implement map API
        // let off = if is_title(self) { -4 } else { 0 };
        // map(self.room.x * 16, self.room.y * 16, off, 0, 16, 16, 2);

        // Draw objects
        for object in &self.objects {
            if object.type_ != ObjectType::Platform && object.type_ != ObjectType::BigChest {
                object.draw(draw, self);
            }
        }

        // Draw fg terrain
        // TODO
        // map(self.room.x * 16, self.room.y * 16, 0, 0, 16, 16, 8);

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
            if let Some(p) = self.objects.iter().find(|o| o.type_ == ObjectType::Player) {
                let diff = f32::min(24., 40. - f32::abs(p.x + 4. - 64.)).floor() as i32;
                draw.rectfill(0, 0, diff, 128, 0);
                draw.rectfill(128 - diff, 0, 128, 128, 0);
            }
        }
    }
}

impl GameState {
    fn begin_game(&mut self) {
        self.frames = 0;
        self.seconds = 0;
        self.minutes = 0;
        self.music_timer = 0;
        self.start_game = false;
        // music(0, 0, 7);
        load_room(self, 0, 0);
    }
}

// k_left=0
// k_right=1
// k_up=2
const K_DOWN: Button = Button::Down;
const K_JUMP: Button = Button::C;
const K_DASH: Button = Button::X;

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

fn title_screen(game_state: &mut GameState) {
    game_state.got_fruit = vec![false; 30];
    game_state.frames = 0;
    game_state.deaths = 0;
    game_state.max_djump = 1;
    game_state.start_game = false;
    game_state.start_game_flash = 0;
    // music(40,0,7)
    load_room(game_state, 7, 3)
}

fn level_index(game_state: &GameState) -> i32 {
    game_state.room.x % 8 + game_state.room.y * 8
}

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
    hair: Vec<HairElement>,
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

    fn create_hair(x: f32, y: f32) -> Vec<HairElement> {
        let mut vec = Vec::with_capacity(5);

        for i in 0..=4 {
            vec.push(HairElement {
                x,
                y,
                size: i32::max(1, i32::min(2, 3 - i)),
            })
        }

        vec
    }
}

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

struct BaseObject {
    x: f32,
    y: f32,
    spr: f32,
    delay: i32,
}

impl Spring {
    fn init() -> Self {
        Self {
            hide_in: 0,
            hide_for: 0,
        }
    }

    fn update(&mut self, object: &mut BaseObject) {
        if self.hide_for > 0 {
            self.hide_for -= 1;

            if self.hide_for <= 0 {
                object.spr = 18.;
                object.delay = 0;
            }
        } else if object.spr == 18. {
            // TODO: Borrowchecker madness
            // let hit = object.collide(player);
            //
            // if let Some(hit) = hit.and_then(|hit| hit.spd.y >= 0) {
            //     object.spr = 19.;
            //     hit.y = object.y - 4.;
            //     hit.spd.x *= 0.2;
            //     hit.spd.y = -3;
            // hit.djump = game_state.max_djump;
            // object.delay = 10;
            //
            // init_object(smoke,this.x,this.y)
            //
            // -- breakable below us
            // local below=this.collide(fall_floor,0,1)
            // if below~=nil then
            //     break_fall_floor(below)
            // end
            //
            // psfx(8)
            // }
        } else if object.delay > 0 {
            object.delay -= 1;

            if object.delay <= 0 {
                object.spr = 18.;
            }
        }

        // begin hiding
        if self.hide_in > 0 {
            self.hide_in -= 1;

            if self.hide_in <= 0 {
                self.hide_for = 60;
                object.spr = 0.;
            }
        }
    }

    fn break_spring(&mut self) {
        self.hide_in = 15;
    }
}

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

#[derive(PartialEq, Clone, Copy)]
struct Hitbox {
    x: f32,
    y: f32,
    w: i32,
    h: i32,
}

#[derive(PartialEq, Clone, Copy)]
struct Object {
    x: f32,
    y: f32,
    hitbox: Hitbox,
    type_: ObjectType,
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

impl Object {
    fn init(game_state: &GameState, kind: ObjectKind, x: f32, y: f32) -> Option<Self> {
        // What this means: If the fruit has been already
        // picked up, don't instantiate this (fake wall containing, flying fruits, chests, etc)
        if kind.if_not_fruit() && game_state.got_fruit[1 + level_index(game_state) as usize] {
            return None;
        }

        let mut object = Self {
            x,
            y,
            type_: todo!(),
            collideable: true,
            is_solid: true,
            // TODO: figure out if we need an option here
            spr: kind.tile().map(|t| t as f32).unwrap_or(-42.),
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
        };

        kind.init(&mut object);

        Some(object)
    }

    fn draw(&self, draw: &mut DrawContext, game_state: &GameState) {
        match self.type_ {
            ObjectType::Platform => todo!(),
            ObjectType::BigChest => todo!(),
            ObjectType::Player => todo!(),
            ObjectType::Smoke => todo!(),
            ObjectType::LifeUp => todo!(),
            ObjectType::Fruit => todo!(),
            ObjectType::Orb => todo!(),
            ObjectType::FakeWall => todo!(),
            ObjectType::FallFloor => todo!(),
            ObjectType::Key => todo!(),
            ObjectType::RoomTitle(room_title) => room_title.draw(draw, game_state),
            _ => {
                if self.spr > 0. {
                    // TODO: Implement version with many arguments
                    // draw.spr(self.spr, self.x, self.y, 1, 1, self.flip.x, self.flip.y);
                    draw.spr(
                        self.spr.floor() as usize,
                        self.x.floor() as i32,
                        self.y.floor() as i32,
                    );
                }
            }
        }
    }

    fn move_(&mut self, state: &State, objects: &[Object], room: Vec2<i32>) {
        let ox = self.spd.x;
        let oy = self.spd.y;

        // [x] get move amount
        self.rem.x += ox;
        let amount_x = (self.rem.x as f32 + 0.5).floor();
        self.rem.x -= amount_x;
        self.move_x(state, objects, room, amount_x as i32, 0);

        // [y] get move amount
        self.rem.y += oy;
        let amount_y = (self.rem.y as f32 + 0.5).floor();
        self.rem.y -= amount_y;
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
        if self.is_solid {
            let step = amount.signum();

            for _ in start..=amount.abs() {
                if !self.is_solid(state, objects, room, step, 0) {
                    self.x += step as f32;
                } else {
                    self.spd.x = 0.;
                    self.rem.x = 0.;
                    break;
                }
            }
        } else {
            self.x += amount as f32;
        }
    }

    fn move_y(&mut self, state: &State, objects: &[Object], room: Vec2<i32>, amount: i32) {
        if self.is_solid {
            let step = amount.signum();

            for _ in 0..=amount.abs() {
                if !self.is_solid(state, objects, room, 0, step) {
                    self.y += step as f32;
                } else {
                    self.spd.y = 0.;
                    self.rem.y = 0.;
                    break;
                }
            }
        } else {
            self.y += amount as f32;
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
            (self.x + self.hitbox.x + ox as f32).floor() as i32,
            (self.y + self.hitbox.y + oy as f32).floor() as i32,
            self.hitbox.w,
            self.hitbox.h,
        ) || self.check(objects, &ObjectType::FallFloor, ox, oy)
            || self.check(objects, &ObjectType::FakeWall, ox, oy)
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
                && other.type_ == self.type_
                && other.collideable
                && other.x + other.hitbox.x + other.hitbox.w as f32
                    > self.x + self.hitbox.x + ox as f32
                && other.y + other.hitbox.y + other.hitbox.h as f32
                    > self.y + self.hitbox.y + oy as f32
                && other.x + other.hitbox.x
                    < self.x + self.hitbox.x + self.hitbox.w as f32 + ox as f32
                && other.y + other.hitbox.y
                    < self.y + self.hitbox.y + self.hitbox.h as f32 + oy as f32
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
            // TODO: Implement api: `fget`
            if state.fget_n(tile_at(room, i, j) as usize, flag as u8) {
                return true;
            }
        }
    }
    false
}

fn tile_at(room: Vec2<i32>, x: i32, y: i32) -> i32 {
    mget(room.x * 16 + x, room.y * 16 + y)
}

fn solid_at(state: &State, room: Vec2<i32>, x: i32, y: i32, w: i32, h: i32) -> bool {
    tile_flag_at(state, room, x, y, w, h, 0)
}

#[derive(PartialEq, Clone, Copy)]
enum ObjectType {
    Platform,
    BigChest,
    Player,
    Smoke,
    LifeUp,
    Fruit,
    Orb,
    FakeWall,
    FallFloor,
    Key,
    RoomTitle(RoomTitle),
}

impl ObjectType {
    // TODO: Figure out what exactly needs to go here
    const TYPES: &'static [ObjectType] = &[Self::BigChest];

    fn init(&self, object: &mut Object) {
        match self {
            ObjectType::Platform => todo!(),
            ObjectType::BigChest => todo!(),
            ObjectType::Player => todo!(),
            ObjectType::Smoke => Smoke::init(object),
            ObjectType::LifeUp => todo!(),
            ObjectType::Fruit => Fruit::init(object),
            ObjectType::Orb => todo!(),
            ObjectType::FakeWall => todo!(),
            ObjectType::FallFloor => todo!(),
            ObjectType::Key => todo!(),
            ObjectType::RoomTitle(_) => todo!(),
        }
    }

    fn tile(&self) -> Option<i32> {
        match self {
            ObjectType::Platform => todo!(),
            ObjectType::BigChest => Some(96),
            ObjectType::Player => todo!(),
            ObjectType::Smoke => todo!(),
            ObjectType::LifeUp => todo!(),
            ObjectType::Fruit => Some(26),
            ObjectType::Orb => todo!(),
            ObjectType::FakeWall => Some(64),
            ObjectType::FallFloor => Some(23),
            ObjectType::Key => Some(8),
            ObjectType::RoomTitle(_) => None,
        }
    }

    fn update(&mut self) -> bool {
        match self {
            ObjectType::Platform => todo!(),
            ObjectType::BigChest => todo!(),
            ObjectType::Player => todo!(),
            ObjectType::Smoke => todo!(),
            ObjectType::LifeUp => todo!(),
            ObjectType::Fruit => todo!(),
            ObjectType::Orb => todo!(),
            ObjectType::FakeWall => todo!(),
            ObjectType::FallFloor => todo!(),
            ObjectType::Key => todo!(),
            ObjectType::RoomTitle(rt) => rt.update(),
        }
    }
}

impl Object {
    fn is_ice(&self, state: &State, room: Vec2<i32>, ox: f32, oy: f32) -> bool {
        ice_at(
            state,
            room,
            (self.x + self.hitbox.x + ox).floor() as i32,
            (self.y + self.hitbox.y + oy).floor() as i32,
            self.hitbox.w,
            self.hitbox.h,
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
            x: obj.x + 4.,
            y: obj.y + 4.,
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

fn next_room(game_state: &mut GameState) {
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
        load_room(game_state, 0, room.y + 1);
    } else {
        load_room(game_state, room.x + 1, room.y);
    }
}

fn mget(x: i32, y: i32) -> i32 {
    // todo!()
    0
}

fn load_room(game_state: &mut GameState, x: i32, y: i32) {
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
            let ftx = tx as f32;
            let fty = ty as f32;
            let tile = mget(game_state.room.x * 16 + tx, game_state.room.y * 16 + ty);
            if tile == 11 {
                let mut platform =
                    Object::init(game_state, ObjectKind::Platform, ftx * 8., fty * 8.).unwrap();
                platform.dir = -1;
                game_state.objects.push(platform);
            } else if tile == 12 {
                let mut platform =
                    Object::init(game_state, ObjectKind::Platform, ftx * 8., fty * 8.).unwrap();
                platform.dir = 1;
                game_state.objects.push(platform);
            } else {
                for kind in ObjectKind::TYPES.iter().copied() {
                    if kind.tile() == Some(tile) {
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

#[allow(dead_code)]
const MAP_DATA: &str = r#"2331252548252532323232323300002425262425252631323232252628282824252525252525323328382828312525253232323233000000313232323232323232330000002432323233313232322525252525482525252525252526282824252548252525262828282824254825252526282828283132323225482525252525
252331323232332900002829000000242526313232332828002824262a102824254825252526002a2828292810244825282828290000000028282900000000002810000000372829000000002a2831482525252525482525323232332828242525254825323338282a283132252548252628382828282a2a2831323232322525
252523201028380000002a0000003d24252523201028292900282426003a382425252548253300002900002a0031252528382900003a676838280000000000003828393e003a2800000000000028002425253232323232332122222328282425252532332828282900002a283132252526282828282900002a28282838282448
3232332828282900000000003f2020244825262828290000002a243300002a2425322525260000000000000000003125290000000021222328280000000000002a2828343536290000000000002839242526212223202123313232332828242548262b000000000000001c00003b242526282828000000000028282828282425
2340283828293a2839000000343522252548262900000000000030000000002433003125333d3f00000000000000003100001c3a3a31252620283900000000000010282828290000000011113a2828313233242526103133202828282838242525262b000000000000000000003b2425262a2828670016002a28283828282425
263a282828102829000000000000312525323300000000110000370000003e2400000037212223000000000000000000395868282828242628290000000000002a2828290000000000002123283828292828313233282829002a002a2828242525332b0c00000011110000000c3b314826112810000000006828282828282425
252235353628280000000000003a282426003d003a3900270000000000002125001a000024252611111111000000002c28382828283831332800000017170000002a000000001111000024261028290028281b1b1b282800000000002a2125482628390000003b34362b000000002824252328283a67003a28282829002a3132
25333828282900000000000000283824252320201029003039000000005824480000003a31323235353536675800003c282828281028212329000000000000000000000000003436003a2426282800003828390000002a29000000000031323226101000000000282839000000002a2425332828283800282828390000001700
2600002a28000000003a283a2828282425252223283900372858390068283132000000282828282820202828283921222829002a28282426000000000000000000000000000020382828312523000000282828290000000000163a67682828003338280b00000010382800000b00003133282828282868282828280000001700
330000002867580000281028283422252525482628286720282828382828212200003a283828102900002a28382824252a0000002838242600000017170000000000000000002728282a283133390000282900000000000000002a28282829002a2839000000002a282900000000000028282838282828282828290000000000
0000003a2828383e3a2828283828242548252526002a282729002a28283432250000002a282828000000002810282425000000002a282426000000000000000000000000000037280000002a28283900280000003928390000000000282800000028290000002a2828000000000000002a282828281028282828675800000000
0000002838282821232800002a28242532322526003a2830000000002a28282400000000002a281111111128282824480000003a28283133000000000000171700013f0000002029000000003828000028013a28281028580000003a28290000002a280c0000003a380c00000000000c00002a2828282828292828290000003a
00013a2123282a313329001111112425002831263a3829300000000000002a310000000000002834222236292a0024253e013a3828292a00000000000000000035353536000020000000003d2a28671422222328282828283900582838283d00003a290000000028280000000000000000002a28282a29000058100012002a28
22222225262900212311112122222525002a3837282900301111110000003a2800013f0000002a282426290000002425222222232900000000000000171700002a282039003a2000003a003435353535252525222222232828282810282821220b10000000000b28100000000b0000002c00002838000000002a283917000028
2548252526111124252222252525482500012a2828673f242222230000003828222223000012002a24260000001224252525252600000000171700000000000000382028392827080028676820282828254825252525262a28282122222225253a28013d0000006828390000000000003c0168282800171717003a2800003a28
25252525252222252525252525252525222222222222222525482667586828282548260000270000242600000021252525254826171700000000000000000000002a2028102830003a282828202828282525252548252600002a2425252548252821222300000028282800000000000022222223286700000000282839002838
2532330000002432323232323232252525252628282828242532323232254825253232323232323225262828282448252525253300000000000000000000005225253232323233313232323233282900262829286700000000002828313232322525253233282800312525482525254825254826283828313232323232322548
26282800000030402a282828282824252548262838282831333828290031322526280000163a28283133282838242525482526000000000000000000000000522526000016000000002a10282838390026281a3820393d000000002a3828282825252628282829003b2425323232323232323233282828282828102828203125
3328390000003700002a3828002a2425252526282828282028292a0000002a313328111111282828000028002a312525252526000000000000000000000000522526000000001111000000292a28290026283a2820102011111121222328281025252628382800003b24262b002a2a38282828282829002a2800282838282831
28281029000000000000282839002448252526282900282067000000000000003810212223283829003a1029002a242532323367000000000000000000004200252639000000212300000000002122222522222321222321222324482628282832323328282800003b31332b00000028102829000000000029002a2828282900
2828280016000000162a2828280024252525262700002a2029000000000000002834252533292a0000002a00111124252223282800002c46472c00000042535325262800003a242600001600002425252525482631323331323324252620283822222328292867000028290000000000283800111100001200000028292a1600
283828000000000000003a28290024254825263700000029000000000000003a293b2426283900000000003b212225252526382867003c56573c4243435363633233283900282426111111111124252525482526201b1b1b1b1b24252628282825252600002a28143a2900000000000028293b21230000170000112867000000
2828286758000000586828380000313232323320000000000000000000272828003b2426290000000000003b312548252533282828392122222352535364000029002a28382831323535353522254825252525252300000000003132332810284825261111113435361111111100000000003b3133111111111127282900003b
2828282810290000002a28286700002835353536111100000000000011302838003b3133000000000000002a28313225262a282810282425252662636400000000160028282829000000000031322525252525252667580000002000002a28282525323535352222222222353639000000003b34353535353536303800000017
282900002a0000000000382a29003a282828283436200000000000002030282800002a29000011110000000028282831260029002a282448252523000000000039003a282900000000000000002831322525482526382900000017000058682832331028293b2448252526282828000000003b201b1b1b1b1b1b302800000017
283a0000000000000000280000002828283810292a000000000000002a3710281111111111112136000000002a28380b2600000000212525252526001c0000002828281000000000001100002a382829252525252628000000001700002a212228282908003b242525482628282912000000001b00000000000030290000003b
3829000000000000003a102900002838282828000000000000000000002a2828223535353535330000000000002828393300000000313225252533000000000028382829000000003b202b00682828003232323233290000000000000000312528280000003b3132322526382800170000000000000000110000370000000000
290000000000000000002a000000282928292a0000000000000000000000282a332838282829000000000000001028280000000042434424252628390000000028002a0000110000001b002a2010292c1b1b1b1b0000000000000000000010312829160000001b1b1b313328106700000000001100003a2700001b0000000000
00000100000011111100000000002a3a2a0000000000000000000000002a2800282829002a000000000000000028282800000000525354244826282800000000290000003b202b39000000002900003c000000000000000000000000000028282800000000000000001b1b2a2829000001000027390038300000000000000000
1111201111112122230000001212002a00010000000000000000000000002900290000000000000000002a6768282900003f01005253542425262810673a3900013f0000002a3829001100000000002101000000000000003a67000000002a382867586800000100000000682800000021230037282928300000000000000000
22222222222324482611111120201111002739000017170000001717000000000001000000001717000000282838393a0021222352535424253328282838290022232b00000828393b27000000001424230000001200000028290000000000282828102867001717171717282839000031333927101228370000000000000000
254825252526242526212222222222223a303800000000000000000000000000001717000000000000003a28282828280024252652535424262828282828283925262b00003a28103b30000000212225260000002700003a28000000000000282838282828390000005868283828000022233830281728270000000000000000
00000000000000008242525252528452339200001323232352232323232352230000000000000000b302000013232352526200a2828342525223232323232323
00000000000000a20182920013232352363636462535353545550000005525355284525262b20000000000004252525262828282425284525252845252525252
00000000000085868242845252525252b1006100b1b1b1b103b1b1b1b1b103b100000000000000111102000000a282425233000000a213233300009200008392
000000000000110000a2000000a28213000000002636363646550000005525355252528462b2a300000000004252845262828382132323232323232352528452
000000000000a201821323525284525200000000000000007300000000007300000000000000b343536300410000011362b2000000000000000000000000a200
0000000000b302b2002100000000a282000000000000000000560000005526365252522333b28292001111024252525262019200829200000000a28213525252
0000000000000000a2828242525252840000000000000000b10000000000b1000000000000000000b3435363930000b162273737373737373737374711000061
000000110000b100b302b20000006182000000000000000000000000005600005252338282828201a31222225252525262820000a20011111100008283425252
0000000000000093a382824252525252000061000011000000000011000000001100000000000000000000020182001152222222222222222222222232b20000
0000b302b200000000b10000000000a200000000000000009300000000000000846282828283828282132323528452526292000000112434440000a282425284
00000000000000a2828382428452525200000000b302b2936100b302b20061007293a30000000000000000b1a282931252845252525252232323232362b20000
000000b10000001100000000000000000000000093000086820000a3000000005262828201a200a282829200132323236211111111243535450000b312525252
00000000000000008282821323232323820000a300b1a382930000b100000000738283931100000000000011a382821323232323528462829200a20173b20061
000000000000b302b2000061000000000000a385828286828282828293000000526283829200000000a20000000000005222222232263636460000b342525252
00000011111111a3828201b1b1b1b1b182938282930082820000000000000000b100a282721100000000b372828283b122222232132333610000869200000000
00100000000000b1000000000000000086938282828201920000a20182a37686526282829300000000000000000000005252845252328283920000b342845252
00008612222232828382829300000000828282828283829200000000000061001100a382737200000000b373a2829211525284628382a2000000a20000000000
00021111111111111111111111110061828282a28382820000000000828282825262829200000000000000000000000052525252526201a2000000b342525252
00000113235252225353536300000000828300a282828201939300001100000072828292b1039300000000b100a282125223526292000000000000a300000000
0043535353535353535353535363b2008282920082829200061600a3828382a28462000000000000000000000000000052845252526292000011111142525252
0000a28282132362b1b1b1b1000000009200000000a28282828293b372b2000073820100110382a3000000110082821362101333610000000000008293000000
0002828382828202828282828272b20083820000a282d3000717f38282920000526200000000000093000000000000005252525284620000b312223213528452
000000828392b30300000000002100000000000000000082828282b303b20000b1a282837203820193000072a38292b162710000000000009300008382000000
00b1a282820182b1a28283a28273b200828293000082122232122232820000a3233300000000000082920000000000002323232323330000b342525232135252
000000a28200b37300000000a37200000010000000111111118283b373b200a30000828273039200828300738283001162930000000000008200008282920000
0000009261a28200008261008282000001920000000213233342846282243434000000000000000082000085860000008382829200000000b342528452321323
0000100082000082000000a2820300002222321111125353630182829200008300009200b1030000a28200008282001262829200000000a38292008282000000
00858600008282a3828293008292610082001000001222222252525232253535000000f3100000a3820000a2010000008292000000009300b342525252522222
0400122232b200839321008683039300528452222262c000a28282820000a38210000000a3738000008293008292001362820000000000828300a38201000000
00a282828292a2828283828282000000343434344442528452525252622535350000001263000083829300008200c1008210d3e300a38200b342525252845252
1232425262b28682827282820103820052525252846200000082829200008282320000008382930000a28201820000b162839300000000828200828282930000
0000008382000000a28201820000000035353535454252525252528462253535000000032444008282820000829300002222223201828393b342525252525252
525252525262b2b1b1b1132323526200845223232323232352522323233382825252525252525252525284522333b2822323232323526282820000b342525252
52845252525252848452525262838242528452522333828292425223232352520000000000000000000000000000000000000000000000000000000000000000
525252845262b2000000b1b1b142620023338276000000824233b2a282018283525252845252232323235262b1b10083921000a382426283920000b342232323
2323232323232323232323526201821352522333b1b1018241133383828242840000000000000000000000000000000000000000000000000000000000000000
525252525262b20000000000a242627682828392000011a273b200a382729200525252525233b1b1b1b11333000000825353536382426282410000b30382a2a2
a1829200a2828382820182426200a2835262b1b10000831232b2000080014252000000000000a300000000000000000000000000000000000000000000000000
528452232333b20000001100824262928201a20000b3720092000000830300002323525262b200000000b3720000a382828283828242522232b200b373928000
000100110092a2829211a2133300a3825262b2000000a21333b20000868242520000000000000100009300000000000000000000000000000000000000000000
525262122232b200a37672b2a24262838292000000b30300000000a3820300002232132333b200000000b303829300a2838292019242845262b2000000000000
00a2b302b2a36182b302b200110000825262b200000000b1b10000a283a2425200000000a30082000083000000000000000000000094a4b4c4d4e4f400000000
525262428462b200a28303b2214262928300000000b3030000000000a203e3415252222232b200000000b30392000000829200000042525262b2000000000000
000000b100a2828200b100b302b211a25262b200000000000000000092b3428400000000827682000001009300000000000000000095a5b5c5d5e5f500000000
232333132362b221008203b2711333008293858693b3031111111111114222225252845262b200001100b303b2000000821111111142528462b2000000000000
000000000000110176851100b1b3026184621111111100000061000000b3135200000000828382670082768200000000000000000096a6b6c6d6e6f600000000
82000000a203117200a203b200010193828283824353235353535353535252845252525262b200b37200b303b2000000824353535323235262b2000011000000
0000000000b30282828372b26100b100525232122232b200000000000000b14200000000a28282123282839200000000000000000097a7b7c7d7e7f700000000
9200110000135362b2001353535353539200a2000001828282829200b34252522323232362b261b30300b3030000000092b1b1b1b1b1b34262b200b372b20000
001100000000b1a2828273b200000000232333132333b200001111000000b342000000868382125252328293a300000000000000000000000000000000000000
00b372b200a28303b2000000a28293b3000000000000a2828382827612525252b1b1b1b173b200b30393b30361000000000000000000b34262b271b303b20000
b302b211000000110092b100000000a3b1b1b1b1b1b10011111232110000b342000000a282125284525232828386000000000000000000000000000000000000
80b303b20000820311111111008283b311111111110000829200928242528452000000a3820000b30382b37300000000000000000000b3426211111103b20000
00b1b302b200b372b200000000000082b21000000000b31222522363b200b3138585868292425252525262018282860000000000000000000000000000000000
00b373b20000a21353535363008292b32222222232111102b20000a21323525200000001839200b3038282820000000011111111930011425222222233b20000
100000b10000b303b200000000858682b27100000000b3425233b1b1000000b182018283001323525284629200a2820000000000000000000000000000000000
9300b100000000b1b1b1b1b100a200b323232323235363b100000000b1b1135200000000820000b30382839200000000222222328283432323232333b2000000
329300000000b373b200000000a20182111111110000b31333b100a30061000000a28293f3123242522333020000820000000000000000000000000000000000
829200001000410000000000000000b39310d30000a28200000000000000824200000086827600b30300a282760000005252526200828200a30182a2006100a3
62820000000000b100000093a382838222222232b20000b1b1000083000000860000122222526213331222328293827600000000000000000000000000000000
017685a31222321111111111002100b322223293000182930000000080a301131000a383829200b373000083920000005284526200a282828283920000000082
62839321000000000000a3828282820152845262b261000093000082a300a3821000135252845222225252523201838200000000000000000000000000000000
828382824252522222222232007100b352526282a38283820000000000838282320001828200000083000082010000005252526271718283820000000000a382
628201729300000000a282828382828252528462b20000a38300a382018283821222324252525252525284525222223200000000000000000000000000000000"#;

struct Platform {}

impl Platform {
    fn init(this: &mut Object) {
        this.x -= 4.;
        this.is_solid = false;
        this.hitbox.w = 16;
        this.last = this.x;
    }

    fn update(
        self_: &mut Object,
        state: &State,
        objects: &[Object],
        room: Vec2<i32>,
    ) -> Option<(usize, Object)> {
        self_.spd.x = self_.dir as f32 * 0.65;
        if self_.x < -16. {
            self_.x = 128.;
        } else if self_.x > 128. {
            self_.x = -16.;
        }
        self_.last = self_.x;

        let ret = if !self_.check(objects, &ObjectType::Player, 0, 0) {
            let (index, hit) = self_.collide(objects, &ObjectType::Player, 0, -1)?;
            let mut hit = *hit;
            hit.move_x(
                state,
                objects,
                room,
                (self_.x - self_.last).floor() as i32,
                1,
            );

            Some((index, hit))
        } else {
            None
        };

        ret
    }

    fn draw(self_: &Object, draw: &mut DrawContext) {
        draw.spr(11, self_.x.floor() as i32, self_.y.floor() as i32 - 1);
        draw.spr(12, self_.x.floor() as i32 + 8, self_.y.floor() as i32 - 1)
    }
}
struct Smoke;

impl Smoke {
    fn init(this: &mut Object) {
        this.spr = 29.;
        this.spd.y = -0.1;
        this.spd.x = 0.3 + rnd(0.2);
        this.x += -1. + rnd(2.);
        this.y += -1. + rnd(2.);
        this.flip.x = maybe();
        this.flip.y = maybe();
        this.is_solid = false;
    }

    fn update(self_: &mut Object) -> bool {
        self_.spr += 0.2;

        // destroy if
        self_.spr >= 32.
    }
}

struct Fruit;

impl Fruit {
    fn init(object: &mut Object) {
        // object.start
    }
}

#[derive(Clone, Copy)]
enum ObjectKind {
    PlayerSpawn,
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
}

impl ObjectKind {
    // I think these are the "instantiable" objects
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

    fn init(&self, object: &mut Object) {
        match self {
            ObjectKind::PlayerSpawn => todo!(),
            ObjectKind::Spring => todo!(),
            ObjectKind::Balloon => todo!(),
            ObjectKind::FallFloor => todo!(),
            ObjectKind::Fruit => todo!(),
            ObjectKind::FlyFruit => todo!(),
            ObjectKind::FakeWall => todo!(),
            ObjectKind::Key => todo!(),
            ObjectKind::Chest => todo!(),
            ObjectKind::Message => todo!(),
            ObjectKind::BigChest => todo!(),
            ObjectKind::Flag => todo!(),
            ObjectKind::RoomTitle => todo!(),
            ObjectKind::Platform => todo!(),
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
