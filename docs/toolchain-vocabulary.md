# Crustini Toolchain Vocabulary

This document is the working source of truth for the Crustini product language and build lifecycle. It defines the terms we will use in docs, UI, examples, CLI planning, and agent notes.

The short version:

```txt
.flour source
  -> bake
  -> oven
  -> loaf

crumbs explain what happened along the way
starter creates a new source tree
```

## Vocabulary

### `.flour`

`.flour` is the Crustini source file extension and the name for user-authored source code.

A `.flour` file should be small, direct, and designed for app logic on constrained targets. It describes app state, setup, update logic, drawing, input handling, display configuration, and later pools/resources. It is not Rust with lighter syntax. It is the user-facing Crustini language that compiles into generated Rust.

Use `.flour` when talking about:

- source files written by users
- language examples
- editor modes
- syntax highlighting
- starters and templates
- source-level diagnostics

Current transition note: the compiler accepts `.flour` source and keeps `.crst` compatibility for existing examples. Some syntax package assets still use `.crs` naming. New documentation should describe `.flour` as the source extension and call out `.crst`/`.crs` only when exact existing paths require it.

Example:

```crustini
app! {
  screen 240 135
  fps 30

  state {
    x: i16 = 120
    y: i16 = 67
    vx: i16 = 1
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

## `bake`

`bake` is the compile/build action. It is the verb for turning `.flour` source into generated Rust, building that Rust, and producing a final artifact.

The intended mental model:

```txt
bake app.flour
```

Under the hood, a bake has phases:

1. Read `.flour` source.
2. Parse the app structure.
3. Validate language-level rules.
4. Emit generated Rust.
5. Write or update the oven.
6. Invoke the platform build backend when requested.
7. Produce a loaf.
8. Write crumbs for diagnostics, warnings, and build metadata.

The CLI exposes this as:

```bash
cargo run -p crustini -- bake examples/bounce/app.crst
```

`build` remains as a compatibility alias, but docs and UX should prefer `bake`.

## `oven`

`oven` is the build workspace and runtime target directory. It is where Crustini stores generated code, generated manifests, build cache data, intermediate files, target-specific configuration, and anything needed to turn source into an artifact.

The oven is not the final shipped app. It is the controlled intermediate environment where source is transformed and compiled.

An oven can contain:

- generated Rust source
- generated `Cargo.toml`
- lockfiles
- target metadata
- generated runtime bindings
- source maps or source spans
- incremental build cache
- crumbs from the latest bake
- backend-specific files for firmware, wasm, desktop, or test builds

The compiler writes generated Rust projects into `.oven/` beside the source file:

```txt
examples/bounce/.oven/
```

## `loaf`

`loaf` is the final artifact produced by a successful bake.

Depending on target, a loaf may be:

- a native binary
- a static library
- firmware image
- wasm bundle
- simulator bundle
- packaged example
- test fixture artifact

The key rule is that a loaf is what the user can run, flash, ship, publish, or inspect as the output of a bake. Generated Rust is usually oven material, not the loaf itself, unless the user explicitly asked for source emission as the artifact.

The CLI should eventually make the loaf path obvious:

```txt
Baked app.flour
Loaf: target/crustini/app
Oven: .oven/
Crumbs: .oven/crumbs/latest.log
```

## `crumbs`

`crumbs` are diagnostics, logs, notes, warnings, errors, traces, and build metadata.

Crumbs should answer:

- What source file was baked?
- Which target was selected?
- Which generated files changed?
- Which phase failed?
- What source span caused the issue?
- What command was invoked under the hood?
- Where did the loaf go?
- Which warnings should the user act on?

Crumbs can be shown in:

- CLI output
- machine-readable JSON
- editor diagnostics
- site playground logs
- CI summaries
- `.oven/crumbs/` files

Crumbs should be terse by default and expandable when debugging. The user should not have to read raw Cargo output first when the problem is a Crustini source problem.

Good crumb style:

```txt
error[E020]: unknown draw command `circ`
  --> app.flour:28:5
   |
28 |     circ x y 8 255
   |     ^^^^ did you mean `circle`?

bake failed before Rust generation
```

Raw backend output can still be attached as lower-level crumbs.

## `starter`

`starter` means a project template or scaffold.

A starter should include enough structure to bake immediately:

```txt
counter/
  app.flour
  crustini.toml
  README.md
