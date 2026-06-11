# Crustini Compiler Architecture

Status: current summary aligned with [`new-spec.md`](new-spec.md).

Older architecture notes in this file were superseded by the new v0 language direction.

## Language Direction

Crustini source is `.flour`.

The user-facing language starts with Processing-style sketching and grows into structured app code:

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

`app! Main` is a Crustini compiler form.

Drawing, input, time, and helper work are normal function calls.

## Desired Pipeline

```txt
main.flour
  -> split TOML front matter from Crustini source
  -> parse TOML metadata
  -> load extra code files from code = [...]
  -> parse Crustini source files
  -> build AST
  -> resolve symbols
  -> type check
  -> lower app! into internal app representation
  -> lower builtins into runtime calls
  -> generate Rust
  -> compile native app
```

The current implementation is smaller than this target. It can stay small while moving toward the pipeline above.

## Core Data Model

The compiler should eventually model:

- front matter metadata,
- user structs and enums,
- the app compiler form,
- state fields,
- lifecycle functions,
- helper functions,
- statements and expressions,
- builtin bindings for drawing, input, and time.

Do not treat generated Rust syntax as the source language model.

## Lowering Direction

Conceptual lowering:

```txt
front matter -> generated constants and host metadata
state fields -> fields on generated app state
update()     -> runtime update method
draw()       -> runtime draw method
clear(...)   -> screen/runtime call
pressed(...) -> input/runtime call
dt()         -> time/runtime value
```

Rust remains the backend. Crustini users should not see Rust borrowing, lifetimes, allocator concerns, or ownership ceremony.

## Mutation Rule

Sketch mode:

```txt
If update() is omitted, draw() may mutate state.
```

App mode:

```txt
If update() exists, mutation belongs in update().
draw() should be rendering-only except for short-lived local drawing calculations.
```

Diagnostics and examples should teach this distinction.

## Implementation Boundaries

- `crates/crustini-lang` parses `.flour`, builds the app model, and emits Rust.
- `crates/crustini-core` exposes the no-std runtime surface used by generated apps.
- `crates/crustini-host` owns native preview behavior.
- `crates/crustini` owns the `rx` CLI, bakery writing, project compatibility, and Cargo orchestration.

Generated Rust is implementation output, not authored source.
