# Crustini Naming Guide

This is the working naming guide for Crustini docs, CLI output, examples, and agent notes.

The active language direction is [`../new-spec.md`](../new-spec.md).

Keep the vocabulary small:

```txt
.flour      source file
rx          command
main.flour  default one-file sketch
proof       validate source
bake        compile/build action
bakery      generated build workspace/cache
starter     scaffolded starting point
```

Use plain technical words for everything else: artifact, diagnostics, logs, generated Rust, runtime, target, cache, manifest.

## `.flour`

`.flour` is the Crustini source file extension.

A `.flour` file is user-authored app source. It is the small creative coding language described in [`../new-spec.md`](../new-spec.md), lowered into generated Rust by Crustini.

Current compatibility note: `.crst` remains accepted as an old source extension, but repo examples and new docs should use `.flour`.

## `rx`

`rx` is the Crustini command.

The default command previews a `.flour` app:

```bash
rx app.flour
```

Use explicit verbs when you want a specific phase:

```bash
rx run app.flour
rx proof app.flour
rx bake app.flour
rx emit app.flour
rx starter hello my-hello
rx starters
```

## `main.flour`

`main.flour` is the product-direction default for a one-file sketch folder:

```txt
my-sketch/
  main.flour
```

The v0 beginner path should prefer one readable `.flour` file with TOML front matter.

## `recipe.flour`

`recipe.flour` is current compatibility project config for project-shaped apps.

Use it only when describing implementation paths that exist today:

```txt
my-app/
  recipe.flour
  src/main.flour
  assets/
```

Do not make `recipe.flour` the beginner-facing product direction in new docs.

## `proof`

`proof` checks source without writing a bakery or building an artifact.

```bash
rx proof app.flour
```

`check` remains a plain-word compatibility alias.

## `bake`

`bake` is the compile/build command.

```bash
rx bake app.flour
```

A bake currently reads source, emits generated Rust, writes a bakery, and invokes Cargo unless told not to.

`build` remains a compatibility alias for `bake`.

## `bakery`

`bakery` is the generated build workspace/cache.

The current single-file directory is:

```txt
.bakery/
```

It contains generated Rust, generated manifests, lockfiles, Cargo output, and other intermediate build files. It is not the shipped app; it is the workspace Crustini uses to turn source into a normal artifact.

Why not `oven`: Oven is already strongly associated with Bun's company/tooling, so Crustini should not use it.

## `starter`

`starter` is a scaffolded starting point.

Current starters still use compatibility project layout in some cases. Product-facing starter examples should follow the new spec when possible:

```txt
my-sketch/
  main.flour
```

## Current Mapping

| Concept | Current implementation |
| --- | --- |
| Source | `.flour`; `.crst` still works as an old alias |
| Command | `rx` |
| Default beginner file | `main.flour` product direction |
| Compatibility project config | `recipe.flour` |
| Check command | `rx proof`; `check` still works |
| Build command | `rx bake`; `build` still works |
| Generated workspace | `.bakery/` |
| Project generated workspace | `.crustini/generated/` |
| Generated Rust | bakery `src/lib.rs` |
| Artifact | Cargo output under the generated workspace |
| Starter | `rx starter hello <dir>` |

## Writing Rules

- Treat [`../new-spec.md`](../new-spec.md) as the language source of truth.
- Use `.flour`, `rx`, `main.flour`, `proof`, `bake`, `bakery`, and `starter`.
- Mention `recipe.flour` only for current compatibility behavior.
- Do not use `oven`, `loaf`, or `crumbs` as product terms.
- Say `artifact`, not `loaf`.
- Say `diagnostics` or `logs`, not `crumbs`.
- Say `generated Rust` when that is what you mean.
- Keep command examples real and prefer `.flour`.

Recommended short description:

```txt
Crustini turns `.flour` source into generated Rust, then bakes it into a native preview or artifact.
```
