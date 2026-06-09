# Brick Breaker

This is the smallest playable Brick Breaker example for the current easy-mode Crustini path.

## Authored Files

```txt
examples/brick-breaker/
  app.flour
  README.md
```

`app.flour` is the original source. It is the only code file you edit.

## Run

```bash
cargo run --bin rx -- examples/brick-breaker/app.flour
```

Once `rx` is installed:

```bash
rx examples/brick-breaker/app.flour
```

That opens the native preview window.

## Bake

```bash
cargo run --bin rx -- bake examples/brick-breaker/app.flour
```

This writes:

```txt
examples/brick-breaker/.bakery/
  Cargo.toml
  src/lib.rs
  target/debug/libcrustini_generated_brick_breaker.rlib
```

The generated `src/lib.rs` contains the Rust `App` implementing `crustini_core::CrustiniApp`.

## Preview Layer

Preview mode also writes:

```txt
examples/brick-breaker/.bakery/preview/
  Cargo.toml
  src/main.rs
```

The preview crate depends on the generated app crate and `crustini-host`. That is where the native window lives.

## Inspect

```bash
cargo run --bin rx -- proof examples/brick-breaker/app.flour
cargo run --quiet --bin rx -- emit examples/brick-breaker/app.flour
env CRUSTINI_FRAME=1 cargo run --quiet --bin rx -- examples/brick-breaker/app.flour
```

More detail: [Compiling Examples](../compiling.md).
