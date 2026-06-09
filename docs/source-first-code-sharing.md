# Source-First Code Sharing

Crustini should not grow a `node_modules`-shaped dependency model.

The default should be visible source, not blind package installation. A project should stay understandable by reading the files that are checked in with it.

## Principle

Reusable code should enter a Crustini project as source the user can inspect, edit, proof, and bake. Tooling can make that convenient, but it should not hide the code behind an opaque dependency folder or pull a large transitive tree during normal app commands.

This applies to the user-facing `.flour` app model. The compiler implementation and generated Rust can keep using ordinary Rust tooling while Crustini is small, but the product direction should avoid making app authors manage invisible package installs.

## Default Rules

- `rx proof`, `rx app.flour`, and `rx bake` should not download new app code as a normal side effect.
- A starter should write real files into the project, not point at remote template code.
- Shared `.flour` code should be checked into the app or starter in readable form.
- Third-party code should carry clear provenance: source URL, version or commit, license, and any local edits.
- If generated Rust needs backend support, that detail should stay separate from the user-facing source-sharing model.

## Desired Workflow

The future command shape should feel closer to importing a reviewed source bundle than installing a package:

```bash
rx inspect https://example.com/tiny-sprites.flour
rx add https://example.com/tiny-sprites.flour
```

Expected behavior:

1. `rx inspect` shows the source files, license, provenance, and any generated manifest before anything changes.
2. `rx add` writes readable files into the project.
3. The added files can be edited locally.
4. `rx proof` validates those files like any other `.flour` source.
5. `rx bake` uses only the source already present in the project and the normal bakery.

For multi-file source bundles, the same rule applies: the command may be convenient, but the result should be visible checked-in source plus a small provenance record.

## Non-Goals

- No hidden package tree for app source.
- No automatic transitive app-code downloads during `proof`, `run`, or `bake`.
- No lockfile-only understanding of what code is in the app.
- No beginner workflow that requires trusting a remote registry before reading source.

## Open Questions

- Where should copied shared source live: `src/`, `shared/`, or another plain directory?
- Should provenance live in `recipe.flour`, beside the copied files, or both?
- How should `rx add` present diffs when updating copied source?
- Should Crustini allow remote source only when it can show the exact fetched files and hashes first?
