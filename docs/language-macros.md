# Language Macro Layout

Crustini macro behavior lives in `crates/crustini-lang/src/macros/`.

The compiler is still intentionally small, so macros are grouped by family instead of giving every tiny macro its own file.

```txt
crates/crustini-lang/src/
  macros/
    app.rs      registry for screen!, fps!
    block.rs    registry for state!, setup!, update!, draw!
    screen.rs   registry for clear!, set!, pixel!, rect!, circle!, line!, text!
    input.rs    registry for up/down/a/b/mouse aliases
  scan.rs       turns source text into the app model
  model.rs      app/state/statement structs
  emit_rust.rs  turns the app model into generated Rust
```

## Rule

The macro vocabulary belongs in `macros/`.

The parser should answer:

```txt
What did the user write?
```

The emitter should answer:

```txt
What Rust should this app model produce?
```

Neither `scan.rs` nor `emit_rust.rs` should grow independent string tables for macro names.

Each family module exposes a small registry:

```rust
APP_MACROS
BLOCK_MACROS
SCREEN_MACROS
INPUT_NAMES
```

Those registries are the source of truth for names, aliases, and simple lowering metadata. For example, `pixel!` is registered in `screen.rs` as a screen macro that lowers to the Rust `Screen::set` method.

Registry tests live in `macros/mod.rs` and check that names and aliases do not silently drift.

## When To Add A File

Keep grouped families for tiny macros:

- `screen.rs` can hold all simple drawing calls while they map directly to `Screen` methods.
- `input.rs` can hold input aliases while they map directly to `Buttons` fields.
- `app.rs` can hold app-level configuration macros while there are only a few.

Split a macro into its own file only when it has real behavior:

- custom validation
- non-trivial lowering
- multiple generated calls
- asset loading
- diagnostics that need more than an arity check

That keeps the language clean without creating a folder full of one-line files.
