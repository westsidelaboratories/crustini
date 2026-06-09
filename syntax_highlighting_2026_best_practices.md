# 2026 Syntax Highlighting Best Practices

## Scope

This document defines the practical 2026 syntax-highlighting architecture for a small custom language, using Crustini as the target example. The goal is editor compatibility, documentation rendering, future language-server support, and clean implementation boundaries. The target user-facing language name is `crustini`; the recommended file extension is `.crst` unless a different extension is already locked.

## Decision

Build syntax support in four layers, in this order:

1. **TextMate grammar** for immediate VS Code/Cursor support and Shiki documentation rendering.
2. **Shiki integration** for docs, examples, Markdown, and static site rendering.
3. **Tree-sitter grammar** for structural highlighting, indentation, folding, editor intelligence, and Neovim/Helix-style support.
4. **LSP semantic tokens** only after the parser, resolver, and symbol table exist.

Do not start with a custom browser-only regex highlighter. Do not start with semantic tokens. Do not make one homemade renderer responsible for every surface.

## Source-backed facts

- VS Code uses TextMate grammars for syntax highlighting; semantic highlighting is an additive layer on top of syntax highlighting.
- TextMate grammars are lexical, regex-based, and single-file oriented.
- Shiki uses TextMate grammars and VS Code-compatible themes and can render code ahead of time with no client runtime.
- Tree-sitter highlighting uses parser output plus query capture files such as `highlights.scm`.
- CodeMirror language packages are parser-backed; its docs recommend building a language package around a parser, commonly a Lezer grammar.
- Monaco uses its own Monarch tokenizer model and is useful when a VS Code-like web IDE is required.

## Recommended repository layout

```txt
crustini-syntax/
  README.md
  package.json
  pnpm-lock.yaml | bun.lock

  grammars/
    textmate/
      crustini.tmLanguage.json
      test-fixtures/
        basic.crst
        strings-comments.crst
        functions.crst
        macros.crst
        errors.crst

    tree-sitter/
      package.json
      grammar.js
      corpus/
        basic.txt
        functions.txt
        expressions.txt
        macros.txt
        errors.txt
      queries/
        highlights.scm
        indents.scm
        folds.scm
        injections.scm

    lezer/
      src/
        crustini.grammar
        index.ts
      test/
        basic.test.ts

  integrations/
    vscode/
      package.json
      language-configuration.json
      syntaxes/
        crustini.tmLanguage.json
      snippets/
        crustini.code-snippets

    shiki/
      crustini.tmLanguage.json
      crustini-theme.json
      highlighter.ts

    codemirror/
      src/
        index.ts
        parser.ts
        highlight.ts

    monaco/
      monarch.ts
      language.ts

  examples/
    hello.crst
    sprite.crst
    tilemap.crst
    input.crst

  scripts/
    test-textmate.ts
    test-shiki.ts
    test-tree-sitter.ts
    build-vscode-extension.ts

  docs/
    syntax.md
    highlighting.md
    grammar-style.md
```

If the project is small, collapse it:

```txt
crustini/
  syntax/
    crustini.tmLanguage.json
    language-configuration.json
    tree-sitter-crustini/
    shiki.ts
  examples/
    *.crst
```

## Build order

### Phase 1: TextMate grammar

Deliverables:

```txt
crustini.tmLanguage.json
language-configuration.json
VS Code extension package.json
10-20 fixture programs
snapshot tests for token scopes
```

Purpose:

- Instant syntax color in VS Code and Cursor.
- Reusable grammar for Shiki.
- Low implementation cost.
- Good enough for docs and examples before the compiler is stable.

TextMate should handle:

```txt
comments
strings
numbers
keywords
primitive types
macro calls
function definitions
function calls
builtin calls
attributes/directives
punctuation
operators
```

TextMate should not attempt:

```txt
name resolution
scope analysis
type inference
control-flow awareness
module resolution
semantic distinction between every identifier category
```

### Phase 2: Shiki docs integration

Deliverables:

