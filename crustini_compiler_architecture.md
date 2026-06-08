# Crustini Compiler Architecture

> Status: stale planning note. Current source UX is `.flour` with macro-shaped syntax such as `app! { ... }`, `screen!(240, 135)`, `state! { ... }`, `update! { ... }`, and `draw! { ... }`. Current generated workspace output is `.bakery/`. This document still contains older `.crst` and command examples and should be rewritten before it is treated as implementation guidance.

## Purpose

Crustini should not try to become a full Rust/Zig competitor.

The first real goal is much smaller and cleaner:

```txt
Crustini source file
  ↓
Crustini frontend compiler
  ↓
generated no_std Rust crate
  ↓
rustc / cargo
  ↓
tiny native firmware or desktop simulator binary
```

The compiler we need is a **Crustini-to-Rust frontend**.

It should take a `.crst` file and generate boring, safe, explicit Rust code.

---

## Core Idea

Do not build a language implementation separately in every project.

Build one small compiler package that every other tool calls.

```txt
input:
  main.crst

output:
  generated Rust crate

then:
  cargo/rustc builds the firmware/app
```

This means the hard compiler backend work is outsourced to Rust.

Crustini does **not** need to build:

```txt
LLVM backend
machine-code emitter
register allocator
linker
borrow checker
optimizer
debug info generator
object file writer
```

Rust already handles those.

Crustini only needs the front half:

```txt
Crustini source
  ↓
lexer
  ↓
parser
  ↓
AST
  ↓
semantic checker
  ↓
simple typed app model
  ↓
Rust code generator
```

---

## What Matters From Zig, Rust, and MicroPython

### From Zig

Use these ideas:

```txt
simple explicit language
clear compile-time/runtime boundary
source → IR → codegen mindset
boring compiler stages
```

Do not copy at first:

```txt
ZIR
AIR
linker internals
self-hosted backends
full compiler backend complexity
```

### From Rust

Use Rust as the backend:

```txt
no_std output
static types
generated safe code
cargo/rustc as backend
embedded target ecosystem
```

Do not copy at first:

```txt
borrow checker internals
MIR architecture
trait solver
proc macro complexity
rustc query system
```

### From MicroPython

Copy the feeling, not the internals.

Use MicroPython as inspiration for:

```txt
small beginner-friendly hardware API
tiny examples
board/port layout
simple module names
fast edit/run loop
```

Do not copy first:

```txt
bytecode VM
dynamic object model
runtime GC
Python semantics
interpreter dispatch loop
```

The best Crustini direction is:

```txt
MicroPython feel
+ Processing-style app loop
+ generated no_std Rust
+ fixed memory by default
+ tiny runtime trait
+ boring generated code
```

---

## Correct Mental Model

Crustini is not:

```txt
A new Rust.
```

Crustini is:

```txt
A tiny MicroPython-feeling language that emits boring no_std Rust.
```

The user writes simple code.

The compiler generates explicit Rust.

Rust generates native machine code.

```txt
User writes cute/simple code
  ↓
Crustini compiler checks it
  ↓
Crustini generates boring safe Rust
  ↓
rustc generates tiny firmware
```

---

## First Repository Shape

Ideal eventual layout:

```txt
crustini/
  crates/
    crustini_cli/
    crustini_compiler/
    crustini_rt/
    crustini_rt_desktop/
    crustini_rt_esp32s3/
  examples/
    blink/
    ball/
    buttons/
```

But for the first crude build, keep it smaller:

```txt
crustini/
  Cargo.toml
  src/
    main.rs
    compiler/
      mod.rs
      token.rs
      lexer.rs
      ast.rs
      parser.rs
      lower.rs
      check.rs
      codegen_rust.rs
      diagnostic.rs
    runtime/
      mod.rs
  examples/
    ball.crst
  tests/
    golden/
      ball.crst
      ball.expected.rs
```

Do not split into many crates until the simple compiler works.

---

## Major Components

### Compiler

The compiler understands `.crst` syntax, checks basic rules, and emits Rust.

```txt
Compiler:
  lexer
  parser
  AST
  lowering
  checker
  Rust code generator
  diagnostics
```

### Runtime

The runtime provides the functions generated Rust code calls.

```txt
Runtime:
  screen functions
  button functions
  time functions
  fixed arrays/pools later
```

### Backend

The backend implements the runtime for a concrete target.

