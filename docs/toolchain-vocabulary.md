# Crustini Naming Guide

This is the working naming guide for Crustini docs, CLI output, examples, and agent notes.

Keep the vocabulary small:

```txt
.flour  source file
rx      command
recipe project config
bake    compile/build action
bakery  generated build workspace/cache
starter project template
```

Use plain technical words for everything else: artifact, diagnostics, logs, generated Rust, runtime, target, cache, manifest.

## `rx`

`rx` is the Crustini command.

The default command previews a `.flour` app:

```bash
rx app.flour
```

Use explicit verbs when you want a specific phase:

```bash
rx run app.flour
rx proof app.flour
rx bake app.flour
rx emit app.flour
rx starter hello my-hello
rx starters
```

## `recipe.flour`

`recipe.flour` is the Crustini project config file.

Single-file apps can still be run directly:

```bash
rx app.flour
```

Project-shaped apps use:

```txt
my-app/
  recipe.flour
  src/main.flour
  assets/
```

Project output is generated under `.crustini/generated/` by default.

## `.flour`

`.flour` is the Crustini source file extension.

A `.flour` file is user-authored app source. It describes state, setup, update logic, drawing, input, display configuration, and later resource pools. It is not Rust with lighter syntax; it is the small Crustini language that emits generated Rust.

Current compatibility note: `.crst` remains accepted as an old source extension, but repo examples and new docs should use `.flour`. Some syntax package internals still use `.crs` naming while editor tooling catches up.

## `bake`

`bake` is the compile/build command.

```bash
rx bake app.flour
```

In this repo during development:

```bash
cargo run --bin rx -- bake fixtures/apps/bounce/app.flour
```

A bake currently:

1. Reads source.
2. Parses the app with a small macro-shaped parser.
3. Emits a generated `#![no_std]` Rust crate.
4. Writes that crate into `.bakery/`.
5. Runs Cargo unless `--no-cargo` is passed.
6. Prints the generated artifact path.

`build` remains a compatibility alias for `bake`.

## `proof`

`proof` checks source without writing a bakery or building an artifact.

```bash
rx proof app.flour
```

`check` remains a plain-word alias for `proof`.

## `bakery`

`bakery` is the generated build workspace/cache.

The directory is:

```txt
.bakery/
```

It contains generated Rust, generated manifests, lockfiles, Cargo output, and other intermediate build files. It is not the shipped app; it is the workspace Crustini uses to turn source into a normal artifact.

Example:

```txt
fixtures/apps/bounce/
  app.flour
  .bakery/
    Cargo.toml
    src/lib.rs
    target/
```

Why not `oven`: Oven is already strongly associated with Bun's company/tooling, so Crustini should not use it.

## `starter`

`starter` is a project template.

The default tiny starter is:

```bash
cargo run --bin rx -- starter hello my-hello
cargo run --bin rx -- bake my-hello/app.flour
```

A starter should be simple enough to bake immediately:

```txt
my-hello/
  app.flour
  crustini.toml
  README.md
```

## Current Mapping

| Concept | Current implementation |
| --- | --- |
| Source | `.flour`; `.crst` still works as an old alias |
| Command | `rx` |
| Project config | `recipe.flour` |
| Check command | `rx proof`; `check` still works |
| Build command | `rx bake`; `build` still works |
| Generated workspace | `.bakery/` |
| Project generated workspace | `.crustini/generated/` |
| Generated Rust | `.bakery/src/lib.rs` |
| Artifact | Cargo output under `.bakery/target/` |
| Starter | `rx starter hello <dir>` |

## Writing Rules

- Use `.flour`, `rx`, `recipe.flour`, `proof`, `bake`, `bakery`, and `starter`.
- Do not use `oven`, `loaf`, or `crumbs` as product terms.
- Say `artifact`, not `loaf`.
- Say `diagnostics` or `logs`, not `crumbs`.
- Say `generated Rust` when that is what you mean.
- Keep command examples real and prefer `.flour`.

Recommended short description:

```txt
Crustini bakes `.flour` source into generated `#![no_std]` Rust in `.bakery/`, then Cargo builds a normal artifact.
```
