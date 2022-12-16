#![deny(missing_docs)]

//! Entrypoints for all games using runty8.

#[doc(inline)]
pub use runty8_core::{flr, load_assets, mid, rnd, sin, App, Button, Pico8};

use runty8_core::Resources;

#[cfg(not(target_arch = "wasm32"))]
#[doc(inline)]
pub use runty8_editor::run_app as run_editor;

#[doc(inline)]
pub use runty8_runtime::run;

/// Run your game in the Editor in `debug` mode, and in the standalone Runtime in `release`.
/// The editor is not currently supported in `wasm`, so in that target only the runtime will be
/// used.
pub fn debug_run<Game: App + 'static>(resources: Resources) -> std::io::Result<()> {
    let run = {
        #[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
        {
            println!("Running editor...");
            runty8_editor::run_app::<Game>
        }

        #[cfg(not(all(debug_assertions, not(target_arch = "wasm32"))))]
        {
            println!("Running runtime...");
            runty8_runtime::run::<Game>
        }
    };

    run(resources)
}
