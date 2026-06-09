use std::path::Path;

pub const DEFAULT_STARTER: &str = "hello";

pub struct Starter {
    pub name: &'static str,
    pub description: &'static str,
    pub flour: &'static str,
    pub recipe: &'static str,
}

pub const STARTERS: &[Starter] = &[
    Starter {
        name: "hello",
        description: "minimal project-shaped hello world starter",
        flour: HELLO_FLOUR,
        recipe: HELLO_RECIPE,
    },
    Starter {
        name: "counter",
        description: "tiny state and input starter",
        flour: COUNTER_FLOUR,
        recipe: COUNTER_RECIPE,
    },
];

pub fn find(name: &str) -> Option<&'static Starter> {
    STARTERS.iter().find(|starter| starter.name == name)
}

pub fn recipe_for_dir(starter: &Starter, dir: &Path) -> String {
    let project_name = dir
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("app");

    starter
        .recipe
        .replace("{{project_name}}", project_name)
        .replace("{{preview_title}}", &title_from_slug(project_name))
}

fn title_from_slug(slug: &str) -> String {
    let mut title = String::new();
    let mut capitalize_next = true;

    for ch in slug.chars() {
        if ch == '-' || ch == '_' {
            title.push(' ');
            capitalize_next = true;
        } else if capitalize_next {
            title.push(ch.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            title.push(ch);
        }
    }

    title
}

const HELLO_FLOUR: &str = r#"app! {
  state! {
    pulse: i16 = 0
    pulse_dir: i16 = 1
  }

  update! {
    pulse = pulse + pulse_dir

    if pulse > 12 {
      pulse_dir = -1
    }

    if pulse < 0 {
      pulse_dir = 1
    }
  }

  draw! {
    clear!(8)
    rect!(14, 18, 152, 84, 18)
    line!(14, 34, 165, 34, 70)
    text!(48, 48 - pulse, "HELLO", 255)
    text!(42, 62 + pulse, "WORLD", 210)
    text!(28, 94, "RX RUNS THIS", 120)
  }
}
"#;

const HELLO_RECIPE: &str = r#"project!:
  name "{{project_name}}"
  version "0.1.0"

screen!:
  size 180, 120
  fps 30

preview!:
  title "{{preview_title}}"
  scale auto

build!:
  generated ".crustini/generated"
"#;

const COUNTER_FLOUR: &str = r#"app! {
  state! {
    count: i32 = 0
    shade: u8 = 120
  }

  update! {
    if a {
      count = count + 1
      shade = 220
    }

    if b {
      count = count - 1
      shade = 80
    }
  }

  draw! {
    clear!(0)
    rect!(8, 8, 164, 104, 24)
    line!(8, 34, 172, 34, 96)
    rect!(74, 54, 32, 18, shade)
    text!(22, 18, "COUNTER", 220)
    text!(18, 94, "SPACE/Z A  ENTER/X B", 130)
  }
}
"#;

const COUNTER_RECIPE: &str = r#"project!:
  name "{{project_name}}"
  version "0.1.0"

screen!:
  size 180, 120
  fps 30

preview!:
  title "{{preview_title}}"
  scale auto

build!:
  generated ".crustini/generated"
"#;
