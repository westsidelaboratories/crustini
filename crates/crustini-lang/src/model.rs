#[derive(Debug, Clone)]
pub struct App {
    pub width: usize,
    pub height: usize,
    pub fps: usize,
    pub state: Vec<StateField>,
    pub setup: Vec<Stmt>,
    pub update: Vec<Stmt>,
    pub draw: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct StateField {
    pub name: String,
    pub ty: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Assign {
        name: String,
        op: AssignOp,
        expr: String,
    },
    If {
        cond: String,
        body: Vec<Stmt>,
    },
    Call {
        name: String,
        args: Vec<String>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignOp {
    Set,
    Add,
    Sub,
    Mul,
    Div,
}

impl AssignOp {
    pub fn rust_token(self) -> &'static str {
        match self {
            AssignOp::Set => "=",
            AssignOp::Add => "+=",
            AssignOp::Sub => "-=",
            AssignOp::Mul => "*=",
            AssignOp::Div => "/=",
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            width: 240,
            height: 135,
            fps: 30,
            state: Vec::new(),
            setup: Vec::new(),
            update: Vec::new(),
            draw: Vec::new(),
        }
    }
}
