#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputNameSpec {
    pub aliases: &'static [&'static str],
    pub field: &'static str,
}

pub const INPUT_NAMES: &[InputNameSpec] = &[
    InputNameSpec {
        aliases: &["up"],
        field: "up",
    },
    InputNameSpec {
        aliases: &["down"],
        field: "down",
    },
    InputNameSpec {
        aliases: &["left"],
        field: "left",
    },
    InputNameSpec {
        aliases: &["right"],
        field: "right",
    },
    InputNameSpec {
        aliases: &["a"],
        field: "a",
    },
    InputNameSpec {
        aliases: &["b"],
        field: "b",
    },
    InputNameSpec {
        aliases: &["start"],
        field: "start",
    },
    InputNameSpec {
        aliases: &["select"],
        field: "select",
    },
    InputNameSpec {
        aliases: &["mouse_x", "mousex"],
        field: "mouse_x",
    },
    InputNameSpec {
        aliases: &["mouse_y", "mousey"],
        field: "mouse_y",
    },
    InputNameSpec {
        aliases: &["mouse_down", "mousedown"],
        field: "mouse_down",
    },
];

pub fn field_name(name: &str) -> Option<&'static str> {
    INPUT_NAMES
        .iter()
        .find(|spec| spec.aliases.contains(&name))
        .map(|spec| spec.field)
}
