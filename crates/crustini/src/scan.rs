use crate::model::{App, StateField, Stmt};

struct Line {
    no: usize,
    indent: usize,
    text: String,
}

pub fn scan_app(src: &str) -> Result<App, String> {
    if src.contains('{') {
        return scan_braced_app(src);
    }

    scan_indented_app(src)
}

fn scan_indented_app(src: &str) -> Result<App, String> {
    let lines = lines(src)?;
    let root = lines.first().ok_or("empty source")?;
    if root.indent != 0 {
        return Err(err(root, "`app` must start at column 1"));
    }

    let root_text = section_name(&root.text);
    if root_text != "app" && root_text != "app!" {
        return Err(err(root, "missing root `app` line"));
    }

    let mut app = App::default();
    let mut i = 1;
    let app_indent = child_indent(&lines, i, root.indent);

    while i < lines.len() {
        let line = &lines[i];
        if line.indent <= root.indent {
            return Err(err(line, "unexpected line after `app` body"));
        }
        if Some(line.indent) != app_indent {
            return Err(err(line, "unexpected indentation"));
        }

        match section_name(&line.text).as_str() {
            "state" => {
                i += 1;
                app.state = parse_state(&lines, &mut i, line.indent)?;
            }
            "setup" => {
                i += 1;
                app.setup = parse_stmts(&lines, &mut i, line.indent)?;
            }
            "update" => {
                i += 1;
                app.update = parse_stmts(&lines, &mut i, line.indent)?;
            }
            "draw" => {
                i += 1;
                app.draw = parse_stmts(&lines, &mut i, line.indent)?;
            }
            _ => {
                parse_app_line(&line.text, &mut app).map_err(|msg| err(line, &msg))?;
                i += 1;
                reject_child(&lines, i, line.indent)?;
            }
        }
    }

    Ok(app)
}

fn scan_braced_app(src: &str) -> Result<App, String> {
    let src = strip_line_comments_braced(src);
    let app_pos = src.find("app!").ok_or("missing `app!` root macro")?;
    let open = find_next_char(&src, app_pos, '{').ok_or("missing `{` after `app!`")?;
    let close = find_matching_brace(&src, open)?;
    let body = &src[open + 1..close];
    parse_braced_app_body(body)
}

fn parse_braced_app_body(body: &str) -> Result<App, String> {
    let mut app = App::default();
    let mut i = 0;

    while i < body.len() {
        i = skip_ws(body, i);
        if i >= body.len() {
            break;
        }

        if let Some((ident, after_ident)) = read_ident(body, i) {
            let mut j = skip_ws(body, after_ident);
            if j < body.len() && body.as_bytes()[j] == b'!' {
                j = skip_ws(body, j + 1);
            }

            if j < body.len() && body.as_bytes()[j] == b'{' {
                let close = find_matching_brace(body, j)?;
                let block_body = &body[j + 1..close];

                match ident.as_str() {
                    "state" => app.state = parse_braced_state_block(block_body)?,
                    "setup" => app.setup = parse_braced_stmt_block(block_body)?,
                    "update" => app.update = parse_braced_stmt_block(block_body)?,
                    "draw" => app.draw = parse_braced_stmt_block(block_body)?,
                    other => return Err(format!("unknown block macro `{}`", other)),
                }

                i = close + 1;
                continue;
            }
        }

        let (line, next) = read_line(body, i);
        parse_braced_app_line(line.trim(), &mut app)?;
        i = next;
    }

    Ok(app)
}

fn parse_braced_app_line(line: &str, app: &mut App) -> Result<(), String> {
    let line = line.trim().trim_end_matches(';').trim();
    if line.is_empty() {
        return Ok(());
    }

    if let Some((name, args)) = parse_paren_call(line)? {
        match name.as_str() {
            "screen" => {
                parse_screen_args(&args, app)?;
                Ok(())
            }
            "fps" => {
                if args.len() != 1 {
                    return Err("fps! expects `fps!(FPS)`".to_string());
                }
                app.fps = int(&args[0], "fps")?;
                Ok(())
            }
            _ => Err(format!("unknown app macro `{}`", name)),
        }
    } else {
        parse_app_line(line, app)
    }
}

fn parse_screen_args(args: &[String], app: &mut App) -> Result<(), String> {
    if args.len() == 1 {
        if let Some((w, h)) = args[0].split_once('x') {
            app.width = int(w.trim(), "screen width")?;
            app.height = int(h.trim(), "screen height")?;
            return Ok(());
        }
    }

    if args.len() != 2 {
        return Err("screen! expects `screen!(WIDTH, HEIGHT)`".to_string());
    }

    app.width = int(&args[0], "screen width")?;
    app.height = int(&args[1], "screen height")?;
    Ok(())
}

