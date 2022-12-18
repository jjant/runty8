// #![deny(missing_docs)]

//! Types and functions required to run a Runty8 game.

mod draw_data;
mod flags;
mod input;
mod map;
mod pico8;
pub mod serialize;
mod sprite_sheet;
mod state;
pub use draw_data::colors;

pub mod draw;
pub mod font;

pub use flags::Flags;
pub use input::Input;
pub use map::Map;
pub use pico8::*;
pub use sprite_sheet::{Sprite, SpriteSheet};

/// A regular pico8 app.
pub trait App {
    fn init(pico8: &mut Pico8) -> Self;
    fn update(&mut self, pico8: &mut Pico8);
    fn draw(&mut self, pico8: &mut Pico8);
}

/// A pico8 color.
///
/// Valid colors are in the range `0..=15`.
pub type Color = u8; // Actually a u4

/// Pico8's supported input buttons.
#[derive(Debug)]
pub enum Button {
    /// Left arrow.
    Left,
    /// Right arrow.
    Right,
    /// Up arrow.
    Up,
    /// Down arrow.
    Down,
    /// X key.
    Cross,
    /// C key.
    Circle,
    /// Left mouse button.
    Mouse,
}

/// Game assets: sprite sheet, map, flags.
// TODO: Rename to assets?
#[derive(Debug)]
pub struct Resources {
    pub assets_path: String,
    pub sprite_sheet: SpriteSheet,
    pub sprite_flags: Flags,
    pub map: Map,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Key state: up or down.
pub enum KeyState {
    Up,
    Down,
}
/// Keyboard keys.
#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub enum Key {
    ///
    A,
    ///
    B,
    ///
    C,
    ///
    D,
    ///
    E,
    ///
    F,
    ///
    G,
    ///
    H,
    ///
    I,
    ///
    J,
    ///
    K,
    ///
    L,
    ///
    M,
    ///
    N,
    ///
    O,
    ///
    P,
    ///
    Q,
    ///
    R,
    ///
    S,
    ///
    T,
    ///
    U,
    ///
    V,
    ///
    W,
    ///
    X,
    ///
    Y,
    ///
    Z,
    ///
    Control,
    ///
    LeftArrow,
    ///
    RightArrow,
    ///
    UpArrow,
    ///
    DownArrow,
    ///
    Escape,
    ///
    Alt,
    ///
    Space,
}

/// Keyboard event (key up/down).
#[derive(Clone, Copy, Debug)]
pub struct KeyboardEvent {
    /// Key that was pressed or released.
    pub key: Key,
    /// Whether the key was pressed or released.
    pub state: KeyState,
}

/// Input events (mouse/keyboard).
#[derive(Clone, Copy, Debug)]
pub enum InputEvent {
    /// Keyboard event
    Keyboard(KeyboardEvent),
    /// Mouse event
    Mouse(MouseEvent),
}

/// Mouse buttons.
#[derive(Clone, Copy, Debug)]
pub enum MouseButton {
    /// Left mouse button
    Left,
    /// Middle mouse button
    Middle,
    /// Right mouse button
    Right,
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
    /// Mouse button pressed/released.
    Button {
        /// Mouse button that was pressed or released.
        button: MouseButton,
        /// Whether the button was pressed or released.
        state: KeyState,
    },
}

/// Runty8 events (input, tick, etc).
#[derive(Clone, Copy, Debug)]
pub enum Event {
    ///
    Input(InputEvent),
    ///
    Tick {
        /// How much time passed since the last [`Event::Tick`], in milliseconds.
        delta_millis: f64,
    },
    // TODO: Remove this
    WindowClosed,
}