```txt
Shiki highlighter loader
registered crustini language
registered docs theme
Markdown code fence support for ```crustini
rendered static examples
```

Purpose:

- Documentation, blog posts, examples, specs, and generated HTML use the same grammar style as the editor.
- Static rendering avoids shipping a heavy runtime highlighter.
- Markdown examples become canonical test cases.

### Phase 3: Tree-sitter grammar

Deliverables:

```txt
grammar.js
corpus tests
queries/highlights.scm
queries/indents.scm
queries/folds.scm
queries/injections.scm
```

Purpose:

- Structural parsing.
- Better editor features.
- Incremental parsing.
- Syntax-aware selection, folding, indentation, and future refactors.
- Useful for Neovim, Helix, Zed-like workflows, and code intelligence tools.

### Phase 4: LSP semantic tokens

Deliverables:

```txt
language server parser bridge
symbol table
scope resolver
type resolver or primitive type classifier
semantic token provider
diagnostics
completion baseline
```

Purpose:

- Meaning-aware coloring.
- Accurate distinction between local variables, state fields, parameters, builtins, modules, functions, and types.
- Smarter editor feedback after the syntax is stable.

Do not build this first. Semantic highlighting depends on parser correctness and symbol resolution.

## Tool choice matrix

| Surface | Primary implementation | Reason |
|---|---:|---|
| VS Code | TextMate grammar | Native path for syntax tokens. |
| Cursor | TextMate grammar | VS Code extension compatibility. |
| Markdown docs | Shiki | Uses TextMate grammars and VS Code-style themes. |
| Astro/Vite/static site | Shiki | Pre-rendered HTML; no client highlighter required. |
| Neovim | Tree-sitter | Structural parser and query highlighting. |
| Helix | Tree-sitter | Structural parser and query highlighting. |
| Zed-style editor support | Tree-sitter/LSP | Structural and semantic layers. |
| Browser notebook/editor | CodeMirror 6 + Lezer | Small, embeddable, parser-backed. |
| Browser IDE | Monaco + Monarch + LSP | VS Code-like editor experience. |
| Compiler tooling | Dedicated parser, optionally shared concepts | Compiler parser should not depend on highlighter hacks. |

## TextMate grammar rules

### Language identity

```json
{
  "name": "Crustini",
  "scopeName": "source.crustini",
  "fileTypes": ["crst"],
  "patterns": [
    { "include": "#comments" },
    { "include": "#strings" },
    { "include": "#numbers" },
    { "include": "#macros" },
    { "include": "#keywords" },
    { "include": "#types" },
    { "include": "#function-definitions" },
    { "include": "#builtins" },
    { "include": "#operators" },
    { "include": "#punctuation" }
  ],
  "repository": {}
}
```

### Scope naming standard

Prefer standard TextMate scopes with a language suffix:

```txt
comment.line.number-sign.crustini
comment.line.double-slash.crustini
string.quoted.double.crustini
constant.numeric.integer.crustini
constant.numeric.float.crustini
constant.language.boolean.crustini
constant.language.none.crustini
keyword.control.crustini
keyword.declaration.crustini
keyword.operator.crustini
storage.type.primitive.crustini
entity.name.function.crustini
entity.name.macro.crustini
support.function.builtin.crustini
variable.other.crustini
variable.parameter.crustini
punctuation.definition.string.begin.crustini
punctuation.definition.string.end.crustini
punctuation.section.block.begin.crustini
punctuation.section.block.end.crustini
```

Avoid custom scopes that themes cannot understand:

```txt
crustini.magic
crustini.cool
crustini.red
crustini.special-function-but-not-always
```

### Recommended MVP token set

```txt
comments:        # ..., // ..., /* ... */ if block comments exist
strings:         "...", '...' only if single quotes are valid
numbers:         10, 10.5, 0xff, 0b1010 if supported
keywords:        app, def, let, const, mut, if, else, for, while, in, return, break, continue
booleans:        true, false
none/null:       none
primitive types: bool, u8, i8, u16, i16, u32, i32, usize, isize, f32
macros:          app!, state!, memory!, screen!, input!, draw!, setup!, sprite!, tilemap!, sound!
builtins:        bg, fill, pixel, line, rect, circle, text, sprite, btn, btnp, rand, ticks
operators:       + - * / % = == != < <= > >= && || ! += -= *= /= -> => : :: .
punctuation:     ( ) [ ] { } , ;
```

### Example TextMate repository entries

```json
{
  "comments": {
    "patterns": [
      {
        "name": "comment.line.number-sign.crustini",
        "match": "#.*$"
      },
      {
        "name": "comment.line.double-slash.crustini",
        "match": "//.*$"
      }
    ]
  },
  "strings": {
    "patterns": [
      {
        "name": "string.quoted.double.crustini",
        "begin": "\"",
        "beginCaptures": {
          "0": { "name": "punctuation.definition.string.begin.crustini" }
        },
        "end": "\"",
        "endCaptures": {
          "0": { "name": "punctuation.definition.string.end.crustini" }
        },
        "patterns": [
          {
            "name": "constant.character.escape.crustini",
            "match": "\\\\[nrt\\\"\\\\]"
          }
        ]
      }
    ]
  },
  "macros": {
    "patterns": [
      {
        "name": "entity.name.macro.crustini",
        "match": "\\b[a-zA-Z_][a-zA-Z0-9_]*!"
      }
    ]
  },
  "keywords": {
    "patterns": [
      {
        "name": "keyword.control.crustini",
        "match": "\\b(if|else|for|while|in|return|break|continue)\\b"
      },
      {
        "name": "keyword.declaration.crustini",
        "match": "\\b(app|def|let|const|mut)\\b"
      },
      {
        "name": "constant.language.boolean.crustini",
        "match": "\\b(true|false)\\b"
      },
      {
        "name": "constant.language.none.crustini",
        "match": "\\bnone\\b"
      }
    ]
  },
  "types": {
    "patterns": [
      {
        "name": "storage.type.primitive.crustini",
        "match": "\\b(bool|u8|i8|u16|i16|u32|i32|usize|isize|f32|Color|Sprite|Button|Screen)\\b"
      }
    ]
  }
}
```

## VS Code extension minimum

```json
{
  "name": "crustini-syntax",
  "displayName": "Crustini Syntax",
  "publisher": "crustini",
  "version": "0.0.1",
  "engines": {
    "vscode": "^1.90.0"
  },
  "categories": ["Programming Languages"],
  "contributes": {
    "languages": [
      {
        "id": "crustini",
        "aliases": ["Crustini", "crustini"],
        "extensions": [".crst"],
        "configuration": "./language-configuration.json"
      }
    ],
    "grammars": [
      {
        "language": "crustini",
        "scopeName": "source.crustini",
        "path": "./syntaxes/crustini.tmLanguage.json"
      }
    ]
  }
}
```

Minimum `language-configuration.json`:

```json
{
  "comments": {
    "lineComment": "#"
  },
  "brackets": [
    ["{", "}"],
    ["[", "]"],
    ["(", ")"]
  ],
  "autoClosingPairs": [
    ["{", "}"],
    ["[", "]"],
    ["(", ")"],
    ["\"", "\""]
  ],
  "surroundingPairs": [
    ["{", "}"],
    ["[", "]"],
    ["(", ")"],
    ["\"", "\""]
  ],
  "folding": {
    "markers": {
      "start": "^\\s*#\\s*region\\b",
      "end": "^\\s*#\\s*endregion\\b"
    }
  }
}
```

## Shiki integration

Recommended API shape:

```ts
import { createHighlighter } from 'shiki'
import crustiniGrammar from './crustini.tmLanguage.json' assert { type: 'json' }

