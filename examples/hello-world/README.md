# Hello World

This is the smallest project-shaped Crustini starter.

Unlike the single-file examples, this folder uses `recipe.flour` plus `src/main.flour`.

## Authored Files

```txt
examples/hello-world/
  recipe.flour
  src/main.flour
  README.md
```

`recipe.flour` describes the project:

- name
- version
- screen size
- fps
- native preview title
- generated output directory

`src/main.flour` is the original app source.

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

This writes the generated app crate to the directory chosen in `recipe.flour`:

```txt
examples/hello-world/.crustini/generated/
  Cargo.toml
  src/lib.rs
  target/debug/libcrustini_generated_hello_world.rlib
```

Preview mode also writes:

```txt
examples/hello-world/.crustini/generated/preview/
  Cargo.toml
  src/main.rs
```

## What Compiles To What

`recipe.flour` supplies:

```txt
screen size 180, 120
fps 30
preview title "Hello World"
generated ".crustini/generated"
```

`src/main.flour` supplies:

```txt
state!  -> Rust App fields
update! -> CrustiniApp::update
draw!   -> CrustiniApp::draw
```

The generated Rust app depends on `crustini-core`. The native preview depends on `crustini-host`.

More detail: [Compiling Examples](../compiling.md).
