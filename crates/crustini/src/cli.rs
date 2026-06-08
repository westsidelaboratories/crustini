use crate::config::{load_project_config, ProjectConfig};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const BAKERY_DIR: &str = ".bakery";
const DEFAULT_STARTER: &str = "hello";
const STARTERS: &[(&str, &str)] = &[
    ("hello", "minimal text-on-screen starter"),
    ("counter", "tiny state and input starter"),
];
const HELLO_STARTER: &str = r#"app! {
  screen!(128, 64)
  fps!(30)

  setup! {
    clear!(0)
  }

  draw! {
    clear!(0)
    text!(28, 24, "HELLO WORLD!", 255)
  }
}
"#;
const HELLO_RECIPE: &str = r#"project!:
  name "hello"
  version "0.1.0"

target!:
  board desktop_sim

screen!:
  size 128, 64
  fps 30

memory!:
  mode static
  scratch 16.kb
  heap off

build!:
  optimize size
  panic abort
  generated ".crustini/generated"
"#;
const COUNTER_STARTER: &str = r#"app! {
  screen!(240, 135)
  fps!(30)

  state! {
    count: i32 = 0
  }

  update! {
    if a {
      count = count + 1
    }

    if b {
      count = count - 1
    }
  }

  draw! {
    clear!(0)
    rect!(8, 8, 224, 119, 24)
    line!(8, 34, 232, 34, 96)
  }
}
"#;
const COUNTER_RECIPE: &str = r#"project!:
  name "counter"
  version "0.1.0"

target!:
  board desktop_sim

screen!:
  size 240, 135
  fps 30

build!:
  optimize size
  panic abort
  generated ".crustini/generated"
"#;

