# Crustini `.flour` v0 Spec

## 1. Core Decision

Crustini is a single-file-first creative coding language.

Its center is:

```txt
Processing immediacy with Rust-shaped structure when the sketch grows up.
```

A `.flour` file is not Rust.

A `.flour` file is parsed by the Crustini compiler, checked by the Crustini type system, lowered into generated Rust, and run by the Crustini runtime.

The v0 target is:

```txt
.flour
  -> Crustini compiler
  -> generated Rust
  -> lightweight native desktop app
```

The beginner unit is one readable `.flour` file.

No embedded target in v0.

No imports in v0.

No modules in v0.

No generics, traits, lifetimes, ownership syntax, or borrow ceremony in v0.

Rust remains the backend and the implementation language. Crustini is the language artists write.

---

## 2. Design Principle

Crustini should feel immediate to an artist.

The first successful program should be:

```flour
+++
crustini = "0.1"
name = "Starter"
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

    if x > 640 {
      x = -24;
    }

    circle(x, 180, 24, Color::White);
  }
}
```

That is the Processing-style entry point:

```txt
state plus draw
one function runs every frame
draw can animate by mutating state
visual feedback is immediate
```

As the sketch grows, it should naturally become:

```flour
app! Main {
  state {
    x: number = 40;
    vx: number = 2;
  }

  fn update() {
    x += vx;

    if x > 640 {
      x = -24;
    }
  }

  fn draw() {
    clear(Color::Black);
    circle(x, 180, 24, Color::White);
  }
}
```

That is the structured app style:

```txt
update owns animation and mutation
draw renders the current state
larger programs become easier to reason about
```

Both are valid Crustini.

---

## 3. The Three Programming Modes

Crustini v0 has one language, but three teaching modes.

### 3.1 Sketch Mode

Sketch mode is the Processing-like entry point.

Use this when the artist wants to make something move quickly.

Shape:

```flour
app! Main {
  state {
    x: number = 40;
  }

  fn draw() {
    clear(Color::Black);
    x += 1;
    rect(x, 100, 24, 24, Color::White);
  }
}
```

Rule:

```txt
If update() is omitted, draw() may mutate state.
```

This keeps the first experience small, visual, and forgiving.

### 3.2 App Mode

App mode is the recommended shape once behavior becomes more than a tiny sketch.

Shape:

```flour
app! Main {
  state {
    x: number = 40;
    speed: number = 120;
  }

  fn update() {
    x += axis_x() * speed * dt();
  }

  fn draw() {
    clear(Color::Black);
    rect(x, 100, 24, 24, Color::White);
  }
}
```

Rule:

```txt
If update() exists, mutation belongs in update().
draw() should be rendering-only except for short-lived local drawing calculations.
```

The compiler does not need to reject every mutation in draw in v0, but examples, docs, starters, and diagnostics should teach this rule.

### 3.3 Structured Mode

Structured mode is for larger sketches, toys, and games.

Use:

```txt
struct
enum
match
helper functions
normal function calls
```

Example:

```flour
enum Mode {
  Title,
  Playing,
  Dead,
}

app! Main {
  state {
    mode: Mode = Mode::Title;
    player: Vec2 = vec2(40, 40);
    speed: number = 120;
  }

  fn update() {
    match mode {
      Mode::Title => {
        if pressed(Button::A) {
          mode = Mode::Playing;
        }
      }

      Mode::Playing => {
        player = move_player(player, speed);

        if pressed(Button::B) {
          mode = Mode::Dead;
        }
      }

      Mode::Dead => {
        if pressed(Button::A) {
          mode = Mode::Title;
          player = vec2(40, 40);
        }
      }
    }
  }

  fn draw() {
    clear(Color::Black);

    match mode {
      Mode::Title => {
        draw_title();
      }

      Mode::Playing => {
        draw_player(player);
      }

      Mode::Dead => {
        draw_dead();
      }
    }
  }
}

fn move_player(pos: Vec2, speed: number) -> Vec2 {
  return pos + stick() * speed * dt();
}

fn draw_player(pos: Vec2) {
  rect(pos.x, pos.y, 24, 24, Color::White);
}

fn draw_title() {
  text(40, 40, "PRESS A", Color::White);
}

fn draw_dead() {
  text(40, 40, "DEAD", Color::Red);
}
```

