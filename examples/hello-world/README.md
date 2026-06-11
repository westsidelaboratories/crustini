# Hello World

This is the smallest current project-shaped starter.

The source follows the active language direction in [`../../new-spec.md`](../../new-spec.md): TOML front matter plus `app! Main` with normal `fn update()` and `fn draw()` blocks.

## Authored Files

```txt
examples/hello-world/
  recipe.flour
  src/main.flour
  README.md
```

`recipe.flour` is compatibility project config for the current CLI project runner. `src/main.flour` is the authored app source.

## Run

From the repo root:

```bash
cargo run --bin rx -- examples/hello-world
```

Once `rx` is installed:

```bash
rx examples/hello-world
```

Inside this directory, a bare `rx` opens the native preview:

```bash
rx
```

## Bake

```bash
cargo run --bin rx -- bake examples/hello-world
```

Inside this directory:

```bash
rx bake .
```

This writes the generated app crate to the directory chosen by compatibility config:

```txt
examples/hello-world/.crustini/generated/
  Cargo.toml
  src/lib.rs
  target/
```

Preview mode also writes:

```txt
examples/hello-world/.crustini/generated/preview/
  Cargo.toml
  src/main.rs
```

The generated Rust app depends on `crustini-core`. The native preview depends on `crustini-host`.

More detail: [Compiling Examples](../compiling.md).