fn parse_braced_state_block(body: &str) -> Result<Vec<StateField>, String> {
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
        ident(name, "state field name")?;

        fields.push(StateField {
            name: name.to_string(),
            ty: ty.trim().to_string(),
            value: value.trim().to_string(),
        });
    }

    Ok(fields)
}

fn parse_braced_stmt_block(body: &str) -> Result<Vec<Stmt>, String> {
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
            let body = parse_braced_stmt_block(nested)?;
            stmts.push(Stmt::If { cond, body });
            i = close + 1;
            continue;
        }

        let (line, next) = read_stmt_line(body, i);
        let line = line.trim().trim_end_matches(';').trim();
        if !line.is_empty() {
            stmts.push(parse_braced_stmt_line(line)?);
        }
        i = next;
    }

    Ok(stmts)
}

fn parse_braced_stmt_line(line: &str) -> Result<Stmt, String> {
    if let Some((name, expr)) = assignment(line) {
        let name = name.trim();
        ident(name, "assignment target")?;
        return Ok(Stmt::Assign {
            name: name.to_string(),
            expr: expr.trim().to_string(),
        });
    }

    if let Some((name, args)) = parse_paren_call(line)? {
        return Ok(Stmt::Call { name, args });
    }

    parse_stmt(line)
}

fn parse_paren_call(line: &str) -> Result<Option<(String, Vec<String>)>, String> {
    let Some(paren) = line.find('(') else {
        return Ok(None);
    };

    if !line.ends_with(')') {
        return Err(format!("call with `(` must end with `)`: `{}`", line));
    }

    let raw_name = line[..paren].trim();
    let name = raw_name.trim_end_matches('!').trim();
    ident(name, "call name")?;
    let inner = &line[paren + 1..line.len() - 1];
    Ok(Some((name.to_string(), split_comma_args(inner))))
}

fn split_comma_args(src: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut start = 0usize;
    let mut in_string = false;
    let mut escaped = false;

    for (i, ch) in src.char_indices() {
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }

        if ch == '"' {
            in_string = true;
        } else if ch == ',' {
            push_arg(&mut args, &src[start..i]);
            start = i + ch.len_utf8();
        }
    }

    push_arg(&mut args, &src[start..]);
    args
}

fn push_arg(args: &mut Vec<String>, arg: &str) {
    let arg = arg.trim();
    if !arg.is_empty() {
        args.push(arg.to_string());
    }
}