export const highlighter = await createHighlighter({
  themes: ['github-dark', 'github-light'],
  langs: [
    {
      ...crustiniGrammar,
      name: 'crustini',
      aliases: ['crst']
    }
  ]
})

export function renderCrustini(code: string, theme = 'github-dark') {
  return highlighter.codeToHtml(code, {
    lang: 'crustini',
    theme
  })
}
```

Docs rule:

````md
```crustini
app!:
  screen 240, 135

  state!:
    x: i16 = 120

  def draw():
    bg(0)
    circle(x, 64, 8)
```
````

Every documented example should be valid syntax or intentionally marked invalid.

## Tree-sitter grammar target

Minimum syntax tree nodes:

```txt
source_file
comment
identifier
number
string
boolean
none
macro_call
app_block
state_block
memory_block
function_definition
parameter_list
parameter
typed_binding
block
assignment
call_expression
argument_list
member_expression
binary_expression
unary_expression
if_statement
for_statement
while_statement
return_statement
break_statement
continue_statement
type_identifier
```

`highlights.scm` baseline:

```scheme
(comment) @comment
(string) @string
(number) @number
(boolean) @boolean
(none) @constant.builtin

[
  "if"
  "else"
  "for"
  "while"
  "in"
  "return"
  "break"
  "continue"
] @keyword.control