```

Later starter commands might look like:

```bash
crustini starter counter my-counter
crustini starter display/st7789 tiny-display
crustini starter wasm-pixels web-demo
```

Starter docs should describe:

- target hardware or runtime
- display assumptions
- input assumptions
- bake command
- expected loaf
- where crumbs are written

Current transition note: `examples/` currently acts as the rough starter inventory, but examples and starters should eventually be split. Examples prove compiler behavior; starters are user-facing scaffolds.

## Full Lifecycle

The target workflow should feel like this:

```txt
create starter
  -> edit .flour
  -> bake
  -> inspect crumbs
  -> run loaf
  -> repeat
```

In more detail:

1. A user creates a starter.
2. The starter gives them an `app.flour` and target config.
3. They edit source in an editor with `.flour` highlighting.
4. They run `bake`.
5. The compiler validates the source and writes generated files into the oven.
6. The backend build runs inside or against the oven.
7. Diagnostics and logs are stored as crumbs.
8. A successful bake produces a loaf.
9. The user runs, flashes, previews, or publishes the loaf.

## Naming Boundaries

The themed terms should be useful, not cute at the expense of clarity. Use them where they map cleanly to concrete toolchain concepts.

Use product terms for user-facing concepts:

| Product term | Generic term |
| --- | --- |
| `.flour` | source file |
| `bake` | compile/build |
| `oven` | generated workspace/build cache |
| `loaf` | binary/artifact |
| `crumbs` | diagnostics/logs |
| `starter` | template/scaffold |

Use technical terms when precision matters:

- Rust crate
- `#![no_std]`
- Cargo manifest
- parser
- emitter
- runtime
- target triple
- linker script
- source span
- diagnostic code

Do not force the metaphor into internals where it hides meaning. For example, `emit_rust.rs` is a better implementation filename than `shape_loaf.rs`.

## Current Repo Mapping

| Future/product concept | Current path or command |
| --- | --- |
| `.flour` source | `app.flour`; existing fixtures include `examples/bounce/app.crst`, `examples/counter/app.crst` |
| `bake` | `cargo run -p crustini -- bake <file.flour>` |
| source emission | `cargo run -p crustini -- emit <file.crst>` |
| `oven` | `<example>/.oven/` |
| generated Rust | `<example>/.oven/src/lib.rs` |
| loaf | Cargo output for generated crate |
| crumbs | CLI errors and Cargo output |
| starter | `cargo run -p crustini -- starter counter <dir>` |

## Migration Plan

The documentation can move first, but implementation should migrate in small compatibility-preserving steps.

1. Accept `.flour` files anywhere `.crst` files are accepted. Done.
2. Add `bake` as a CLI command alias for `build`. Done.
3. Write generated workspaces to `.oven/`. Done.
4. Add a tiny `starter counter` scaffold. Done.
5. Update examples or duplicate one example as `app.flour`.
6. Teach syntax packages and editor extensions to recognize `.flour`.
7. Add crumb formatting with source spans and diagnostic codes.
8. Split user-facing starters from compiler regression examples.
9. Make site examples use `.flour` once the example files are migrated.

Compatibility rule: old `.crst` fixtures should keep working until there is an explicit breaking-change decision.

## Open Questions

- Should the CLI binary remain `crustini`, or should there eventually be a shorter dedicated command?
- Should `loaf` refer only to executable artifacts, or also to generated libraries and firmware bundles?
- Should crumbs be written by default, or only when a bake fails or verbose mode is enabled?
- Should starters live in `starters/`, `templates/`, or remain in `examples/` until the language stabilizes?

## Writing Guidelines

Use this sentence shape when introducing the toolchain:

```txt
Crustini bakes `.flour` source into a loaf through a generated Rust oven, with crumbs for diagnostics.
```

Use this shape when being more precise:

```txt
Today, `crustini bake` reads `.flour` source, emits a generated `#![no_std]` Rust crate into `.oven/`, and lets Cargo build it. Existing `.crst` source still works as a compatibility path.
```

Avoid writing docs that only use the metaphor without the real technical noun nearby. First-time readers should be able to understand both the friendly vocabulary and the underlying build system.
