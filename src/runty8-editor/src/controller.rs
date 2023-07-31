use std::fmt::Debug;

use crate::ui::DispatchEvent;
use crate::{
    app::{AppCompat, ElmApp},
    editor::{self, key_combo::KeyCombos, Editor},
    ui::Element,
    Resources,
};
use runty8_core::{DrawData, Event, InputEvent, Key, KeyboardEvent, MouseEvent, Pico8};

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
    pico8: Pico8,
    /// The editor and the game can modify the "draw state" (`draw_data`): camera, palette, etc.
    /// In order for these settings not to spill from the game to the editor, and viceversa,
    /// we keep an alternate [`DrawData`] that we swap, when the scene changes.
    alternate_draw_data: DrawData,
}

impl<T> Controller<T> {
    pub(crate) fn screen_buffer(&self) -> &[u8] {
        self.pico8.draw_data.buffer()
    }

    pub(crate) fn take_new_title(&mut self) -> Option<String> {
        self.pico8.take_new_title()
    }
}

impl<Game: AppCompat> Controller<Game> {
    pub fn init(scene: Scene, resources: Resources) -> Self {
        let mut pico8 = Pico8::new(resources);

        Self {
            scene,
            editor: <Editor as ElmApp>::init(),
            app: Game::init(&mut pico8),
            key_combos: KeyCombos::new()
                .push(KeyComboAction::RestartGame, Key::R, &[Key::Control])
                .push(KeyComboAction::SwitchScene, Key::Escape, &[]),
            pico8,
            alternate_draw_data: DrawData::new(),
        }
    }

    fn update(&mut self, msg: &Msg<Game::Msg>) {
        match msg {
            Msg::Editor(editor_msg) => {
                <Editor as ElmApp>::update(&mut self.editor, editor_msg, &mut self.pico8.resources);
            }
            Msg::App(msg) => {
                self.app.update(msg, &mut self.pico8);
            }

            &Msg::KeyboardEvent(event) => {
                self.handle_key_combos(event);
            }
            &Msg::MouseEvent(_) => {}
            &Msg::Tick => {}
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
            Event::Input(InputEvent::Mouse(mouse_event)) => Some(Msg::MouseEvent(*mouse_event)),
            Event::Input(InputEvent::Keyboard(keyboard_event)) => {
                Some(Msg::KeyboardEvent(*keyboard_event))
            }
            Event::Tick { .. } => Some(Msg::Tick),
            Event::WindowClosed => todo!("WindowClosed event not yet handled"),
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
    // TODO: Why doesn't this function simply return a [`Msg`]?
    fn handle_key_combos(&mut self, key_event: KeyboardEvent) {
        self.key_combos.on_event(key_event, |action| match action {
            KeyComboAction::RestartGame => {
                self.app = Game::init(&mut self.pico8);
                self.scene = Scene::App;
            }
            KeyComboAction::SwitchScene => {
                std::mem::swap(&mut self.pico8.draw_data, &mut self.alternate_draw_data);
                self.scene.flip()
            }
        });
    }

    /// Thing that actually calls update/orchestrates stuff
    pub(crate) fn step(&mut self, event: Event) {
        let mut view = view(
            &self.scene,
            &mut self.editor,
            &mut self.app,
            &mut self.pico8.resources,
        );

        let mut msg_queue = vec![];
        let dispatch_event = &mut DispatchEvent::new(&mut msg_queue);

        let cursor_position = (self.pico8.state.mouse_x, self.pico8.state.mouse_y);
        view.as_widget_mut()
            .on_event(event, cursor_position, dispatch_event);

        view.as_widget_mut().draw(&mut self.pico8);
        drop(view);

        for subscription_msg in self.subscriptions(&event) {
            msg_queue.push(subscription_msg);
        }

        for msg in msg_queue.into_iter() {
            self.update(&msg);
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