[
  "app"
  "def"
  "let"
  "const"
  "mut"
] @keyword

(type_identifier) @type

(function_definition
  name: (identifier) @function)

(call_expression
  function: (identifier) @function.call)

(macro_call
  name: (identifier) @function.macro)

(parameter
  name: (identifier) @variable.parameter)

[
  "+"
  "-"
  "*"
  "/"
  "%"
  "="
  "=="
  "!="
  "<"
  "<="
  ">"
  ">="
  "&&"
  "||"
  "!"
  "->"
  "=>"
] @operator

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

[
  ","
  ";"
  ":"
  "."
] @punctuation.delimiter
```

`indents.scm` baseline:

```scheme
(block) @indent
(app_block) @indent
(state_block) @indent
(memory_block) @indent
(function_definition) @indent
(if_statement) @indent
(for_statement) @indent
(while_statement) @indent
```

`folds.scm` baseline:

```scheme
(block) @fold
(app_block) @fold
(state_block) @fold
(memory_block) @fold
(function_definition) @fold
```

## LSP semantic tokens

Only implement semantic highlighting when these are present:

```txt
parser
AST
symbol table
lexical scopes
module/import model if imports exist
builtin registry
type registry
state-field registry
macro registry
```

Recommended semantic token types:

```txt
namespace
class
struct
enum
type
function
method
macro
variable
parameter
property
keyword
number
string
comment
operator
```

Recommended semantic token modifiers:

```txt
declaration
definition
readonly
static
defaultLibrary
mutable
```

Crustini-specific mapping:

```txt
app!                  macro + defaultLibrary
state!                macro + defaultLibrary
memory!               macro + defaultLibrary
x in state block      property + declaration + mutable
x in draw body        property or variable depending resolver
circle                function + defaultLibrary
user-defined draw     function + declaration
u8/i16/f32            type + defaultLibrary
button enum values    enumMember + defaultLibrary if implemented
```

Do not use LSP semantic tokens to color strings, comments, or punctuation. TextMate already does that better and cheaper.

## CodeMirror 6 / Lezer path

Use this when building a custom browser editor, notebook, playground, or teaching UI.

Recommended shape:

```txt
@crustini/codemirror/
  src/
    crustini.grammar
    parser.ts
    highlight.ts
    index.ts
```

Language package should provide:

```ts
import { LRLanguage, LanguageSupport } from '@codemirror/language'
import { styleTags, tags as t } from '@lezer/highlight'
import { parser } from './parser'

export const crustiniLanguage = LRLanguage.define({
  parser: parser.configure({
    props: [
      styleTags({
        'if else for while in return break continue': t.controlKeyword,
        'def let const mut app': t.definitionKeyword,
        Boolean: t.bool,
        Number: t.number,
        String: t.string,
        LineComment: t.lineComment,
        Type: t.typeName,
        MacroName: t.macroName,
        FunctionName: t.function(t.variableName),
        Builtin: t.standard(t.function(t.variableName))
      })
    ]
  }),
  languageData: {
    commentTokens: { line: '#' },
    closeBrackets: { brackets: ['(', '[', '{', '"'] }
  }
})

