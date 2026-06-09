use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    pub target: String,
    pub source: PathBuf,
    pub screen_width: usize,
    pub screen_height: usize,
    pub fps: usize,
    pub memory_mode: String,
    pub scratch_bytes: usize,
    pub heap_enabled: bool,
    pub optimize: String,
    pub panic: String,
    pub generated: PathBuf,
    pub preview_title: Option<String>,
    pub preview_scale: String,
    pub assets: Vec<AssetConfig>,
    pub has_recipe: bool,
}

#[derive(Debug, Clone)]
pub struct AssetConfig {
    pub kind: String,
    pub path: String,
}

impl ProjectConfig {
    pub fn defaults(root: &Path) -> Self {
        Self {
            name: root
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("app")
                .to_string(),
            version: "0.1.0".to_string(),
            target: "desktop_sim".to_string(),
            source: PathBuf::from("src/main.flour"),
            screen_width: 240,
            screen_height: 135,
            fps: 30,
            memory_mode: "static".to_string(),
            scratch_bytes: 16 * 1024,
            heap_enabled: false,
            optimize: "size".to_string(),
            panic: "abort".to_string(),
            generated: PathBuf::from(".crustini/generated"),
            preview_title: None,
            preview_scale: "auto".to_string(),
            assets: Vec::new(),
            has_recipe: false,
        }
    }
}

pub fn load_project_config(root: &Path) -> Result<ProjectConfig, String> {
    let mut config = ProjectConfig::defaults(root);
    let recipe = root.join("recipe.flour");

    if recipe.exists() {
        let src = std::fs::read_to_string(&recipe)
            .map_err(|e| format!("failed to read `{}`: {}", recipe.display(), e))?;
        parse_recipe(&src, &recipe, &mut config)?;
        config.has_recipe = true;
    }

    validate_config(&config)?;
    Ok(config)
}

fn parse_recipe(src: &str, path: &Path, config: &mut ProjectConfig) -> Result<(), String> {
    let mut block = "";

    for (idx, raw) in src.lines().enumerate() {
        let line_no = idx + 1;
        let line = strip_comment(raw).trim_end();
        if line.trim().is_empty() {
            continue;
        }

        if !raw.starts_with(' ') && line.ends_with("!:") {
            block = line.trim_end_matches("!:").trim();
            validate_block(block, path, line_no)?;
            continue;
        }

        if block.is_empty() {
            return Err(config_err(
                path,
                line_no,
                line,
                "config items must be inside a known block like `project!:`",
            ));
        }

        let trimmed = line.trim();
        let mut parts = trimmed.splitn(2, char::is_whitespace);
        let key = parts.next().unwrap_or_default();
        let rest = parts.next().unwrap_or_default().trim();

        match block {
            "project" => parse_project_item(key, rest, config, path, line_no, trimmed)?,
            "target" => parse_target_item(key, rest, config, path, line_no, trimmed)?,
            "screen" => parse_screen_item(key, rest, config, path, line_no, trimmed)?,
            "memory" => parse_memory_item(key, rest, config, path, line_no, trimmed)?,
            "buttons" => parse_buttons_item(key, rest, path, line_no, trimmed)?,
            "preview" => parse_preview_item(key, rest, config, path, line_no, trimmed)?,
            "assets" => parse_assets_item(key, rest, config, path, line_no, trimmed)?,
            "build" => parse_build_item(key, rest, config, path, line_no, trimmed)?,
            _ => unreachable!(),
        }
    }

    Ok(())
}

fn validate_block(block: &str, path: &Path, line_no: usize) -> Result<(), String> {
    match block {
        "project" | "target" | "screen" | "memory" | "buttons" | "preview" | "assets"
        | "build" => Ok(()),
        _ => Err(config_err(
            path,
            line_no,
            block,
            "unknown config block; use project!, target!, screen!, memory!, buttons!, preview!, assets!, or build!",
        )),
    }
}

fn parse_project_item(
    key: &str,
    rest: &str,
    config: &mut ProjectConfig,
    path: &Path,
    line_no: usize,
    line: &str,
) -> Result<(), String> {
    match key {
        "name" => config.name = string_value(rest, path, line_no, line)?,
        "version" => config.version = string_value(rest, path, line_no, line)?,
        "source" => config.source = PathBuf::from(string_value(rest, path, line_no, line)?),
        _ => {
            return Err(unknown_key(
                path,
                line_no,
                line,
                "project",
                "name, version, source",
            ))
        }
    }
    Ok(())
}

