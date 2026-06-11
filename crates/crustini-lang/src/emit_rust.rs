use crate::macros::{input, screen::ScreenMacro, verb::VerbMacro};
use crate::model::{App, Stmt};

pub fn emit_rust(app: &App) -> String {
    let state_names: Vec<String> = app.state.iter().map(|f| f.name.clone()).collect();
    let mut out = String::new();

    pushln(&mut out, "#![no_std]");
    pushln(&mut out, "");
    pushln(
        &mut out,
        "use crustini_core::{Buttons, CrustiniApp, Screen};",
    );
    pushln(&mut out, "");
    pushln(
        &mut out,
        &format!("pub const WIDTH: usize = {};", app.width),
    );
    pushln(
        &mut out,
        &format!("pub const HEIGHT: usize = {};", app.height),
    );
    pushln(&mut out, &format!("pub const FPS: usize = {};", app.fps));
    pushln(&mut out, "");

    pushln(&mut out, "pub struct App {");
    for field in &app.state {
        pushln(
            &mut out,
            &format!("    pub {}: {},", field.name, lower_type(&field.ty)),
        );
    }
    pushln(&mut out, "}");
    pushln(&mut out, "");

    pushln(&mut out, "impl App {");
    pushln(&mut out, "    pub const fn new() -> Self {");
    pushln(&mut out, "        Self {");
    for field in &app.state {
        let value = rewrite_expr(&field.value, &state_names, false);
        pushln(&mut out, &format!("            {}: {},", field.name, value));
    }
    pushln(&mut out, "        }");
    pushln(&mut out, "    }");
    pushln(&mut out, "}");
    pushln(&mut out, "");

    pushln(&mut out, "impl Default for App {");
    pushln(&mut out, "    fn default() -> Self {");
    pushln(&mut out, "        Self::new()");
    pushln(&mut out, "    }");
    pushln(&mut out, "}");
    pushln(&mut out, "");

    pushln(&mut out, "impl CrustiniApp for App {");
    pushln(
        &mut out,
        "    fn setup(&mut self, screen: &mut Screen<'_>) {",
    );
    if app.setup.is_empty() {
        pushln(&mut out, "        let _ = screen;");
    } else {
        emit_stmts(&mut out, &app.setup, &state_names, true, true, 2);
    }
    pushln(&mut out, "    }");
    pushln(&mut out, "");

    pushln(&mut out, "    fn update(&mut self, input: Buttons) {");
    if app.update.is_empty() {
        pushln(&mut out, "        let _ = input;");
    } else {
        if !stmts_use_input(&app.update) {
            pushln(&mut out, "        let _ = input;");
        }
        emit_stmts(&mut out, &app.update, &state_names, false, true, 2);
    }
    pushln(&mut out, "    }");
    pushln(&mut out, "");

    pushln(
        &mut out,
        "    fn draw(&mut self, screen: &mut Screen<'_>) {",
    );
    if app.draw.is_empty() {
        pushln(&mut out, "        let _ = screen;");
    } else {
        emit_stmts(&mut out, &app.draw, &state_names, true, false, 2);
    }
    pushln(&mut out, "    }");
    pushln(&mut out, "}");

    out
}

fn emit_stmts(
    out: &mut String,
    stmts: &[Stmt],
    state_names: &[String],
    screen_available: bool,
    input_available: bool,
    indent: usize,
) {
    for stmt in stmts {
        emit_stmt(
            out,
            stmt,
            state_names,
            screen_available,
            input_available,
            indent,
        );
    }
}

fn emit_stmt(
    out: &mut String,
    stmt: &Stmt,
    state_names: &[String],
    screen_available: bool,
    input_available: bool,
    indent: usize,
) {
    let pad = "    ".repeat(indent);

    match stmt {
        Stmt::Assign { name, op, expr } => {
            let expr = rewrite_expr(expr, state_names, input_available);
            pushln(
                out,
                &format!("{}self.{} {} {};", pad, name, op.rust_token(), expr),
            );
        }
        Stmt::If { cond, body } => {
            let cond = rewrite_expr(cond, state_names, input_available);
            pushln(out, &format!("{}if {} {{", pad, cond));
            emit_stmts(
                out,
                body,
                state_names,
                screen_available,
                input_available,
                indent + 1,
            );
            pushln(out, &format!("{}}}", pad));
        }
        Stmt::Call { name, args } => {
            let args: Vec<String> = args
                .iter()
                .map(|arg| rewrite_expr(arg, state_names, input_available))
                .collect();
            let args = args.join(", ");

            if screen_available {
                if let Some(screen_macro) = ScreenMacro::parse(name) {
                    pushln(
                        out,
                        &format!("{}screen.{}({});", pad, screen_macro.rust_method(), args),
                    );
                } else {
                    // Unknown calls intentionally lower to a method call.
                    // This keeps the compiler simple and lets Rust produce the final error.
                    pushln(out, &format!("{}self.{}({});", pad, name, args));
                }
            } else {
                // Unknown calls intentionally lower to a method call.
                // This keeps the compiler simple and lets Rust produce the final error.
                pushln(out, &format!("{}self.{}({});", pad, name, args));
            }
        }
    }
}

