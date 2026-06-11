# New One-File Examples

These examples are the new beginner-facing shape for Crustini:

```txt
one-file.flour
```

Each file uses TOML front matter plus `app! Main` with normal lifecycle functions.

Run checks from the repo root:

```bash
cargo run --bin rx -- proof examples/new-examples/sketch-pulse.flour
cargo run --bin rx -- emit examples/new-examples/sketch-pulse.flour
cargo run --bin rx -- bake examples/new-examples/sketch-pulse.flour
env CRUSTINI_FRAME=1 cargo run --quiet --bin rx -- examples/new-examples/sketch-pulse.flour
```

For named standalone files, `rx bake` writes each generated crate under its own output directory:

```txt
examples/new-examples/.bakery/sketch-pulse/
examples/new-examples/.bakery/keyboard-mover/
examples/new-examples/.bakery/mouse-follow/
```
