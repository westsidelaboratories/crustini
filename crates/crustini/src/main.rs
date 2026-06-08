use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const OVEN_DIR: &str = ".oven";
const DEFAULT_STARTER: &str = "counter";
const COUNTER_STARTER: &str = r#"app! {
  screen 240 135
  fps 30

  state {
    count: i32 = 0
  }

  update {
    if a {
      count = count + 1
    }

    if b {
      count = count - 1
    }
  }

  draw {
    clear 0
    rect 8 8 224 119 24
    line 8 34 232 34 96
  }
}
"#;

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {}", err);
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() || args[0] == "help" || args[0] == "--help" || args[0] == "-h" {
        print_help();
        return Ok(());
    }

    let cmd = args.remove(0);
    match cmd.as_str() {
        "emit" => {
            let input = args.first().ok_or("usage: crustini emit <file.flour>")?;
            let src = fs::read_to_string(input)
                .map_err(|e| format!("failed to read `{}`: {}", input, e))?;
            let rust = crustini::compile_to_rust(&src)?;
            print!("{}", rust);
            Ok(())
        }
        "bake" | "build" => {
            let input = args
                .first()
                .ok_or("usage: crustini bake <file.flour> [--no-cargo]")?;
            let run_cargo = !args.iter().any(|a| a == "--no-cargo");
            bake_file(Path::new(input), run_cargo)
        }
        "starter" | "new" => {
            let name = args.first().map(String::as_str).unwrap_or(DEFAULT_STARTER);
            let dir = args.get(1).map(String::as_str).unwrap_or(name);
            create_starter(name, Path::new(dir))
        }
        path if is_source_path(path) => bake_file(Path::new(path), true),
        other => Err(format!("unknown command `{}`", other)),
    }
}

fn print_help() {
    println!("Crustini");
    println!();
    println!("USAGE:");
    println!("  crustini bake <file.flour> [--no-cargo]");
    println!("  crustini emit <file.flour>");
    println!("  crustini starter [counter] [dir]");
    println!("  crustini <file.flour>");
    println!();
    println!("ALIASES:");
    println!("  build   same as bake");
    println!("  new     same as starter");
    println!();
    println!("OUTPUT:");
    println!("  oven    {}", OVEN_DIR);
    println!("  loaf    generated Cargo artifact when Cargo runs");
}

fn bake_file(input: &Path, run_cargo: bool) -> Result<(), String> {
    if !is_source_path(input.to_string_lossy().as_ref()) {
        return Err(format!(
            "expected a `.flour` source file, got `{}`",
            input.display()
        ));
    }

    let src = fs::read_to_string(input)
        .map_err(|e| format!("failed to read `{}`: {}", input.display(), e))?;
    let rust = crustini::compile_to_rust(&src)?;

    let input_dir = input.parent().unwrap_or_else(|| Path::new("."));
    let out_dir = input_dir.join(OVEN_DIR);
    let src_dir = out_dir.join("src");

    fs::create_dir_all(&src_dir)
        .map_err(|e| format!("failed to create `{}`: {}", src_dir.display(), e))?;

    fs::write(src_dir.join("lib.rs"), rust)
        .map_err(|e| format!("failed to write generated lib.rs: {}", e))?;

    let package_name = generated_package_name(input);
    let core_path = core_crate_path()?;
    fs::write(
        out_dir.join("Cargo.toml"),
        generated_cargo_toml(&package_name, &core_path),
    )
    .map_err(|e| format!("failed to write generated Cargo.toml: {}", e))?;

    println!("bake: {}", input.display());
    println!("oven: {}", out_dir.display());

    if run_cargo {
        let status = Command::new("cargo")
            .arg("build")
            .arg("--manifest-path")
            .arg(out_dir.join("Cargo.toml"))
            .status()
            .map_err(|e| format!("failed to run cargo build: {}", e))?;

        if !status.success() {
            return Err("generated Rust failed to build".to_string());
        }

        println!("loaf: {}", loaf_path(&out_dir, &package_name).display());
    } else {
        println!("loaf: skipped (--no-cargo)");
    }

    Ok(())
}

fn create_starter(name: &str, dir: &Path) -> Result<(), String> {
    if name != DEFAULT_STARTER {
        return Err(format!(
            "unknown starter `{}`; available: {}",
            name, DEFAULT_STARTER
        ));
    }

    if dir.exists() {
        return Err(format!(
            "starter destination already exists: {}",
            dir.display()
        ));
    }

    fs::create_dir_all(dir)
        .map_err(|e| format!("failed to create starter `{}`: {}", dir.display(), e))?;
    fs::write(dir.join("app.flour"), COUNTER_STARTER)
        .map_err(|e| format!("failed to write app.flour: {}", e))?;
    fs::write(
        dir.join("crustini.toml"),
        "name = \"counter\"\nstarter = \"counter\"\n",
    )
    .map_err(|e| format!("failed to write crustini.toml: {}", e))?;
    fs::write(
        dir.join("README.md"),
        "Bake this starter with:\n\n```bash\ncrustini bake app.flour\n```\n",
    )
    .map_err(|e| format!("failed to write README.md: {}", e))?;

    println!("starter: {}", name);
    println!("flour: {}", dir.join("app.flour").display());
    println!("bake: crustini bake {}", dir.join("app.flour").display());
    Ok(())
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

fn loaf_path(out_dir: &Path, package_name: &str) -> PathBuf {
    let lib_name = package_name.replace('-', "_");
    out_dir
        .join("target")
        .join("debug")
        .join(format!("lib{}.rlib", lib_name))
}

fn path_for_toml(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
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

fn is_source_path(path: &str) -> bool {
    path.ends_with(".flour") || path.ends_with(".crst")
}