pub fn main() {
    if let Err(err) = run() {
        eprintln!("error: {}", err);
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let cmd_name = command_name();

    if args.is_empty() || args[0] == "help" || args[0] == "--help" || args[0] == "-h" {
        print_help(&cmd_name);
        return Ok(());
    }

    let cmd = args.remove(0);
    match cmd.as_str() {
        "emit" => {
            let input = args
                .first()
                .ok_or_else(|| format!("usage: {} emit <file.flour>", cmd_name))?;
            let src = fs::read_to_string(input)
                .map_err(|e| format!("failed to read `{}`: {}", input, e))?;
            let rust = crate::compile_to_rust(&src)?;
            print!("{}", rust);
            Ok(())
        }
        "proof" | "check" => {
            let input = input_target(args.first().map(String::as_str))?;
            proof_target(&input)
        }
        "bake" | "build" => {
            let input = input_target(args.first().map(String::as_str))?;
            let run_cargo = !args.iter().any(|a| a == "--no-cargo");
            bake_target(&input, run_cargo)
        }
        "run" => {
            let input = input_target(args.first().map(String::as_str))?;
            run_target(&input)
        }
        "starter" | "new" => {
            let name = args.first().map(String::as_str).unwrap_or(DEFAULT_STARTER);
            let dir = args.get(1).map(String::as_str).unwrap_or(name);
            create_starter(name, Path::new(dir))
        }
        "starters" => {
            print_starters();
            Ok(())
        }
        path if is_source_path(path) || Path::new(path).is_dir() => {
            run_target(&input_target(Some(path))?)
        }
        other => Err(format!("unknown command `{}`", other)),
    }
}

enum InputTarget {
    SourceFile(PathBuf),
    ProjectRoot(PathBuf),
}

fn input_target(input: Option<&str>) -> Result<InputTarget, String> {
    let path = input
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    if is_source_path(path.to_string_lossy().as_ref()) {
        return Ok(InputTarget::SourceFile(path));
    }

    if path.is_dir() {
        return Ok(InputTarget::ProjectRoot(path));
    }

    Err(format!(
        "expected a `.flour` source file or project directory, got `{}`",
        path.display()
    ))
}

fn command_name() -> String {
    env::args()
        .next()
        .and_then(|arg| {
            Path::new(&arg)
                .file_name()
                .and_then(|name| name.to_str())
                .map(str::to_string)
        })
        .unwrap_or_else(|| "crustini".to_string())
}

fn print_help(cmd_name: &str) {
    println!("Crustini");
    println!();
    println!("USAGE:");
    println!("  {} <file.flour>", cmd_name);
    println!("  {} proof [file.flour|project-dir]", cmd_name);
    println!("  {} bake [file.flour|project-dir] [--no-cargo]", cmd_name);
    println!("  {} run [file.flour|project-dir]", cmd_name);
    println!("  {} emit <file.flour>", cmd_name);
    println!("  {} starter [hello|counter] [dir]", cmd_name);
    println!("  {} starters", cmd_name);
    println!();
    println!("ALIASES:");
    println!("  check   same as proof");
    println!("  build   same as bake");
    println!("  new     same as starter");
    println!("  <file>  same as run <file>");
    println!();
    println!("OUTPUT:");
    println!("  bakery  {}", BAKERY_DIR);
    println!("  artifact generated Cargo artifact when Cargo runs");
}

fn print_starters() {
    println!("starters:");
    for (name, description) in STARTERS {
        println!("  {:<8} {}", name, description);
    }
}

fn proof_target(input: &InputTarget) -> Result<(), String> {
    match input {
        InputTarget::SourceFile(path) => proof_file(path),
        InputTarget::ProjectRoot(root) => proof_project(root),
    }
}

fn bake_target(input: &InputTarget, run_cargo: bool) -> Result<(), String> {
    match input {
        InputTarget::SourceFile(path) => bake_file(path, run_cargo),
        InputTarget::ProjectRoot(root) => bake_project(root, run_cargo),
    }
}

fn run_target(input: &InputTarget) -> Result<(), String> {
    match input {
        InputTarget::SourceFile(path) => run_file(path),
        InputTarget::ProjectRoot(root) => run_project(root),
    }
}

fn proof_file(input: &Path) -> Result<(), String> {
    if !is_source_path(input.to_string_lossy().as_ref()) {
        return Err(format!(
            "expected a `.flour` source file, got `{}`",
            input.display()
        ));
    }

    let src = fs::read_to_string(input)
        .map_err(|e| format!("failed to read `{}`: {}", input.display(), e))?;
    crate::compile_to_rust(&src)?;

    println!("proof: {}", input.display());
    println!("status: ok");
    Ok(())
}

fn proof_project(root: &Path) -> Result<(), String> {
    let (config, source_path, source) = read_project_source(root)?;
    compile_project_source(&source, &config)?;

    println!("proof: {}", source_path.display());
    println!("recipe: {}", root.join("recipe.flour").display());
    println!("status: ok");
    Ok(())
}

fn bake_file(input: &Path, run_cargo: bool) -> Result<(), String> {
    let output = write_bakery(input)?;

    println!("bake: {}", input.display());
    println!("bakery: {}", output.out_dir.display());

    if run_cargo {
        cargo_build(&output.out_dir)?;

        println!(
            "artifact: {}",
            artifact_path(&output.out_dir, &output.package_name).display()
        );
    } else {
        println!("artifact: skipped (--no-cargo)");
    }

    Ok(())
}

fn bake_project(root: &Path, run_cargo: bool) -> Result<(), String> {
    let output = write_project(root)?;

    println!("bake: {}", output.source_path.display());
    if output.config.has_recipe {
        println!("recipe: {}", output.root.join("recipe.flour").display());
    }
    println!("bakery: {}", output.out_dir.display());

    if run_cargo {
        cargo_build(&output.out_dir)?;

        println!(
            "artifact: {}",
            artifact_path(&output.out_dir, &output.package_name).display()
        );
    } else {
        println!("artifact: skipped (--no-cargo)");
    }

    Ok(())
}

fn run_file(input: &Path) -> Result<(), String> {
    let output = write_bakery(input)?;

    println!("bake: {}", input.display());
    println!("bakery: {}", output.out_dir.display());

    let status = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .arg("--manifest-path")
        .arg(output.out_dir.join("Cargo.toml"))
        .status()
        .map_err(|e| format!("failed to run generated preview: {}", e))?;

    if !status.success() {
        return Err("generated preview failed to run".to_string());
    }

    Ok(())
}

fn run_project(root: &Path) -> Result<(), String> {
    let output = write_project(root)?;

    println!("bake: {}", output.source_path.display());
    if output.config.has_recipe {
        println!("recipe: {}", output.root.join("recipe.flour").display());
    }
    println!("bakery: {}", output.out_dir.display());

    let status = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .arg("--manifest-path")
        .arg(output.out_dir.join("Cargo.toml"))
        .status()
        .map_err(|e| format!("failed to run generated preview: {}", e))?;

    if !status.success() {
        return Err("generated preview failed to run".to_string());
    }

    Ok(())
}

struct BakeryOutput {
    out_dir: PathBuf,
    package_name: String,
}

struct ProjectOutput {
    root: PathBuf,
    source_path: PathBuf,
    out_dir: PathBuf,
    package_name: String,
    config: ProjectConfig,
}

fn write_bakery(input: &Path) -> Result<BakeryOutput, String> {
    if !is_source_path(input.to_string_lossy().as_ref()) {
        return Err(format!(
            "expected a `.flour` source file, got `{}`",
            input.display()
        ));
    }

    let src = fs::read_to_string(input)
        .map_err(|e| format!("failed to read `{}`: {}", input.display(), e))?;
    let rust = crate::compile_to_rust(&src)?;

    let input_dir = input.parent().unwrap_or_else(|| Path::new("."));
    let out_dir = input_dir.join(BAKERY_DIR);
    let src_dir = out_dir.join("src");

    fs::create_dir_all(&src_dir)
        .map_err(|e| format!("failed to create `{}`: {}", src_dir.display(), e))?;

    fs::write(src_dir.join("lib.rs"), rust)
        .map_err(|e| format!("failed to write generated lib.rs: {}", e))?;

    let package_name = generated_package_name(input);
    let frame_path = out_dir.join("frame.ppm");
    fs::write(
        src_dir.join("main.rs"),
        generated_main_rs(&package_name, &frame_path),
    )
    .map_err(|e| format!("failed to write generated main.rs: {}", e))?;

    let core_path = core_crate_path()?;
    fs::write(
        out_dir.join("Cargo.toml"),
        generated_cargo_toml(&package_name, &core_path),
    )
    .map_err(|e| format!("failed to write generated Cargo.toml: {}", e))?;

    Ok(BakeryOutput {
        out_dir,
        package_name,
    })
}

fn write_project(root: &Path) -> Result<ProjectOutput, String> {
    let root = root
        .canonicalize()
        .map_err(|e| format!("failed to locate project `{}`: {}", root.display(), e))?;
    let (config, source_path, source) = read_project_source(&root)?;
    let rust = compile_project_source(&source, &config)?;
    let out_dir = root.join(&config.generated);
    let src_dir = out_dir.join("src");

    fs::create_dir_all(&src_dir)
        .map_err(|e| format!("failed to create `{}`: {}", src_dir.display(), e))?;

    fs::write(src_dir.join("lib.rs"), rust)
        .map_err(|e| format!("failed to write generated lib.rs: {}", e))?;

    let package_name = generated_package_name_from_name(&config.name);
    let frame_path = out_dir.join("frame.ppm");
    fs::write(
        src_dir.join("main.rs"),
        generated_main_rs(&package_name, &frame_path),
    )
    .map_err(|e| format!("failed to write generated main.rs: {}", e))?;

    let core_path = core_crate_path()?;
    fs::write(
        out_dir.join("Cargo.toml"),
        generated_cargo_toml(&package_name, &core_path),
    )
    .map_err(|e| format!("failed to write generated Cargo.toml: {}", e))?;

    Ok(ProjectOutput {
        root,
        source_path,
        out_dir,
        package_name,
        config,
    })
}

fn read_project_source(root: &Path) -> Result<(ProjectConfig, PathBuf, String), String> {
    let config = load_project_config(root)?;
    validate_project_files(root, &config)?;
    let source_path = root.join(&config.source);
    let source = fs::read_to_string(&source_path)
        .map_err(|e| format!("failed to read `{}`: {}", source_path.display(), e))?;
    Ok((config, source_path, source))
}

fn validate_project_files(root: &Path, config: &ProjectConfig) -> Result<(), String> {
    let source_path = root.join(&config.source);
    if !source_path.exists() {
        return Err(format!(
            "project source does not exist: {}\n\nTry creating `src/main.flour` or setting `project!: source \"...\"` in recipe.flour.",
            source_path.display()
        ));
    }

    for asset in &config.assets {
        let path = root.join(&asset.path);
        if !path.exists() {
            return Err(format!(
                "{} asset does not exist: {}",
                asset.kind,
                path.display()
            ));
        }
    }

    Ok(())
}

fn compile_project_source(source: &str, config: &ProjectConfig) -> Result<String, String> {
    if config.has_recipe {
        crate::compile_to_rust_with_screen(
            source,
            config.screen_width,
            config.screen_height,
            config.fps,
        )
    } else {
        crate::compile_to_rust(source)
    }
}

fn cargo_build(out_dir: &Path) -> Result<(), String> {
    let status = Command::new("cargo")
        .arg("build")
        .arg("--manifest-path")
        .arg(out_dir.join("Cargo.toml"))
        .status()
        .map_err(|e| format!("failed to run cargo build: {}", e))?;

    if !status.success() {
        return Err("generated Rust failed to build".to_string());
    }

    Ok(())
}

fn create_starter(name: &str, dir: &Path) -> Result<(), String> {
    let (flour, recipe) = match name {
        "hello" => (HELLO_STARTER, HELLO_RECIPE),
        "counter" => (COUNTER_STARTER, COUNTER_RECIPE),
        _ => return Err(format!("unknown starter `{}`; run `rx starters`", name)),
    };

    if dir.exists() {
        return Err(format!(
            "starter destination already exists: {}",
            dir.display()
        ));
    }

    fs::create_dir_all(dir)
        .map_err(|e| format!("failed to create starter `{}`: {}", dir.display(), e))?;
    fs::create_dir_all(dir.join("src"))
        .map_err(|e| format!("failed to create starter src directory: {}", e))?;
    fs::create_dir_all(dir.join("assets"))
        .map_err(|e| format!("failed to create starter assets directory: {}", e))?;
    fs::write(dir.join("recipe.flour"), starter_recipe(recipe, dir))
        .map_err(|e| format!("failed to write recipe.flour: {}", e))?;
    fs::write(dir.join("src/main.flour"), flour)
        .map_err(|e| format!("failed to write src/main.flour: {}", e))?;
    fs::write(
        dir.join("README.md"),
        "Run this starter with:\n\n```bash\nrx .\n```\n\nBake an artifact with:\n\n```bash\nrx bake .\n```\n",
    )
    .map_err(|e| format!("failed to write README.md: {}", e))?;

    println!("starter: {}", name);
    println!("recipe: {}", dir.join("recipe.flour").display());
    println!("flour: {}", dir.join("src/main.flour").display());
    println!("run: rx {}", dir.display());
    println!("bake: rx bake {}", dir.display());
    Ok(())
}

fn starter_recipe(recipe: &str, dir: &Path) -> String {
    let project_name = dir
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("app");
    recipe
        .replacen("name \"hello\"", &format!("name \"{}\"", project_name), 1)
        .replacen("name \"counter\"", &format!("name \"{}\"", project_name), 1)
}

fn core_crate_path() -> Result<PathBuf, String> {
    // At compile time this points to `crates/crustini`.
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join("../crustini-core");
    path.canonicalize()
        .map_err(|e| format!("failed to locate crustini-core: {}", e))
}

fn generated_cargo_toml(package_name: &str, core_path: &Path) -> String {
    format!(
        "[workspace]\n\n[package]\nname = \"{}\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n[dependencies]\ncrustini-core = {{ path = \"{}\" }}\n\n[lib]\ncrate-type = [\"rlib\"]\n",
        package_name,
        path_for_toml(core_path)
    )
}

fn generated_main_rs(package_name: &str, frame_path: &Path) -> String {
    format!(
        r#"use crustini_core::{{Buttons, CrustiniApp, Screen}};
use {package_name}::{{App, HEIGHT, WIDTH}};
use std::fs::File;
use std::io::Write;

const FRAME_PATH: &str = "{frame_path}";

fn main() -> Result<(), Box<dyn std::error::Error>> {{
    let mut app = App::new();
    let mut pixels = vec![0_u8; WIDTH * HEIGHT];

    {{
        let mut screen = Screen::new(WIDTH, HEIGHT, &mut pixels);
        app.setup(&mut screen);
        app.update(Buttons::default());
        app.draw(&mut screen);
    }}

    write_ppm(FRAME_PATH, WIDTH, HEIGHT, &pixels)?;
    println!("frame: {{}}", FRAME_PATH);
    Ok(())
}}

fn write_ppm(path: &str, width: usize, height: usize, pixels: &[u8]) -> std::io::Result<()> {{
    let mut file = File::create(path)?;
    write!(file, "P6\n{{}} {{}}\n255\n", width, height)?;

    for &pixel in pixels {{
        file.write_all(&[pixel, pixel, pixel])?;
    }}

    Ok(())
}}
"#,
        package_name = package_name.replace('-', "_"),
        frame_path = path_for_rust_string(frame_path)
    )
}

fn artifact_path(out_dir: &Path, package_name: &str) -> PathBuf {
    let lib_name = package_name.replace('-', "_");
    out_dir
        .join("target")
        .join("debug")
        .join(format!("lib{}.rlib", lib_name))
}

fn path_for_toml(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn path_for_rust_string(path: &Path) -> String {
    path.to_string_lossy()
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
}

fn generated_package_name(input: &Path) -> String {
    let stem = input
        .parent()
        .and_then(|p| p.file_name())
        .or_else(|| input.file_stem())
        .and_then(|s| s.to_str())
        .unwrap_or("app");

    let mut out = String::from("crustini_generated_");
    for ch in stem.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push('_');
        }
    }
    out
}

fn generated_package_name_from_name(name: &str) -> String {
    let mut out = String::from("crustini_generated_");
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push('_');
        }
    }
    out
}

fn is_source_path(path: &str) -> bool {
    path.ends_with(".flour") || path.ends_with(".crst")
}
