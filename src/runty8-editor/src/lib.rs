#![allow(clippy::new_without_default)]
// #![deny(missing_docs)]
mod app;
pub mod ui;

pub use app::ElmApp;
pub use runty8_core::colors;
pub use runty8_core::App;
pub use runty8_core::Button;
pub use runty8_core::Color;
use runty8_core::Resources;
pub use runty8_core::{rnd, sin, KeyboardEvent, Pico8};

mod controller;
mod editor;
mod graphics;
mod run;
mod util;
use app::{AppCompat, ElmAppCompat, Pico8AppCompat};
use controller::Scene;
use runty8_core::{Map, Sprite};

/// Run a Pico8 application.
pub fn run_app<T: App + 'static>(resources: Resources) -> std::io::Result<()> {
    run_app_compat::<Pico8AppCompat<T>>(resources)
}

/// Run an Elm-style application.
pub fn run_elm_app<T: ElmApp + 'static>(resources: Resources) -> std::io::Result<()> {
    run_app_compat::<ElmAppCompat<T>>(resources)
}
// TODO: add example
fn run_app_compat<T: AppCompat + 'static>(resources: Resources) -> std::io::Result<()> {
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
