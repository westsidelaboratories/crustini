# Crustini Config File Plan

## Decision

Crustini should use:

```txt
recipe.flour
```

as the project config file.

The user-facing config should **not** be Rust structs, TOML, YAML, JSON, or Cargo-style metadata. It should be a tiny restricted Crustini-style file that feels like part of the language.

The clean project shape should be:

```txt
hood-wars/
  recipe.flour
  src/
    main.flour
    player.flour
    map.flour
  assets/
    sprites.png
    world.map
    tiny.font
  .crustini/
    generated/
      Cargo.toml
      src/
        main.rs
```

The beginner should mostly see:

```txt
recipe.flour
src/main.flour
assets/
```

The Rust backend should exist, but it should be generated and hidden inside `.crustini/generated`.

---

## Core Rule

```txt
Crustini user files use .flour.
Rust output uses .rs but is generated.
Project config is recipe.flour.
Tiny apps can skip recipe.flour and put app config at the top of main.flour.
TOML only appears in generated Cargo.toml.
```

The build command should be:

```sh
crustini bake
```

The command name should stay simple and themed:

```txt
write code  -> .flour files
build app   -> bake
output      -> generated Rust
```

---

## Why `.flour` Is the Right Public Format

`.flour` fits the Crustini baking metaphor without making the system feel like a joke.

Good names:

```txt
main.flour
recipe.flour
assets/
crustini bake
```

Bad names for the user-facing config:

```txt
crustini.toml
project.rs
config.rs
crustini.json
bake.yaml
```

The goal is not to make the most industry-standard config file. The goal is to make the first version feel obvious to a brand-new programmer.

A new programmer should be able to open the project and think:

```txt
recipe.flour tells Crustini what this project is.
main.flour is where the app starts.
assets/ holds images, maps, and fonts.
crustini bake builds the thing.
```

That is enough.

---

## `recipe.flour` Should Use Restricted Crustini Syntax

`recipe.flour` should look like Crustini, but it should not allow normal code.

Allowed:

```txt
project!:
target!:
screen!:
memory!:
buttons!:
assets!:
build!:
```

Not allowed:

```txt
functions
loops
conditionals
variables
runtime logic
imports
macros besides known config blocks
arbitrary expressions
```

This makes the parser small and keeps the file beginner-safe.

The config language should be declarative only.

Meaning:

```txt
The file describes the project.
The file does not run.
The file does not compute.
The file does not contain app logic.
```

---

## Example `recipe.flour`

```crustini
project!:
    name "hood-wars"
    version "0.1.0"

target!:
    board esp32_s3_reverse_tft

screen!:
    size 240, 135
    fps 30

memory!:
    mode static
    scratch 16.kb
    heap off

buttons!:
    up
    down
    left
    right
    a
    b
    start

assets!:
    image "assets/sprites.png"
    map "assets/world.map"
    font "assets/tiny.font"

build!:
    optimize size
    panic abort
    generated ".crustini/generated"
```

This should be the ideal full version.

It reads like plain instructions:

```txt
name this project hood-wars
use the esp32-s3 reverse tft target
use a 240x135 screen
run at 30 fps
use static memory
load these assets
optimize for size
generate Rust into .crustini/generated
```

---

## Even Simpler First Version

The first version does not need a separate `recipe.flour`.

For tiny apps, allow the config to live at the top of `src/main.flour`.

```crustini
app!:
    name "hood-wars"
    target esp32_s3_reverse_tft
    screen 240, 135
    fps 30

    memory!:
        mode static
        scratch 16.kb
        heap off

state!:
    x = 120
    y = 67

start:
    bg 0

draw:
    bg 12
    sprite "player", x, y
```

This is the best first-build experience because there is only one file:

```txt
src/main.flour
```

Then later, when a project grows, the config can move into:

```txt
recipe.flour
```

The compiler should support both.

Priority order:

```txt
1. If recipe.flour exists, use recipe.flour.
2. If no recipe.flour exists, read app!: from src/main.flour.
3. If neither exists, use defaults.
```

---

## Phased Implementation Plan

### Phase 1: One-File Apps

Only require:

```txt
src/main.flour
```

Allow app config at the top:

```crustini
app!:
    name "demo"
    screen 240, 135
    fps 30
```

Default everything else.

Example defaults:

```txt
target        desktop_sim
screen        240x135
fps           30
memory mode   static
heap          off
optimize      size
panic         abort
output        .crustini/generated
```

This gives Crustini the fastest possible working beginner flow.

Command:

```sh
crustini new demo
cd demo
crustini bake
crustini run
```

Generated shape:

```txt
demo/
  src/
    main.flour
  .crustini/
    generated/
      Cargo.toml
      src/main.rs
```

---

### Phase 2: Optional `recipe.flour`

