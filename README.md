# Runty8

A Pico8 clone in rust

## Missing features

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
