#![allow(clippy::new_without_default, clippy::mem_replace_with_default)]
// #![deny(missing_docs)]

//! The Runty8 editor.

mod app;

/// The editor's own UI system.
pub mod ui;

pub use app::ElmApp;
use runty8_core::{App, Resources};

mod controller;
mod editor;
mod pico8;
mod util;

use app::{AppCompat, ElmAppCompat, Pico8AppCompat};
use controller::Scene;

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
    implementation::run_app::<T>(starting_scene, resources);

    Ok(())
}

fn start_scene() -> Scene {
    if std::env::args().any(|arg| arg == "--game") {
        Scene::App
    } else {
        Scene::Editor
    }
}

mod implementation {
    use crate::app::AppCompat;
    use crate::controller::{Controller, Scene};
    use crate::Resources;
    use runty8_core::Event;
    use runty8_winit::ScreenInfo;

    pub(super) fn run_app<Game: AppCompat + 'static>(scene: Scene, resources: Resources) {
        let mut screen_info = ScreenInfo::new(640.0, 640.0);
        screen_info.scale_factor = 1.0;

        let mut controller = Controller::<Game>::init(scene, resources);

        runty8_event_loop::event_loop(move |event, control_flow, draw, set_title| {
            controller.step(event);

            if let Some(new_title) = controller.take_new_title() {
                set_title(&new_title);
            }

            if let Event::Tick { .. } = event {
                draw(controller.screen_buffer(), control_flow);
            }
        });
    }
}
