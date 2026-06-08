use crate::model::{App, StateField, Stmt};

pub fn scan_app(src: &str) -> Result<App, String> {
    let src = strip_line_comments(src);
    let app_pos = src.find("app!").ok_or("missing `app!` root block")?;
    let open = find_next_char(&src, app_pos, '{').ok_or("missing `{` after `app!`")?;
    let close = find_matching_brace(&src, open)?;
    let body = &src[open + 1..close];
    parse_app_body(body)
}

fn parse_app_body(body: &str) -> Result<App, String> {
    let mut app = App::default();
    let mut i = 0;

    while i < body.len() {
        i = skip_ws(body, i);
        if i >= body.len() {
            break;
        }

        if let Some((ident, after_ident)) = read_ident(body, i) {
            let j = skip_ws(body, after_ident);

            if j < body.len() && body.as_bytes()[j] == b'{' {
                let close = find_matching_brace(body, j)?;
                let block_body = &body[j + 1..close];

                match ident.as_str() {
                    "state" => app.state = parse_state_block(block_body)?,
                    "setup" => app.setup = parse_stmt_block(block_body)?,
                    "update" => app.update = parse_stmt_block(block_body)?,
                    "draw" => app.draw = parse_stmt_block(block_body)?,
                    other => return Err(format!("unknown block `{}`", other)),
                }

                i = close + 1;
                continue;
            }
        }

        let (line, next) = read_line(body, i);
        parse_top_level_line(line.trim(), &mut app)?;
        i = next;
    }

    Ok(app)
}

fn parse_top_level_line(line: &str, app: &mut App) -> Result<(), String> {
    if line.is_empty() {
        return Ok(());
    }

    if let Some(rest) = line.strip_prefix("screen") {
        let rest = rest.trim().trim_start_matches(':').trim();

        if let Some((w, h)) = rest.split_once('x') {
            app.width = parse_usize(w.trim(), "screen width")?;
            app.height = parse_usize(h.trim(), "screen height")?;
            return Ok(());
        }

        let parts: Vec<&str> = rest.split_whitespace().collect();
        if parts.len() != 2 {
            return Err(format!(
                "screen expects `screen WIDTH HEIGHT` or `screen WIDTHxHEIGHT`, got `{}`",
                line
            ));
        }

        app.width = parse_usize(parts[0], "screen width")?;
        app.height = parse_usize(parts[1], "screen height")?;
        return Ok(());
    }

    if let Some(rest) = line.strip_prefix("fps") {
        let rest = rest.trim().trim_start_matches(':').trim();
        app.fps = parse_usize(rest, "fps")?;
        return Ok(());
    }

    Err(format!("unknown top-level line `{}`", line))
}

fn parse_state_block(body: &str) -> Result<Vec<StateField>, String> {
    let mut fields = Vec::new();

    for raw in body.lines() {
        let line = raw.trim().trim_end_matches(';').trim();
        if line.is_empty() {
            continue;
        }

        let (name, rest) = line
            .split_once(':')
            .ok_or_else(|| format!("state field missing `:` in `{}`", line))?;
        let (ty, value) = rest
            .split_once('=')
            .ok_or_else(|| format!("state field missing `=` in `{}`", line))?;

        let name = name.trim();
        assert_ident(name, "state field name")?;

        fields.push(StateField {
            name: name.to_string(),
            ty: ty.trim().to_string(),
            value: value.trim().to_string(),
        });
    }

    Ok(fields)
}

fn parse_stmt_block(body: &str) -> Result<Vec<Stmt>, String> {
    let mut stmts = Vec::new();
    let mut i = 0;

    while i < body.len() {
        i = skip_ws(body, i);
        if i >= body.len() {
            break;
        }

        if starts_with_keyword(body, i, "if") {
            let after_if = skip_ws(body, i + 2);
            let open = find_next_char(body, after_if, '{')
                .ok_or_else(|| "if statement missing `{`".to_string())?;
            let cond = body[after_if..open].trim().to_string();
            let close = find_matching_brace(body, open)?;
            let nested = &body[open + 1..close];
            let nested_stmts = parse_stmt_block(nested)?;
            stmts.push(Stmt::If {
                cond,
                body: nested_stmts,
            });
            i = close + 1;
            continue;
        }

        let (line, next) = read_stmt_line(body, i);
        let line = line.trim().trim_end_matches(';').trim();
        if !line.is_empty() {
            stmts.push(parse_stmt_line(line)?);
        }
        i = next;
    }

    Ok(stmts)
}

fn parse_stmt_line(line: &str) -> Result<Stmt, String> {
    if let Some((name, expr)) = split_assignment(line) {
        let name = name.trim();
        assert_ident(name, "assignment target")?;
        return Ok(Stmt::Assign {
            name: name.to_string(),
            expr: expr.trim().to_string(),
        });
    }

    parse_call(line)
}

