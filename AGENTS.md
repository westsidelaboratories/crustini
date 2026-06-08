# AGENTS.md

Guidance for agents working in this repository.

## Project Direction

Crustini uses a small baking-themed vocabulary. Use these names in new notes, docs, UI copy, examples, and planning unless you are describing current implementation details that still use older names.

| Term | Meaning | Current implementation note |
| --- | --- | --- |
| `.flour` | Crustini source code file. This is the source language users write. | The compiler accepts `.flour`; `.crst` remains an old compatibility extension. |
| `rx` | Short command for running, baking, and emitting Crustini apps. | A bare `.flour` path runs/previews the app; `rx bake` builds an artifact. |
| `recipe.flour` | Project config for project-shaped apps. | Starters write `recipe.flour` plus `src/main.flour`; generated Rust goes under `.crustini/generated/`. |
| `proof` | Check/validate source without writing a bakery or building. | Current CLI command is `rx proof`; `check` is a compatibility alias. |
| `bake` | Compile/build action. Turns source into generated Rust and then an artifact. | Current CLI command is `rx bake`; `build` is a compatibility alias. |
| `bakery` | Build cache, generated target directory, and intermediate workspace. | Current generated directory is `.bakery/`. |
| `starter` | Project template or scaffolded starting point. | Current examples are under `examples/`. |

## Documentation Rules

- Prefer `.flour`, `rx`, `recipe.flour`, `proof`, `bake`, `bakery`, and `starter` for product-facing naming.
- Use plain words for artifact, diagnostics, logs, generated Rust, and runtime.
- When documenting exact commands that exist today, prefer `rx bake` and mention `build` only as a compatibility alias.
- Do not silently rename code paths in docs if the code still expects the old name.
- Keep the distinction clear between source language, compiler implementation, generated Rust, runtime crate, and final artifact.
- Add new language/tooling design notes under `docs/` and link them from `README.md`.

## Technical Constraints

- The compiler is intentionally small for now: macro-shaped parser, Rust string emitter.
- Rust remains the real borrow checker. Crustini users should not see Rust borrowing, lifetimes, allocator concerns, or ownership ceremony.
- Generated programs should move toward one `App` struct with fixed pools, handles instead of references, frame scratch memory, and no default heap requirement.
- `update` owns mutation. `draw` should stay mostly read-only.
- Deletion should mark objects dead and sweep after iteration.
- Desktop heap or managed modes can exist later, but not as the first embedded path.

## Repo Notes

- `README.md` is the short orientation.
- `docs/toolchain-vocabulary.md` is the dense naming and lifecycle document.
- `notes.txt` is the scratchpad for compact design principles.
- `crates/crustini` contains the host compiler and CLI.
- `crates/crustini-core` contains the `#![no_std]` runtime surface used by generated apps.
- `packages/crustini-syntax` contains shared syntax-highlighting assets.
- `apps/site` contains the Astro website and docs tooling.
- `examples` contains source fixtures and generated test projects.
