# Crustini Cheatsheet

## File Shape

```flour
+++
crustini = "0.1"
name = "My Sketch"
fps = 30
window = [220, 140]
+++

app! Main {
  state {
    x: number = 110;
  }

  fn update() {
    x += axis_x() * 3;
  }

  fn draw() {
    clear(Color::Black);
    circle(x, 70, 12, Color::White);
  }
}
```

## Useful Calls

- `clear(color)`
- `rect(x, y, w, h, color)`
- `circle(x, y, radius, color)`
- `line(x0, y0, x1, y1, color)`
- `text(x, y, "TEXT", color)`
- `pressed(Button::A)`
- `axis_x()`, `axis_y()`
- `mouse_x()`, `mouse_y()`, `mouse_down()`
- `dt()`
- `clamp(value, min, max)`

## Current Color Names

- `Color::Black`
- `Color::White`
- `Color::Gray`
- `Color::Red`
- `Color::Green`
- `Color::Blue`
- `Color::Yellow`