This is the ideal mature v0 style.

---

## 4. Macros and Compiler Forms

Crustini can use `!` syntax without using Rust macros.

This:

```flour
app! Main {
  ...
}
```

is Crustini syntax, not a Rust macro invocation.

The Crustini parser sees:

```txt
app!
identifier
block
```

and lowers that into generated Rust.

Conceptually:

```rust
struct MainState {
    // state fields
}

impl crustini_runtime::App for MainState {
    // setup, update, draw
}
```

`app!` is worth keeping because it means:

```txt
this is not a normal function
this is the whole sketch/app definition
the compiler will lower it into app scaffolding
```

Do not use macro syntax for normal runtime work.

Good:

```flour
clear(Color::Black);
rect(x, y, 20, 20, Color::White);
pressed(Button::A);
```

For v0, the only required visible `!` form is:

```flour
app! Main {
  ...
}
```

Future compiler-level forms may use `!`, but they should be rare.

---

## 5. File Shape

A canonical `.flour` file has two parts:

```txt
optional TOML front matter
Crustini source code
```

Use plain `+++` delimiters.

Example:

```flour
+++
crustini = "0.1"
name = "Starter"
fps = 30
window = [640, 360]
+++

app! Main {
  fn draw() {
    clear(Color::Black);
    text(24, 24, "hello crustini", Color::White);
  }
}
```

No front matter title.

Do not use:

```flour
+++ config
...
+++
```

Do not use:

```flour
+++ sketch
...
+++
```

The delimiter already says this is metadata. The `crustini` key says what kind of metadata it is.

---

## 6. Front Matter Keys

Starter files should include:

```toml
crustini = "0.1"
name = "Starter"
fps = 30
window = [640, 360]
```

Required for starter files:

```toml
crustini = "0.1"
name = "Starter"
```

Optional:

```toml
fps = 30
window = [640, 360]
```

Defaults:

```txt
crustini = current compiler compatibility
name = filename
fps = 30
window = runtime default
```

Use `window`, not `size`, for v0.

Reason:

```txt
v0 target is a native desktop preview
window is honest
window is clear to beginners
```

---

## 7. Project Model

Crustini should be single-file-first.

A loose file should work:

```txt
hello.flour
```

Run:

```bash
rx hello.flour
```

A beginner starter should be able to be one file:

```txt
my-sketch/
  main.flour
```

Run from that folder:

```bash
rx
```

Existing project-shaped tooling may continue to use `recipe.flour` and `src/main.flour` while the implementation catches up. Do not silently rename code paths until the CLI supports the new shape.

The product direction is:

```txt
single-file first
front matter before separate config
project folders only when the app actually needs more files
```

---

## 8. App Structure

The app block has this shape:

```flour
app! Main {
  state {
    // persistent values
  }

  fn setup() {
    // optional, runs once
  }

  fn update() {
    // optional, runs before draw
  }

  fn draw() {
    // runs every frame
  }
}
```

`state` is optional.

`setup` is optional.

`update` is optional.

`draw` is the main visual loop.

Runtime behavior:

```txt
setup once

each frame:
  if update exists:
    update()
    draw()
  else:
    draw()
```

Teaching behavior:

```txt
start with state plus draw
graduate to update plus draw
use enum, match, structs, and helpers as the sketch grows
```

---

## 9. State

Persistent values go in `state`.

```flour
state {
  x: number = 40;
  y: number = 40;
  label: text = "score";
  alive: bool = true;
  pos: Vec2 = vec2(40, 40);
}
```

State fields are automatically available inside app functions.

Write:

```flour
x += 1;
```

Not:

```rust
self.x += 1;
```

The compiler lowers state access into generated Rust fields.

Conceptual lowering:

