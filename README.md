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
crates/crustini           host compiler crate and CLI
crates/crustini-core      no_std runtime crate used by generated apps
packages/crustini-syntax  TypeScript syntax highlighting package
editors/vscode            VS Code language extension
examples                  app fixtures
scripts                   repo-level helper scripts
```

## Vocabulary

The naming guide lives in [`docs/toolchain-vocabulary.md`](docs/toolchain-vocabulary.md).

Short mapping:

| Product term | Meaning today |
| --- | --- |
| `.flour` | Source extension for user-authored Crustini apps. |
| `rx` | Short command for running, baking, and emitting Crustini apps. |
| `recipe.flour` | Project config for multi-file apps. |
| `proof` | Check/validate source without writing a bakery or building. |
| `bake` | Build/compile action; `build` remains a compatibility alias. |
| `bakery` | Generated build workspace/cache; current directory is `.bakery/`. |
| `starter` | Project template; current rough equivalents are `examples/`. |

## Shape

```txt
.flour source
  -> bake
  -> .bakery/
  -> artifact
```

Current implementation shape:

```txt
.flour source
  -> crustini host compiler
  -> generated no_std Rust crate in .bakery/
  -> cargo build
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

The `examples/all-macros/app.flour` fixture covers that whole basic set.

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
assets!:
build!:
```

The `examples/hood-wars` project is the first real project-shaped example:

```bash
cargo run --bin rx -- proof examples/hood-wars
cargo run --bin rx -- bake examples/hood-wars
cargo run --bin rx -- examples/hood-wars
```

The bake path is still intentionally simple:

```txt
examples/hello/app.flour
  -> crustini parser
  -> generated no_std Rust in examples/hello/.bakery/src/lib.rs
  -> generated std preview host in examples/hello/.bakery/src/main.rs
  -> cargo build
  -> examples/hello/.bakery/target/debug/libcrustini_generated_hello.rlib
```

For the most basic visible output, use `run`:

```bash
cargo run --bin rx -- examples/hello/app.flour
```

That is the same as `rx run examples/hello/app.flour` after installing the tool. It bakes the app, runs one frame through the generated preview host, and writes:

```txt
examples/hello/.bakery/frame.ppm
```

Project output uses `.crustini/generated` from `recipe.flour`:

```txt
examples/hood-wars/
  recipe.flour
  src/main.flour
  .crustini/generated/
    Cargo.toml
    src/lib.rs
    src/main.rs
    frame.ppm
```

## Generate and build the bounce example

```bash
cargo run --bin rx -- bake examples/bounce/app.flour
cargo build --manifest-path examples/bounce/.bakery/Cargo.toml
```

## Check without writing a bakery

```bash
cargo run --bin rx -- proof examples/all-macros/app.flour
```

## Emit generated Rust to stdout

```bash
cargo run --bin rx -- emit examples/bounce/app.flour
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

    if x > 232 {
      vx = -1
    }

    if x < 8 {
      vx = 1
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
