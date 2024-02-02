#![deny(missing_docs)]

//! Compatibility layer for Runty8 crates that need to leverage winit.

use runty8_core::{Event, InputEvent, Key, KeyState, KeyboardEvent, MouseButton, MouseEvent};
use winit::dpi::{LogicalPosition, LogicalSize};

/// Information about the current viewport for rendering.
#[derive(Debug)]
pub struct ScreenInfo {
    /// DPI factor.
    pub scale_factor: f64,
    /// Display resolution in "logical" units.
    pub logical_size: LogicalSize<f64>,
}

impl ScreenInfo {
    ///
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            scale_factor: 1.0,
            logical_size: LogicalSize::new(width, height),
        }
    }
}

/// Extension trait to convert a [`winit::event::Event`] into a [`runty8_core::Event`].
pub trait Runty8EventExt: Sized {
    /// Convert a [`winit::event::Event`] into a [`runty8_core::Event`].
    fn from_winit(
        event: &winit::event::Event<()>,
        current_time: &mut f64,
        screen_info: &mut ScreenInfo,
    ) -> Option<Self>;
}

impl Runty8EventExt for Event {
    /// Translates a winit::event::Event into a runty8 Event.
    fn from_winit(
        event: &winit::event::Event<()>,
        current_time: &mut f64,
        screen_info: &mut ScreenInfo,
    ) -> Option<Event> {
        use winit::event::ElementState;

        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => Some(Event::WindowClosed),
                // TODO: Force aspect ratio on resize.
                &winit::event::WindowEvent::Resized(new_size) => {
                    screen_info.logical_size = new_size.to_logical(screen_info.scale_factor);

                    None
                }
                &winit::event::WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                    screen_info.scale_factor = scale_factor;
                    None
                }
                winit::event::WindowEvent::CursorMoved { position, .. } => {
                    let logical_mouse: LogicalPosition<f64> =
                        position.to_logical(screen_info.scale_factor);

                    let window_size = screen_info.logical_size;
                    Some(Event::Input(InputEvent::Mouse(MouseEvent::Move {
                        x: (logical_mouse.x / window_size.width * 128.).floor() as i32,
                        y: (logical_mouse.y / window_size.height * 128.).floor() as i32,
                    })))
                }
                winit::event::WindowEvent::MouseInput {
                    button: winit::event::MouseButton::Left,
                    state: input_state,
                    ..
                } => {
                    let mouse_button_state = match input_state {
                        ElementState::Pressed => KeyState::Down,
                        ElementState::Released => KeyState::Up,
                    };

                    let mouse_event = MouseEvent::Button {
                        button: MouseButton::Left,
                        state: mouse_button_state,
                    };
                    Some(Event::Input(InputEvent::Mouse(mouse_event)))
                }
                winit::event::WindowEvent::KeyboardInput { input, .. } => {
                    KeyboardEvent::from_winit(*input)
                        .map(InputEvent::Keyboard)
                        .map(Event::Input)
                }
                _ => None,
            },
            winit::event::Event::NewEvents(cause) => match cause {
                winit::event::StartCause::Init => Some(Event::Tick { delta_millis: 0.0 }),
                winit::event::StartCause::Poll => {
                    let new_time = instant::now();
                    let delta_millis = new_time - *current_time;
                    *current_time = new_time;

                    Some(Event::Tick { delta_millis })
                }
                winit::event::StartCause::ResumeTimeReached { .. } => None,
                winit::event::StartCause::WaitCancelled { .. } => None,
            },
            _ => None,
        }
    }
}
trait Runty8KeyboardEventExt: Sized {
    fn from_winit(input: winit::event::KeyboardInput) -> Option<Self>;
}

impl Runty8KeyboardEventExt for KeyboardEvent {
    fn from_winit(input: winit::event::KeyboardInput) -> Option<KeyboardEvent> {
        let key = input.virtual_keycode?;
        let runty8_key = Key::from_virtual_keycode(key)?;
        let state = KeyState::from_state(input.state);

        Some(KeyboardEvent {
            key: runty8_key,
            state,
        })
    }
}

trait Runty8KeyExt: Sized {
    fn from_virtual_keycode(key: winit::event::VirtualKeyCode) -> Option<Self>;
}

impl Runty8KeyExt for Key {
    fn from_virtual_keycode(key: winit::event::VirtualKeyCode) -> Option<Self> {
        use winit::event::VirtualKeyCode;

        match key {
            VirtualKeyCode::A => Some(Self::A),
            VirtualKeyCode::B => Some(Self::B),
            VirtualKeyCode::C => Some(Self::C),
            VirtualKeyCode::D => Some(Self::D),
            VirtualKeyCode::E => Some(Self::E),
            VirtualKeyCode::F => Some(Self::F),
            VirtualKeyCode::G => Some(Self::G),
            VirtualKeyCode::H => Some(Self::H),
            VirtualKeyCode::I => Some(Self::I),
            VirtualKeyCode::J => Some(Self::J),
            VirtualKeyCode::K => Some(Self::K),
            VirtualKeyCode::L => Some(Self::L),
            VirtualKeyCode::M => Some(Self::M),
            VirtualKeyCode::N => Some(Self::N),
            VirtualKeyCode::O => Some(Self::O),
            VirtualKeyCode::P => Some(Self::P),
            VirtualKeyCode::Q => Some(Self::Q),
            VirtualKeyCode::R => Some(Self::R),
            VirtualKeyCode::S => Some(Self::S),
            VirtualKeyCode::T => Some(Self::T),
            VirtualKeyCode::U => Some(Self::U),
            VirtualKeyCode::V => Some(Self::V),
            VirtualKeyCode::W => Some(Self::W),
            VirtualKeyCode::X => Some(Self::X),
            VirtualKeyCode::Y => Some(Self::Y),
            VirtualKeyCode::Z => Some(Self::Z),
            VirtualKeyCode::LControl => Some(Self::Control),
            VirtualKeyCode::Left => Some(Self::LeftArrow),
            VirtualKeyCode::Right => Some(Self::RightArrow),
            VirtualKeyCode::Up => Some(Self::UpArrow),
            VirtualKeyCode::Down => Some(Self::DownArrow),
            VirtualKeyCode::Escape => Some(Self::Escape),
            VirtualKeyCode::LAlt => Some(Self::Alt),
            VirtualKeyCode::Space => Some(Self::Space),
            VirtualKeyCode::Tab => Some(Self::Tab),
            _ => None,
        }
    }
}

trait Runty8KeyStateExt: Sized {
    fn from_state(state: winit::event::ElementState) -> Self;
}

impl Runty8KeyStateExt for KeyState {
    fn from_state(state: winit::event::ElementState) -> Self {
        use winit::event::ElementState;

        match state {
            ElementState::Pressed => Self::Down,
            ElementState::Released => Self::Up,
        }
    }
}
