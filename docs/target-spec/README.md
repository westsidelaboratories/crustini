# Target Spec Experiment

This folder is for early design work around how a Crustini project says what it is being built for.

The goal is not to make users pick a known board first. A `.flour` app should start from the shape of the thing it needs:

- screen size
- later: color format
- later: buttons
- later: memory limits
- later: frame timing

Crustini can map that shape to real hardware, generated Rust, desktop previews, or teaching tools later.

## Problem

Current `recipe.flour` can name a board:

```flour
target!:
  board desktop_sim
```

That is too specific as the main beginner-facing model. For tiny games, education, and unknown microcontrollers, users usually know the limits before they know the exact board:

```txt
I have a 160 x 90 screen.
I have a few buttons.
I probably do not have much memory.
```

The target spec should let users say that directly.

## First Capability: Screen Size

For now, the target spec only needs screen size.

Current source-level syntax already supports this:

```flour
app! {
  screen!(160, 90)
  fps!(30)

  draw! {
    clear!(0)
    text!(12, 12, "HELLO", 255)
  }
}
```

Current project-level syntax already supports this:

```flour
screen!:
  size 160, 90
  fps 30
```

The experiment is about making this feel like the central target description, not an implementation detail under a board.

## Proposed Beginner Shape

For a single-file education app, the simplest working form should stay one file:

```flour
app! {
  screen!(160, 90)

  draw! {
    clear!(0)
    text!(12, 12, "CRUSTINI", 255)
  }
}
```

Expected command:

```bash
rx app.flour
```

Expected behavior:

1. Crustini proofs the source.
2. Crustini bakes generated Rust into the bakery.
3. A preview window opens immediately.
4. The window uses the declared screen size.

This is the Processing-style path: write code, run it, see a window.

## Proposed Project Shape

For project-shaped apps, keep `recipe.flour`, but make the screen declaration the visible first target capability:

```flour
project!:
  name "tiny-demo"

screen!:
  size 160, 90
```

Defaults:

```txt
fps: 30
preview: window
generated: .crustini/generated
```

This keeps the first project config small. Users can add more constraints only when they need them.

Preview window settings are tooling config, not `.flour` language features:

```flour
preview!:
  title "Tiny Demo"
  scale auto
```

Accepted preview scales are `auto`, `1x`, `2x`, `4x`, and `8x`.

## Later Capabilities

These are intentionally out of scope for the first pass, but the syntax should leave room for them:

```flour
screen!:
  size 160, 90
  color mono

buttons!:
  left
  right
  a

memory!:
  scratch 16.kb
  heap off
```

These describe constraints, not brands or boards.

## Education Mode

Education mode should optimize for immediacy:

```bash
rx app.flour
```

should open a desktop preview window with no project setup. The current preview host writes a `frame.ppm`; that is useful for tests, but it is not the beginner experience.

Desired preview behavior:

- A window opens by default for `rx app.flour`.
- The window title can be the file or project name.
- The logical screen size comes from `screen!(W, H)` or `screen!: size W, H`.
- The window can scale pixels up for visibility, but the app still thinks in logical pixels.
- `rx bake app.flour` remains the explicit artifact path.
- Headless frame output can stay available for tests and CI.

## Naming Direction

Use plain words:

- `screen size`, not display bus details.
- `buttons`, not GPIO pins.
- `memory`, not allocator ceremony.
- `preview window`, not simulator unless we are describing internals.

Avoid making beginners choose from board names before they have learned the shape of their app.

## Open Questions

- Should `fps` be part of `screen!` or a separate timing block long term?
- Should a future target block be named `target!`, `device!`, or stay as simple capability blocks?
- Should `rx app.flour` always open a window, or should it print a frame path in headless environments?
- How should scaling be chosen for very small screens?
- Which window crate should the preview host use?