```flour
state {
  x: number = 40;
}

fn update() {
  x += 1;
}
```

becomes:

```rust
struct MainState {
    x: f32,
}

impl App for MainState {
    fn update(&mut self, ctx: &mut Ctx) {
        self.x += 1.0;
    }
}
```

The user never writes `self`.

---

## 10. Types

Crustini v0 has a tiny type system.

Primitive types:

```txt
number
text
bool
```

Built-in app types:

```txt
Vec2
Color
Button
Sprite
```

User-defined types:

```txt
struct
enum
```

Use `number`, not Rust's numeric surface.

Avoid in v0:

```txt
i8
u8
i16
u16
i32
u32
f32
f64
usize
```

For v0 native apps, `number` may lower to `f32`.

Later, Crustini can add integer-specific types if they become necessary.

---

## 11. Variables

Use `let` for local variables.

```flour
fn update() {
  let speed: number = 120;
  x += speed * dt();
}
```

Use `mut` only for mutable local variables.

```flour
fn update() {
  let mut total: number = 0;

  for i in 0..10 {
    total += i;
  }
}
```

State variables do not need `mut`.

This is allowed:

```flour
state {
  x: number = 40;
}

fn update() {
  x += 1;
}
```

Reason:

```txt
state variables are app state
app state is mutable by design
```

---

## 12. Functions

Use Rust-shaped functions.

```flour
fn reset() {
  x = 40;
}
```

Parameters:

```flour
fn draw_label(x: number, y: number, msg: text) {
  text(x, y, msg, Color::White);
}
```

Return values:

```flour
fn is_dead(hp: number) -> bool {
  return hp <= 0;
}
```

For v0, use explicit `return` in returning functions.

That is easier for beginners and easier for compiler diagnostics.

No methods in v0.

No `impl` blocks in v0.

---

## 13. Structs

Structs are plain data.

```flour
struct Player {
  pos: Vec2,
  speed: number,
  alive: bool,
}
```

Create a value:

```flour
state {
  player: Player = Player {
    pos: vec2(40, 40),
    speed: 120,
    alive: true,
  };
}
```

Use fields with dot syntax:

```flour
fn update() {
  player.pos += stick() * player.speed * dt();
}

fn draw() {
  rect(player.pos.x, player.pos.y, 24, 24, Color::White);
}
```

Structs are for organizing data, not for introducing object-oriented ceremony.

---

## 14. Enums and Match

Enums are the main way to model modes.

```flour
enum Mode {
  Title,
  Playing,
  Dead,
}
```

Use:

```flour
state {
  mode: Mode = Mode::Title;
}
```

Match:

```flour
match mode {
  Mode::Title => {
    text(40, 40, "PRESS A", Color::White);
  }

  Mode::Playing => {
    text(40, 40, "PLAYING", Color::Green);
  }

  Mode::Dead => {
    text(40, 40, "DEAD", Color::Red);
  }
}
```

Enums plus `match` give Crustini its modern declarative feel without adding a separate reactive system.

---

## 15. Control Flow

Use Rust-shaped blocks.

If:

```flour
if x > 100 {
  x = 0;
} else {
  x += 1;
}
```

Boolean operators:

```txt
&&
||
!
```

For loop:

```flour
for i in 0..10 {
  rect(i * 32, 40, 24, 24, Color::White);
}
```

Match:

```flour
match mode {
  Mode::Title => {
    draw_title();
  }

  Mode::Playing => {
    draw_game();
  }
}
```

---

## 16. Namespaces and `::`

Use `::` for named constants, enum variants, and generated asset names.

Examples:

```flour
Color::Black
Color::White
Color::Red

Button::A
Button::B
Button::Left
Button::Right

Sprite::Player
Sprite::Enemy

Mode::Title
Mode::Playing
Mode::Dead
```

Good:

```flour
clear(Color::Black);
rect(x, y, 20, 20, Color::White);

if pressed(Button::A) {
  mode = Mode::Playing;
}
```

Avoid:

```flour
clear(black);
pressed(A);
mode = playing;
```