Add:

```txt
recipe.flour
```

For projects that need assets, board targets, memory settings, and build settings.

Example:

```crustini
project!:
    name "demo"
    version "0.1.0"

target!:
    board desktop_sim

screen!:
    size 240, 135
    fps 30

assets!:
    image "assets/sprites.png"
```

At this point, Crustini can support:

```txt
multi-file projects
assets
board targets
basic memory config
generated Rust crate settings
```

---

### Phase 3: Generated Rust Backend

The compiler should lower `recipe.flour` into a Rust config struct internally.

The user writes this:

```crustini
screen!:
    size 240, 135
    fps 30
```

Crustini generates something like this:

```rust
pub const PROJECT: crustini_runtime::ProjectConfig =
    crustini_runtime::ProjectConfig {
        screen_width: 240,
        screen_height: 135,
        fps: 30,
        ..crustini_runtime::ProjectConfig::DEFAULT
    };
```

The Rust output should not be treated as user source. It is a compiler artifact.

Generated Rust belongs here:

```txt
.crustini/generated/
```

Not here:

```txt
src/
```

The user should not need to read it unless debugging or contributing to Crustini itself.

---

## Why Not Rust Structs?

Rust structs are good internally but bad as the user-facing config.

A Rust config would look like this:

```rust
pub const CONFIG: CrustiniConfig = CrustiniConfig {
    name: "hood-wars",
    target: Target::Esp32S3ReverseTft,
    screen: Screen {
        width: 240,
        height: 135,
    },
    fps: 30,
    memory: Memory {
        mode: MemoryMode::Static,
        scratch_bytes: 16 * 1024,
        heap: false,
    },
};
```

This is too much for Crustini’s public surface.

Problems:

```txt
Too many symbols.
Too many names.
Too much punctuation.
Too Rust-specific.
Too much type noise.
Too much nesting.
Bad for brand-new programmers.
Makes Crustini feel fake because users are already writing Rust-shaped config.
```

Rust structs should exist behind the compiler boundary.

The correct split is:

```txt
User-facing:
  recipe.flour

Compiler-facing:
  Rust structs

Build-facing:
  generated Cargo.toml
```

---

## Why Not TOML?

TOML is a solid boring option.

A TOML config would look like:

```toml
[project]
name = "hood-wars"
version = "0.1.0"

[target]
board = "esp32_s3_reverse_tft"

[screen]
width = 240
height = 135
fps = 30

[memory]
mode = "static"
scratch = "16kb"
heap = false
```

This is not terrible.

But it is not ideal for Crustini.

Problems:

```txt
It feels like config instead of code.
It adds a second syntax.
It feels like Cargo, Vite, or DevOps.
It makes beginners learn Crustini plus TOML.
It weakens the baking/flour/recipe metaphor.
```

TOML should still exist in generated output:

```txt
.crustini/generated/Cargo.toml
```

But the user should not need to edit it.

---

## Why Not YAML?

YAML should be avoided.

Reasons:

```txt
Whitespace-sensitive in annoying ways.
Has weird implicit types.
Can be confusing for beginners.
Commonly causes invisible formatting bugs.
Feels like CI/CD config.
```

Example of what to avoid:

```yaml
project:
  name: hood-wars
  version: 0.1.0

screen:
  width: 240
  height: 135
  fps: 30
```

It looks simple, but YAML gets ugly as soon as the project grows.

Crustini should not inherit YAML’s baggage.

---

## Why Not JSON?

JSON is too noisy for humans.

Example:

```json
{
  "project": {
    "name": "hood-wars",
    "version": "0.1.0"
  },
  "screen": {
    "width": 240,
    "height": 135,
    "fps": 30
  }
}
```

Problems:

```txt
Too many braces.
Too many quotes.
Too many commas.
No comments unless extended.
Bad for hand-written beginner config.
```

JSON is good for machines, not for Crustini’s public syntax.

---

## Naming Rules

Recommended names:

```txt
recipe.flour         project config
src/main.flour       app entry file
.flour               Crustini source extension
crustini bake        build command
.crustini/           generated output/cache folder
```

Avoid:

```txt
crustini.toml
Cargo.toml as public config
project.rs
config.rs
config.yaml
bake.json
```

The name `recipe.flour` is strong because:

```txt
It is clear.
It is themed.
It is not too cute.
It implies instructions for building.
It avoids generic config naming.
```

---

## Minimal Config Grammar

The first grammar can be extremely small.

Informal grammar:

```txt
file        = block*
block       = ident "!:" newline indented_items
item        = key value*
key         = ident
value       = string | number | ident | size | path | boolean
boolean     = on | off
size        = number "." unit
unit        = kb | mb
```

Example parsed forms:

