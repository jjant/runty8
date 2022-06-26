use crate::{
    app::{self, App, AppCompat, ElmApp},
    editor::{
        self,
        key_combo::{KeyCombo, KeyCombos},
        Editor,
    },
    runtime::{
        draw_context::DrawData,
        state::{InternalState, Scene, State},
    },
    ui::{DrawFn, Element},
    Event, Key, KeyState, KeyboardEvent, Resources,
};

#[derive(Debug)]
struct Controller<Game> {
    scene: Scene,
    editor: Editor,
    app: Game,
    key_combos: KeyCombos<KeyComboAction>,
}

#[derive(Debug, Clone, Copy)]
enum Msg {
    Editor(editor::Msg),
    App(app::Pico8AppMsg),
    KeyboardEvent(KeyboardEvent),
}

#[derive(Copy, Clone, Debug)]
enum KeyComboAction {
    RestartGame,
}

impl<Game: App> AppCompat for Controller<Game> {
    type Msg = Msg;

    fn init(state: &State) -> Self {
        Self {
            scene: Scene::Editor,
            editor: <Editor as ElmApp>::init(),
            app: App::init(state),
            key_combos: KeyCombos::new().push(KeyCombo::new(
                KeyComboAction::RestartGame,
                Key::R,
                &[Key::Control],
            )),
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
                let out_msg = <Editor as ElmApp>::update(&mut self.editor, editor_msg, resources);
            }
            Msg::App(app_msg) => {
                let mut state = State::new(internal_state, resources);
                let out_msg = self.app.update(&state);
            }
            &Msg::KeyboardEvent(event) => {
                self.handle_key_combos(event, &State::new(internal_state, resources));
            }
        }
    }

    fn view(
        &mut self,
        resources: &mut Resources,
        internal_state: &mut InternalState,
        draw_data: &mut DrawData,
    ) -> Element<'_, Self::Msg> {
        match self.scene {
            Scene::Editor => {
                <Editor as ElmApp>::view(&mut self.editor, &resources).map(Msg::Editor)
            }
            Scene::App => DrawFn::new(|draw| {
                self.app.draw(draw);
            })
            .into(),
        }
    }

    fn subscriptions(&self, event: &Event) -> Option<Self::Msg> {
        match event {
            Event::Mouse(_) => todo!(),
            Event::Keyboard(event) => Some(Msg::KeyboardEvent(*event)),
            Event::Tick { .. } => todo!(),
        }
    }
}

impl<Game: App> Controller<Game> {
    fn handle_key_combos(&mut self, key_event: KeyboardEvent, state: &State) {
        self.key_combos.on_event(key_event, |action| match action {
            KeyComboAction::RestartGame => {
                self.app = App::init(state);
                self.scene = Scene::App;
            }
        });
    }
}
