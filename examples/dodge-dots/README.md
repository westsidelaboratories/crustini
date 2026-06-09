# Dodge Dots

This example proves simple movement, collision checks, score state, and reset behavior.

It is written fully in `.flour`.

## Authored Files

```txt
examples/dodge-dots/
  app.flour
  README.md
```

## Run

```bash
cargo run --bin rx -- examples/dodge-dots/app.flour
```

Once `rx` is installed:

```bash
rx examples/dodge-dots/app.flour
```

Use arrows or click/hold the mouse to move.

## Compile Path

`app.flour` contains:

```txt
screen!(180, 120)
fps!(30)
state! { player, dots, tick, score, hit }
update! { input, movement, bounds, collisions, reset }
draw! { clear, rect, text, line, circle }
```

`rx bake` generates:

```txt
examples/dodge-dots/.bakery/
  Cargo.toml
  src/lib.rs
  target/debug/libcrustini_generated_dodge_dots.rlib
```

Preview mode generates:

```txt
examples/dodge-dots/.bakery/preview/
  Cargo.toml
  src/main.rs
```

The generated app crate is no-std and only depends on `crustini-core`. The native preview shell is separate.

## Inspect

```bash
cargo run --bin rx -- proof examples/dodge-dots/app.flour
cargo run --quiet --bin rx -- emit examples/dodge-dots/app.flour
cargo run --bin rx -- bake examples/dodge-dots/app.flour
env CRUSTINI_FRAME=1 cargo run --quiet --bin rx -- examples/dodge-dots/app.flour
```

More detail: [Compiling Examples](../compiling.md).