fn parse_target_item(
    key: &str,
    rest: &str,
    config: &mut ProjectConfig,
    path: &Path,
    line_no: usize,
    line: &str,
) -> Result<(), String> {
    match key {
        "board" => config.target = ident_value(rest, path, line_no, line)?.to_string(),
        _ => return Err(unknown_key(path, line_no, line, "target", "board")),
    }
    Ok(())
}

fn parse_screen_item(
    key: &str,
    rest: &str,
    config: &mut ProjectConfig,
    path: &Path,
    line_no: usize,
    line: &str,
) -> Result<(), String> {
    match key {
        "size" => {
            let (width, height) = pair_value(rest, path, line_no, line)?;
            config.screen_width = width;
            config.screen_height = height;
        }
        "fps" => config.fps = number_value(rest, path, line_no, line)?,
        _ => return Err(unknown_key(path, line_no, line, "screen", "size, fps")),
    }
    Ok(())
}

fn parse_memory_item(
    key: &str,
    rest: &str,
    config: &mut ProjectConfig,
    path: &Path,
    line_no: usize,
    line: &str,
) -> Result<(), String> {
    match key {
        "mode" => config.memory_mode = ident_value(rest, path, line_no, line)?.to_string(),
        "scratch" => config.scratch_bytes = size_value(rest, path, line_no, line)?,
        "heap" => config.heap_enabled = bool_value(rest, path, line_no, line)?,
        _ => {
            return Err(unknown_key(
                path,
                line_no,
                line,
                "memory",
                "mode, scratch, heap",
            ))
        }
    }
    Ok(())
}

fn parse_buttons_item(
    key: &str,
    rest: &str,
    path: &Path,
    line_no: usize,
    line: &str,
) -> Result<(), String> {
    if !rest.is_empty() {
        return Err(config_err(
            path,
            line_no,
            line,
            "button entries should be bare names like `a` or `start`",
        ));
    }

    match key {
        "up" | "down" | "left" | "right" | "a" | "b" | "start" | "select" => Ok(()),
        _ => Err(config_err(
            path,
            line_no,
            line,
            "unknown button; use up, down, left, right, a, b, start, or select",
        )),
    }
}

fn parse_assets_item(
    key: &str,
    rest: &str,
    config: &mut ProjectConfig,
    path: &Path,
    line_no: usize,
    line: &str,
) -> Result<(), String> {
    match key {
        "image" | "map" | "font" => config.assets.push(AssetConfig {
            kind: key.to_string(),
            path: string_value(rest, path, line_no, line)?,
        }),
        _ => {
            return Err(unknown_key(
                path,
                line_no,
                line,
                "assets",
                "image, map, font",
            ))
        }
    }
    Ok(())
}

fn parse_preview_item(
    key: &str,
    rest: &str,
    config: &mut ProjectConfig,
    path: &Path,
    line_no: usize,
    line: &str,
) -> Result<(), String> {
    match key {
        "title" => config.preview_title = Some(string_value(rest, path, line_no, line)?),
        "scale" => config.preview_scale = scale_value(rest, path, line_no, line)?.to_string(),
        _ => return Err(unknown_key(path, line_no, line, "preview", "title, scale")),
    }
    Ok(())
}

fn parse_build_item(
    key: &str,
    rest: &str,
    config: &mut ProjectConfig,
    path: &Path,
    line_no: usize,
    line: &str,
) -> Result<(), String> {
    match key {
        "optimize" => config.optimize = ident_value(rest, path, line_no, line)?.to_string(),
        "panic" => config.panic = ident_value(rest, path, line_no, line)?.to_string(),
        "generated" => config.generated = PathBuf::from(string_value(rest, path, line_no, line)?),
        _ => {
            return Err(unknown_key(
                path,
                line_no,
                line,
                "build",
                "optimize, panic, generated",
            ))
        }
    }
    Ok(())
}

fn validate_config(config: &ProjectConfig) -> Result<(), String> {
    match config.target.as_str() {
        "desktop_sim" | "esp32_s3_reverse_tft" | "esp32_s3_feather" => {}
        other => {
            return Err(format!(
                "unknown board target `{}`\n\nKnown targets:\n  desktop_sim\n  esp32_s3_reverse_tft\n  esp32_s3_feather",
                other
            ));
        }
    }

    match config.memory_mode.as_str() {
        "static" | "managed" => {}
        other => {
            return Err(format!(
                "memory mode must be `static` or `managed`, got `{}`",
                other
            ))
        }
    }

    match config.optimize.as_str() {
        "size" | "speed" | "debug" => {}
        other => {
            return Err(format!(
                "optimize must be `size`, `speed`, or `debug`, got `{}`",
                other
            ))
        }
    }

    match config.panic.as_str() {
        "abort" | "unwind" => {}
        other => {
            return Err(format!(
                "panic must be `abort` or `unwind`, got `{}`",
                other
            ))
        }
    }

    if config.screen_width == 0 || config.screen_height == 0 {
        return Err("screen size must be greater than zero".to_string());
    }
    if config.fps == 0 {
        return Err("fps must be greater than zero".to_string());
    }

    match config.preview_scale.as_str() {
        "auto" | "1x" | "2x" | "4x" | "8x" => {}
        other => {
            return Err(format!(
                "preview scale must be `auto`, `1x`, `2x`, `4x`, or `8x`, got `{}`",
                other
            ))
        }
    }

    Ok(())
}

