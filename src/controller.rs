use crate::runtime::input::Keys;
use crate::{
    app::{self, App, AppCompat, ElmApp},
    editor::{self, key_combo::KeyCombos, Editor},
    runtime::{
        draw_context::DrawData,
        state::{InternalState, State},
    },
    ui::{DrawFn, Element},
    Event, Key, KeyboardEvent, MouseButton, MouseEvent, Resources,
};

#[derive(Debug)]
pub(crate) struct Controller<Game> {
    scene: Scene,
    editor: Editor,
    app: Game,
    key_combos: KeyCombos<KeyComboAction>,
    keys: Keys,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Msg {
    Editor(editor::Msg),
    App(app::Pico8AppMsg),
    KeyboardEvent(KeyboardEvent),
    MouseEvent(MouseEvent),
    Tick,
}

#[derive(Copy, Clone, Debug)]
enum KeyComboAction {
    RestartGame,
    SwitchScene,
}

impl<Game: App> AppCompat for Controller<Game> {
    type Msg = Msg;

    fn init(state: &State) -> Self {
        Self {
            scene: Scene::Editor,
            editor: <Editor as ElmApp>::init(),
            app: App::init(state),
            key_combos: KeyCombos::new()
                .push(KeyComboAction::RestartGame, Key::R, &[Key::Control])
                .push(KeyComboAction::SwitchScene, Key::Escape, &[]),
            keys: Keys::new(),
        }
    }

    fn update(
        &mut self,
        msg: &Self::Msg,
        resources: &mut Resources,
        internal_state: &InternalState,
    ) {
        match msg {
            Msg::Editor(editor_msg) => {
                <Editor as ElmApp>::update(&mut self.editor, editor_msg, resources);
            }
            Msg::App(_) => {
                let state = State::new(internal_state, resources);
                self.app.update(&state);
            }
            &Msg::MouseEvent(MouseEvent::Move { x, y }) => {
                // internal_state.on_mouse_move(x, y);
            }
            &Msg::MouseEvent(event) => {
                let left_pressed = match event {
                    MouseEvent::Down(MouseButton::Left) => Some(true),
                    MouseEvent::Up(MouseButton::Left) => Some(false),
                    _ => None,
                };

                if let Some(left_pressed) = left_pressed {
                    self.keys.mouse = Some(left_pressed);
                }
            }

            &Msg::KeyboardEvent(event) => {
                self.handle_key_combos(event, &State::new(internal_state, resources));
                self.keys.on_event(event);
            }
            &Msg::Tick => {
                // internal_state.update_keys(&self.keys);
            }
        }
    }

    fn view(
        &mut self,
        resources: &mut Resources,
        _: &mut InternalState,
        _: &mut DrawData,
    ) -> Element<'_, Self::Msg> {
        match self.scene {
            Scene::Editor => <Editor as ElmApp>::view(&mut self.editor, resources).map(Msg::Editor),
            Scene::App => DrawFn::new(|draw| {
                self.app.draw(draw);
            })
            .into(),
        }
    }

    fn subscriptions(&self, event: &Event) -> Vec<Self::Msg> {
        let sub_msgs: Vec<Self::Msg> = match self.scene {
            Scene::Editor => <Editor as ElmApp>::subscriptions(&self.editor, event)
                .into_iter()
                .map(Msg::Editor)
                .collect(),

            Scene::App => <Game as AppCompat>::subscriptions(&self.app, event)
                .into_iter()
                .map(Msg::App)
                .collect(),
        };

        let own_msgs = match event {
            Event::Mouse(_) => None,
            Event::Keyboard(event) => Some(Msg::KeyboardEvent(*event)),
            Event::Tick { .. } => Some(Msg::Tick),
        }
        .into_iter();

        sub_msgs.into_iter().chain(own_msgs).collect()
    }
}

impl<Game: App> Controller<Game> {
    fn handle_key_combos(&mut self, key_event: KeyboardEvent, state: &State) {
        self.key_combos.on_event(key_event, |action| match action {
            KeyComboAction::RestartGame => {
                self.app = App::init(state);
                self.scene = Scene::App;
            }
            KeyComboAction::SwitchScene => self.scene.flip(),
        });
    }
}

#[derive(Debug)]
pub enum Scene {
    Editor,
    App,
}

impl Scene {
    fn initial() -> Self {
        Scene::Editor
    }

    pub fn flip(&mut self) {
        *self = match self {
            Scene::Editor => Scene::App,
            Scene::App => Scene::Editor,
        }
    }
}
