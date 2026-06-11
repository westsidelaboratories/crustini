# Compiling Examples

This page shows the current implementation path from authored Crustini source into generated Rust and then into something runnable.

The active language direction is [`../new-spec.md`](../new-spec.md). Existing examples may still use compatibility syntax until the compiler catches up.

## Short Version

```txt
.flour source
  -> crustini-lang
  -> generated Rust app crate
  -> Cargo artifact
  -> optional native preview shell
  -> native window
```

The compiler and the preview host are intentionally separate. `crustini-lang` turns `.flour` into a small Rust app. `crustini-host` opens a native window and feeds input into that app.

## Authored Source

For a single-file app, the source is one file:

```txt
examples/brick-breaker-plus/
  app.flour
  README.md
```

For a current compatibility project-shaped app, config and source are split:

```txt
examples/hello-world/
  recipe.flour
  src/main.flour
  README.md
```

The authored files are the only files users should edit.

## Language Compile

Run:

```bash
cargo run --bin rx -- emit examples/brick-breaker-plus/app.flour
```

That calls into `crustini-lang`.

The compiler reads source, builds an in-memory app model, then emits Rust shaped like this:

```rust
#![no_std]

use crustini_core::{Buttons, CrustiniApp, Screen};

pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 240;
pub const FPS: usize = 50;

pub struct App {
    pub paddle_x: i16,
    pub ball_x: i16,
    pub ball_y: i16,
}

impl App {
    pub const fn new() -> Self {
        Self {
            paddle_x: 132,
            ball_x: 160,
            ball_y: 188,
        }
    }
}

impl CrustiniApp for App {
    fn update(&mut self, input: Buttons) {
        if input.left {
            self.paddle_x = self.paddle_x - 5;
        }
    }

    fn draw(&self, screen: &mut Screen<'_>) {
        screen.clear(5);
        screen.rect(self.paddle_x, 202, 52, 7, 245);
    }
}
```

That snippet is shortened for readability. The actual generated file contains every state field and statement from the app.

## Conceptual Mapping

The new spec should eventually lower like this:

```flour
+++
crustini = "0.1"
name = "Mover"
fps = 30
window = [640, 360]
+++

app! Main {
  state {
    x: number = 40;
  }

  fn update() {
    x += axis_x() * 120 * dt();
  }

  fn draw() {
    clear(Color::Black);
    rect(x, 100, 24, 24, Color::White);
  }
}
```

Conceptually becomes:

```txt
front matter -> generated constants and host metadata
state fields -> fields on the generated app struct
update()     -> CrustiniApp::update
draw()       -> CrustiniApp::draw
builtin calls -> runtime screen/input/time calls
```

The generated app depends on `crustini-core`, not on the native window host.

## Generated App Crate

Run:

```bash
cargo run --bin rx -- bake examples/brick-breaker-plus/app.flour
```

For a single-file example, `rx bake` writes:

```txt
examples/brick-breaker-plus/.bakery/
  Cargo.toml
  src/lib.rs
  target/
```

For a current compatibility project-shaped app, generated Rust may be written under:

```txt
examples/hello-world/.crustini/generated/
  Cargo.toml
  src/lib.rs
  target/
```

Do not edit generated Rust as source.

## Preview Host

Run:

```bash
cargo run --bin rx -- examples/brick-breaker-plus/app.flour
```

Preview mode writes a small host crate under the generated workspace:

```txt
.bakery/preview/
  Cargo.toml
  src/main.rs
```

The preview crate depends on:

- the generated app crate,
- `crustini-core`,
- `crustini-host`.

Then Cargo builds and runs the preview crate. `crustini-host` owns the native window; the language compiler does not know about windows.

## Headless Frame

For CI or quick visual checks:

```bash
env CRUSTINI_FRAME=1 cargo run --quiet --bin rx -- examples/brick-breaker-plus/app.flour
```

That writes one rendered frame under the generated workspace.
