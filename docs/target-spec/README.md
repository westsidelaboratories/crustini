# Target Spec Notes

The active language direction is [`../../new-spec.md`](../../new-spec.md).

For v0, beginners should not choose a board or device first. They should describe the sketch they want to run.

The first target surface is TOML front matter:

```flour
+++
crustini = "0.1"
name = "Tiny Demo"
fps = 30
window = [640, 360]
+++

app! Main {
  fn draw() {
    clear(Color::Black);
    text(24, 24, "hello", Color::White);
  }
}
```

## Product Direction

Use front matter for beginner-visible app metadata:

```toml
crustini = "0.1"
name = "Tiny Demo"
fps = 30
window = [640, 360]
```

`window` is the right v0 word because the v0 target is a native desktop preview.

Later target capabilities may include:

```txt
color format
buttons
memory limits
frame timing
asset constraints
hardware boards
```

Do not make beginners pick from board names before they have learned the shape of their app.

## Compatibility Note

Current project-shaped compatibility mode can still carry preview and generation settings outside the source file. Document that only as current implementation behavior, not as the beginner-facing target model.

## Desired Preview Behavior

```bash
rx app.flour
```

should:

1. proof the source,
2. bake generated Rust into a bakery,
3. open a native preview window,
4. use the `window` and `fps` values from front matter.

Headless frame output can stay available for tests and CI.

## Naming Direction

Use plain words:

- `window`, not display bus details, for v0 desktop preview size.
- `buttons`, not GPIO pins.
- `memory`, not allocator ceremony.
- `preview window`, not simulator unless describing internals.

## Open Questions

- How should future embedded targets extend front matter without making the beginner path heavier?
- Should hardware-specific capabilities live in front matter, a sidecar config, or both?
- How should scaling be chosen for very small logical windows?
- Which future window and input APIs should become stable builtins?
