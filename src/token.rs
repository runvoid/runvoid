#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Directives
    Remove,
    Add,
    GarbageC,
    Basic,
    Advanced,

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

    // Identifiers
    Ident(String),

    // Operators & Punctuation
    Equal,          // =
    DoubleEqual,    // ==
    NotEqual,       // !=
    Less,           // <
    Greater,        // >
    LessEqual,      // <=
    GreaterEqual,   // >=

    Plus,           // +
    Minus,          // -
    Star,           // *
    Slash,          // /
    Percent,        // %

    OpenBrace,      // {
    CloseBrace,     // }
    OpenParen,      // (
    CloseParen,     // )
    Comma,          // ,
    Colon,          // :

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
