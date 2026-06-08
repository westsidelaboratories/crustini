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
        pushln(&mut out, &format!("    pub {}: {},", field.name, field.ty));
    }
    pushln(&mut out, "}");
    pushln(&mut out, "");

    pushln(&mut out, "impl App {");
    pushln(&mut out, "    pub const fn new() -> Self {");
    pushln(&mut out, "        Self {");
    for field in &app.state {
        pushln(
            &mut out,
            &format!("            {}: {},", field.name, field.value),
        );
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

    pushln(&mut out, "    fn draw(&self, screen: &mut Screen<'_>) {");
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
        Stmt::Assign { name, expr } => {
            let expr = rewrite_expr(expr, state_names, input_available);
            pushln(out, &format!("{}self.{} = {};", pad, name, expr));
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

            if screen_available && is_screen_call(name) {
                let rust_name = screen_method_name(name);
                pushln(out, &format!("{}screen.{}({});", pad, rust_name, args));
            } else {
                // Unknown calls intentionally lower to a method call.
                // This keeps the compiler simple and lets Rust produce the final error.
                pushln(out, &format!("{}self.{}({});", pad, name, args));
            }
        }
    }
}

fn is_screen_call(name: &str) -> bool {
    matches!(name, "clear" | "pixel" | "set" | "rect" | "circle" | "line")
}

fn screen_method_name(name: &str) -> &str {
    match name {
        "pixel" => "set",
        other => other,
    }
}

fn rewrite_expr(src: &str, state_names: &[String], input_available: bool) -> String {
    let mut out = String::new();
    let bytes = src.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        let b = bytes[i];
        if is_ident_start(b) {
            let start = i;
            i += 1;
            while i < bytes.len() && is_ident_continue(bytes[i]) {
                i += 1;
            }

            let ident = &src[start..i];
            let prev_is_dot = start > 0 && bytes[start - 1] == b'.';

            if prev_is_dot {
                out.push_str(ident);
            } else if contains_ident(state_names, ident) {
                out.push_str("self.");
                out.push_str(ident);
            } else if input_available && is_input_ident(ident) {
                out.push_str("input.");
                out.push_str(ident);
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

fn contains_ident(items: &[String], ident: &str) -> bool {
    items.iter().any(|item| item == ident)
}

fn is_input_ident(ident: &str) -> bool {
    matches!(ident, "up" | "down" | "left" | "right" | "a" | "b")
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
            if !prev_is_dot && is_input_ident(ident) {
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
