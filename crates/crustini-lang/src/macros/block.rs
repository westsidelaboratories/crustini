#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockMacro {
    State,
    Setup,
    Update,
    Draw,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockMacroSpec {
    pub name: &'static str,
    pub kind: BlockMacro,
}

pub const BLOCK_MACROS: &[BlockMacroSpec] = &[
    BlockMacroSpec {
        name: "state",
        kind: BlockMacro::State,
    },
    BlockMacroSpec {
        name: "setup",
        kind: BlockMacro::Setup,
    },
    BlockMacroSpec {
        name: "update",
        kind: BlockMacro::Update,
    },
    BlockMacroSpec {
        name: "draw",
        kind: BlockMacro::Draw,
    },
];

impl BlockMacro {
    pub fn parse(name: &str) -> Option<Self> {
        BLOCK_MACROS
            .iter()
            .find(|spec| spec.name == name)
            .map(|spec| spec.kind)
    }

    pub fn name(self) -> &'static str {
        BLOCK_MACROS
            .iter()
            .find(|spec| spec.kind == self)
            .map(|spec| spec.name)
            .unwrap_or("unknown")
    }
}