```txt
Backends:
  desktop simulator
  ESP32-S3
  RP2040
  WASM canvas
```

The compiler should not know how to draw pixels on an ESP32.

It should only know that:

```crustini
circle(x, y, 8)
```

becomes:

```rust
ctx.circle(self.x, self.y, 8);
```

The backend decides what `ctx.circle` actually does.

---

## Architecture Diagram

```txt
                ┌────────────────────┐
                │    main.crst        │
                └─────────┬──────────┘
                          ↓
                ┌────────────────────┐
                │ crustini compiler   │
                │ lexer/parser/check  │
                └─────────┬──────────┘
                          ↓
                ┌────────────────────┐
                │ generated Rust app  │
                │ App struct + impl   │
                └─────────┬──────────┘
                          ↓
                ┌────────────────────┐
                │ crustini_rt trait   │
                └─────────┬──────────┘
                          ↓
       ┌──────────────────┼──────────────────┐
       ↓                  ↓                  ↓
 desktop runtime     ESP32-S3 runtime    WASM runtime
```

---

## First Syntax Surface

Do not support a full language at first.

Support only this shape:

```crustini
app!:
    screen 240, 135
    fps 30

    state!:
        x: i16 = 120
        y: i16 = 67
        vx: i16 = 1

    def setup():
        bg(0)

    def draw():
        bg(12)
        fill(255)
        circle(x, y, 8)

        x += vx

        if x > 232 or x < 8:
            vx = -vx
```

Supported top-level items:

```txt
app!
screen
fps
state!
def setup()
def draw()
```

Supported statements:

```txt
function call
assignment
+=
-=
if
if/else later
while later
```

Supported expressions:

```txt
integer literals
bool literals
variable references
binary ops: + - * / % > < >= <= == != and or
unary ops: - not
function calls to known builtins
```

Supported types:

```txt
i8
i16
i32
u8
u16
u32
bool
```

Add later:

```txt
arrays
structs
fixed pools
colors
sprites
text
buttons
storage
```

Avoid at first:

```txt
classes
closures
traits
lifetimes
generics
async
dynamic dispatch
imports
GC
heap strings
dictionaries
exceptions
```

---

## Example Generated Rust

Crustini source:

```crustini
app!:
    screen 240, 135
    fps 30

    state!:
        x: i16 = 120
        y: i16 = 67
        vx: i16 = 1

    def setup():
        bg(0)

    def draw():
        bg(12)
        fill(255)
        circle(x, y, 8)

        x += vx

        if x > 232 or x < 8:
            vx = -vx
```

Generated Rust should look like this:

```rust
#![no_std]

use crustini_rt::*;

pub struct App {
    pub x: i16,
    pub y: i16,
    pub vx: i16,
}

impl App {
    pub const WIDTH: u16 = 240;
    pub const HEIGHT: u16 = 135;
    pub const FPS: u16 = 30;

    pub const fn new() -> Self {
        Self {
            x: 120,
            y: 67,
            vx: 1,
        }
    }

    pub fn setup<C: CrustiniCtx>(&mut self, ctx: &mut C) {
        ctx.bg(0);
    }

    pub fn draw<C: CrustiniCtx>(&mut self, ctx: &mut C) {
        ctx.bg(12);
        ctx.fill(255);
        ctx.circle(self.x, self.y, 8);

        self.x += self.vx;

        if self.x > 232 || self.x < 8 {
            self.vx = -self.vx;
        }
    }
}
```

This is the whole first compiler target:

```txt
Crustini syntax → safe boring Rust
```

---

## Minimal Compiler Pipeline

Use this exact pipeline:

```txt
compile_file(path)
  ↓
lex(source)
  ↓
parse(tokens)
  ↓
lower(ast)
  ↓
check(app_model)
  ↓
emit_rust(app_model)
  ↓
write generated crate
```

Main compiler API shape:

```rust
pub fn compile(source: &str, options: CompileOptions) -> CompileResult {
    let tokens = lexer::lex(source)?;
    let ast = parser::parse(tokens)?;
    let app = lower::lower(ast)?;
    checker::check(&app)?;
    let rust = codegen::emit_rust(&app)?;

    Ok(CompileResult {
        files: vec![
            GeneratedFile {
                path: "src/lib.rs".into(),
                contents: rust,
            },
        ],
        diagnostics: vec![],
    })
}
```

---

## Lexer Requirements

