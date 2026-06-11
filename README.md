# Crustini

Crustini is a single-file-first creative coding language.

Its language direction is:

```txt
Processing immediacy with Rust-shaped structure when the sketch grows up.
```

Artists write `.flour` source. Crustini parses it, checks it, lowers it into generated Rust, and runs it through a lightweight native preview host.

The active v0 language spec is [`new-spec.md`](new-spec.md).

## Current Status

This repository is in transition.

The product direction is the new `.flour` syntax in [`new-spec.md`](new-spec.md):

```flour
+++
crustini = "0.1"
name = "Sketch"
fps = 30
window = [640, 360]
+++

app! Main {
  state {
    x: number = 40;
  }

  fn draw() {
    clear(Color::Black);
    x += 2;
    circle(x, 180, 24, Color::White);
  }
}
```

The current compiler is still smaller than that target. It accepts `.flour`, keeps `.crst` as an old compatibility extension, and still supports legacy implementation syntax in existing examples and fixtures. Treat that old surface as implementation compatibility, not the product direction.

## Language Shape

Crustini v0 has one language with three teaching modes.

### Sketch Mode

Sketch mode is the Processing-style entry point.

```flour
app! Main {
  state {
    x: number = 40;
  }

  fn draw() {
    clear(Color::Black);
    x += 1;
    rect(x, 100, 24, 24, Color::White);
  }
}
```

Rule:

```txt
If update() is omitted, draw() may mutate state.
```

This keeps the first experience immediate, visual, and forgiving.

### App Mode

App mode is the recommended shape once behavior becomes more than a tiny sketch.

```flour
app! Main {
  state {
    x: number = 40;
    speed: number = 120;
  }

  fn update() {
    x += axis_x() * speed * dt();
  }

  fn draw() {
    clear(Color::Black);
    rect(x, 100, 24, 24, Color::White);
  }
}
```

Rule:

```txt
If update() exists, mutation belongs in update().
draw() should be rendering-only except for short-lived local drawing calculations.
```

### Structured Mode

Structured mode is for larger sketches, toys, and games.

Use `struct`, `enum`, `match`, and helper functions:

```flour
enum Mode {
  Title,
  Playing,
}

app! Main {
  state {
    mode: Mode = Mode::Title;
  }

  fn update() {
    if pressed(Button::A) {
      mode = Mode::Playing;
    }
  }

  fn draw() {
    clear(Color::Black);

    match mode {
      Mode::Title => {
        text(40, 40, "PRESS A", Color::White);
      }

      Mode::Playing => {
        text(40, 40, "PLAYING", Color::Green);
      }
    }
  }
}
```

## Syntax Principles

Keep `app! Main { ... }` as the visible compiler-level app form.

Normal runtime work should use normal function calls:

```flour
clear(Color::Black);
rect(x, y, 20, 20, Color::White);
pressed(Button::A);
dt();
```

Use `::` for named constants, enum variants, and assets:

```flour
Color::Black
Button::A
Sprite::Player
Mode::Title
```

Use tiny beginner-facing primitive types:

```txt
number
text
bool
```

Do not expose imports, modules, generics, traits, lifetimes, references, or ownership syntax in the v0 user surface.

## File Shape

A canonical `.flour` file has optional TOML front matter and Crustini source code:

```flour
+++
crustini = "0.1"
name = "Starter"
fps = 30
window = [640, 360]
+++

app! Main {
  fn draw() {
    clear(Color::Black);
    text(24, 24, "hello crustini", Color::White);
  }
}
```

Use plain `+++` delimiters. Do not write `+++ config` or `+++ sketch`.

Starter files should include:

```toml
crustini = "0.1"
name = "Starter"
fps = 30
window = [640, 360]
```

## Vocabulary

Project vocabulary is intentionally small.

| Product term | Meaning |
| --- | --- |
| `.flour` | Source extension for user-authored Crustini apps. |
| `rx` | Short command for running, proofing, baking, and emitting Crustini apps. |
| `main.flour` | Product-direction default file for a one-file sketch folder. |
| `recipe.flour` | Current compatibility project config for project-shaped apps. |
| `proof` | Check/validate source without writing a bakery or building. |
| `bake` | Build/compile action; `build` remains a compatibility alias. |
| `bakery` | Generated build workspace/cache; current directory is `.bakery/`. |
| `starter` | Project template. |

