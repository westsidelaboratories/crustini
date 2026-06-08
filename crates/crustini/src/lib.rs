pub mod cli;
pub mod config;
pub mod emit_rust;
pub mod model;
pub mod scan;

pub fn compile_to_rust(src: &str) -> Result<String, String> {
    let app = scan::scan_app(src)?;
    Ok(emit_rust::emit_rust(&app))
}

pub fn compile_to_rust_with_screen(
    src: &str,
    width: usize,
    height: usize,
    fps: usize,
) -> Result<String, String> {
    let mut app = scan::scan_app(src)?;
    app.width = width;
    app.height = height;
    app.fps = fps;
    Ok(emit_rust::emit_rust(&app))
}

#[cfg(test)]
mod tests {
    use super::compile_to_rust;

    #[test]
    fn text_macro_lowers_to_screen_text() {
        let rust = compile_to_rust(
            r#"app! {
  draw! {
    text!(1, 2, "HELLO, WORLD!", 255)
  }
}
"#,
        )
        .unwrap();

        assert!(rust.contains(r#"screen.text(1, 2, "HELLO, WORLD!", 255);"#));
    }
}