export function crustini() {
  return new LanguageSupport(crustiniLanguage)
}
```

Use CodeMirror when the page needs to feel like a lightweight notebook or embedded code cell. Use Monaco only when the product needs a full IDE shell.

## Monaco / Monarch path

Use Monaco when these are true:

```txt
browser IDE
multiple files
panels
diagnostics
hover
completion
LSP bridge
VS Code-like interaction model
```

Do not use Monaco for a tiny static docs site or lightweight notebook unless the IDE feel is required.

Minimum Monarch tokenizer:

```ts
export const crustiniMonarch = {
  defaultToken: '',
  tokenPostfix: '.crustini',

  keywords: [
    'app', 'def', 'let', 'const', 'mut',
    'if', 'else', 'for', 'while', 'in',
    'return', 'break', 'continue',
    'true', 'false', 'none'
  ],

  typeKeywords: [
    'bool', 'u8', 'i8', 'u16', 'i16', 'u32', 'i32', 'usize', 'isize', 'f32',
    'Color', 'Sprite', 'Button', 'Screen'
  ],

  operators: [
    '=', '>', '<', '!', '~', '?', ':', '==', '<=', '>=', '!=', '&&', '||',
    '+', '-', '*', '/', '%', '->', '=>'
  ],

  tokenizer: {
    root: [
      [/#.*$/, 'comment'],
      [/"([^"\\]|\\.)*$/, 'string.invalid'],
      [/"/, 'string', '@string'],
      [/\b\d+(\.\d+)?\b/, 'number'],
      [/\b[a-zA-Z_][\w]*!/, 'metatag'],
      [/\b[a-zA-Z_][\w]*\b/, {
        cases: {
          '@keywords': 'keyword',
          '@typeKeywords': 'type',
          '@default': 'identifier'
        }
      }],
      [/[{}()[\]]/, '@brackets'],
      [/[+\-*/%=!<>:&|]+/, 'operator']
    ],
    string: [
      [/[^\\"]+/, 'string'],
      [/\\./, 'string.escape'],
      [/"/, 'string', '@pop']
    ]
  }
}
```

## Visual design rules

Use fewer categories than the grammar can technically emit.

Recommended visible groups:

```txt
comment
string
number
keyword
type
macro
function/builtin
variable/property
operator/punctuation neutral
```

Do not give separate colors to every builtin, every integer type, every block kind, or every punctuation class.

Correct tone:

```txt
calm
readable
high contrast
minimal category count
works in dark and light themes
```

Bad tone:

```txt
rainbow colors
over-specific builtins
block-specific novelty colors
custom scopes themes cannot map
```

## Language syntax recommendations that make highlighting easier

Use syntax that is easy to tokenize and parse:

```txt
macros end in !
primitive types are reserved words
comments have one official line-comment form
strings have one official quote form initially
blocks use either indentation or braces, not both ambiguously
function definitions have one canonical form
state declarations have one canonical form
```

Avoid syntax that hurts tooling:

```txt
context-sensitive keywords everywhere
implicit string delimiters
multiple equivalent comment forms without need
ambiguous macro/function syntax
operator overloading before parser stability
layout rules that need semantic knowledge
```

## Crustini example fixture

```crustini
# hello.crst
app!:
  screen 240, 135

  state!:
    x: i16 = 120
    y: i16 = 64
    speed: i16 = 2

  def update():
    if btn(left):
      x = x - speed
    else if btn(right):
      x = x + speed

  def draw():
    bg(0)
    fill(1)
    circle(x, y, 8)
```

Expected token behavior:

```txt
app!       entity.name.macro / @function.macro
screen     support.function.builtin or keyword depending final syntax
state!     entity.name.macro / @function.macro
x,y,speed  variable/property depending layer
i16        storage.type.primitive / @type
if,else    keyword.control / @keyword.control
btn        support.function.builtin / @function.call or defaultLibrary semantic function
left,right enum-like builtins if reserved; otherwise identifiers
draw       entity.name.function / @function
bg,fill    support.function.builtin / @function.call
numbers    constant.numeric / @number
comments   comment.line / @comment
```

## Testing requirements

### TextMate tests

Test using fixtures and expected scope snapshots.

Required cases:

```txt
comments
strings with escapes
unterminated strings
integers
floats
primitive types
macro calls
function declarations
function calls
operators
punctuation
keywords next to identifiers
identifiers that contain keyword substrings
```

Examples:

```txt
if      -> keyword.control
iffy    -> variable.other, not keyword.control
state!  -> entity.name.macro
stateful -> variable.other, not macro
u8      -> storage.type.primitive
u8x     -> variable.other, not primitive type
```

### Shiki tests

Verify:

```txt
crustini language registers
```crustini fences render
HTML output contains stable classes/spans
light and dark themes both render
no client runtime highlighter required in static docs
```

### Tree-sitter tests

Verify:

```txt
parser corpus passes
highlight query produces expected captures
indent query handles nested blocks
fold query handles app/state/memory/function blocks
invalid syntax recovers gracefully
```

### LSP tests

Verify later:

```txt
state fields resolve as properties
locals resolve as variables
parameters resolve as parameters
builtins resolve as defaultLibrary functions
undefined identifiers produce diagnostics
semantic tokens do not duplicate basic TextMate work unnecessarily
```

## CI checklist

Run on every change:

```txt
validate JSON grammar
run TextMate token snapshot tests
run Shiki render smoke test
run Tree-sitter parser corpus tests
run Tree-sitter highlight query tests
lint TypeScript integration code
build VS Code extension package
render sample docs page
```

Minimum CI commands:

```bash
bun run test:textmate
bun run test:shiki
bun run test:tree-sitter
bun run build:vscode
bun run build:docs
```

## Versioning

Use independent versions:

```txt
crustini language version
syntax package version
VS Code extension version
Tree-sitter grammar version
LSP version
```

Do not tie all versions together too tightly. Syntax tooling often needs patch releases independent of compiler changes.

Recommended compatibility policy:

```txt
0.1.x syntax package supports Crustini grammar draft 0.1
0.2.x syntax package supports Crustini grammar draft 0.2
breaking syntax changes require minor version bump before 1.0
breaking syntax changes require major version bump after 1.0
```

## Acceptance criteria

The syntax package is acceptable when:

```txt
.crst files open with correct language mode in VS Code/Cursor
basic examples highlight correctly in VS Code/Cursor
Markdown ```crustini examples render through Shiki
comments, strings, numbers, keywords, macros, types, functions, and builtins are visually distinct
identifiers containing keywords are not incorrectly colored
Tree-sitter parser can parse all canonical examples
Tree-sitter highlight query covers the same basic visible categories as TextMate
invalid syntax does not destroy highlighting for the rest of the file
there is no custom one-off highlighter in docs or editor UI
```

## Non-goals

Do not implement these in the first syntax package:

```txt
full compiler parser
complete type inference
semantic tokens
macro expansion
formatter
rename refactor
cross-file symbol resolution
package manager awareness
complex embedded-language injections
custom theme engine
```

## Final recommendation

The best 2026 implementation path is:

```txt
1. TextMate grammar first.
2. Reuse that grammar in Shiki for docs.
3. Add Tree-sitter when syntax stabilizes.
4. Add LSP semantic tokens only after parser + symbol table exist.
5. Use CodeMirror/Lezer only for the browser notebook/editor surface.
6. Use Monaco only for a full browser IDE.
```

For Crustini specifically:

```txt
Language id:    crustini
Extension:      .crst
Code fence:     ```crustini
First artifact: crustini.tmLanguage.json
Second artifact: VS Code/Cursor extension wrapper
Third artifact: Shiki docs integration
Fourth artifact: tree-sitter-crustini
Fifth artifact: crustini-lsp semantic tokens
```

This gives immediate usefulness, avoids a dead custom highlighter, keeps documentation and editor rendering consistent, and leaves a clean path to serious language tooling.

## References

- Visual Studio Code Extension API: Syntax Highlight Guide.
- Visual Studio Code Extension API: Semantic Highlight Guide.
- Shiki documentation.
- Tree-sitter documentation: Syntax Highlighting.
- CodeMirror documentation: Writing a Language Package.
- Lezer reference documentation.
- Monaco Editor documentation: Monarch tokenizer.