fn rewrite_expr(src: &str, state_names: &[String], input_available: bool) -> String {
    let mut out = String::new();
    let bytes = src.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        let b = bytes[i];
        if b == b'"' || b == b'\'' {
            let (literal, next) = read_quoted_literal(src, i, b);
            out.push_str(literal);
            i = next;
        } else if is_ident_start(b) {
            let start = i;
            i += 1;
            while i < bytes.len() && is_ident_continue(bytes[i]) {
                i += 1;
            }

            let ident = &src[start..i];
            let prev_is_dot = start > 0 && bytes[start - 1] == b'.';

            if prev_is_dot {
                out.push_str(ident);
            } else if ident == "Color" && src[i..].starts_with("::") {
                if let Some((color, next)) = read_path_variant(src, i + 2) {
                    out.push_str(color_value(color).unwrap_or("0"));
                    i = next;
                } else {
                    out.push_str(ident);
                }
            } else if i < bytes.len() && bytes[i] == b'!' {
                if let Some(verb_macro) = VerbMacro::parse(ident) {
                    out.push_str(verb_macro.spec().rust_path);
                    i += 1;
                } else {
                    out.push_str(ident);
                    out.push('!');
                    i += 1;
                }
            } else if let Some(verb_macro) = VerbMacro::parse(ident) {
                let call_start = skip_ws(src, i);
                if call_start < bytes.len() && bytes[call_start] == b'(' {
                    out.push_str(verb_macro.spec().rust_path);
                } else if contains_ident(state_names, ident) {
                    out.push_str("self.");
                    out.push_str(ident);
                } else {
                    out.push_str(ident);
                }
            } else if let Some((lowered, next)) = lower_input_call(src, ident, i, input_available) {
                out.push_str(&lowered);
                i = next;
            } else if contains_ident(state_names, ident) {
                out.push_str("self.");
                out.push_str(ident);
            } else if let Some(field) = input_available.then(|| input::field_name(ident)).flatten()
            {
                out.push_str("input.");
                out.push_str(field);
            } else {
                out.push_str(ident);
            }
        } else {
            out.push(b as char);
            i += 1;
        }
    }

    out
}

fn lower_type(ty: &str) -> &str {
    match ty.trim() {
        "number" => "i16",
        "text" => "&'static str",
        "bool" => "bool",
        "Color" => "u8",
        "i16" => "i16",
        "i32" => "i32",
        "u8" => "u8",
        "usize" => "usize",
        other => other,
    }
}

fn lower_input_call(
    src: &str,
    ident: &str,
    after_ident: usize,
    input_available: bool,
) -> Option<(String, usize)> {
    if !input_available {
        return None;
    }

    let call_start = skip_ws(src, after_ident);
    if call_start >= src.len() || src.as_bytes()[call_start] != b'(' {
        return None;
    }
    let close = find_matching_paren(src, call_start).ok()?;
    let args = src[call_start + 1..close].trim();

    let lowered = match ident {
        "pressed" | "down" => {
            let field = button_field(args)?;
            format!("input.{}", field)
        }
        "released" => "false".to_string(),
        "axis_x" if args.is_empty() => "((input.right as i16) - (input.left as i16))".to_string(),
        "axis_y" if args.is_empty() => "((input.down as i16) - (input.up as i16))".to_string(),
        "mouse_x" if args.is_empty() => "input.mouse_x".to_string(),
        "mouse_y" if args.is_empty() => "input.mouse_y".to_string(),
        "mouse_down" if args.is_empty() => "input.mouse_down".to_string(),
        "dt" if args.is_empty() => "1".to_string(),
        _ => return None,
    };

    Some((lowered, close + 1))
}

