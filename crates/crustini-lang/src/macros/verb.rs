#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VerbMacro {
    Clamp,
    HitRect,
    Abs,
    Min,
    Max,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VerbMacroSpec {
    pub name: &'static str,
    pub kind: VerbMacro,
    pub rust_path: &'static str,
    pub args: VerbArgs,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VerbArgs {
    Exact(usize),
}

pub const VERB_MACROS: &[VerbMacroSpec] = &[
    VerbMacroSpec {
        name: "clamp",
        kind: VerbMacro::Clamp,
        rust_path: "crustini_core::clamp_i16",
        args: VerbArgs::Exact(3),
    },
    VerbMacroSpec {
        name: "hit_rect",
        kind: VerbMacro::HitRect,
        rust_path: "crustini_core::hit_rect",
        args: VerbArgs::Exact(8),
    },
    VerbMacroSpec {
        name: "abs",
        kind: VerbMacro::Abs,
        rust_path: "crustini_core::abs_i16",
        args: VerbArgs::Exact(1),
    },
    VerbMacroSpec {
        name: "min",
        kind: VerbMacro::Min,
        rust_path: "core::cmp::min",
        args: VerbArgs::Exact(2),
    },
    VerbMacroSpec {
        name: "max",
        kind: VerbMacro::Max,
        rust_path: "core::cmp::max",
        args: VerbArgs::Exact(2),
    },
];

impl VerbMacro {
    pub fn parse(name: &str) -> Option<Self> {
        VERB_MACROS
            .iter()
            .find(|spec| spec.name == name)
            .map(|spec| spec.kind)
    }

    pub fn spec(self) -> &'static VerbMacroSpec {
        VERB_MACROS
            .iter()
            .find(|spec| spec.kind == self)
            .expect("verb macro registry missing enum variant")
    }
}
