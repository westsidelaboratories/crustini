# `@crustini/syntax`

Syntax highlighting kit for Crustini source files.

The active source language direction is [`../../new-spec.md`](../../new-spec.md). `.flour` is the user-authored file extension.

Some implementation names still contain `crs` because the syntax package predates the `.flour` naming pass. Treat those as transitional package internals, not user-facing vocabulary.

## What To Highlight

The new Crustini surface should make these categories visually distinct:

- compiler form: `app! Main`
- declarations: `state`, `fn`, `struct`, `enum`
- control flow: `if`, `else`, `for`, `match`, `return`
- types: `number`, `text`, `bool`, `Vec2`, `Color`, `Button`, `Sprite`
- namespaced constants and variants: `Color::Black`, `Button::A`, `Mode::Title`
- builtin calls: `clear(...)`, `rect(...)`, `pressed(...)`, `dt()`
- strings, numbers, comments, punctuation, and operators

Example source:

```flour
+++
crustini = "0.1"
name = "Sketch"
fps = 30
window = [640, 360]
+++

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

## Recommended Source of Truth

Keep tokenizer and grammar behavior aligned:

```txt
src/crs-tokenizer.ts
grammars/crs.tmLanguage.json
```

The tokenizer powers app/docs rendering.

The TextMate grammar powers VS Code and Shiki-style renderers.

When the package internals migrate from `crs` naming, keep compatibility aliases for existing integrations.

## Plain Browser Usage

```html
<link rel="stylesheet" href="/themes/crustini-dark.css" />
<script src="/dist/crustini-highlight.js"></script>

<pre class="crs-code"><code class="language-flour">app! Main {
  fn draw() {
    clear(Color::Black);
    text(24, 24, "hello", Color::White);
  }
}</code></pre>

<script>
  CrustiniHighlight.highlightAll({ lineNumbers: true });
</script>
```

Rebuild the standalone script after changing tokenizer or browser behavior:

```sh
bun --filter @crustini/syntax build:browser
```

That writes `dist/crustini-highlight.js` and syncs the static copy used by the site live editor.

## CodeMirror Usage

```ts
import { EditorView, basicSetup } from "codemirror";
import { crsLanguage } from "@crustini/syntax/codemirror";

new EditorView({
  doc: `app! Main {
  fn draw() {
    clear(Color::Black);
  }
}`,
  extensions: [
    basicSetup,
    crsLanguage
  ],
  parent: document.querySelector("#editor")!
});
```

## VS Code Usage

Copy the `vscode-extension` folder into an extension project, then package/publish it later.

For local testing:

```txt
code --extensionDevelopmentPath ./vscode-extension
```

Files included:

```txt
vscode-extension/package.json
vscode-extension/language-configuration.json
vscode-extension/syntaxes/crs.tmLanguage.json
vscode-extension/snippets/crs.code-snippets
```

## Shiki Usage

Shiki uses TextMate grammars. Use:

```txt
grammars/crs.tmLanguage.json
```

as the Crustini grammar with scope:

```txt
source.flour
```

## CSS Classes

The standalone HTML highlighter scans `language-flour` and `language-crustini` blocks.

Theme output in:

```txt
themes/crustini-dark.css
```

Current class names are implementation API and may still use `crs` prefixes during the transition.