#[doc(hidden)]
#[macro_export]
macro_rules! include_dir_hack {
    (#[doc = $arg:tt]) => {{
        use $crate::include_dir;

        include_dir::include_dir!($arg)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! include_assets {
    ($arg:tt) => {
        $crate::paste! {
            $crate::include_dir_hack!(#[doc = "$CARGO_MANIFEST_DIR/" $arg ])
        }
    };
}

#[doc(hidden)]
pub use include_dir;
#[doc(hidden)]
pub use paste::paste;
/// Embed game assets in your binary

pub fn create_asset<T: Default>(
    deserialize: fn(&str) -> Result<T, String>,
    asset_name: &str,
    file_contents: Option<&str>,
) -> Result<T, String> {
    match file_contents {
        Some(file_contents) => deserialize(file_contents),
        None => {
            println!("Couldn't find file for asset: {asset_name}, creating a blank one.");
            Ok(T::default())
        }
    }
}

/// Web: Try to load path from local storage and log it.
/// Native: Returns `None`.
fn load(_file_path: &str) -> Option<String> {
    #[cfg(not(target_arch = "wasm32"))]
    return None;

    #[cfg(target_arch = "wasm32")]
    {
        let wasm_contents = wasm::load(_file_path);
        log::info!(
            "Loading assets from: {}... {}.",
            _file_path,
            wasm_contents
                .as_ref()
                .map(|_| "success")
                .unwrap_or("not found")
        );

        wasm_contents
    }
}

pub fn load_file(
    dir: &include_dir::Dir,
    assets_path: &str,
    file_name: &str,
) -> Result<Option<String>, String> {
    let file_path = format!("{assets_path}/{file_name}");

    let wasm_contents = load(&file_path);
    if let Some(wasm_contents) = wasm_contents {
        return Ok(Some(wasm_contents));
    }

    let asset_file = dir.get_file(file_name);
    match asset_file {
        Some(file) => {
            let contents = file
                .contents_utf8()
                .ok_or_else(|| "File contents were not in UTF8".to_owned())?;

            Ok(Some(contents.to_owned()))
        }
        None => Ok(None),
    }
}

/// Embed game assets in your binary (that is, loading them at compile time).
#[macro_export]
macro_rules! load_assets {
    ($path:tt) => {{
        use $crate::include_dir;
        static DIR: include_dir::Dir = $crate::include_assets!($path);

        (|| {
            #[cfg(target_arch = "wasm32")]
            $crate::wasm::setup_console_log_panic_hook();

            let assets_path = concat!(env!("CARGO_MANIFEST_DIR"), "/", $path).to_owned();

            let map_contents = $crate::load_file(&DIR, &assets_path, "map.txt")?;
            let sprite_flags_contents =
                $crate::load_file(&DIR, &assets_path, &$crate::Flags::file_name())?;
            let sprite_sheet_contents =
                $crate::load_file(&DIR, &assets_path, &$crate::SpriteSheet::file_name())?;

            let map =
                $crate::create_asset($crate::Map::deserialize, "map", map_contents.as_deref())?;

            let sprite_flags = $crate::create_asset(
                $crate::Flags::deserialize,
                "sprite flags",
                sprite_flags_contents.as_deref(),
            )?;

            let sprite_sheet = $crate::create_asset(
                $crate::SpriteSheet::deserialize,
                "sprite_sheet",
                sprite_sheet_contents.as_deref(),
            )?;

            Ok::<$crate::Resources, String>($crate::Resources {
                map,
                sprite_flags,
                sprite_sheet,
                assets_path,
            })
        })()
    }};
}

#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
pub mod wasm {
    #[doc(hidden)]
    pub fn load(file_name: &str) -> Option<String> {
        let storage = web_sys::window()
            .unwrap()
            .local_storage()
            .expect("Couldn't access local storage object")
            .expect("Couldn't access local storage object");

        // get_item returns `Result<Option<String>, JsValue>`, but it looks like
        // it can only return an `Err` (exception on JS side) in old browsers.
        // Not totally sure about it tho, MDN doens't mention it.
        storage.get_item(file_name).unwrap()
    }

    pub fn setup_console_log_panic_hook() {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Debug).unwrap();
    }
}
