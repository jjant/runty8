pub mod currency;
pub mod enemy;
pub mod entity;
pub mod inventory;
pub mod item;
pub mod modifier;
pub mod player;
pub mod rect;

pub fn animate(base: usize, count: usize, every_num_frames: usize, t: usize) -> usize {
    base + (t / every_num_frames) % count
}

pub fn clamp(val: i32, a: i32, b: i32) -> i32 {
    a.max(b.min(val))
}
