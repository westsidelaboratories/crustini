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

    pub fn text(&mut self, x: i16, mut y: i16, text: &str, color: u8) {
        let start_x = x;
        let mut x = x;

        for byte in text.bytes() {
            if byte == b'\n' {
                x = start_x;
                y += 8;
                continue;
            }

            draw_glyph(self, x, y, glyph_rows(byte), color);
            x += 6;
        }
    }
}

fn draw_glyph(screen: &mut Screen<'_>, x: i16, y: i16, rows: [u8; 7], color: u8) {
    let mut row = 0usize;
    while row < rows.len() {
        let bits = rows[row];
        let mut col = 0usize;
        while col < 5 {
            if bits & (1 << (4 - col)) != 0 {
                screen.set(x + col as i16, y + row as i16, color);
            }
            col += 1;
        }
        row += 1;
    }
}

fn glyph_rows(byte: u8) -> [u8; 7] {
    match byte {
        b' ' => [
            0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000,
        ],
        b'!' => [
            0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00000, 0b00100,
        ],
        b'?' => [
            0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b00000, 0b00100,
        ],
        b'0' => [
            0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110,
        ],
        b'1' => [
            0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
        ],
        b'2' => [
            0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111,
        ],
        b'3' => [
            0b11110, 0b00001, 0b00001, 0b01110, 0b00001, 0b00001, 0b11110,
        ],
        b'4' => [
            0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010,
        ],
        b'5' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b00001, 0b00001, 0b11110,
        ],
        b'6' => [
            0b01110, 0b10000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110,
        ],
        b'7' => [
            0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000,
        ],
        b'8' => [
            0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110,
        ],
        b'9' => [
            0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00001, 0b01110,
        ],
        b'A' | b'a' => [
            0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        b'B' | b'b' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110,
        ],
        b'C' | b'c' => [
            0b01110, 0b10001, 0b10000, 0b10000, 0b10000, 0b10001, 0b01110,
        ],
        b'D' | b'd' => [
            0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110,
        ],
        b'E' | b'e' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111,
        ],
        b'F' | b'f' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        b'G' | b'g' => [
            0b01110, 0b10001, 0b10000, 0b10111, 0b10001, 0b10001, 0b01111,
        ],
        b'H' | b'h' => [
            0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        b'I' | b'i' => [
            0b01110, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
        ],
        b'J' | b'j' => [
            0b00111, 0b00010, 0b00010, 0b00010, 0b10010, 0b10010, 0b01100,
        ],
        b'K' | b'k' => [
            0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001,
        ],
        b'L' | b'l' => [
            0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111,
        ],
        b'M' | b'm' => [
            0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001,
        ],
        b'N' | b'n' => [
            0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001,
        ],
        b'O' | b'o' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        b'P' | b'p' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        b'Q' | b'q' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101,
        ],
        b'R' | b'r' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001,
        ],
        b'S' | b's' => [
            0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110,
        ],
        b'T' | b't' => [
            0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        b'U' | b'u' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        b'V' | b'v' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b01010, 0b00100,
        ],
        b'W' | b'w' => [
            0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b11011, 0b10001,
        ],
        b'X' | b'x' => [
            0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001,
        ],
        b'Y' | b'y' => [
            0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        b'Z' | b'z' => [
            0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111,
        ],
        _ => [
            0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b00000, 0b00100,
        ],
    }
}

fn abs_i16(v: i16) -> i16 {
    if v < 0 {
        -v
    } else {
        v
    }
}
