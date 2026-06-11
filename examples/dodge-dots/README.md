# Dodge Dots

This runnable example proves simple movement, collision checks, score state, and reset behavior.

The active language direction is [`../../new-spec.md`](../../new-spec.md). This example may still use compatibility syntax until the compiler catches up.

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

`rx bake` generates:

```txt
examples/dodge-dots/.bakery/
  Cargo.toml
  src/lib.rs
  target/
```

Preview mode also generates:

```txt
examples/dodge-dots/.bakery/preview/
  Cargo.toml
  src/main.rs
```

The generated app crate depends on `crustini-core`. The native preview shell is separate.

## Inspect

```bash
cargo run --bin rx -- proof examples/dodge-dots/app.flour
cargo run --quiet --bin rx -- emit examples/dodge-dots/app.flour
cargo run --bin rx -- bake examples/dodge-dots/app.flour
env CRUSTINI_FRAME=1 cargo run --quiet --bin rx -- examples/dodge-dots/app.flour
```

More detail: [Compiling Examples](../compiling.md).