```crustini
name "hood-wars"
version "0.1.0"
board esp32_s3_reverse_tft
size 240, 135
fps 30
scratch 16.kb
heap off
```

Supported primitive values:

```txt
string       "hood-wars"
integer      30
pair         240, 135
identifier   esp32_s3_reverse_tft
boolean      on / off
size         16.kb / 2.mb
path         "assets/sprites.png"
```

Do not add complex expressions in config.

Avoid this:

```crustini
scratch 8.kb + 8.kb
fps if debug then 15 else 30
target env("CRUSTINI_TARGET")
```

Keep it dumb.

---

## Recommended Defaults

A tiny Crustini app should build even with almost no config.

Default project settings:

```txt
name          folder name
version       "0.1.0"
target        desktop_sim
screen        240, 135
fps           30
memory mode   static
scratch       16.kb
heap          off
panic         abort
optimize      size
generated     ".crustini/generated"
```

The smallest app should be:

```crustini
draw:
    bg 0
```

And it should still compile.

The compiler can infer:

```txt
desktop simulation target
default screen size
default frame rate
no assets
static memory
```

This matters because brand-new programmers should not be blocked by config.

---

## Compiler Behavior

The compiler should process config like this:

```txt
1. Find project root.
2. Look for recipe.flour.
3. If recipe.flour exists, parse it.
4. If recipe.flour does not exist, parse app!: block from src/main.flour.
5. Apply defaults for missing values.
6. Validate target, screen, memory, assets, and build options.
7. Generate Rust config structs.
8. Generate Cargo.toml.
9. Generate Rust app entry.
10. Build through cargo or esp tooling.
```

Validation should be strict but friendly.

Bad:

```txt
error: invalid config
```

Good:

```txt
recipe.flour:8
    fps fast
        ^^^^

fps must be a number.

Try:

    fps 30
```

---

## Error Style

Crustini errors should be blunt and beginner-friendly.

Example:

```txt
recipe.flour:12
    scratch 16.megabytes
             ^^^^^^^^^^^^

Unknown size unit: megabytes

Use one of:

    kb
    mb

Example:

    scratch 16.kb
```

Another example:

```txt
recipe.flour:5
    board esp32_s4
          ^^^^^^^^

Unknown board target: esp32_s4

Known targets:

    desktop_sim
    esp32_s3_reverse_tft
    esp32_s3_feather
```

The error should always say:

```txt
where it broke
what is wrong
what values are allowed
what to write instead
```

---

## Internal Rust Representation

Internally, Crustini should use Rust structs.

Example:

```rust
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    pub target: Target,
    pub screen: ScreenConfig,
    pub memory: MemoryConfig,
    pub buttons: Vec<Button>,
    pub assets: Vec<Asset>,
    pub build: BuildConfig,
}

pub struct ScreenConfig {
    pub width: u16,
    pub height: u16,
    pub fps: u16,
}

pub struct MemoryConfig {
    pub mode: MemoryMode,
    pub scratch_bytes: usize,
    pub heap_enabled: bool,
}

pub enum Target {
    DesktopSim,
    Esp32S3ReverseTft,
    Esp32S3Feather,
}

pub enum MemoryMode {
    Static,
    Managed,
}
```

This is the right place for typed Rust config.

The user should never have to write this.

---

## Final Public Surface

The normal user experience should be:

```sh
crustini new hood-wars
cd hood-wars
crustini bake
crustini run
```

Generated files:

```txt
.crustini/generated/
```

User-written files:

```txt
recipe.flour
src/main.flour
assets/
```

The default generated project should be:

```txt
hood-wars/
  recipe.flour
  src/
    main.flour
  assets/
  .crustini/
```

Starter `recipe.flour`:

```crustini
project!:
    name "hood-wars"

target!:
    board desktop_sim

screen!:
    size 240, 135
    fps 30

build!:
    optimize size
```

Starter `src/main.flour`:

```crustini
state!:
    x = 120
    y = 67

start:
    bg 0

draw:
    bg 12
    text "hello crustini", 20, 60
```

---

## Final Recommendation

Use:

```txt
recipe.flour
```

Make it a restricted Crustini-style config file.

Do not make users write Rust structs.

Do not use TOML as the public project config.

Do not expose Cargo config.

Do not add YAML or JSON.

Support one-file apps first, then add optional `recipe.flour`.

The long-term split should be:

```txt
.flour files       beginner-facing Crustini source
recipe.flour       beginner-facing Crustini project config
.rs files          generated Rust backend
Cargo.toml         generated build backend
.crustini/         generated output/cache
```

This gives Crustini the best mix of:

```txt
fast implementation
cool naming
tiny parser
clean Rust backend
beginner-friendly config
strong project identity
```

The project should feel like this:

```txt
You write flour.
Crustini bakes it.
Rust comes out the back.
```