fn string_value(rest: &str, path: &Path, line_no: usize, line: &str) -> Result<String, String> {
    let rest = rest.trim();
    if rest.len() >= 2 && rest.starts_with('"') && rest.ends_with('"') {
        Ok(rest[1..rest.len() - 1].to_string())
    } else {
        Err(config_err(path, line_no, line, "expected a quoted string"))
    }
}

fn ident_value<'a>(
    rest: &'a str,
    path: &Path,
    line_no: usize,
    line: &str,
) -> Result<&'a str, String> {
    let rest = rest.trim();
    if !rest.is_empty()
        && rest
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
    {
        Ok(rest)
    } else {
        Err(config_err(path, line_no, line, "expected an identifier"))
    }
}

fn scale_value<'a>(
    rest: &'a str,
    path: &Path,
    line_no: usize,
    line: &str,
) -> Result<&'a str, String> {
    let rest = rest.trim();
    match rest {
        "auto" | "1x" | "2x" | "4x" | "8x" => Ok(rest),
        _ => Err(config_err(
            path,
            line_no,
            line,
            "expected `auto`, `1x`, `2x`, `4x`, or `8x`",
        )),
    }
}

fn number_value(rest: &str, path: &Path, line_no: usize, line: &str) -> Result<usize, String> {
    rest.trim()
        .parse::<usize>()
        .map_err(|_| config_err(path, line_no, line, "expected a number"))
}

fn pair_value(
    rest: &str,
    path: &Path,
    line_no: usize,
    line: &str,
) -> Result<(usize, usize), String> {
    let (left, right) = rest
        .split_once(',')
        .ok_or_else(|| config_err(path, line_no, line, "expected two numbers like `240, 135`"))?;
    let left = left
        .trim()
        .parse::<usize>()
        .map_err(|_| config_err(path, line_no, line, "screen width must be a number"))?;
    let right = right
        .trim()
        .parse::<usize>()
        .map_err(|_| config_err(path, line_no, line, "screen height must be a number"))?;
    Ok((left, right))
}

fn size_value(rest: &str, path: &Path, line_no: usize, line: &str) -> Result<usize, String> {
    let (number, unit) = rest
        .trim()
        .split_once('.')
        .ok_or_else(|| config_err(path, line_no, line, "expected a size like `16.kb`"))?;
    let number = number
        .parse::<usize>()
        .map_err(|_| config_err(path, line_no, line, "size number must be a number"))?;
    match unit {
        "kb" => Ok(number * 1024),
        "mb" => Ok(number * 1024 * 1024),
        _ => Err(config_err(
            path,
            line_no,
            line,
            "size unit must be `kb` or `mb`",
        )),
    }
}

fn bool_value(rest: &str, path: &Path, line_no: usize, line: &str) -> Result<bool, String> {
    match rest.trim() {
        "on" => Ok(true),
        "off" => Ok(false),
        _ => Err(config_err(path, line_no, line, "expected `on` or `off`")),
    }
}

fn unknown_key(path: &Path, line_no: usize, line: &str, block: &str, keys: &str) -> String {
    config_err(
        path,
        line_no,
        line,
        &format!("unknown `{}` item; use {}", block, keys),
    )
}

fn config_err(path: &Path, line_no: usize, line: &str, message: &str) -> String {
    format!(
        "{}:{}\n    {}\n\n{}",
        path.display(),
        line_no,
        line,
        message
    )
}

fn strip_comment(line: &str) -> &str {
    let mut in_string = false;
    let mut escaped = false;
    let bytes = line.as_bytes();
    let mut i = 0;

    while i + 1 < bytes.len() {
        let b = bytes[i];
        if in_string {
            if escaped {
                escaped = false;
            } else if b == b'\\' {
                escaped = true;
            } else if b == b'"' {
                in_string = false;
            }
        } else if b == b'"' {
            in_string = true;
        } else if b == b'/' && bytes[i + 1] == b'/' {
            return &line[..i];
        }
        i += 1;
    }

    line
}
