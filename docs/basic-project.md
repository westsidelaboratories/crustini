# Basic Crustini Project

The active language direction is [`../new-spec.md`](../new-spec.md).

The beginner-facing project shape is one readable `.flour` file.

```txt
my-sketch/
  main.flour
```

Run it from the folder:

```bash
rx
```

or from elsewhere:

```bash
rx path/to/my-sketch/main.flour
```

## `main.flour`

A starter `main.flour` should use TOML front matter plus normal Crustini source:

```flour
+++
crustini = "0.1"
name = "Starter"
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

This is sketch mode. If `update()` is omitted, `draw()` may mutate state.

As behavior grows, use app mode:

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

If `update()` exists, mutation belongs in `update()` and `draw()` should render the current state.

## Compatibility Project Shape

The current CLI still supports project-shaped apps with compatibility config:

```txt
my-app/
  recipe.flour
  src/
    main.flour
  assets/
  .crustini/
    generated/
```

Use this shape only when describing implementation behavior that exists today. Do not present it as the beginner default.

Only edit authored files:

- `.flour` source files
- compatibility config when using the current project mode
- files under `assets/`

Do not edit generated Rust under `.bakery/` or `.crustini/generated/`.

## Commands

Preview:

```bash
rx app.flour
```

Build the artifact without opening a window:

```bash
rx bake app.flour
```

Validate without writing generated Rust:

```bash
rx proof app.flour
```

## Generation Boundary

The source of truth is user-authored `.flour`.

Generated Rust is implementation output. If generation needs to change:

- `.flour` parsing and Rust emitting live in `crates/crustini-lang`.
- `rx`, project config, starter writing, and bakery layout live in `crates/crustini`.
- starter templates live in `crates/crustini/src/starter.rs`.
- native window behavior lives in `crates/crustini-host`.

That keeps app source, generated Rust, and host tooling separate.
