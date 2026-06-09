pub fn clamp_i16(value: i16, min: i16, max: i16) -> i16 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn hit_rect(ax: i16, ay: i16, aw: i16, ah: i16, bx: i16, by: i16, bw: i16, bh: i16) -> bool {
    ax < bx + bw && ax + aw > bx && ay < by + bh && ay + ah > by
}

pub fn abs_i16(value: i16) -> i16 {
    if value < 0 {
        -value
    } else {
        value
    }
}