Because the syntax is indentation-based, the lexer needs indentation tokens.

Token types:

```txt
IDENT
NUMBER
COLON
COMMA
LPAREN
RPAREN
PLUS
MINUS
STAR
SLASH
EQ
PLUS_EQ
MINUS_EQ
NEWLINE
INDENT
DEDENT
EOF
```

Example source:

```crustini
if x > 232 or x < 8:
    vx = -vx
```

Token stream:

```txt
IF IDENT(x) GT NUMBER(232) OR IDENT(x) LT NUMBER(8) COLON NEWLINE
INDENT
IDENT(vx) EQ MINUS IDENT(vx) NEWLINE
DEDENT
EOF
```

This is enough for the first version.

---

## Parser Grammar

First grammar:

```txt
file
  app_block

app_block
  "app!" ":" NEWLINE INDENT app_item* DEDENT

app_item
  screen_decl
  fps_decl
  state_block
  func_def

screen_decl
  "screen" number "," number NEWLINE

fps_decl
  "fps" number NEWLINE

state_block
  "state!" ":" NEWLINE INDENT state_field* DEDENT

state_field
  ident ":" type "=" expr NEWLINE

func_def
  "def" ident "(" ")" ":" NEWLINE INDENT stmt* DEDENT

stmt
  call_stmt
  assign_stmt
  add_assign_stmt
  sub_assign_stmt
  if_stmt

if_stmt
  "if" expr ":" NEWLINE INDENT stmt* DEDENT

expr
  expression with precedence:
    or
    and
    equality
    comparison
    add/sub
    mul/div
    unary
    primary
```

---

## AST Shape

Use a tiny AST.

```rust
pub struct AppAst {
    pub screen: Option<ScreenAst>,
    pub fps: Option<u16>,
    pub state: Vec<StateFieldAst>,
    pub funcs: Vec<FuncAst>,
}

pub struct StateFieldAst {
    pub name: String,
    pub ty: TypeAst,
    pub init: ExprAst,
}

pub struct FuncAst {
    pub name: String,
    pub body: Vec<StmtAst>,
}

pub enum StmtAst {
    Assign {
        name: String,
        expr: ExprAst,
    },
    AddAssign {
        name: String,
        expr: ExprAst,
    },
    SubAssign {
        name: String,
        expr: ExprAst,
    },
    Call {
        name: String,
        args: Vec<ExprAst>,
    },
    If {
        cond: ExprAst,
        then_body: Vec<StmtAst>,
        else_body: Vec<StmtAst>,
    },
}

pub enum ExprAst {
    Int(i64),
    Bool(bool),
    Name(String),
    Unary {
        op: UnaryOp,
        expr: Box<ExprAst>,
    },
    Binary {
        op: BinaryOp,
        left: Box<ExprAst>,
        right: Box<ExprAst>,
    },
    Call {
        name: String,
        args: Vec<ExprAst>,
    },
}
```

Do not over-engineer this.

---

## Lowering Step

The AST is raw syntax.

Lower it into a cleaner internal app model.

Example:

```crustini
x += vx
```

Lowered meaning:

```txt
target:
  app_state_field x: i16

rhs:
  app_state_field vx: i16

operation:
  add_assign i16
```

Then codegen emits:

```rust
self.x += self.vx;
```

The lowerer should resolve names:

```txt
x       -> StateField("x")
vx      -> StateField("vx")
bg      -> Builtin("bg")
circle  -> Builtin("circle")
```

This matters because codegen should not be guessing.

---

## Checker Step

The checker should stay tiny.

It enforces:

```txt
app! exists
screen exists
fps exists
setup exists or generate empty setup
draw exists
state field names are unique
function names are unique
assignments target real state/local variables
builtin calls have correct arity
if condition is bool-ish
types are allowed
no heap types yet
no strings yet, except maybe static text later
```

Builtin signature table example:

```rust
pub struct BuiltinSig {
    pub name: &'static str,
    pub args: &'static [Type],
    pub returns: Type,
}

pub const BUILTINS: &[BuiltinSig] = &[
    BuiltinSig {
        name: "bg",
        args: &[Type::U8],
        returns: Type::Unit,
    },
    BuiltinSig {
        name: "fill",
        args: &[Type::U8],
        returns: Type::Unit,
    },
    BuiltinSig {
        name: "circle",
        args: &[Type::I16, Type::I16, Type::I16],
        returns: Type::Unit,
    },
];
```

