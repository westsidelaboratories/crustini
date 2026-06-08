# AGENTS.md

Guidance for agents working in this repository.

## Project Direction

Crustini is moving toward a baking-themed toolchain vocabulary. Use this vocabulary in new notes, docs, UI copy, examples, and planning unless you are describing current implementation details that still use older names.

| Term | Meaning | Current implementation note |
| --- | --- | --- |
| `.flour` | Crustini source code file. This is the source language users write. | The compiler and examples currently use `.crst`; docs should call that transitional. |
| `bake` | Compile/build action. Turns source into generated Rust and then an artifact. | Current CLI command is `crustini bake`; `build` is a compatibility alias. |
| `oven` | Build cache, generated runtime target directory, and intermediate workspace. | Current generated directory is `.oven/`. |
| `loaf` | Final binary, library, firmware image, or other shipped artifact. | Current examples produce a Cargo-built generated Rust crate. |
| `crumbs` | Diagnostics, logs, compiler notes, warnings, errors, traces, and build metadata. | Current errors are mostly plain strings and Cargo output. |
| `starter` | Project template or scaffolded starting point. | Current examples are under `examples/`. |

## Documentation Rules

- Prefer `.flour`, `bake`, `oven`, `loaf`, `crumbs`, and `starter` for product-facing naming.
- When documenting exact commands that exist today, prefer `crustini bake` and mention `build` only as a compatibility alias.
- Do not silently rename code paths in docs if the code still expects the old name.
- Keep the distinction clear between source language, compiler implementation, generated Rust, runtime crate, and final artifact.
- Add new language/tooling design notes under `docs/` and link them from `README.md`.

## Technical Constraints

- The compiler is intentionally small for now: section scanner, tiny line parser, Rust string emitter.
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
