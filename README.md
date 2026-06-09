# Crustini

Tiny app syntax that compiles into boring `#![no_std]` Rust.

Project vocabulary is intentionally small:

```txt
.flour source code
rx     command
recipe project config
bake   compile/build
bakery generated build workspace/cache
starter project template
```

Use plain words for everything else: artifact, diagnostics, logs, generated Rust. The current compiler accepts `.flour` source, keeps `.crst` as an old compatibility extension, and writes generated crates into `.bakery/`.

This is intentionally crude. It is not a serious compiler yet. It is a tiny macro-shaped parser + Rust string emitter.

## Monorepo layout

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

## Vocabulary

The naming guide lives in [`docs/toolchain-vocabulary.md`](docs/toolchain-vocabulary.md).
The basic project spec lives in [`docs/basic-project.md`](docs/basic-project.md).
The language macro layout lives in [`docs/language-macros.md`](docs/language-macros.md).
The early target-shape spec lives in [`docs/target-spec/`](docs/target-spec/).
The source-sharing principle lives in [`docs/source-first-code-sharing.md`](docs/source-first-code-sharing.md).
The examples compile walkthrough lives in [`examples/compiling.md`](examples/compiling.md).

Short mapping:

| Product term | Meaning today |
| --- | --- |
| `.flour` | Source extension for user-authored Crustini apps. |
| `rx` | Short command for running, baking, and emitting Crustini apps. |
| `recipe.flour` | Project config for multi-file apps. |
| `proof` | Check/validate source without writing a bakery or building. |
| `bake` | Build/compile action; `build` remains a compatibility alias. |
| `bakery` | Generated build workspace/cache; current directory is `.bakery/`. |
| `starter` | Project template. |

## Shape

```txt
rx app.flour
  -> .bakery/
  -> native preview window

rx bake app.flour
  -> .bakery/
  -> artifact only
```

Current implementation shape:

```txt
.flour source
  -> crustini-lang compiler
  -> generated no_std Rust crate in .bakery/
  -> rx runs native host for preview or cargo build for bake
```

## Run An App

The easy path opens a native preview window:

```bash
cargo run --bin rx -- examples/brick-breaker/app.flour
```

After installing `rx`, that becomes:

```bash
rx examples/brick-breaker/app.flour
```

Inside a project directory with `recipe.flour`, a bare `rx` opens that project:

```bash
rx
```

Use `bake` when you only want the generated artifact:

```bash
rx bake examples/brick-breaker/app.flour
```

## Build the compiler

```bash
cargo build
```

## Repo commands

The root Bun workspace is the command layer for the whole repo. Cargo stays focused on Rust crates.

```bash
bun run ci
bun run check
bun run build
bun run bake:example
bun run clean:dry
bun run clean
bun run port:clean 4321
```

Command flow lives in `scripts/*.ts`; `package.json` is just the command menu. The shell scripts in `scripts/*.sh` are compatibility wrappers.

## Build the site

```bash
bun install
bun run site:dev
```

The Astro site lives in `apps/site` and imports the shared syntax package from `packages/crustini-syntax`.

## Create a starter

```bash
cargo run --bin rx -- starter hello my-hello
cargo run --bin rx -- my-hello
cargo run --bin rx -- my-hello/app.flour
cargo run --bin rx -- proof my-hello
cargo run --bin rx -- bake my-hello
```

The older counter starter is still available:

```bash
cargo run --bin rx -- starter counter my-counter
cargo run --bin rx -- my-counter
```

List starters with:

```bash
cargo run --bin rx -- starters
```

## Hello world

```flour
app! {
  screen!(128, 64)
  fps!(30)

  setup! {
    clear!(0)
  }

  draw! {
    clear!(0)
    text!(28, 24, "HELLO WORLD!", 255)
  }
}
```

What each part does:

| Source | Meaning |
| --- | --- |
| `app!` | Root macro for one Crustini app. |
| `screen!(128, 64)` | Sets the framebuffer size constants in generated Rust. |
| `fps!(30)` | Sets the app frame-rate constant. |
| `setup!` | Runs once before the frame loop; this starter clears the screen. |
| `draw!` | Runs when rendering; this starter clears and draws text. |
| `clear!(0)` | Calls `Screen::clear` in the no-std runtime. |
| `text!(28, 24, "HELLO WORLD!", 255)` | Calls `Screen::text` with a tiny built-in bitmap font. |

Current basic macro set:

