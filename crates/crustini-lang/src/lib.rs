pub mod emit_rust;
pub mod macros;
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

    #[test]
    fn expression_rewrite_leaves_text_literals_alone() {
        let rust = compile_to_rust(
            r#"app! {
  state! {
    right: i16 = 1
    shade: u8 = 255
  }

  draw! {
    text!(1, 2, "right shade", shade)
  }
}
"#,
        )
        .unwrap();

        assert!(rust.contains(r#"screen.text(1, 2, "right shade", self.shade);"#));
    }

    #[test]
    fn screen_macros_validate_argument_counts() {
        let err = compile_to_rust(
            r#"app! {
  draw! {
    rect!(1, 2, 3, 4)
  }
}
"#,
        )
        .unwrap_err();

        assert!(err.contains("rect! expects 5 arguments, got 4"));
    }

    #[test]
    fn basic_input_names_lower_to_buttons_fields() {
        let rust = compile_to_rust(
            r#"app! {
  state! {
    x: i16 = 0
    y: i16 = 0
    pressed: bool = false
  }

  update! {
    if right {
      x = x + 1
    }

    if mouse_down {
      pressed = true
    }

    x = mouse_x
    y = mouse_y
  }
}
"#,
        )
        .unwrap();

        assert!(rust.contains("if input.right {"));
        assert!(rust.contains("if input.mouse_down {"));
        assert!(rust.contains("self.x = input.mouse_x;"));
        assert!(rust.contains("self.y = input.mouse_y;"));
    }

    #[test]
    fn compact_mouse_aliases_lower_to_mouse_fields() {
        let rust = compile_to_rust(
            r#"app! {
  state! {
    x: i16 = 0
    y: i16 = 0
  }

  update! {
    x = mousex
    y = mousey
  }
}
"#,
        )
        .unwrap();

        assert!(rust.contains("self.x = input.mouse_x;"));
        assert!(rust.contains("self.y = input.mouse_y;"));
    }
}
