# Runty8

A Pico8 clone in rust

## Things to do

- [ ] Support Ctrl+r/Cmd+r to restart game
- [ ] Modify editor W, A, S, D key short cuts (shift sprite) to use arrow keys
- [ ] Implement sprite editor tools: line, circle, selection tool, zoom, etc
- [ ] Editor currently gets its assets (ui icons, etc) like a regular pico8 game, which means it renders the wrong UI when running a proper game
- [ ] Finish porting the pico8 API (missing functions like `peek`, `poke`, `circ`, etc)
- [ ] Sound effects (both playing sounds in games, and the whole sound editor thing)
- [ ] Building/packaging your game as a single file.
      Currently the library stores your assets (sprite sheet, map, sprite flags (and sound in the future))
      in separate files, and the application loads them at runtime.
      It'd be cool to have a way to bundle all the code and assets together in a single executable file for ease of distribution.
      This should also facilitate using wasm.
- [ ] Wasm support

## Running

Plain editor

```bash
cargo run
```

Celeste

```bash
cargo run --bin celeste
```

## License
