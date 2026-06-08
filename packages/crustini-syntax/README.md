# `@crustini/syntax`

Permanent syntax highlighting kit for Crustini source files.

Crustini source uses `.flour` for user-authored files. This package still has `.crs` names in tokenizer classes and TextMate scopes because those are current implementation names. Treat `.crs` as transitional syntax-package naming and `.flour` as the source file UX.

This is intentionally small and dependency-light. It gives you the same visual language in every place you care about:

- standalone browser rendering
- docs pages / Astro / Svelte / raw HTML
- CodeMirror editor integration
- TextMate grammar for VS Code and Shiki
- Prism integration
- Highlight.js integration

## Recommended source of truth

The source of truth should be:

```txt
src/crs-tokenizer.ts
grammars/crs.tmLanguage.json
```

The tokenizer powers your own app/docs renderer.

The TextMate grammar powers VS Code and Shiki-style renderers.

## Plain browser usage

```html
<link rel="stylesheet" href="/themes/crustini-dark.css" />
<script src="/dist/crustini-highlight.js"></script>

<pre class="crs-code"><code class="language-crs">app! {
  screen!(240, 135)
  fps!(30)

  state! {
    count: i32 = 0
  }

  draw! {
    clear!(0)
    rect!(8, 8, 80, 24, 12)
  }
}</code></pre>

<script>
  CrustiniHighlight.highlightAll({ lineNumbers: true });
</script>
```

## CodeMirror usage

```ts
import { EditorView, basicSetup } from "codemirror";
import { crsLanguage } from "@crustini/syntax/codemirror";

new EditorView({
  doc: `app! {
  screen!(240, 135)
  fps!(30)
}`,
  extensions: [
    basicSetup,
    crsLanguage
  ],
  parent: document.querySelector("#editor")!
});
```

## VS Code usage

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

## Shiki usage

Shiki uses TextMate grammars. Use:

```txt
grammars/crs.tmLanguage.json
```

as the Crustini grammar with scope:

```txt
source.crs
```

When the `.flour` migration reaches syntax tooling, add `source.flour` while keeping `source.crs` as a compatibility alias for existing integrations.

## Syntax classes

The standalone HTML highlighter emits:

```txt
.crs-macro
.crs-declaration
.crs-keyword
.crs-type
.crs-constant
.crs-builtin
.crs-property
.crs-string
.crs-number
.crs-comment
.crs-punctuation
.crs-operator
```

Theme them in:

```txt
themes/crustini-dark.css
```

## Strategy

Do not make syntax highlighting depend on the Crustini compiler yet.

Keep the highlighter forgiving and lexical:

```txt
syntax highlighter = fast lexical tokenizer
compiler/parser    = real semantic parser
```

Later, once the `.flour` grammar hardens, replace the CodeMirror stream mode with a real Lezer grammar and keep compatibility aliases for existing `.crs` integrations.
