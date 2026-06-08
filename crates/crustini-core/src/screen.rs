/// A tiny indexed-color framebuffer view.
///
/// This avoids unstable generic const expressions like `[u8; W * H]`.
/// The board/runtime can store the actual buffer wherever it wants and pass
/// a mutable slice into this type.
pub struct Screen<'a> {
    pub width: usize,
    pub height: usize,
    pub pixels: &'a mut [u8],
}

impl<'a> Screen<'a> {
    pub fn new(width: usize, height: usize, pixels: &'a mut [u8]) -> Self {
        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn clear(&mut self, color: u8) {
        let mut i = 0;
        while i < self.pixels.len() {
            self.pixels[i] = color;
            i += 1;
        }
    }

    pub fn set(&mut self, x: i16, y: i16, color: u8) {
        if x < 0 || y < 0 {
            return;
        }

        let x = x as usize;
        let y = y as usize;

        if x >= self.width || y >= self.height {
            return;
        }

        let idx = y * self.width + x;
        if idx < self.pixels.len() {
            self.pixels[idx] = color;
        }
    }

    pub fn rect(&mut self, x: i16, y: i16, w: i16, h: i16, color: u8) {
        if w <= 0 || h <= 0 {
            return;
        }

        let mut yy = 0;
        while yy < h {
            let mut xx = 0;
            while xx < w {
                self.set(x + xx, y + yy, color);
                xx += 1;
            }
            yy += 1;
        }
    }

    pub fn circle(&mut self, cx: i16, cy: i16, r: i16, color: u8) {
        if r <= 0 {
            return;
        }

        let rr = r * r;
        let mut y = -r;
        while y <= r {
            let mut x = -r;
            while x <= r {
                if x * x + y * y <= rr {
                    self.set(cx + x, cy + y, color);
                }
                x += 1;
            }
            y += 1;
        }
    }

    pub fn line(&mut self, mut x0: i16, mut y0: i16, x1: i16, y1: i16, color: u8) {
        let dx = abs_i16(x1 - x0);
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -abs_i16(y1 - y0);
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            self.set(x0, y0, color);

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = err * 2;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }
}

fn abs_i16(v: i16) -> i16 {
    if v < 0 {
        -v
    } else {
        v
    }
}