| Macro | Where | Meaning |
| --- | --- | --- |
| `app!` | root | One app source unit. |
| `screen!(WIDTH, HEIGHT)` | app | Framebuffer size. |
| `fps!(FPS)` | app | Frame-rate constant. |
| `state!` | app | App-owned fields. |
| `setup!` | app | Runs once before frames. |
| `update!` | app | Owns mutation and input handling. |
| `draw!` | app | Draws the current frame. |
| `if CONDITION` | blocks | Conditional statements. |
| `NAME = EXPR` | blocks | State assignment. |
| `clear!(COLOR)` | setup/draw | Fill the screen. |
| `set!(X, Y, COLOR)` | setup/draw | Set one pixel. |
| `pixel!(X, Y, COLOR)` | setup/draw | Alias for `set!`. |
| `rect!(X, Y, W, H, COLOR)` | setup/draw | Filled rectangle. |
| `circle!(X, Y, R, COLOR)` | setup/draw | Filled circle. |
| `line!(X0, Y0, X1, Y1, COLOR)` | setup/draw | Line. |
| `text!(X, Y, "TEXT", COLOR)` | setup/draw | Tiny bitmap text. |
| `clamp!(VALUE, MIN, MAX)` | expressions | Lowers to `crustini_core::clamp_i16`. |
| `hit_rect!(AX, AY, AW, AH, BX, BY, BW, BH)` | expressions | Axis-aligned rectangle collision. |
| `abs!(VALUE)` | expressions | Lowers to `crustini_core::abs_i16`. |
| `min!(A, B)` | expressions | Lowers to `core::cmp::min`. |
| `max!(A, B)` | expressions | Lowers to `core::cmp::max`. |

Current basic input names are available in `update!`:

| Name | Meaning |
| --- | --- |
| `up`, `down`, `left`, `right` | Direction buttons. In preview, arrow keys and WASD. |
| `a`, `b` | Action buttons. In preview, Space/Z and Enter/X. |
| `start`, `select` | Menu buttons. In preview, Enter and Right Shift/Backspace. |
| `mouse_x`, `mouse_y` | Mouse position in preview window coordinates. |
| `mousex`, `mousey` | Short aliases for `mouse_x` and `mouse_y`. |
| `mouse_down` | Left mouse button state. |

The `fixtures/apps/all-macros/app.flour` fixture covers that whole basic set.

## Project shape

Project starters use `recipe.flour` and keep source under `src/`:

```txt
my-hello/
  recipe.flour
  src/
    main.flour
  assets/
  .crustini/
    generated/
```

The current restricted `recipe.flour` blocks are:

```txt
project!:
target!:
screen!:
memory!:
buttons!:
preview!:
assets!:
build!:
```

The optional `preview!:` block configures the local preview surface used by `rx run`; it is not part of the `.flour` language:

```flour
preview!:
  title "Tiny Demo"
  scale auto
```

Accepted scales are `auto`, `1x`, `2x`, `4x`, and `8x`.

`fixtures/projects/hood-wars` is the first project-shaped fixture:

```bash
cargo run --bin rx -- proof fixtures/projects/hood-wars
cargo run --bin rx -- bake fixtures/projects/hood-wars
cargo run --bin rx -- fixtures/projects/hood-wars
```

The bake path is still intentionally simple:

```txt
fixtures/apps/bounce/app.flour
  -> crustini-lang parser and Rust emitter
  -> generated no_std Rust in fixtures/apps/bounce/.bakery/src/lib.rs
  -> rx runs cargo build
  -> fixtures/apps/bounce/.bakery/target/debug/libcrustini_generated_bounce.rlib
```

For the most basic visible output, use `run`:

```bash
cargo run --bin rx -- examples/brick-breaker/app.flour
```

That is the same as `rx run examples/brick-breaker/app.flour` after installing the tool. It bakes the app library, writes a separate preview host under `.bakery/preview/`, opens a native window, and runs the app loop using the declared `screen!(WIDTH, HEIGHT)` and `fps!(FPS)`.

For headless checks and CI, set `CRUSTINI_FRAME=1`. That runs one frame through the separate preview host and writes:

```txt
examples/brick-breaker/.bakery/frame.ppm
```

Project output uses `.crustini/generated` from `recipe.flour`:

```txt
fixtures/projects/hood-wars/
  recipe.flour
  src/main.flour
  .crustini/generated/
    Cargo.toml
    src/lib.rs
    preview/
    frame.ppm
```

## Generate and build the bounce fixture

```bash
cargo run --bin rx -- bake fixtures/apps/bounce/app.flour
cargo build --manifest-path fixtures/apps/bounce/.bakery/Cargo.toml
```

## Check without writing a bakery

```bash
cargo run --bin rx -- proof fixtures/apps/all-macros/app.flour
```

## Emit generated Rust to stdout

```bash
cargo run --bin rx -- emit fixtures/apps/bounce/app.flour
```

## Crustini syntax currently supported

```flour
app! {
  screen!(240, 135)
  fps!(30)

  state! {
    x: i16 = 120
    y: i16 = 67
    vx: i16 = 1
  }

  setup! {
    clear!(0)
  }

  update! {
    x = x + vx
    x = clamp!(x, 8, 232)

    if hit_rect!(x - 8, y - 8, 16, 16, 220, 48, 12, 12) {
      vx = -1
    }

    if x == 8 {
      vx = abs!(vx)
    }

    if x == 232 {
      vx = 0 - abs!(vx)
    }
  }

  draw! {
    clear!(12)
    circle!(x, y, 8, 255)
    text!(8, 8, "HI", 255)
  }
}
```

## What this does not have yet

- no LSP
- no tree-sitter
- no formatter
- no parser generator
- no package manager
- no imports
- no modules
- no heap requirement
- no allocator requirement

The point of this repo is to prove this path:

```txt
simple Crustini source -> generated no_std Rust compiles
```