Then this Crustini code:

```crustini
circle(x, y)
```

should produce:

```txt
circle expects 3 arguments, got 2
```

---

## Rust Codegen Rules

The Rust emitter can be simple string generation.

Mapping rules:

```txt
state field           -> self.field
builtin call          -> ctx.name(...)
local variable        -> local_name
Crustini `or`         -> Rust ||
Crustini `and`        -> Rust &&
Crustini `not`        -> Rust !
Crustini if           -> Rust if
Crustini assignment   -> Rust assignment
```

Examples:

```txt
bg(12)
  -> ctx.bg(12);

fill(255)
  -> ctx.fill(255);

circle(x, y, 8)
  -> ctx.circle(self.x, self.y, 8);

if x > 232 or x < 8:
  -> if self.x > 232 || self.x < 8 { ... }
```

Keep the generated Rust boring.

---

## Runtime Trait

The generated Rust should depend on one tiny trait.

```rust
#![no_std]

pub trait CrustiniCtx {
    fn bg(&mut self, color: u8);
    fn fill(&mut self, color: u8);
    fn pixel(&mut self, x: i16, y: i16);
    fn line(&mut self, x0: i16, y0: i16, x1: i16, y1: i16);
    fn rect(&mut self, x: i16, y: i16, w: i16, h: i16);
    fn circle(&mut self, x: i16, y: i16, r: i16);

    fn btn_a(&self) -> bool;
    fn btn_b(&self) -> bool;
    fn btn_up(&self) -> bool;
    fn btn_down(&self) -> bool;
    fn btn_left(&self) -> bool;
    fn btn_right(&self) -> bool;
}
```

Generated app code only knows this trait.

Backends implement it:

```txt
desktop backend -> pixels in a desktop window
ESP32-S3 backend -> TFT SPI calls
WASM backend -> canvas calls
```

Same Crustini app.

Different runtime backend.

---

## Generated Project Shape

The compiler should generate:

```txt
build/
  generated/
    Cargo.toml
    src/
      lib.rs
      main.rs maybe
```

Desktop flow:

```txt
main.crst
  ↓
rx build --target desktop
  ↓
generated desktop Rust crate
  ↓
cargo run
```

Embedded flow later:

```txt
main.crst
  ↓
rx build --target esp32s3
  ↓
generated no_std Rust crate
  ↓
cargo build --target xtensa-esp32s3-none-elf
```

Do not worry about ESP32 first.

First compile to desktop Rust so the output can be seen and tested.

---

## CLI Commands

First commands:

```bash
rx check examples/ball.crst
rx emit examples/ball.crst --out build/generated
rx run examples/ball.crst
```

Meaning:

```txt
check:
  parse and type-check only

emit:
  generate Rust files

run:
  generate Rust files and run desktop backend
```

Later commands:

```bash
rx build examples/ball.crst --target esp32s3
crustini flash examples/ball.crst --target esp32s3
```

Do not build the later commands first.

---

## Clean Compiler Contract

Every tool should use this contract:

```txt
Input:
  source text
  filename
  target name

Output:
  diagnostics
  generated files
  metadata
```

Metadata example:

```rust
pub struct AppMetadata {
    pub width: u16,
    pub height: u16,
    pub fps: u16,
    pub state_fields: Vec<StateFieldMeta>,
    pub used_builtins: Vec<String>,
}
```

This metadata helps:

```txt
editor tooling
docs
emulator
build system
syntax viewer
project website
```

---

## How Other Tools Plug In

The compiler package should be the center.

```txt
Docs project:
  imports examples/*.crst
  renders docs

Syntax highlighter:
  uses same token definitions if possible

Desktop emulator:
  runs generated Rust with desktop runtime

ESP32 backend:
  implements CrustiniCtx

Examples repo:
  stores .crst examples

Website:
  shows source + generated Rust side-by-side
```

The other projects should not invent their own parser.

Everything should call:

```txt
crustini compile
```

or use the compiler library directly.

---

## Memory Model

First Crustini should avoid heap allocation by default.

Use:

```txt
default:
  static stack/state only

optional:
  fixed arrays

later:
  pools

much later:
  arena or tiny GC
```

Example Crustini later:

```crustini
state!:
    particles: Particle[64]
    count: u8 = 0
```

Generated Rust:

```rust
pub struct App {
    particles: [Particle; 64],
    count: u8,
}
```

This is ideal for microcontrollers.

No heap.

No GC.

No allocator.

No surprise.

---

## GC-Like Ease Without Real GC

Instead of real garbage collection, use fixed pools and handles.

Crustini syntax idea:

```crustini
memory!:
    pool particles: Particle[64]
```

Runtime shape:

```rust
pub struct Pool<T, const N: usize> {
    items: [MaybeUninit<T>; N],
    used: [bool; N],
}
```

User sees:

```crustini
p = particles.spawn()
p.x = 10
p.y = 20
```

Compiler/runtime sees:

```rust
let p = self.particles.spawn();
self.particles[p].x = 10;
self.particles[p].y = 20;
```

This gives a garbage-collection feeling without a garbage collector.

For embedded, this is probably better.

---

## Rust Borrowing Rule For Generated Code

Generated Rust should avoid complex borrowing.

Use this rule:

```txt
All user state lives in one App struct.
All builtins are methods on ctx.
All user variables become locals or self.field.
Never generate long-lived references.
Never expose Rust lifetimes to the user.
```

Good generated Rust:

```rust
self.x += self.vx;

if self.x > 232 || self.x < 8 {
    self.vx = -self.vx;
}
```

Avoid clever code like:

```rust
let a = &mut self.x;
let b = &mut self.x;
*a += *b;
```

Rust will reject that, and Crustini should not generate it.

No references.

No borrowing tricks.

No async.

No closures.

---

## Golden Tests

A golden test compares generated Rust against an expected file.

Example files:

```txt
input:
  examples/ball.crst

expected:
  tests/golden/ball.expected.rs
```

Test shape:

```rust
#[test]
fn ball_codegen_matches() {
    let src = std::fs::read_to_string("examples/ball.crst").unwrap();
    let generated = crustini_compiler::compile_to_rust(&src).unwrap();
    let expected = std::fs::read_to_string("tests/golden/ball.expected.rs").unwrap();

    assert_eq!(normalize(&generated), normalize(&expected));
}
```

This prevents the compiler from randomly changing output.

---

## First Real Milestone

The first real milestone:

```bash
rx emit examples/ball.crst --out build/ball
```

It generates:

```txt
build/ball/
  Cargo.toml
  src/lib.rs
```

Then:

```bash
cargo check
```

passes.

That is the first victory.

Not flashing ESP32.

Not a full game engine.

Not syntax highlighting.

Just:

```txt
.crst → Rust that compiles
```

---

## Best First Implementation Order

Do this in order:

```txt
1. Hardcode one example .crst file.
2. Write lexer.
3. Print tokens.
4. Write parser.
5. Print AST.
6. Write Rust codegen.
7. Generate src/lib.rs.
8. Run cargo check on generated output.
9. Add checker errors.
10. Add golden tests.
```

Do not start with the perfect grammar.

Do not start with embedded.

Do not start with autocomplete.

---

## Things To Avoid Right Now

Avoid all of this:

```txt
custom IR too early
LLVM
bytecode VM
GC
module system
package manager
generic types
classes
traits
lifetimes
async
full Rust interop
full Python syntax
smart optimizer
incremental compiler
```

Every one of those makes the project much larger.

The first compiler should be almost embarrassingly small.

---

## Practical Definition Of Version 0

Crustini v0 should include:

```txt
.crst file
app! root
state!
setup()
draw()
basic arithmetic
if/else
fixed-size primitive state fields
screen/button builtins
Rust codegen
desktop simulator backend
```

Crustini v1 can add:

```txt
ESP32-S3 backend
RP2040 backend
asset bundling
fixed pools
simple modules
better errors
```

Crustini v2 can maybe add:

```txt
interpreter/bytecode mode
live reload
optional alloc
tiny GC/arena
```

---

## Final Summary

The first deliverable should be:

```txt
A tiny Rust CLI that compiles:

examples/ball.crst

into:

build/generated/src/lib.rs
```

Everything else should plug into that center.

The best project shape is:

```txt
Crustini syntax
  ↓
Crustini frontend compiler
  ↓
generated no_std Rust
  ↓
crustini_rt trait
  ↓
backend implementation
```

The correct first milestone is not a perfect language.

It is:

```txt
.crst → generated Rust → cargo check passes
```

Once that works, the rest of the ecosystem has a stable foundation.
