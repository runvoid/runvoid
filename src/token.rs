#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Directives
    Remove,
    Add,
    GarbageC,
    Basic,
    Advanced,
    Linux,
    Freestanding,

    // Keywords
    Say,
    SaySame,
    Ask,
    Remember,
    If,
    Otherwise,
    Repeat,
    As,
    While,
    Stop,
    Skip,
    Action,
    Give,
    Run,

    // Version 0.2.0: Files, Time, Random, Modules, GUI
    Use,
    Read,
    Write,
    Into,
    Wait,
    Random,
    To,
    Window,
    Button,
    Label,
    Checkbox,
    Draw,

    // Beginner & System Extensions
    // Colors
    Green,
    Red,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    Color,
    Same,

    // Dialogs & Sound
    Alert,
    Beep,
    Speak,

    // Web & Internet
    Open,
    Web,
    Download,

    // 2D Screen & Shapes
    Screen,
    Circle,
    Box,
    Line,
    At,
    From,
    Size,

    // Console & Interaction
    Clear,
    Cursor,
    Hidden,
    Choose,

    // Files & Folders
    Folder,
    Create,
    Delete,
    Copy,
    File,
    Exists,
    All,
    In,

    // Strings
    Replace,
    With,
    Make,
    Uppercase,
    Lowercase,
    Trim,
    Starts,
    Ends,

    // Lists & Collections
    Has,
    For,
    Every,
    How,
    Many,
    Count,
    List,

    // Benchmarking
    Measure,
    Time,
    Cycles,

    // Pro Mode, Memory, Concurrency & FFI
    Asm,
    AsmBlock(String),
    Addr,
    Alloc,
    Free,
    Struct,
    Extern,
    Thread,
    Atomic,
    Lib,

    // Logical & comparison keywords
    Is,
    IsNot,
    And,
    Or,
    Not,

    // Literals
    IntLit(i64),
    BoolLit(bool),
    StringLit(String),

    // String interpolation tokens
    // E.g. "Hello, {name}!" ->
    // InterpStringBegin("Hello, "), Ident("name"), InterpStringEnd("!")
    InterpStringBegin(String),
    InterpStringMid(String),
    InterpStringEnd(String),

    // Types
    TypeInt,
    TypeString,
    TypeBool,
    TypePtr,

    // Identifiers
    Ident(String),

    // Operators & Punctuation
    Equal,        // =
    DoubleEqual,  // ==
    NotEqual,     // !=
    Less,         // <
    Greater,      // >
    LessEqual,    // <=
    GreaterEqual, // >=

    Plus,    // +
    Minus,   // -
    Star,    // *
    Slash,   // /
    Percent, // %

    AtSign, // @
    Dot,    // .
    Arrow,  // ->

    OpenBrace,  // {
    CloseBrace, // }
    OpenParen,  // (
    CloseParen, // )
    Comma,      // ,
    Colon,      // :

    Newline,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub col: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, col: usize) -> Self {
        Self { kind, line, col }
    }
}
