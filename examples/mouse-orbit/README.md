# Mouse Orbit

This example proves the basic mouse input path.

The app is written fully in Crustini source. No Rust is hand-written for this example.

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

`app.flour` contains:

```txt
screen!(180, 120)
fps!(30)
state! { x, y, pulse, shade, clicked }
update! { mouse_x, mouse_y, mouse_down }
draw! { clear, text, line, circle }
```

`rx bake` generates:

```txt
examples/mouse-orbit/.bakery/
  Cargo.toml
  src/lib.rs
  target/debug/libcrustini_generated_mouse_orbit.rlib
```

Preview mode generates:

```txt
examples/mouse-orbit/.bakery/preview/
  Cargo.toml
  src/main.rs
```

The compiler lowers `mouse_x`, `mouse_y`, and `mouse_down` into fields on `crustini_core::Buttons`.

## Inspect

```bash
cargo run --bin rx -- proof examples/mouse-orbit/app.flour
cargo run --quiet --bin rx -- emit examples/mouse-orbit/app.flour
cargo run --bin rx -- bake examples/mouse-orbit/app.flour
env CRUSTINI_FRAME=1 cargo run --quiet --bin rx -- examples/mouse-orbit/app.flour
```

More detail: [Compiling Examples](../compiling.md).
