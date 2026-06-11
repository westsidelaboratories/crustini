# Crustini Empty Starter

This is the clean full-project starter shape for new Crustini sketches.

The source files are `.flour` only:

```txt
source-code/
  app.flour
  interface.flour
```

`source-code/app.flour` is the file to run, proof, bake, and emit today. `source-code/interface.flour` is starter boilerplate for UI helper ideas and syntax-highlighting checks; it is not loaded by the current compiler yet.

## Run

From the repo root:

```bash
cargo run --bin rx -- proof examples/full-project-example/empty-starter/source-code/app.flour
cargo run --bin rx -- emit examples/full-project-example/empty-starter/source-code/app.flour
cargo run --bin rx -- bake examples/full-project-example/empty-starter/source-code/app.flour
env CRUSTINI_FRAME=1 cargo run --quiet --bin rx -- examples/full-project-example/empty-starter/source-code/app.flour
```

Generated output goes to:

```txt
source-code/.bakery/
```

## Zed

This folder includes `.zed/settings.json` mapping `.flour` files to Rust highlighting. Reopen this folder or reopen the `.flour` files after changing Zed settings.
