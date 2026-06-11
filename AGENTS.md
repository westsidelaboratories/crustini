# AGENTS.md

Guidance for agents working in this repository.

## Source of Truth

The active language direction is [`new-spec.md`](new-spec.md).

When product-facing docs, examples, UI copy, planning notes, or implementation plans describe the future Crustini language, follow that spec exactly:

```txt
Processing immediacy with Rust-shaped structure when the sketch grows up.
```

Crustini is a single-file-first creative coding language that starts with Processing-style `draw` sketches and grows into structured `update`/`draw` apps with explicit state, normal function calls, and generated Rust underneath.

## Project Direction

Use the current vocabulary consistently.

| Term | Meaning | Current implementation note |
| --- | --- | --- |
| `.flour` | Crustini source code file. This is the source language artists write. | The compiler accepts `.flour`. |
| `rx` | Short command for running, proofing, baking, and emitting Crustini apps. | A bare `.flour` path runs/previews the app; `rx bake` builds an artifact. |
| `main.flour` | Product-direction default file for a one-file sketch folder. | Current project-shaped starters still use `recipe.flour` plus `src/main.flour`. |
| `recipe.flour` | Compatibility project config for current project-shaped apps. | Do not silently rename this in docs or code until the CLI supports the new shape. |
| `proof` | Check/validate source without writing a bakery or building. | Current CLI command is `rx proof`; `check` is a compatibility alias. |
| `bake` | Compile/build action. Turns source into generated Rust and then an artifact. | Current CLI command is `rx bake`; `build` is a compatibility alias. |
| `bakery` | Build cache, generated target directory, and intermediate workspace. | Current generated directory is `.bakery/` for single-file apps. |
| `starter` | Project template or scaffolded starting point. | Real runnable apps are under `examples/`; compiler fixtures are under `fixtures/`. |

Use plain words for artifact, diagnostics, logs, generated Rust, and runtime.

## Language Rules

For new product-facing Crustini syntax, use the `new-spec.md` rules:

- Keep `app! Main { ... }` as the visible compiler-level app form.
- Do not use `!` for normal runtime work.
- Prefer normal calls: `clear(...)`, `rect(...)`, `circle(...)`, `text(...)`, `pressed(...)`, `dt()`.
- Prefer TOML front matter with plain `+++` delimiters for metadata: `crustini`, `name`, `fps`, and `window`.
- Use `state { ... }`, `fn setup()`, `fn update()`, and `fn draw()` inside `app! Main`.
- Use `number`, `text`, and `bool` as beginner-facing primitive types.
- Use `Color::Black`, `Button::A`, `Sprite::Player`, and enum variants like `Mode::Title`.
- Use `struct`, `enum`, `match`, and helper functions for larger sketches.
- Do not add imports, modules, generics, traits, lifetimes, references, or ownership ceremony to the v0 user surface.

The intentional teaching progression is:

1. Sketch mode: `state` plus `draw`.
2. App mode: `state` plus `update` plus `draw`.
3. Structured mode: `struct`, `enum`, `match`, and helper functions.

Sketch mode rule:

```txt
If update() is omitted, draw() may mutate state.
```

App mode rule:

```txt
If update() exists, mutation belongs in update().
draw() should be rendering-only except for short-lived local drawing calculations.
```

The compiler does not need to reject every `draw` mutation in v0, but docs, starters, examples, and diagnostics should teach these rules.

## Documentation Rules

- Treat `new-spec.md` as the active v0 language spec.
- Keep `README.md` as the short orientation and link to `new-spec.md`.
- Prefer `.flour`, `rx`, `main.flour`, `proof`, `bake`, `bakery`, and `starter` for product-facing naming.
- Mention `recipe.flour` only when describing current project-shaped implementation details or compatibility.
- When documenting exact commands that exist today, prefer `rx bake` and mention `build` only as a compatibility alias.
- Do not silently rename code paths in docs if the code still expects the old name.
- Keep the distinction clear between source language, compiler implementation, generated Rust, runtime crate, preview host, and final artifact.
- When showing future-facing source examples, use the new syntax from `new-spec.md`.
- When showing current implementation fixtures, label legacy syntax as current implementation compatibility.
- Add new language/tooling design notes under `docs/` and link them from `README.md`, unless the user explicitly asks for a root-level draft like `new-spec.md`.

## Technical Constraints

- The current compiler is intentionally small: a compatibility parser and Rust string emitter.
- The language direction is a typed AST pipeline, but do not turn that into a large rewrite unless the task asks for it.
- Rust remains the real backend and borrow checker. Crustini users should not see Rust borrowing, lifetimes, allocator concerns, or ownership ceremony.
- Generated programs should move toward one app state struct with fixed pools, handles instead of references, frame scratch memory, and no default heap requirement.
- Deletion should mark objects dead and sweep after iteration.
- Desktop heap or managed modes can exist later, but not as the first embedded path.

## Repo Notes

- `README.md` is the short orientation.
- `new-spec.md` is the active v0 language direction.
- `docs/toolchain-vocabulary.md` is the dense naming and lifecycle document.
- `notes.txt` is the scratchpad for compact design principles.
- `crates/crustini` contains the `rx` CLI, project config, bakery writing, and Cargo orchestration.
- `crates/crustini-lang` contains the `.flour` parser, app model, and generated Rust emitter.
- `crates/crustini-core` contains the `#![no_std]` runtime surface used by generated apps.
- `crates/crustini-host` contains native std preview host code for easy-mode `rx run`.
- `crates/crustini-web-host` contains the parked browser preview host experiment and is not wired to `rx`.
- `packages/crustini-syntax` contains shared syntax-highlighting assets.
- `apps/site` contains the Astro website and docs tooling.
- `examples` contains real runnable app examples.
- `fixtures` contains compiler and CLI source fixtures.
