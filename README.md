# Crustini

Tiny app syntax that compiles into boring `#![no_std]` Rust.

Project vocabulary is moving toward the baking-themed toolchain:

```txt
.flour source code
bake   compile/build
oven   build cache/runtime target dir
loaf   final binary/artifact
crumbs diagnostics/logs
starter project template
```

The current compiler accepts `.flour` source, keeps `.crst` compatibility for existing examples, and writes generated crates into `.oven/`.

This is intentionally crude. It is not a serious compiler yet. It is a section scanner + tiny line parser + Rust string emitter.

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

The dense vocabulary and lifecycle document lives in [`docs/toolchain-vocabulary.md`](docs/toolchain-vocabulary.md).

Short mapping:

| Product term | Meaning today |
| --- | --- |
| `.flour` | Source extension for user-authored Crustini apps. |
| `bake` | Build/compile action; `build` remains a compatibility alias. |
| `oven` | Generated build workspace; current directory is `.oven/`. |
| `loaf` | Final artifact from Cargo, firmware, wasm, or another backend. |
| `crumbs` | Diagnostics, logs, warnings, errors, and build metadata. |
| `starter` | Project template; current rough equivalents are `examples/`. |

## Shape

```txt
.flour source
  -> bake
  -> oven
  -> loaf
```

Current implementation shape:

```txt
.flour source
  -> crustini host compiler
  -> generated no_std Rust crate in .oven/
  -> cargo build
```

## Build the compiler

```bash
cargo build
```

## Build the site

```bash
bun install
bun run site:dev
```

The Astro site lives in `apps/site` and imports the shared syntax package from `packages/crustini-syntax`.

## Create a starter

```bash
cargo run -p crustini -- starter counter my-counter
cargo run -p crustini -- bake my-counter/app.flour
```

## Generate and build the bounce example

```bash
cargo run -p crustini -- bake examples/bounce/app.crst
cargo build --manifest-path examples/bounce/.oven/Cargo.toml
```

## Emit generated Rust to stdout

```bash
cargo run -p crustini -- emit examples/bounce/app.crst
```

## Crustini syntax currently supported

```crustini
app! {
  screen 240 135
  fps 30

  state {
    x: i16 = 120
    y: i16 = 67
    vx: i16 = 1
  }

  setup {
    clear 0
  }

  update {
    x = x + vx

    if x > 232 {
      vx = -1
    }

    if x < 8 {
      vx = 1
    }
  }

  draw {
    clear 12
    circle x y 8 255
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
