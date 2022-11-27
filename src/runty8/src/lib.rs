//! Entrypoints for all games using runty8.

#[doc(inline)]
pub use runty8_core::{mid, rnd, sin, App, Button, Pico8};
#[doc(inline)]
pub use runty8_editor::run_app;
#[doc(inline)]
pub use runty8_runtime::run;

/// Run your game in the Editor in `debug` mode, and in the standalone Runtime in `release`.
pub fn debug_run<Game: App + 'static>(assets_path: String) -> std::io::Result<()> {
    let run = {
        if cfg!(debug_assertions) {
            println!("Running editor...");
            runty8_editor::run_app::<Game>
        } else {
            println!("Running runtime...");
            runty8_runtime::run::<Game>
        }
    };

    run(assets_path)
}