Use plain words for artifact, diagnostics, logs, generated Rust, and runtime.

The dense naming guide lives in [`docs/toolchain-vocabulary.md`](docs/toolchain-vocabulary.md).

## Shape

Product direction:

```txt
.flour source
  -> Crustini compiler
  -> generated Rust
  -> lightweight native desktop app
```

Current implementation shape:

```txt
rx app.flour
  -> .bakery/
  -> native preview window

rx bake app.flour
  -> .bakery/
  -> artifact only
```

For current project-shaped compatibility mode, `recipe.flour` plus `src/main.flour` can write generated Rust under `.crustini/generated/`.

## Run An App

The easy path opens a native preview window:

```bash
cargo run --bin rx -- examples/brick-breaker/app.flour
```

After installing `rx`, that becomes:

```bash
rx examples/brick-breaker/app.flour
```

Inside a current project directory with `recipe.flour`, a bare `rx` opens that project:

```bash
rx
```

Use `proof` to check source without writing a bakery:

```bash
rx proof examples/brick-breaker/app.flour
```

Use `bake` when you only want the generated artifact:

```bash
rx bake examples/brick-breaker/app.flour
```

Emit generated Rust to stdout:

```bash
rx emit examples/brick-breaker/app.flour
```

For headless checks and CI, set `CRUSTINI_FRAME=1`. That runs one frame through the preview host and writes a `frame.ppm` under the app bakery.

## Build

Build the Rust crates:

```bash
cargo build
```

The root Bun workspace is the command layer for the whole repo:

```bash
bun run ci
bun run check
bun run build
bun run bake:example
bun run clean:dry
bun run clean
bun run port:clean 4321
```

Command flow lives in `scripts/*.ts`; `package.json` is the command menu. The shell scripts in `scripts/*.sh` are compatibility wrappers.

## Build the Site

```bash
bun install
bun run site:dev
```

The Astro site lives in `apps/site` and imports the shared syntax package from `packages/crustini-syntax`.

## Create a Starter

Current starters still use the compatibility project shape:

```bash
cargo run --bin rx -- starter hello my-hello
cargo run --bin rx -- my-hello
cargo run --bin rx -- proof my-hello
cargo run --bin rx -- bake my-hello
```

List starters with:

```bash
cargo run --bin rx -- starters
```

The current compatibility project layout is:

```txt
my-hello/
  recipe.flour
  src/
    main.flour
  assets/
  .crustini/
    generated/
```

The product direction is simpler for beginners:

```txt
my-sketch/
  main.flour
```

Do not silently rename implementation paths until the CLI supports that shape.

## Monorepo Layout

```txt
apps/site                 Astro website and docs tools
crates/crustini           rx CLI, project config, bakery orchestration
crates/crustini-lang      .flour parser, app model, diagnostics, Rust emitter
crates/crustini-core      no_std runtime crate used by generated apps
crates/crustini-host      native std preview host for easy-mode rx run
crates/crustini-web-host  parked browser preview host experiment
packages/crustini-syntax  TypeScript syntax highlighting package
editors/vscode            VS Code language extension
examples                  real runnable app examples
fixtures                  compiler and CLI source fixtures
scripts                   repo-level helper scripts
```

## Related Docs

- [`new-spec.md`](new-spec.md): active v0 language direction.
- [`AGENTS.md`](AGENTS.md): repository instructions for agents.
- [`docs/toolchain-vocabulary.md`](docs/toolchain-vocabulary.md): naming and lifecycle notes.
- [`docs/basic-project.md`](docs/basic-project.md): current simple project shape.
- [`docs/compatibility-parser.md`](docs/compatibility-parser.md): current compatibility parser layout.
- [`docs/target-spec/`](docs/target-spec/): early target-shape notes.
- [`docs/source-first-code-sharing.md`](docs/source-first-code-sharing.md): source-sharing principle.
- [`examples/compiling.md`](examples/compiling.md): current compile walkthrough.

## What This Does Not Have Yet

- no full parser for the complete new v0 spec
- no LSP
- no tree-sitter
- no formatter
- no package manager
- no imports
- no modules
- no heap requirement
- no allocator requirement

The point of this repo is to prove this path:

```txt
simple Crustini source -> generated Rust compiles -> native preview runs
```
