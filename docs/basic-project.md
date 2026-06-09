# Basic Crustini Project

This is the current simple project shape. It is intentionally small and Processing-like:

```bash
rx
```

opens the native preview window when run inside a project directory.

## File Tree

```txt
my-app/
  recipe.flour
  src/
    main.flour
  assets/
  .crustini/
    generated/
```

Only edit:

- `recipe.flour`
- `src/main.flour`
- files under `assets/`

Do not edit `.crustini/generated/`. It is written by `rx`.

## `recipe.flour`

`recipe.flour` describes the project and preview surface.

```flour
project!:
  name "hello-world"
  version "0.1.0"

screen!:
  size 180, 120
  fps 30

preview!:
  title "Hello World"
  scale auto

build!:
  generated ".crustini/generated"
```

For now, this is the basic mode:

- `project!` names the app.
- `screen!` sets logical pixels and frame rate.
- `preview!` sets the native preview window title and scale.
- `build!` says where generated Rust goes.

Accepted preview scales are `auto`, `1x`, `2x`, `4x`, and `8x`.

## `src/main.flour`

`src/main.flour` is the app source.

```flour
app! {
  draw! {
    clear!(8)
    text!(42, 56, "HELLO WORLD", 255)
  }
}
```

In project mode, `screen!:` from `recipe.flour` supplies the size and fps, so the source can focus on app behavior.

## Commands

Inside a project directory:

```bash
rx
```

Open a project from elsewhere:

```bash
rx path/to/my-app
```

Build the artifact without opening a window:

```bash
rx bake .
```

Validate without writing generated Rust:

```bash
rx proof .
```

## Repo Checks

In this repository, use one command before shipping changes:

```bash
bun run check
```

That runs Rust formatting, Rust checks, tests, clippy, TypeScript checks, site checks, and bakes fixtures/examples.

Use this to build everything:

```bash
bun run build
```

Use this to run the default example:

```bash
bun run run:example
```

## Generation Boundary

The source of truth is user-authored `.flour` plus `recipe.flour`.

Generated Rust lives in:

```txt
.crustini/generated/
```

The native preview host lives in a generated sub-crate:

```txt
.crustini/generated/preview/
```

If generation needs to change:

- `.flour` parsing and Rust emitting live in `crates/crustini-lang`.
- `rx`, project config, starter writing, and bakery layout live in `crates/crustini`.
- starter templates live in `crates/crustini/src/starter.rs`.
- native window behavior lives in `crates/crustini-host`.

That keeps app source, generated Rust, and host tooling separate.
