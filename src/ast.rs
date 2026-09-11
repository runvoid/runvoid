#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    String,
    Bool,
    Void,
    Auto,
    Ptr,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Directives {
    pub remove_gc: bool,
    pub remove_basic: bool,
    pub add_advanced: bool,
    pub remove_linux: bool,
    pub add_freestanding: bool,
    pub modules: Vec<String>,
    pub libs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExternAction {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub return_type: Type,
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

    // Beginner & System Expressions
    AskUser(Box<Expr>),
    AskHidden(Box<Expr>),
    Choose {
        prompt: Box<Expr>,
        options: Vec<Expr>,
    },
    ReadWeb(Box<Expr>),
    FileExists(Box<Expr>),
    ListLiteral(Vec<Expr>),
    ListHas {
        list: String,
        item: Box<Expr>,
    },
    ListCount(String),
    StrReplace {
        target: Box<Expr>,
        replacement: Box<Expr>,
        source: Box<Expr>,
    },
    StrStartsWith {
        source: Box<Expr>,
        prefix: Box<Expr>,
    },
    StrEndsWith {
        source: Box<Expr>,
        suffix: Box<Expr>,
    },

    // Pro Mode, Memory & Struct Expressions
    AddrOf(String),
    Deref(Box<Expr>),
    Alloc(Box<Expr>),
    Free(Box<Expr>),
    FieldAccess {
        target: Box<Expr>,
        field: String,
    },
    StructInit {
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum MakeStringOp {
    Uppercase,
    Lowercase,
    Trim,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Use(String),
    Say {
        expr: Expr,
        newline: bool,
        color: Option<String>,
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
    Alert(Expr),
    Beep,
    Speak(Expr),
    OpenWeb(Expr),
    DownloadWeb {
        url: Expr,
        target: Expr,
    },
    Screen {
        title: Expr,
        width: Option<Expr>,
        height: Option<Expr>,
        body: Vec<Stmt>,
    },
    DrawCircle {
        x: Expr,
        y: Expr,
        radius: Expr,
        color: Option<Expr>,
    },
    DrawRect {
        x: Expr,
        y: Expr,
        width: Expr,
        height: Expr,
        color: Option<Expr>,
    },
    DrawLine {
        x1: Expr,
        y1: Expr,
        x2: Expr,
        y2: Expr,
        color: Option<Expr>,
    },
    DrawText {
        text: Expr,
        x: Expr,
        y: Expr,
        color: Option<Expr>,
    },
    ClearScreen,
    CursorAt {
        x: Expr,
        y: Expr,
    },
    CreateFolder(Expr),
    DeleteFile(Expr),
    DeleteFolder(Expr),
    CopyFile {
        src: Expr,
        dest: Expr,
    },
    AddToList {
        item: Expr,
        list: String,
    },
    RemoveFromList {
        item: Expr,
        list: String,
    },
    ForEvery {
        item_var: String,
        list_var: String,
        body: Vec<Stmt>,
    },
    MakeString {
        var_name: String,
        op: MakeStringOp,
    },
    MeasureTime {
        body: Vec<Stmt>,
    },
    ExprStmt(Expr),

    // Pro Mode, Low-Level, Concurrency & FFI Statements
    UseLib(String),
    InlineAsm(String),
    MeasureCycles {
        body: Vec<Stmt>,
    },
    DerefAssign {
        ptr_expr: Expr,
        value_expr: Expr,
    },
    FieldAssign {
        target: String,
        field: String,
        value: Expr,
    },
    StructDef {
        name: String,
        fields: Vec<(String, Type)>,
    },
    ExternBlock {
        abi: String,
        actions: Vec<ExternAction>,
    },
    ThreadSpawn {
        body: Vec<Stmt>,
    },
    AtomicAdd {
        var: String,
        val: Expr,
    },
}
