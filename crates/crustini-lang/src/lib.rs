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
    fn new_flour_front_matter_and_app_form_compile() {
        let rust = compile_to_rust(
            r#"+++
crustini = "0.1"
name = "Mover"
fps = 24
window = [640, 360]
+++

app! Main {
  state {
    x: number = 40;
  }

  fn draw() {
    clear(Color::Black);
    x += 2;
    circle(x, 180, 24, Color::White);
  }
}
"#,
        )
        .unwrap();

        assert!(rust.contains("pub const WIDTH: usize = 640;"));
        assert!(rust.contains("pub const HEIGHT: usize = 360;"));
        assert!(rust.contains("pub const FPS: usize = 24;"));
        assert!(rust.contains("pub x: i16,"));
        assert!(rust.contains("fn draw(&mut self, screen: &mut Screen<'_>) {"));
        assert!(rust.contains("screen.clear(0);"));
        assert!(rust.contains("self.x += 2;"));
        assert!(rust.contains("screen.circle(self.x, 180, 24, 255);"));
    }

    #[test]
    fn new_input_helpers_lower_to_buttons_fields() {
        let rust = compile_to_rust(
            r#"+++
crustini = "0.1"
window = [180, 120]
+++

app! Main {
  state {
    x: number = 40;
  }

  fn update() {
    if pressed(Button::A) {
      x += axis_x();
    }
  }
}
"#,
        )
        .unwrap();

        assert!(rust.contains("if input.a {"));
        assert!(rust.contains("self.x += ((input.right as i16) - (input.left as i16));"));
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
    fn verb_macros_lower_to_runtime_helpers() {
        let rust = compile_to_rust(
            r#"app! {
  state! {
    x: i16 = 10
    y: i16 = 20
    hit: bool = false
  }

  update! {
    x = clamp!(x + 4, 0, 100)
    y = abs!(y)
    if hit_rect!(x, y, 4, 4, 10, 10, 8, 8) {
      hit = true
    }
  }

  draw! {
    rect!(clamp!(x, 0, 100), y, 4, 4, 255)
  }
}
"#,
        )
        .unwrap();

        assert!(rust.contains("self.x = crustini_core::clamp_i16(self.x + 4, 0, 100);"));
        assert!(rust.contains("self.y = crustini_core::abs_i16(self.y);"));
        assert!(rust.contains("if crustini_core::hit_rect(self.x, self.y, 4, 4, 10, 10, 8, 8) {"));
        assert!(rust
            .contains("screen.rect(crustini_core::clamp_i16(self.x, 0, 100), self.y, 4, 4, 255);"));
    }

    #[test]
    fn verb_macros_validate_argument_counts() {
        let err = compile_to_rust(
            r#"app! {
  update! {
    x = clamp!(1, 2)
  }
}
"#,
        )
        .unwrap_err();

        assert!(err.contains("clamp! expects 3 arguments, got 2"));
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