The `::` syntax teaches structured names early without exposing Rust modules.

---

## 17. Drawing API

Drawing uses normal runtime functions.

```flour
clear(Color::Black);
rect(x, y, width, height, Color::White);
circle(x, y, radius, Color::Yellow);
line(x1, y1, x2, y2, Color::Red);
text(x, y, "hello", Color::White);
sprite(Sprite::Player, x, y);
```

These are not macros.

They are compiler-known builtins that lower to runtime calls.

Conceptually:

```txt
clear(color)          -> gfx.clear(color)
rect(x, y, w, h, c)   -> gfx.rect(x, y, w, h, c)
circle(x, y, r, c)    -> gfx.circle(x, y, r, c)
text(x, y, msg, c)    -> gfx.text(x, y, msg, c)
sprite(sprite, x, y)  -> gfx.sprite(sprite, x, y)
```

---

## 18. Input API

Input uses normal functions and `Button::` constants.

```flour
pressed(Button::A)
down(Button::A)
released(Button::A)

axis_x()
axis_y()
stick()
mouse_x()
mouse_y()
mouse_down()
```

Example:

```flour
fn update() {
  if down(Button::Right) {
    x += speed * dt();
  }

  if pressed(Button::A) {
    count += 1;
  }
}
```

Default native keyboard mapping:

```txt
Arrow keys -> directional buttons
Z          -> Button::A
X          -> Button::B
Enter      -> Button::Start
Shift      -> Button::Select
```

---

## 19. Time API

Use `dt()` for frame delta time.

```flour
fn update() {
  x += speed * dt();
}
```

Use frame-based mutation in tiny sketch examples when that is clearer:

```flour
fn draw() {
  x += 2;
  circle(x, 100, 24, Color::White);
}
```

Use `dt()` in structured examples and docs for movement that should be frame-rate independent.

---

## 20. Functional and Declarative Style

Do not add a separate functional subsystem in v0.

Crustini should be:

```txt
declarative in front matter
imperative in sketch/update/draw code
functional in helper functions where useful
declarative for modes through enum plus match
```

Good:

```flour
fn move_player(pos: Vec2, speed: number) -> Vec2 {
  return pos + stick() * speed * dt();
}

fn update() {
  player = move_player(player, speed);
}
```

Avoid in v0:

```txt
closures
map/filter/reduce as a teaching surface
pipes
signals
reactive computed state
component DSLs
effects systems
```

The modern feel should come from:

```txt
explicit state
pure-ish helpers
enum plus match
clear lifecycle
small vocabulary
```

---

## 21. Assets and Multiple Files

Single-file sketches need no asset keys.

When a sketch needs assets or extra code, list them in front matter.

Example:

```flour
+++
crustini = "0.1"
name = "Sprite Demo"
fps = 30
window = [640, 360]

code = [
  "player.flour",
  "ui.flour"
]

[sprites]
Player = "assets/player.png"
Enemy = "assets/enemy.png"
+++

app! Main {
  state {
    pos: Vec2 = vec2(40, 40);
  }

  fn draw() {
    clear(Color::Black);
    sprite(Sprite::Player, pos.x, pos.y);
  }
}
```

This gives source code:

```flour
Sprite::Player
Sprite::Enemy
```

No imports.

No `use`.

No module syntax.

The main file lists the extra source and asset files.

---

## 22. Canonical Starter

The canonical starter should teach sketch mode.

```flour
+++
crustini = "0.1"
name = "Starter"
fps = 30
window = [640, 360]
+++

app! Main {
  state {
    x: number = 40;
    y: number = 180;
    vx: number = 3;
  }

  fn draw() {
    clear(Color::Black);

    x += vx;

    if x < 0 || x > 640 {
      vx = -vx;
    }

    circle(x, y, 24, Color::White);
  }
}
```

It shows:

```txt
front matter
app container
state
draw loop
clear screen
draw shape
mutation
if statement
```

The next lesson should introduce `update`.

---

## 23. Canonical Structured Starter

The structured starter should teach app mode.

