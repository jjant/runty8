<p align="center">
  <img src="img/logo.png" alt="Runty8 Logo" />
</p>

# Runty8

A Pico8 clone in Rust.

## Contributing

See [the contributing guide](./CONTRIBUTING.md) for instructions on how to get started.

## Examples

<p align="center">
  <a href="./src/bin/celeste.rs">
    <img src="img/celeste.gif" alt="Celeste playthrough" />
  </a>
</p>

- [Celeste](./src/bin/celeste.rs): A Rust port of Maddy Thorson and Noel Berry's [Celeste](https://www.lexaloffle.com/bbs/?tid=2145)
- [Confetti mouse demo](./src/bin/confetti.rs)
- [Moving box](./src/bin/moving_box.rs)

## Running

Run editor with a default "empty" game:

```bash
cargo run
```

Run examples (`celeste`, `moving_box`, `confetti`) with:

```bash
cargo run --example example_name -- --game
```

Or run `cargo run --example` to get a list of the available examples.

Press escape to switch between the game and the editor.

## Things to do

- [ ] Unify data structures (sprite sheet/map/etc) under a single Memory byte array?
      Not sure this is good (tho it may simplify having the map and sprite sheet overlapping in memory, otherwise that has to be programmed manually).
- [ ] Modify editor W, A, S, D key short cuts (shift sprite) to use arrow keys
- [ ] Implement sprite editor tools: line, circle, selection tool, zoom, etc
- [ ] Finish porting the pico8 API (missing functions like `peek`, `poke`, `circ`, etc)
- [ ] Building/packaging your game as a single file.
      Currently the library stores your assets (sprite sheet, map, sprite flags (and sound in the future))
      in separate files, and the application loads them at runtime.
      It'd be cool to have a way to bundle all the code and assets together in a single executable file for ease of distribution.
      This should also facilitate using wasm.
- [ ] Wasm support
- [ ] Add a concept of "active widget" in the sprite editor.
      If you're click-dragging the mouse in the color picker, moving the mouse away will trigger interactions in other components, this is wrong
- [ ] Some rudimentary console-like thing like in Pico8 (to run graphic commands, etc)

