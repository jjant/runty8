<p align="center">
  <img src="img/logo.png" alt="Runty8 Logo" />
</p>

# Runty8

A Pico8 clone in Rust.

## Making your own games

Head over to our [template](https://github.com/jjant/runty8-game-template) for an example to get started!

## Contributing

See [the contributing guide](./CONTRIBUTING.md) for instructions on how to get started.

## Examples

<p align="center">
  <a href="./examples/celeste/main.rs">
    <img src="img/celeste.gif" alt="Celeste playthrough" />
  </a>
</p>

- [Celeste](./examples/celeste/main.rs): A Rust port of Maddy Thorson and Noel Berry's [Celeste](https://www.lexaloffle.com/bbs/?tid=2145)
- [Confetti mouse demo](./examples/confetti/main.rs)
- [Moving box](./examples/moving-box/main.rs)

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

