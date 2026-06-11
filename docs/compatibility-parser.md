# Compatibility Parser Layout

The active language direction is [`../new-spec.md`](../new-spec.md).

New product-facing docs and examples should use the new spec syntax:

```flour
app! Main {
  state {
    x: number = 40;
  }

  fn draw() {
    clear(Color::Black);
    x += 2;
    circle(x, 180, 24, Color::White);
  }
}
```

This document describes current compiler compatibility internals only.

## Current Implementation Boundary

The current compiler still has a small compatibility parser and registry in `crates/crustini-lang`.

That layout is an implementation detail. It should not define the product language.

The parser should answer:

```txt
What did the user write?
```

The emitter should answer:

```txt
What Rust should this app model produce?
```

Do not grow new product language by adding more compatibility forms. The direction is a typed AST pipeline that recognizes `app! Main` as a compiler-level form and treats drawing, input, time, and helpers as normal calls.

## Rule For New Work

- Product-facing syntax follows [`../new-spec.md`](../new-spec.md).
- Compatibility registries may remain while existing examples still compile.
- New docs should not teach compatibility syntax.
- New compiler work should move toward named AST nodes, builtin bindings, and normal function-call lowering.

## When To Add Implementation Files

Add or split implementation files when there is real behavior:

- custom validation
- non-trivial lowering
- multiple generated calls
- asset loading
- diagnostics that need more than an arity check

Do not create new user-facing syntax merely because it is convenient for the current scanner.
