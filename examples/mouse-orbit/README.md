# Mouse Orbit

This runnable example proves the basic mouse input path.

The active language direction is [`../../new-spec.md`](../../new-spec.md). This example may still use compatibility syntax until the compiler catches up.

## Authored Files

```txt
examples/mouse-orbit/
  app.flour
  README.md
```

## Run

```bash
cargo run --bin rx -- examples/mouse-orbit/app.flour
```

Once `rx` is installed:

```bash
rx examples/mouse-orbit/app.flour
```

Move the mouse around the window and click to change the animation.

## Compile Path

`rx bake` generates:

```txt
examples/mouse-orbit/.bakery/
  Cargo.toml
  src/lib.rs
  target/
```

Preview mode also generates:

```txt
examples/mouse-orbit/.bakery/preview/
  Cargo.toml
  src/main.rs
```

The compiler lowers mouse input into generated Rust that reads `crustini_core::Buttons`.

## Inspect

```bash
cargo run --bin rx -- proof examples/mouse-orbit/app.flour
cargo run --quiet --bin rx -- emit examples/mouse-orbit/app.flour
cargo run --bin rx -- bake examples/mouse-orbit/app.flour
env CRUSTINI_FRAME=1 cargo run --quiet --bin rx -- examples/mouse-orbit/app.flour
```

More detail: [Compiling Examples](../compiling.md).
