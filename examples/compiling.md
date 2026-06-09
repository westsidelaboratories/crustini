# Compiling Examples

This page shows exactly what happens when an example moves from authored Crustini source into generated Rust and then into something runnable.

The short version:

```txt
.flour source
  -> crustini-lang
  -> generated no_std Rust app crate
  -> Cargo artifact
  -> optional native preview shell
  -> native window
```

The compiler and the preview host are intentionally separate. `crustini-lang` knows how to turn `.flour` into a small Rust app. `crustini-host` knows how to open a native window and feed input into that app.

## Layer 0: Authored Source

For a single-file app, the source is one file:

```txt
examples/brick-breaker-plus/
  app.flour
  README.md
```

For a project-shaped app, config and source are split:

```txt
examples/hello-world/
  recipe.flour
  src/main.flour
  README.md
```

The authored files are the only files users should edit.

## Layer 1: Language Compile

Run:

```bash
cargo run --bin rx -- emit examples/brick-breaker-plus/app.flour
```

That calls into `crustini-lang`.

The compiler reads:

```txt
examples/brick-breaker-plus/app.flour
```

Then it builds an in-memory app model:

```txt
App
  width
  height
  fps
  state fields
  setup statements
  update statements
  draw statements
```

Then it emits Rust shaped like this:

```rust
#![no_std]

use crustini_core::{Buttons, CrustiniApp, Screen};

pub const WIDTH: usize = 240;
pub const HEIGHT: usize = 180;
pub const FPS: usize = 45;

pub struct App {
    pub paddle_x: i16,
    pub ball_x: i16,
    pub ball_y: i16,
}

impl App {
    pub const fn new() -> Self {
        Self {
            paddle_x: 96,
            ball_x: 120,
            ball_y: 142,
        }
    }
}

impl CrustiniApp for App {
    fn update(&mut self, input: Buttons) {
        if input.left {
            self.paddle_x = self.paddle_x - 4;
        }
    }

    fn draw(&self, screen: &mut Screen<'_>) {
        screen.clear(5);
        screen.rect(self.paddle_x, 154, 44, 5, 245);
    }
}
```

That snippet is shortened for readability. The actual generated file contains every state field and every statement from the app.

## Source To Rust Mapping

Screen and frame rate:

```rust
screen!(240, 180)
fps!(45)
```

becomes:

```rust
pub const WIDTH: usize = 240;
pub const HEIGHT: usize = 180;
pub const FPS: usize = 45;
```

State:

```rust
state! {
  paddle_x: i16 = 96
  score: u32 = 0
}
```

becomes:

```rust
pub struct App {
    pub paddle_x: i16,
    pub score: u32,
}

impl App {
    pub const fn new() -> Self {
        Self {
            paddle_x: 96,
            score: 0,
        }
    }
}
```

Input:

```rust
if left {
  paddle_x = paddle_x - 4
}
```

becomes:

```rust
if input.left {
    self.paddle_x = self.paddle_x - 4;
}
```

Drawing:

```rust
rect!(paddle_x, 154, 44, 5, 245)
circle!(ball_x, ball_y, 3, 255)
```

becomes:

```rust
screen.rect(self.paddle_x, 154, 44, 5, 245);
screen.circle(self.ball_x, self.ball_y, 3, 255);
```

The generated app depends on `crustini-core`, not on the native window host.

## Layer 2: Generated App Crate

Run:

```bash
cargo run --bin rx -- bake examples/brick-breaker-plus/app.flour
```

For a single-file example, `rx bake` writes:

```txt
examples/brick-breaker-plus/.bakery/
  Cargo.toml
  src/lib.rs
  target/debug/libcrustini_generated_brick_breaker_plus.rlib
```

`src/lib.rs` is the generated no-std Rust app.

`Cargo.toml` is a tiny generated manifest:

```toml
[dependencies]
crustini-core = { path = "/absolute/path/to/crustini/crates/crustini-core" }
```

The build output is a Rust library artifact:

```txt
.bakery/target/debug/libcrustini_generated_brick_breaker_plus.rlib
```

That artifact is not the native preview window. It is the compiled app crate.

## Layer 3: Native Preview Shell

Run:

```bash
cargo run --bin rx -- examples/brick-breaker-plus/app.flour
```

The bare path means preview mode. `rx` still generates the app crate, then writes a second generated crate:

```txt
examples/brick-breaker-plus/.bakery/preview/
  Cargo.toml
  src/main.rs
```

The generated preview `main.rs` has this shape:

```rust
use crustini_core::{Buttons, CrustiniApp, Screen};
use crustini_host::{run_window, WindowConfig, WindowScale};
use crustini_generated_brick_breaker_plus::{App, FPS, HEIGHT, WIDTH};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = WindowConfig::new("Brick Breaker Plus", WIDTH, HEIGHT, FPS);
    config.scale = WindowScale::Auto;
    run_window::<App>(config)
}
```

That preview crate depends on:

```txt
generated app crate
crustini-core
crustini-host
```

`crustini-host` owns the native minifb window, keyboard mapping, mouse mapping, frame pacing, pause/reset controls, and framebuffer display.

## Headless Frame Check

Run:

```bash
env CRUSTINI_FRAME=1 cargo run --quiet --bin rx -- examples/brick-breaker-plus/app.flour
```

That uses the same generated preview crate, but instead of opening a window it runs one frame and writes:

```txt
examples/brick-breaker-plus/.bakery/frame.ppm
```

This is useful for tests and visual sanity checks.

## Project-Shaped Output

`hello-world` uses `recipe.flour`:

```txt
examples/hello-world/
  recipe.flour
  src/main.flour
```

The recipe controls screen size, fps, preview title, scale, and generated output directory.

Run:

```bash
cargo run --bin rx -- bake examples/hello-world
```

That writes:

```txt
examples/hello-world/.crustini/generated/
  Cargo.toml
  src/lib.rs
  target/debug/libcrustini_generated_hello_world.rlib
```

When previewing the project:

```bash
cargo run --bin rx -- examples/hello-world
```

`rx` also writes:

```txt
examples/hello-world/.crustini/generated/preview/
  Cargo.toml
  src/main.rs
```

## Commands To Inspect Everything

Proof without writing generated files:

```bash
cargo run --bin rx -- proof examples/brick-breaker-plus/app.flour
```

Print generated Rust:

```bash
cargo run --quiet --bin rx -- emit examples/brick-breaker-plus/app.flour
```

Bake the generated app artifact:

```bash
cargo run --bin rx -- bake examples/brick-breaker-plus/app.flour
```

Open the native preview:

```bash
cargo run --bin rx -- examples/brick-breaker-plus/app.flour
```

Run the repo-level validation:

```bash
bun run check
```

## What Is Not Happening

The language compiler does not:

- open windows
- know about minifb
- know about desktop input APIs
- know about ESP32, web, or any board package

Those are host concerns.

The `.flour` app compiles to a generic `CrustiniApp`. Hosts decide how to display pixels and where input comes from.