fn split_assignment(line: &str) -> Option<(&str, &str)> {
    let bytes = line.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'=' {
            let prev = if i > 0 { bytes[i - 1] } else { 0 };
            let next = if i + 1 < bytes.len() { bytes[i + 1] } else { 0 };
            if prev != b'=' && next != b'=' && prev != b'!' && prev != b'<' && prev != b'>' {
                return Some((&line[..i], &line[i + 1..]));
            }
        }
        i += 1;
    }
    None
}

fn parse_call(line: &str) -> Result<Stmt, String> {
    if let Some(paren) = line.find('(') {
        if !line.ends_with(')') {
            return Err(format!("call with `(` must end with `)`: `{}`", line));
        }
        let name = line[..paren].trim();
        assert_ident(name, "call name")?;
        let inner = &line[paren + 1..line.len() - 1];
        let args = split_comma_args(inner);
        return Ok(Stmt::Call {
            name: name.to_string(),
            args,
        });
    }

    let mut parts = line.split_whitespace();
    let name = parts.next().ok_or("empty call")?;
    assert_ident(name, "call name")?;
    let args = parts.map(|s| s.to_string()).collect();
    Ok(Stmt::Call {
        name: name.to_string(),
        args,
    })
}

fn split_comma_args(src: &str) -> Vec<String> {
    let mut args = Vec::new();
    for part in src.split(',') {
        let part = part.trim();
        if !part.is_empty() {
            args.push(part.to_string());
        }
    }
    args
}

fn strip_line_comments(src: &str) -> String {
    let mut out = String::new();
    for line in src.lines() {
        if let Some(pos) = line.find("//") {
            out.push_str(&line[..pos]);
        } else {
            out.push_str(line);
        }
        out.push('\n');
    }
    out
}

fn read_line(src: &str, start: usize) -> (&str, usize) {
    let bytes = src.as_bytes();
    let mut i = start;
    while i < bytes.len() && bytes[i] != b'\n' {
        i += 1;
    }
    (&src[start..i], if i < bytes.len() { i + 1 } else { i })
}

fn read_stmt_line(src: &str, start: usize) -> (&str, usize) {
    let bytes = src.as_bytes();
    let mut i = start;
    while i < bytes.len() && bytes[i] != b'\n' && bytes[i] != b'}' {
        i += 1;
    }
    (
        &src[start..i],
        if i < bytes.len() && bytes[i] == b'\n' {
            i + 1
        } else {
            i
        },
    )
}

fn read_ident(src: &str, start: usize) -> Option<(String, usize)> {
    let bytes = src.as_bytes();
    if start >= bytes.len() || !is_ident_start(bytes[start]) {
        return None;
    }

    let mut i = start + 1;
    while i < bytes.len() && is_ident_continue(bytes[i]) {
        i += 1;
    }

    Some((src[start..i].to_string(), i))
}

fn starts_with_keyword(src: &str, start: usize, kw: &str) -> bool {
    if !src[start..].starts_with(kw) {
        return false;
    }

    let before_ok = start == 0 || !is_ident_continue(src.as_bytes()[start - 1]);
    let end = start + kw.len();
    let after_ok = end >= src.len() || !is_ident_continue(src.as_bytes()[end]);

    before_ok && after_ok
}

fn skip_ws(src: &str, mut i: usize) -> usize {
    let bytes = src.as_bytes();
    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
        i += 1;
    }
    i
}

fn find_next_char(src: &str, start: usize, needle: char) -> Option<usize> {
    let needle = needle as u8;
    let bytes = src.as_bytes();
    let mut i = start;
    while i < bytes.len() {
        if bytes[i] == needle {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn find_matching_brace(src: &str, open: usize) -> Result<usize, String> {
    let bytes = src.as_bytes();
    if open >= bytes.len() || bytes[open] != b'{' {
        return Err("internal error: expected `{`".to_string());
    }

    let mut depth = 0usize;
    let mut i = open;
    while i < bytes.len() {
        match bytes[i] {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Ok(i);
                }
            }
            _ => {}
        }
        i += 1;
    }

    Err("unclosed `{` block".to_string())
}

fn parse_usize(src: &str, label: &str) -> Result<usize, String> {
    src.parse::<usize>()
        .map_err(|_| format!("invalid {} `{}`", label, src))
}

fn assert_ident(src: &str, label: &str) -> Result<(), String> {
    let bytes = src.as_bytes();
    if bytes.is_empty() || !is_ident_start(bytes[0]) {
        return Err(format!("invalid {} `{}`", label, src));
    }

    for &b in &bytes[1..] {
        if !is_ident_continue(b) {
            return Err(format!("invalid {} `{}`", label, src));
        }
    }

    Ok(())
}

fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_'
}

fn is_ident_continue(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}
