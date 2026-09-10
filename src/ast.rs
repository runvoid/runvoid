#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    String,
    Bool,
    Void,
    Auto,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Directives {
    pub remove_gc: bool,
    pub remove_basic: bool,
    pub add_advanced: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub directives: Directives,
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(i64),
    Str(String),
    Bool(bool),
    Var(String),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Call {
        callee: String,
        args: Vec<Expr>,
    },
    InterpolatedString(Vec<Expr>),
    Ask(Box<Expr>),
    Run(Box<Expr>),
    ReadFile(Box<Expr>),
    Random {
        min: Box<Expr>,
        max: Box<Expr>,
    },
    ImrvDraw {
        element: String,
        label: Box<Expr>,
        extra: Option<Box<Expr>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Use(String),
    Say {
        expr: Expr,
        newline: bool,
    },
    Remember {
        name: String,
        explicit_type: Option<Type>,
        value: Expr,
    },
    Assign {
        name: String,
        value: Expr,
    },
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        otherwise_ifs: Vec<(Expr, Vec<Stmt>)>,
        otherwise_branch: Option<Vec<Stmt>>,
    },
    Repeat {
        count: Expr,
        var_name: Option<String>,
        body: Vec<Stmt>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    Stop,
    Skip,
    ActionDef {
        name: String,
        params: Vec<(String, Option<Type>)>,
        return_type: Option<Type>,
        body: Vec<Stmt>,
    },
    Give(Option<Expr>),
    RunCommand {
        command: Expr,
    },
    WriteFile {
        data: Expr,
        target: Expr,
    },
    Wait(Expr),
    Window {
        title: Expr,
        width: Option<Expr>,
        height: Option<Expr>,
        body: Vec<Stmt>,
    },
    GuiButton {
        label: Expr,
        action: Vec<Stmt>,
    },
    GuiLabel {
        text: Expr,
    },
    GuiCheckbox {
        label: Expr,
        initial_val: Option<Expr>,
    },
    ImrvStmt {
        element: String,
        label: Expr,
        extra: Option<Expr>,
    },
    ExprStmt(Expr),
}
