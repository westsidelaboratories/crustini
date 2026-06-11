# Crustini Examples

These are real Crustini apps that can be run, baked, inspected, and changed.

The active language direction is [`../new-spec.md`](../new-spec.md). Some examples still use current implementation compatibility syntax until the compiler catches up.

Compiler fixtures live under `fixtures/`. This directory is for user-facing starter material and runnable apps.

## The Important Split

Every example has two kinds of files:

| Kind | Owned by | Edit this? | Purpose |
| --- | --- | --- | --- |
| Authored `.flour` | You | Yes | Crustini source code. |
| `recipe.flour` | You | Yes | Compatibility config for current project-shaped examples. |
| `.bakery/` or `.crustini/generated/` | `rx` | No | Generated Rust, Cargo files, preview shell, build output. |

The full compile walkthrough is here:

- [Compiling Examples](./compiling.md)

## Run All Example Bakes

```bash
bun run bake:example
```

For the full repo check:

```bash
bun run check
```

## Examples

| Example | Shape | Screen | What it proves |
| --- | --- | --- | --- |
| [hello-world](./hello-world/) | Project | `180x120` | Current compatibility project shape and generated project output. |
| [brick-breaker](./brick-breaker/) | Single file | `160x90` | The smallest playable brick breaker. |
| [brick-breaker-plus](./brick-breaker-plus/) | Single file | `320x240` | Bigger game with the deepest compile write-up. |
| [mouse-orbit](./mouse-orbit/) | Single file | `180x120` | Mouse position, click state, simple animation. |
| [dodge-dots](./dodge-dots/) | Single file | `180x120` | Arrows, mouse input, collision checks, reset state. |

## Quick Commands

Project-shaped app:

```bash
cargo run --bin rx -- examples/hello-world
cargo run --bin rx -- bake examples/hello-world
```

Single-file app:

```bash
cargo run --bin rx -- examples/brick-breaker-plus/app.flour
cargo run --bin rx -- bake examples/brick-breaker-plus/app.flour
```

Once `rx` is installed, the same commands become:

```bash
rx examples/hello-world
rx examples/brick-breaker-plus/app.flour
rx bake examples/brick-breaker-plus/app.flour
```

## What `rx` Does

Preview mode:

```txt
app.flour
  -> crustini-lang parses and emits Rust
  -> .bakery/src/lib.rs
  -> .bakery/preview/src/main.rs
  -> Cargo builds the preview binary
  -> native window opens through crustini-host
```

Bake mode:

```txt
app.flour
  -> crustini-lang parses and emits Rust
  -> .bakery/src/lib.rs
  -> Cargo builds the generated app crate
  -> .bakery/target/debug/libcrustini_generated_<name>.rlib
```

Project mode uses the generated directory from `recipe.flour`; `hello-world` writes to `.crustini/generated/`.
