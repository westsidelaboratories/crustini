#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScreenMacro {
    Clear,
    Set,
    Pixel,
    Rect,
    Circle,
    Line,
    Text,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScreenMacroSpec {
    pub name: &'static str,
    pub kind: ScreenMacro,
    pub rust_method: &'static str,
    pub args: ScreenArgs,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScreenArgs {
    Exact(usize),
}

pub const SCREEN_MACROS: &[ScreenMacroSpec] = &[
    ScreenMacroSpec {
        name: "clear",
        kind: ScreenMacro::Clear,
        rust_method: "clear",
        args: ScreenArgs::Exact(1),
    },
    ScreenMacroSpec {
        name: "set",
        kind: ScreenMacro::Set,
        rust_method: "set",
        args: ScreenArgs::Exact(3),
    },
    ScreenMacroSpec {
        name: "pixel",
        kind: ScreenMacro::Pixel,
        rust_method: "set",
        args: ScreenArgs::Exact(3),
    },
    ScreenMacroSpec {
        name: "rect",
        kind: ScreenMacro::Rect,
        rust_method: "rect",
        args: ScreenArgs::Exact(5),
    },
    ScreenMacroSpec {
        name: "circle",
        kind: ScreenMacro::Circle,
        rust_method: "circle",
        args: ScreenArgs::Exact(4),
    },
    ScreenMacroSpec {
        name: "line",
        kind: ScreenMacro::Line,
        rust_method: "line",
        args: ScreenArgs::Exact(5),
    },
    ScreenMacroSpec {
        name: "text",
        kind: ScreenMacro::Text,
        rust_method: "text",
        args: ScreenArgs::Exact(4),
    },
];

impl ScreenMacro {
    pub fn parse(name: &str) -> Option<Self> {
        SCREEN_MACROS
            .iter()
            .find(|spec| spec.name == name)
            .map(|spec| spec.kind)
    }

    pub fn spec(self) -> &'static ScreenMacroSpec {
        SCREEN_MACROS
            .iter()
            .find(|spec| spec.kind == self)
            .expect("screen macro registry missing enum variant")
    }

    pub fn rust_method(self) -> &'static str {
        self.spec().rust_method
    }
}
