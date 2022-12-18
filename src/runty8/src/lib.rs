#![deny(missing_docs)]

//! Entrypoints for all games using runty8.

#[doc(inline)]
pub use runty8_core::{flr, load_assets, mid, rnd, sin, App, Button, Pico8};

use runty8_core::Resources;

#[doc(inline)]
pub use runty8_editor::run_app as run_editor;

#[doc(inline)]
pub use runty8_runtime::run;

/// Run your game in the Editor in `debug` mode, and in the standalone Runtime in `release`.
pub fn debug_run<Game: App + 'static>(resources: Resources) -> std::io::Result<()> {
    let run = {
        #[cfg(debug_assertions)]
        {
            println!("Running editor...");
            run_editor::<Game>
        }

        #[cfg(not(debug_assertions))]
        {
            println!("Running runtime...");
            runty8_runtime::<Game>
        }
    };

    run(resources)
}
