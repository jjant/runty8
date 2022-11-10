#![allow(clippy::new_without_default)]
// #![deny(missing_docs)]
mod app;
mod pico8;
mod runtime;
pub mod ui;

pub use app::ElmApp;
pub use pico8::{rnd, sin, Pico8};
pub use runtime::draw_data::colors;
pub use runtime::sprite_sheet::Color;
pub use runtime::state::Button;
pub use runty8_runtime::App;

mod controller;
mod draw;
mod editor;
mod font;
mod graphics;
mod run;
mod util;
use app::{AppCompat, ElmAppCompat, Pico8AppCompat};
use controller::Scene;
use glium::glutin::event::VirtualKeyCode;
use runtime::{
    flags::Flags,
    map::Map,
    sprite_sheet::{Sprite, SpriteSheet},
};
use std::fmt::Debug;

/// Mouse buttons.
#[derive(Clone, Copy, Debug)]
pub enum MouseButton {
    // TODO: Handle other mouse buttons?
    Left,
    Right,
    Middle,
}

/// Mouse events (mouse move, button presses).
#[derive(Clone, Copy, Debug)]
pub enum MouseEvent {
    /// Mouse move event.
    // Contains the current position of the mouse.
    Move {
        ///
        x: i32,
        ///
        y: i32,
    },
    // TODO: Refactor these two below to factor out the MouseButton
    /// Mouse button pressed.
    Down(MouseButton),
    /// Mouse button released.
    Up(MouseButton),
}

/// Keyboard events (key up, key down).
#[derive(Clone, Copy, Debug)]
pub enum Event {
    Mouse(MouseEvent),
    Keyboard(KeyboardEvent),
    Tick { delta_millis: f64 },
}

fn create_sprite_flags(assets_path: &str) -> Flags {
    let path = format!(
        "{}{}{}",
        assets_path,
        std::path::MAIN_SEPARATOR,
        Flags::file_name()
    );

    if let Ok(content) = std::fs::read_to_string(&path) {
        Flags::deserialize(&content).unwrap()
    } else {
        println!("Couldn't read flags from {}, creating new flags.", path);
        Flags::new()
    }
}

fn create_map(assets_path: &str) -> Map {
    let path = format!(
        "{}{}{}",
        assets_path,
        std::path::MAIN_SEPARATOR,
        Map::file_name()
    );

    if let Ok(content) = std::fs::read_to_string(&path) {
        Map::deserialize(&content).unwrap()
    } else {
        println!("Couldn't read map from {}, creating new map.", path);
        Map::new()
    }
}

fn create_sprite_sheet(assets_path: &str) -> SpriteSheet {
    let path = format!(
        "{}{}{}",
        assets_path,
        std::path::MAIN_SEPARATOR,
        SpriteSheet::file_name()
    );

    if let Ok(content) = std::fs::read_to_string(&path) {
        SpriteSheet::deserialize(&content).unwrap()
    } else {
        println!(
            "Couldn't read sprite sheet from {}, creating new sprite sheet.",
            path
        );
        SpriteSheet::new()
    }
}

fn create_directory(path: &str) -> std::io::Result<()> {
    if let Err(e) = std::fs::create_dir(path) {
        match e.kind() {
            std::io::ErrorKind::AlreadyExists => {
                // This directory already existing is not really an error.
                Ok(())
            }
            _ => {
                eprintln!("Couldn't create assets directory: `{path}`.");

                Err(e)
            }
        }
    } else {
        Ok(())
    }
}

/// Run a Pico8 application.
pub fn run_app<T: App + 'static>(assets_path: String) -> std::io::Result<()> {
    run_app_compat::<Pico8AppCompat<T>>(assets_path)
}

/// Run an Elm-style application.
pub fn run_elm_app<T: ElmApp + 'static>(assets_path: String) -> std::io::Result<()> {
    run_app_compat::<ElmAppCompat<T>>(assets_path)
}
// TODO: add example
fn run_app_compat<T: AppCompat + 'static>(assets_path: String) -> std::io::Result<()> {
    create_directory(&assets_path)?;

    let map: Map = create_map(&assets_path);
    let sprite_flags: Flags = create_sprite_flags(&assets_path);
    let sprite_sheet = create_sprite_sheet(&assets_path);

    let resources = Resources {
        assets_path,
        sprite_sheet,
        sprite_flags,
        map,
    };

    let starting_scene = start_scene();
    crate::run::run_app::<T>(starting_scene, resources);

    Ok(())
}

fn start_scene() -> Scene {
    if std::env::args().any(|arg| arg == "--game") {
        Scene::App
    } else {
        Scene::Editor
    }
}
