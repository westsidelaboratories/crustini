#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppMacro {
    Screen,
    Fps,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AppMacroSpec {
    pub name: &'static str,
    pub kind: AppMacro,
}

pub const APP_MACROS: &[AppMacroSpec] = &[
    AppMacroSpec {
        name: "screen",
        kind: AppMacro::Screen,
    },
    AppMacroSpec {
        name: "fps",
        kind: AppMacro::Fps,
    },
];

impl AppMacro {
    pub fn parse(name: &str) -> Option<Self> {
        APP_MACROS
            .iter()
            .find(|spec| spec.name == name)
            .map(|spec| spec.kind)
    }

    pub fn name(self) -> &'static str {
        APP_MACROS
            .iter()
            .find(|spec| spec.kind == self)
            .map(|spec| spec.name)
            .unwrap_or("unknown")
    }
}
