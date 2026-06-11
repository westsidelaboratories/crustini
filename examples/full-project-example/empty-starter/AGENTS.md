# AGENTS.md

Guidance for agents working inside this starter.

## Source Rules

- Use `.flour` only.
- Keep `source-code/app.flour` runnable by the current compiler.
- Use TOML front matter with `+++`.
- Use `app! Main { ... }`, `state { ... }`, and normal `fn update()` / `fn draw()` blocks.
- Prefer normal calls like `clear(Color::Black)` and `pressed(Button::A)`, not old drawing macros.

## Current Compiler Boundary

The current compiler runs one source file at a time. Keep `app.flour` self-contained until multi-file loading is implemented.

`interface.flour` can hold future helper/UI patterns and syntax-highlighting test code, but it should not be required for `app.flour` to compile.
