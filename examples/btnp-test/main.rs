use runty8::{self, load_assets, App, Button};

struct Game;

impl App for Game {
    fn init(_pico8: &mut runty8::Pico8) -> Self {
        Self
    }

    fn update(&mut self, pico8: &mut runty8::Pico8) {
        // let btn = pico8.btn(Button::Circle);
        // if btn {
        //     println!("C Button held");
        // }
        let btnp = pico8.btnp(Button::Circle);
        if btnp {
            println!("C Button pressed");
        }
    }

    fn draw(&mut self, _pico8: &mut runty8::Pico8) {}
}

#[derive(Debug)]
enum RuntimeOrEditor {
    Runtime,
    Editor,
}

impl RuntimeOrEditor {
    fn from_strs(strs: &[&str]) -> Self {
        for str in strs {
            if let Some(thing) = Self::from_str(str) {
                return thing;
            }
        }
        panic!("whoops");
    }

    fn from_str(str: &str) -> Option<Self> {
        match str {
            "--runtime" => Some(Self::Runtime),
            "--editor" => Some(Self::Editor),
            _ => None,
        }
    }
}
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let runtime_or_editor =
        RuntimeOrEditor::from_strs(&args.iter().map(|s| s.as_ref()).collect::<Vec<&str>>());

    println!("Running in {:?}", runtime_or_editor);

    let run_fn = match runtime_or_editor {
        RuntimeOrEditor::Runtime => runty8::run::<Game>,
        RuntimeOrEditor::Editor => runty8::run_editor::<Game>,
    };

    let resources = load_assets!("./").unwrap();
    run_fn(resources).unwrap();
}
