# Welcome to Runty8 contributing guide

Thank you for investing your time in helping out with Runty8's development.
The project is in very early stages, so there's a lot to do! That also means that these docs can be fairly incomplete.

If you notice you're missing any particular piece of information here, please [open an issue](https://github.com/jjant/runty8/issues/new) or pull request.

## Project structure

Largely, the project is divided in the following way:

```bash
src
├── bin // Example games (Celeste, etc) used for testing
├── editor // Source code for the actual "editor" (sprite, level, sound, etc)
├── runtime // Data structures required for the runtime of the game (SpriteSheet, Map, Sprite flags, Sound, etc)
├── ui // Internal UI framework used to implement the editor
└── pico8.rs // Actual implementation of the Pico8 API
```

## Useful commands

Check all examples compile:
```bash
cargo check --all-targets
```

Generate docs:
```bash
cargo doc --open
```
