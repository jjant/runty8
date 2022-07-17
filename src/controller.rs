use std::fmt::Debug;

use crate::pico8::Pico8Impl;
use crate::runtime::draw_context::DrawData;
use crate::runtime::input::Keys;
use crate::ui::DispatchEvent;
use crate::{
    app::{AppCompat, ElmApp},
    editor::{self, key_combo::KeyCombos, Editor},
    runtime::state::InternalState,
    ui::Element,
    Event, Key, KeyboardEvent, MouseButton, MouseEvent, Resources,
};

#[derive(Debug, Clone, Copy)]
pub(crate) enum Msg<AppMsg> {
    Editor(editor::Msg),
    App(AppMsg),
    KeyboardEvent(KeyboardEvent),
    MouseEvent(MouseEvent),
    Tick,
}

#[derive(Copy, Clone, Debug)]
enum KeyComboAction {
    RestartGame,
    SwitchScene,
}

#[derive(Debug)]
pub(crate) struct Controller<Game> {
    scene: Scene,
    editor: Editor,
    app: Game,
    key_combos: KeyCombos<KeyComboAction>,
    keys: Keys,
    internal_state: InternalState,
    resources: Resources,
}

impl<Game: AppCompat> Controller<Game> {
    pub fn init(scene: Scene, mut resources: Resources, draw_data: &mut DrawData) -> Self {
        let internal_state = InternalState::new();
        let app = Game::init(&mut resources, &internal_state, draw_data);

        Self {
            scene,
            editor: <Editor as ElmApp>::init(),
            app,
            key_combos: KeyCombos::new()
                .push(KeyComboAction::RestartGame, Key::R, &[Key::Control])
                .push(KeyComboAction::SwitchScene, Key::Escape, &[]),
            keys: Keys::new(),
            internal_state,
            resources,
        }
    }

    fn update(&mut self, msg: &Msg<Game::Msg>, draw_data: &mut DrawData) {
        match msg {
            Msg::Editor(editor_msg) => {
                <Editor as ElmApp>::update(&mut self.editor, editor_msg, &mut self.resources);
            }
            Msg::App(msg) => {
                self.app
                    .update(msg, &mut self.resources, &self.internal_state, draw_data);
            }
            &Msg::MouseEvent(MouseEvent::Move { x, y }) => {
                self.internal_state.on_mouse_move(x, y);
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
                self.handle_key_combos(event, draw_data);
                self.keys.on_event(event);
            }
            &Msg::Tick => {
                self.internal_state.update_keys(&self.keys);
            }
        }
    }

    fn subscriptions(&self, event: &Event) -> Vec<Msg<Game::Msg>> {
        let sub_msgs: Vec<Msg<Game::Msg>> = match self.scene {
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
            Event::Mouse(mouse_event) => Some(Msg::MouseEvent(*mouse_event)),
            Event::Keyboard(keyboard_event) => Some(Msg::KeyboardEvent(*keyboard_event)),
            Event::Tick { .. } => Some(Msg::Tick),
        }
        .into_iter();

        sub_msgs.into_iter().chain(own_msgs).collect()
    }
}

fn view<'a, Game: AppCompat>(
    scene: &'a Scene,
    editor: &'a mut Editor,
    app: &'a mut Game,
    resources: &mut Resources,
) -> Element<'a, Msg<Game::Msg>> {
    match scene {
        Scene::Editor => <Editor as ElmApp>::view(editor, resources).map(Msg::Editor),
        Scene::App => app.view(resources).map(Msg::App),
    }
}

impl<Game: AppCompat> Controller<Game> {
    fn handle_key_combos(&mut self, key_event: KeyboardEvent, draw_data: &mut DrawData) {
        self.key_combos.on_event(key_event, |action| match action {
            KeyComboAction::RestartGame => {
                self.app = Game::init(&mut self.resources, &self.internal_state, &mut draw_data);
                self.scene = Scene::App;
            }
            KeyComboAction::SwitchScene => self.scene.flip(),
        });
    }

    /// Thing that actually calls update/orchestrates stuff
    pub(crate) fn step(&mut self, draw_data: &mut DrawData, event: Option<Event>) {
        let mut view = view(
            &self.scene,
            &mut self.editor,
            &mut self.app,
            &mut self.resources,
        );

        let mut msg_queue = vec![];
        let dispatch_event = &mut DispatchEvent::new(&mut msg_queue);

        let cursor_position = (self.internal_state.mouse_x, self.internal_state.mouse_y);
        if let Some(event) = event {
            view.as_widget_mut()
                .on_event(event, cursor_position, dispatch_event);
        }

        let pico8impl = Pico8Impl::new(&mut draw_data, &self.internal_state, &mut self.resources);
        view.as_widget_mut().draw(&mut pico8impl);
        drop(view);

        for subscription_msg in event.into_iter().flat_map(|e| self.subscriptions(&e)) {
            msg_queue.push(subscription_msg);
        }
        for msg in msg_queue.into_iter() {
            self.update(&msg, draw_data);
        }
    }
}

#[derive(Debug)]
pub enum Scene {
    Editor,
    App,
}

impl Scene {
    pub fn flip(&mut self) {
        *self = match self {
            Scene::Editor => Scene::App,
            Scene::App => Scene::Editor,
        }
    }
}