```flour
+++
crustini = "0.1"
name = "Mover"
fps = 30
window = [640, 360]
+++

app! Main {
  state {
    pos: Vec2 = vec2(40, 180);
    speed: number = 120;
  }

  fn update() {
    pos += stick() * speed * dt();
  }

  fn draw() {
    clear(Color::Black);
    rect(pos.x, pos.y, 24, 24, Color::White);
  }
}
```

It shows:

```txt
state
update
draw
input
delta time
rendering-only draw
```

---

## 24. Compiler Architecture

The compiler pipeline should become:

```txt
main.flour
  -> split TOML front matter from Crustini source
  -> parse TOML metadata
  -> load extra code files from code = [...]
  -> parse Crustini source files
  -> build AST
  -> resolve symbols
  -> type check
  -> lower app! into internal app representation
  -> lower builtins into runtime calls
  -> generate Rust
  -> compile native app
```

The important implementation idea:

```txt
Crustini source does not need to become Rust syntax one-to-one.
Crustini source becomes a typed AST.
The typed AST generates Rust.
```

The current implementation may stay smaller while moving in this direction.

Do not confuse surface syntax cleanup with a complete compiler rewrite. Stage the work.

---

## 25. Lowering Model

Crustini:

```flour
app! Main {
  state {
    x: number = 40;
  }

  fn draw() {
    clear(Color::Black);
    circle(x, 160, 20, Color::White);
  }
}
```

Conceptual generated Rust:

```rust
struct MainState {
    x: f32,
}

impl crustini_runtime::App for MainState {
    fn draw(&mut self, ctx: &mut Ctx, gfx: &mut Gfx) {
        gfx.clear(Color::Black);
        gfx.circle(self.x, 160.0, 20.0, Color::White);
    }
}
```

Builtins are compiler-recognized bindings:

```txt
clear(color)       -> gfx.clear(color)
rect(...)          -> gfx.rect(...)
pressed(button)    -> ctx.input.pressed(button)
dt()               -> ctx.time.dt
```

They are not Rust macros.

---

## 26. What Not To Do

Do not expose real Rust macros to learners.

Do not use `!` everywhere.

Do not make drawing and input look like macros.

Do not start beginners with package structure.

Do not add imports in v0.

Do not add modules in v0.

Do not add generics, traits, lifetimes, references, or ownership syntax in v0.

Do not make Crustini a wrapper around Rust source files.

Do not add a separate reactive or functional language inside the language.

Do not optimize the beginner surface for compiler convenience at the expense of visual immediacy.

---

## 27. What Is Figured Out

The v0 language direction is:

```txt
1. .flour is a custom language.
2. The first experience is Processing-like sketching.
3. state plus draw is valid sketch mode.
4. update plus draw is the recommended growth path.
5. app! is a Crustini compiler form, not a Rust macro.
6. Normal runtime work uses normal function calls.
7. :: is used for colors, buttons, enum variants, and assets.
8. enum plus match provide the declarative app-mode feel.
9. Helper functions provide the functional feel without a separate subsystem.
10. The compiler lowers Crustini into generated Rust.
```

This is not half-Rust and not half-DSL.

It is:

```txt
a creative coding language with Rust-shaped structure and a Rust backend
```

---

## 28. Final v0 Syntax Target

The center of Crustini is:

```flour
+++
crustini = "0.1"
name = "App Name"
fps = 30
window = [640, 360]
+++

app! Main {
  state {
    x: number = 40;
    y: number = 180;
    speed: number = 120;
  }

  fn update() {
    x += axis_x() * speed * dt();
    y += axis_y() * speed * dt();
  }

  fn draw() {
    clear(Color::Black);
    rect(x, y, 24, 24, Color::White);
  }
}
```

The first lesson may be simpler:

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

Both are Crustini.

The first is the door in.

The second is where bigger work should land.

---

## 29. One-Sentence Definition

Crustini is a single-file-first creative coding language that starts with Processing-style `draw` sketches and grows into Rust-shaped `update`/`draw` apps with explicit state, normal function calls, and generated Rust underneath.
