use crate::token::{Token, TokenKind};

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
    interp_stack: Vec<usize>, // brace depths for active interpolations
    current_brace_depth: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
            interp_stack: Vec::new(),
            current_brace_depth: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while !self.is_eof() {
            let ch = self.peek();

            // Handle comments (# and //)
            if ch == '#' || (ch == '/' && self.peek_ahead(1) == '/') {
                self.skip_comment();
                continue;
            }

            // Handle whitespace
            if ch.is_whitespace() {
                if ch == '\n' {
                    let tok_line = self.line;
                    let tok_col = self.col;
                    self.advance();
                    // Optional: keep track of newlines if needed, or ignore
                    // In runvoid, braces delineate blocks, so newlines can be separators
                    tokens.push(Token::new(TokenKind::Newline, tok_line, tok_col));
                } else {
                    self.advance();
                }
                continue;
            }

            // String literals and interpolations
            if ch == '"' {
                let start_line = self.line;
                let start_col = self.col;
                self.advance(); // consume opening '"'
                self.lex_string(&mut tokens, start_line, start_col, true)?;
                continue;
            }

            if ch == '\'' {
                let start_line = self.line;
                let start_col = self.col;
                let is_possessive = if self.peek_ahead(1) == 's' {
                    let after_s = self.peek_ahead(2);
                    let valid_follower = after_s == '\0'
                        || (!after_s.is_alphanumeric() && after_s != '_' && after_s != '\'');
                    let prev_is_expr = tokens
                        .last()
                        .map(|t| {
                            matches!(
                                t.kind,
                                TokenKind::Ident(_)
                                    | TokenKind::CloseParen
                                    | TokenKind::CloseBracket
                            )
                        })
                        .unwrap_or(false);
                    prev_is_expr && valid_follower
                } else {
                    false
                };

                if is_possessive {
                    self.advance(); // consume '\''
                    self.advance(); // consume 's'
                    tokens.push(Token::new(TokenKind::ApostropheS, start_line, start_col));
                    continue;
                } else {
                    self.advance(); // consume opening '\''
                    let tok = self.lex_single_quoted_string(start_line, start_col)?;
                    tokens.push(tok);
                    continue;
                }
            }

            // Numbers
            if ch.is_ascii_digit() {
                let start_line = self.line;
                let start_col = self.col;
                let num = self.lex_number()?;
                tokens.push(Token::new(TokenKind::IntLit(num), start_line, start_col));
                continue;
            }

            // Identifiers or Keywords
            if ch.is_alphabetic() || ch == '_' {
                let start_line = self.line;
                let start_col = self.col;
                let ident = self.lex_ident();

                if ident == "asm" {
                    let mut look = self.pos;
                    while look < self.chars.len() && self.chars[look].is_whitespace() {
                        look += 1;
                    }
                    if look < self.chars.len() && self.chars[look] == '{' {
                        while self.pos <= look {
                            let c = self.chars[self.pos];
                            if c == '\n' {
                                self.line += 1;
                                self.col = 1;
                            } else {
                                self.col += 1;
                            }
                            self.pos += 1;
                        }
                        let mut asm_code = String::new();
                        let mut brace_depth = 1;
                        while self.pos < self.chars.len() && brace_depth > 0 {
                            let c = self.chars[self.pos];
                            if c == '\n' {
                                self.line += 1;
                                self.col = 1;
                            } else {
                                self.col += 1;
                            }
                            self.pos += 1;
                            if c == '{' {
                                brace_depth += 1;
                                asm_code.push(c);
                            } else if c == '}' {
                                brace_depth -= 1;
                                if brace_depth > 0 {
                                    asm_code.push(c);
                                }
                            } else {
                                asm_code.push(c);
                            }
                        }
                        tokens.push(Token::new(
                            TokenKind::AsmBlock(asm_code.trim().to_string()),
                            start_line,
                            start_col,
                        ));
                        continue;
                    }
                }

                let kind = match ident.as_str() {
                    "remove" => TokenKind::Remove,
                    "add" => TokenKind::Add,
                    "garbageC" => TokenKind::GarbageC,
                    "Basic" => TokenKind::Basic,
                    "Advanced" => TokenKind::Advanced,
                    "Linux" => TokenKind::Linux,
                    "Freestanding" => TokenKind::Freestanding,

                    "asm" => TokenKind::Asm,
                    "cycles" => TokenKind::Cycles,
                    "addr" => TokenKind::Addr,
                    "alloc" => TokenKind::Alloc,
                    "free" => TokenKind::Free,
                    "struct" => TokenKind::Struct,
                    "extern" => TokenKind::Extern,
                    "thread" => TokenKind::Thread,
                    "atomic" => TokenKind::Atomic,
                    "lib" => TokenKind::Lib,

                    "say" => TokenKind::Say,
                    "say_same" => TokenKind::SaySame,
                    "ask" => TokenKind::Ask,
                    "remember" => TokenKind::Remember,
                    "if" => TokenKind::If,
                    "otherwise" => TokenKind::Otherwise,
                    "repeat" => TokenKind::Repeat,
                    "as" => TokenKind::As,
                    "while" => TokenKind::While,
                    "stop" => TokenKind::Stop,
                    "skip" => TokenKind::Skip,
                    "action" => TokenKind::Action,
                    "give" => TokenKind::Give,
                    "run" => TokenKind::Run,

                    "use" => TokenKind::Use,
                    "read" => TokenKind::Read,
                    "write" => TokenKind::Write,
                    "into" => TokenKind::Into,
                    "wait" => TokenKind::Wait,
                    "random" => TokenKind::Random,
                    "to" => TokenKind::To,
                    "window" => TokenKind::Window,
                    "button" => TokenKind::Button,
                    "label" => TokenKind::Label,
                    "checkbox" => TokenKind::Checkbox,
                    "draw" => TokenKind::Draw,

                    "green" => TokenKind::Green,
                    "red" => TokenKind::Red,
                    "blue" => TokenKind::Blue,
                    "yellow" => TokenKind::Yellow,
                    "cyan" => TokenKind::Cyan,
                    "magenta" => TokenKind::Magenta,
                    "color" => TokenKind::Color,
                    "same" => TokenKind::Same,

                    "alert" => TokenKind::Alert,
                    "beep" => TokenKind::Beep,
                    "speak" => TokenKind::Speak,

                    "open" => TokenKind::Open,
                    "web" => TokenKind::Web,
                    "download" => TokenKind::Download,

                    "screen" => TokenKind::Screen,
                    "circle" => TokenKind::Circle,
                    "box" => TokenKind::Box,
                    "line" => TokenKind::Line,
                    "at" => TokenKind::At,
                    "from" => TokenKind::From,
                    "size" => TokenKind::Size,

                    "clear" => TokenKind::Clear,
                    "cursor" => TokenKind::Cursor,
                    "hidden" => TokenKind::Hidden,
                    "choose" => TokenKind::Choose,

                    "folder" => TokenKind::Folder,
                    "create" => TokenKind::Create,
                    "delete" => TokenKind::Delete,
                    "copy" => TokenKind::Copy,
                    "file" => TokenKind::File,
                    "exists" => TokenKind::Exists,
                    "all" => TokenKind::All,
                    "in" => TokenKind::In,

                    "replace" => TokenKind::Replace,
                    "with" => TokenKind::With,
                    "make" => TokenKind::Make,
                    "uppercase" => TokenKind::Uppercase,
                    "lowercase" => TokenKind::Lowercase,
                    "trim" => TokenKind::Trim,
                    "starts" => TokenKind::Starts,
                    "ends" => TokenKind::Ends,

                    "has" => TokenKind::Has,
                    "for" => TokenKind::For,
                    "every" => TokenKind::Every,
                    "how" => TokenKind::How,
                    "many" => TokenKind::Many,
                    "count" => TokenKind::Count,
                    "list" => TokenKind::List,

                    "measure" => TokenKind::Measure,
                    "time" => TokenKind::Time,

                    "is" => TokenKind::Is,
                    "and" => TokenKind::And,
                    "or" => TokenKind::Or,
                    "not" => TokenKind::Not,

                    "true" => TokenKind::BoolLit(true),
                    "false" => TokenKind::BoolLit(false),

                    "Int" => TokenKind::TypeInt,
                    "String" => TokenKind::TypeString,
                    "Bool" => TokenKind::TypeBool,
                    "Ptr" => TokenKind::TypePtr,

                    "match" => TokenKind::Match,
                    "when" => TokenKind::When,
                    "verify" => TokenKind::Verify,
                    "test" => TokenKind::Test,
                    "map" => TokenKind::Map,
                    "keys" => TokenKind::Keys,
                    "values" => TokenKind::Values,
                    "play" => TokenKind::Play,
                    "synth" => TokenKind::Synth,
                    "duration" => TokenKind::Duration,
                    "freq" => TokenKind::Freq,
                    "bit" => TokenKind::Bit,
                    "shift" => TokenKind::Shift,
                    "left" => TokenKind::Left,
                    "right" => TokenKind::Right,
                    "xor" => TokenKind::Xor,

                    _ => TokenKind::Ident(ident),
                };
                tokens.push(Token::new(kind, start_line, start_col));
                continue;
            }

            // Symbols & Operators
            let start_line = self.line;
            let start_col = self.col;
            let kind = match ch {
                '{' => {
                    self.current_brace_depth += 1;
                    self.advance();
                    TokenKind::OpenBrace
                }
                '}' => {
                    self.advance();
                    if self.interp_stack.last() == Some(&self.current_brace_depth) {
                        // Interpolation block ends, resume string scanning!
                        self.interp_stack.pop();
                        self.current_brace_depth -= 1;
                        self.lex_string(&mut tokens, start_line, start_col, false)?;
                        continue;
                    }
                    if self.current_brace_depth > 0 {
                        self.current_brace_depth -= 1;
                    }
                    TokenKind::CloseBrace
                }
                '(' => {
                    self.advance();
                    TokenKind::OpenParen
                }
                ')' => {
                    self.advance();
                    TokenKind::CloseParen
                }
                ',' => {
                    self.advance();
                    TokenKind::Comma
                }
                ':' => {
                    self.advance();
                    TokenKind::Colon
                }
                '+' => {
                    self.advance();
                    TokenKind::Plus
                }
                '-' => {
                    self.advance();
                    if self.peek() == '>' {
                        self.advance();
                        TokenKind::Arrow
                    } else {
                        TokenKind::Minus
                    }
                }
                '*' => {
                    self.advance();
                    TokenKind::Star
                }
                '@' => {
                    self.advance();
                    TokenKind::AtSign
                }
                '.' => {
                    self.advance();
                    TokenKind::Dot
                }
                '/' => {
                    self.advance();
                    TokenKind::Slash
                }
                '%' => {
                    self.advance();
                    TokenKind::Percent
                }
                '=' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        TokenKind::DoubleEqual
                    } else {
                        TokenKind::Equal
                    }
                }
                '!' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        TokenKind::NotEqual
                    } else {
                        return Err(format!(
                            "Unexpected character '!' at line {}, col {}. Did you mean '!=' or 'not'?",
                            start_line, start_col
                        ));
                    }
                }
                '[' => {
                    self.advance();
                    TokenKind::OpenBracket
                }
                ']' => {
                    self.advance();
                    TokenKind::CloseBracket
                }
                '|' => {
                    self.advance();
                    if self.peek() == '>' {
                        self.advance();
                        TokenKind::PipeGreater
                    } else if self.peek() == '|' {
                        self.advance();
                        TokenKind::Or
                    } else {
                        TokenKind::Pipe
                    }
                }
                '&' => {
                    self.advance();
                    if self.peek() == '&' {
                        self.advance();
                        TokenKind::And
                    } else {
                        TokenKind::Ampersand
                    }
                }
                '^' => {
                    self.advance();
                    TokenKind::Caret
                }
                '~' => {
                    self.advance();
                    TokenKind::Tilde
                }
                '<' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        TokenKind::LessEqual
                    } else if self.peek() == '<' {
                        self.advance();
                        TokenKind::DoubleLess
                    } else {
                        TokenKind::Less
                    }
                }
                '>' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        TokenKind::GreaterEqual
                    } else if self.peek() == '>' {
                        self.advance();
                        TokenKind::DoubleGreater
                    } else {
                        TokenKind::Greater
                    }
                }
                _ => {
                    return Err(format!(
                        "Unexpected character '{}' at line {}, col {}",
                        ch, start_line, start_col
                    ));
                }
            };

            tokens.push(Token::new(kind, start_line, start_col));
        }

        tokens.push(Token::new(TokenKind::Eof, self.line, self.col));
        Ok(tokens)
    }

    fn lex_string(
        &mut self,
        tokens: &mut Vec<Token>,
        start_line: usize,
        start_col: usize,
        is_start: bool,
    ) -> Result<(), String> {
        let mut buf = String::new();

        while !self.is_eof() {
            let ch = self.peek();
            if ch == '\\' {
                self.advance();
                let esc = match self.peek() {
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    '\\' => '\\',
                    '"' => '"',
                    '{' => '{',
                    '}' => '}',
                    other => {
                        return Err(format!(
                            "Invalid escape sequence '\\{}' at line {}, col {}",
                            other, self.line, self.col
                        ));
                    }
                };
                buf.push(esc);
                self.advance();
            } else if ch == '"' {
                self.advance(); // consume closing '"'
                if is_start {
                    tokens.push(Token::new(TokenKind::StringLit(buf), start_line, start_col));
                } else {
                    tokens.push(Token::new(
                        TokenKind::InterpStringEnd(buf),
                        start_line,
                        start_col,
                    ));
                }
                return Ok(());
            } else if ch == '{' {
                self.advance(); // consume '{'
                self.current_brace_depth += 1;
                self.interp_stack.push(self.current_brace_depth);

                if is_start {
                    tokens.push(Token::new(
                        TokenKind::InterpStringBegin(buf),
                        start_line,
                        start_col,
                    ));
                } else {
                    tokens.push(Token::new(
                        TokenKind::InterpStringMid(buf),
                        start_line,
                        start_col,
                    ));
                }
                return Ok(());
            } else {
                buf.push(ch);
                self.advance();
            }
        }

        Err(format!(
            "Unterminated string literal starting at line {}, col {}",
            start_line, start_col
        ))
    }

    fn lex_number(&mut self) -> Result<i64, String> {
        let mut buf = String::new();
        while !self.is_eof() && self.peek().is_ascii_digit() {
            buf.push(self.peek());
            self.advance();
        }
        buf.parse::<i64>()
            .map_err(|e| format!("Invalid integer '{}': {}", buf, e))
    }

    fn lex_ident(&mut self) -> String {
        let mut buf = String::new();
        while !self.is_eof() {
            let ch = self.peek();
            if ch.is_alphanumeric() || ch == '_' {
                buf.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        buf
    }

    fn skip_comment(&mut self) {
        while !self.is_eof() && self.peek() != '\n' {
            self.advance();
        }
    }

    fn lex_single_quoted_string(
        &mut self,
        start_line: usize,
        start_col: usize,
    ) -> Result<Token, String> {
        let mut buf = String::new();
        while !self.is_eof() {
            let ch = self.peek();
            if ch == '\\' {
                self.advance();
                let esc = match self.peek() {
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    '\\' => '\\',
                    '\'' => '\'',
                    '"' => '"',
                    other => {
                        return Err(format!(
                            "Invalid escape sequence '\\{}' at line {}, col {}",
                            other, self.line, self.col
                        ));
                    }
                };
                buf.push(esc);
                self.advance();
            } else if ch == '\'' {
                self.advance(); // consume closing '\''
                return Ok(Token::new(TokenKind::StringLit(buf), start_line, start_col));
            } else {
                buf.push(ch);
                self.advance();
            }
        }

        Err(format!(
            "Unterminated single-quoted string literal starting at line {}, col {}",
            start_line, start_col
        ))
    }

    fn peek(&self) -> char {
        if self.pos < self.chars.len() {
            self.chars[self.pos]
        } else {
            '\0'
        }
    }

    fn peek_ahead(&self, offset: usize) -> char {
        if self.pos + offset < self.chars.len() {
            self.chars[self.pos + offset]
        } else {
            '\0'
        }
    }

    fn advance(&mut self) -> char {
        let ch = self.peek();
        self.pos += 1;
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        ch
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.chars.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let code = r#"
            remember x = 42
            say "Hello, World!"
            repeat 3 as i {
                say i
            }
        "#;
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Remember)));
        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Say)));
        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Repeat)));
        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::As)));
    }

    #[test]
    fn test_interpolation() {
        let code = r#"say "Hello, {name}! Age: {age}.""#;
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();

        assert!(
            tokens
                .iter()
                .any(|t| matches!(&t.kind, TokenKind::InterpStringBegin(s) if s == "Hello, "))
        );
        assert!(
            tokens
                .iter()
                .any(|t| matches!(&t.kind, TokenKind::Ident(s) if s == "name"))
        );
        assert!(
            tokens
                .iter()
                .any(|t| matches!(&t.kind, TokenKind::InterpStringMid(s) if s == "! Age: "))
        );
        assert!(
            tokens
                .iter()
                .any(|t| matches!(&t.kind, TokenKind::Ident(s) if s == "age"))
        );
        assert!(
            tokens
                .iter()
                .any(|t| matches!(&t.kind, TokenKind::InterpStringEnd(s) if s == "."))
        );
    }

    #[test]
    fn test_directives() {
        let code = r#"
            remove garbageC
            remove Basic
            add Advanced
        "#;
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Remove)));
        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::GarbageC)));
        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Basic)));
        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Add)));
        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Advanced)));
    }
}
