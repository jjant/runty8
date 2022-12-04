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
    X,
    /// C key.
    C,
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
    (#[doc = $arg:tt]) => {
        include_dir::include_dir!($arg);
    };
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

pub use paste::paste;
/// Embed game assets in your binary

pub fn create_asset<T: Default>(
    deserialize: fn(&str) -> Result<T, String>,
    asset_name: &str,
    file: Option<&include_dir::File>,
) -> Result<T, String> {
    match file {
        Some(file) => {
            let contents = file
                .contents_utf8()
                .ok_or_else(|| "File contents were not utf8".to_owned())?;
            deserialize(contents)
        }
        None => {
            println!(
                "Couldn't find file for asset: {asset_name}, creating a blank one."
            );
            Ok(T::default())
        }
    }
}

/// Embed game assets in your binary (that is, loading them at compile time).
#[macro_export]
macro_rules! load_assets {
    ($path:tt) => {{
        static DIR: include_dir::Dir = $crate::include_assets!($path);

        (|| {
            let assets_path = concat!(env!("CARGO_MANIFEST_DIR"), "/", $path).to_owned();
            println!("Loading assets from: {}", assets_path);

            let map =
                $crate::create_asset($crate::Map::deserialize, "map", DIR.get_file("map.txt"))?;
            let sprite_flags = $crate::create_asset(
                $crate::Flags::deserialize,
                "sprite flags",
                DIR.get_file("sprite_flags.txt"),
            )?;
            let sprite_sheet = $crate::create_asset(
                $crate::SpriteSheet::deserialize,
                "sprite_sheet",
                DIR.get_file("sprite_sheet.txt"),
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