fn button_field(src: &str) -> Option<&'static str> {
    let name = src
        .trim()
        .strip_prefix("Button::")
        .unwrap_or(src.trim())
        .trim();

    match name {
        "Up" | "up" => Some("up"),
        "Down" | "down" => Some("down"),
        "Left" | "left" => Some("left"),
        "Right" | "right" => Some("right"),
        "A" | "a" => Some("a"),
        "B" | "b" => Some("b"),
        "Start" | "start" => Some("start"),
        "Select" | "select" => Some("select"),
        "Mouse" | "MouseLeft" | "mouse" | "mouse_down" => Some("mouse_down"),
        _ => None,
    }
}

fn color_value(name: &str) -> Option<&'static str> {
    match name {
        "Black" => Some("0"),
        "White" => Some("255"),
        "Gray" | "Grey" => Some("128"),
        "Red" => Some("224"),
        "Green" => Some("180"),
        "Blue" => Some("96"),
        "Yellow" => Some("240"),
        "Cyan" => Some("190"),
        "Magenta" => Some("210"),
        _ => None,
    }
}

fn read_path_variant(src: &str, start: usize) -> Option<(&str, usize)> {
    let bytes = src.as_bytes();
    if start >= bytes.len() || !is_ident_start(bytes[start]) {
        return None;
    }

    let mut i = start + 1;
    while i < bytes.len() && is_ident_continue(bytes[i]) {
        i += 1;
    }

    Some((&src[start..i], i))
}

fn skip_ws(src: &str, mut i: usize) -> usize {
    let bytes = src.as_bytes();
    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
        i += 1;
    }
    i
}

fn find_matching_paren(src: &str, open: usize) -> Result<usize, String> {
    let bytes = src.as_bytes();
    if open >= bytes.len() || bytes[open] != b'(' {
        return Err("internal error: expected `(`".to_string());
    }

    let mut depth = 0usize;
    let mut i = open;
    while i < bytes.len() {
        match bytes[i] {
            b'"' | b'\'' => {
                let (_, next) = read_quoted_literal(src, i, bytes[i]);
                i = next;
                continue;
            }
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return Ok(i);
                }
            }
            _ => {}
        }
        i += 1;
    }

    Err("unclosed `(`".to_string())
}

fn read_quoted_literal(src: &str, start: usize, quote: u8) -> (&str, usize) {
    let bytes = src.as_bytes();
    let mut i = start + 1;
    let mut escaped = false;

    while i < bytes.len() {
        let b = bytes[i];
        if escaped {
            escaped = false;
        } else if b == b'\\' {
            escaped = true;
        } else if b == quote {
            i += 1;
            return (&src[start..i], i);
        }
        i += 1;
    }

    (&src[start..], bytes.len())
}

fn contains_ident(items: &[String], ident: &str) -> bool {
    items.iter().any(|item| item == ident)
}

fn stmts_use_input(stmts: &[Stmt]) -> bool {
    stmts.iter().any(stmt_uses_input)
}

fn stmt_uses_input(stmt: &Stmt) -> bool {
    match stmt {
        Stmt::Assign { expr, .. } => expr_uses_input(expr),
        Stmt::If { cond, body } => expr_uses_input(cond) || stmts_use_input(body),
        Stmt::Call { args, .. } => args.iter().any(|arg| expr_uses_input(arg)),
    }
}

fn expr_uses_input(src: &str) -> bool {
    let bytes = src.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if is_ident_start(bytes[i]) {
            let start = i;
            i += 1;
            while i < bytes.len() && is_ident_continue(bytes[i]) {
                i += 1;
            }

            let ident = &src[start..i];
            let prev_is_dot = start > 0 && bytes[start - 1] == b'.';
            if !prev_is_dot && input::field_name(ident).is_some() {
                return true;
            }
            if !prev_is_dot
                && matches!(
                    ident,
                    "pressed"
                        | "down"
                        | "released"
                        | "axis_x"
                        | "axis_y"
                        | "mouse_x"
                        | "mouse_y"
                        | "mouse_down"
                        | "dt"
                )
            {
                return true;
            }
        } else {
            i += 1;
        }
    }

    false
}

fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_'
}

fn is_ident_continue(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

fn pushln(out: &mut String, line: &str) {
    out.push_str(line);
    out.push('\n');
}