fn strip_line_comments_braced(src: &str) -> String {
    let mut out = String::new();
    for line in src.lines() {
        out.push_str(strip_comment(line));
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

fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_'
}

fn is_ident_continue(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

fn parse_app_line(text: &str, app: &mut App) -> Result<(), String> {
    let (head, rest) = split_head(text).ok_or("empty app line")?;
    let rest = rest.trim().trim_start_matches(':').trim();

    match head {
        "screen" => {
            if let Some((w, h)) = rest.split_once('x') {
                app.width = int(w.trim(), "screen width")?;
                app.height = int(h.trim(), "screen height")?;
                return Ok(());
            }

            let parts: Vec<&str> = rest.split_whitespace().collect();
            if parts.len() != 2 {
                return Err("screen expects `screen WIDTH HEIGHT`".to_string());
            }

            app.width = int(parts[0], "screen width")?;
            app.height = int(parts[1], "screen height")?;
            Ok(())
        }
        "fps" => {
            app.fps = int(rest, "fps")?;
            Ok(())
        }
        _ => Err(format!("unknown app line `{}`", text)),
    }
}

fn parse_state(lines: &[Line], i: &mut usize, parent: usize) -> Result<Vec<StateField>, String> {
    let mut fields = Vec::new();
    let indent = child_indent(lines, *i, parent);

    while *i < lines.len() {
        let line = &lines[*i];
        if line.indent <= parent {
            break;
        }
        if Some(line.indent) != indent {
            return Err(err(line, "unexpected indentation in state"));
        }

        let text = line.text.trim_end_matches(';').trim();
        let (name, rest) = text
            .split_once(':')
            .ok_or_else(|| err(line, "state field needs `name: Type = value`"))?;
        let (ty, value) = rest
            .split_once('=')
            .ok_or_else(|| err(line, "state field needs `name: Type = value`"))?;
        ident(name.trim(), "state field name").map_err(|msg| err(line, &msg))?;

        fields.push(StateField {
            name: name.trim().to_string(),
            ty: ty.trim().to_string(),
            value: value.trim().to_string(),
        });

        *i += 1;
        reject_child(lines, *i, line.indent)?;
    }

    Ok(fields)
}

fn parse_stmts(lines: &[Line], i: &mut usize, parent: usize) -> Result<Vec<Stmt>, String> {
    let mut stmts = Vec::new();
    let indent = child_indent(lines, *i, parent);

    while *i < lines.len() {
        let line = &lines[*i];
        if line.indent <= parent {
            break;
        }
        if Some(line.indent) != indent {
            return Err(err(line, "unexpected indentation in block"));
        }

        if let Some(cond) = if_cond(&line.text) {
            *i += 1;
            let body = parse_stmts(lines, i, line.indent)?;
            if body.is_empty() {
                return Err(err(line, "`if` needs an indented body"));
            }
            stmts.push(Stmt::If { cond, body });
            continue;
        }

        stmts.push(parse_stmt(&line.text).map_err(|msg| err(line, &msg))?);
        *i += 1;
        reject_child(lines, *i, line.indent)?;
    }

    Ok(stmts)
}

fn parse_stmt(text: &str) -> Result<Stmt, String> {
    let text = text.trim().trim_end_matches(';').trim();

    if let Some((name, expr)) = assignment(text) {
        let name = name.trim();
        ident(name, "assignment target")?;
        return Ok(Stmt::Assign {
            name: name.to_string(),
            expr: expr.trim().to_string(),
        });
    }

    let mut parts = text.split_whitespace();
    let name = parts.next().ok_or("empty statement")?;
    ident(name, "call name")?;
    Ok(Stmt::Call {
        name: name.to_string(),
        args: parts.map(str::to_string).collect(),
    })
}

fn if_cond(text: &str) -> Option<String> {
    let (head, rest) = split_head(text)?;
    if head != "if" {
        return None;
    }

    let cond = rest.trim().trim_end_matches(':').trim();
    (!cond.is_empty()).then(|| cond.to_string())
}

fn lines(src: &str) -> Result<Vec<Line>, String> {
    let mut out = Vec::new();

    for (n, raw) in src.lines().enumerate() {
        for b in raw.bytes() {
            match b {
                b' ' => {}
                b'\t' => return Err(format!("line {}: tabs are not valid indentation", n + 1)),
                _ => break,
            }
        }

        let raw = strip_comment(raw).trim_end();
        if raw.trim().is_empty() {
            continue;
        }

        let indent = raw.bytes().take_while(|b| *b == b' ').count();
        out.push(Line {
            no: n + 1,
            indent,
            text: raw[indent..].to_string(),
        });
    }

    Ok(out)
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

fn child_indent(lines: &[Line], i: usize, parent: usize) -> Option<usize> {
    lines.get(i).and_then(|line| {
        if line.indent > parent {
            Some(line.indent)
        } else {
            None
        }
    })
}

fn reject_child(lines: &[Line], i: usize, parent: usize) -> Result<(), String> {
    if let Some(line) = lines.get(i) {
        if line.indent > parent {
            return Err(err(line, "only sections and `if` can contain nested lines"));
        }
    }
    Ok(())
}

fn section_name(text: &str) -> String {
    text.trim().trim_end_matches(':').trim().to_string()
}

fn split_head(text: &str) -> Option<(&str, &str)> {
    let text = text.trim();
    match text.find(char::is_whitespace) {
        Some(i) => Some((&text[..i], &text[i..])),
        None if !text.is_empty() => Some((text, "")),
        None => None,
    }
}

fn assignment(text: &str) -> Option<(&str, &str)> {
    let bytes = text.as_bytes();
    for i in 0..bytes.len() {
        if bytes[i] != b'=' {
            continue;
        }

        let prev = i.checked_sub(1).map(|j| bytes[j]).unwrap_or_default();
        let next = bytes.get(i + 1).copied().unwrap_or_default();
        if prev != b'=' && prev != b'!' && prev != b'<' && prev != b'>' && next != b'=' {
            return Some((&text[..i], &text[i + 1..]));
        }
    }

    None
}

fn int(text: &str, label: &str) -> Result<usize, String> {
    text.parse::<usize>()
        .map_err(|_| format!("{} must be a positive integer, got `{}`", label, text))
}

fn ident(text: &str, label: &str) -> Result<(), String> {
    let mut chars = text.chars();
    match chars.next() {
        Some(ch) if ch.is_ascii_alphabetic() || ch == '_' => {}
        _ => return Err(format!("{} must be an identifier, got `{}`", label, text)),
    }
    if chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_') {
        Ok(())
    } else {
        Err(format!("{} must be an identifier, got `{}`", label, text))
    }
}

fn err(line: &Line, message: &str) -> String {
    format!("line {}: {}", line.no, message)
}
