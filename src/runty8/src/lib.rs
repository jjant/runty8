//! Entrypoints for all games using runty8.

#[doc(inline)]
pub use runty8_core::{load_assets, mid, rnd, sin, App, Button, Pico8};

use runty8_core::{Flags, Map, Resources};

#[cfg(not(target_arch = "wasm32"))]
#[doc(inline)]
pub use runty8_editor::run_app;

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

pub fn load_runtime_assets(assets_path: String) -> std::io::Result<Resources> {
    assets::create_directory(&assets_path)?;

    let map: Map = assets::create_map(&assets_path);
    let sprite_flags: Flags = assets::create_sprite_flags(&assets_path);
    let sprite_sheet = assets::create_sprite_sheet(&assets_path);

    let resources = Resources {
        assets_path,
        sprite_sheet,
        sprite_flags,
        map,
    };

    Ok(resources)
}

mod assets {
    use runty8_core::{Flags, Map, SpriteSheet};

    pub(crate) fn create_sprite_flags(assets_path: &str) -> Flags {
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

    pub(crate) fn create_map(assets_path: &str) -> Map {
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

    pub(crate) fn create_sprite_sheet(assets_path: &str) -> SpriteSheet {
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

    pub(crate) fn create_directory(path: &str) -> std::io::Result<()> {
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
}
