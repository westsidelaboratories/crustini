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
    Assign { name: String, expr: String },
    If { cond: String, body: Vec<Stmt> },
    Call { name: String, args: Vec<String> },
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
