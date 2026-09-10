use crate::ast::*;
use crate::token::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let directives = self.parse_directives()?;
        let mut statements = Vec::new();

        self.skip_newlines();
        while !self.is_eof() {
            statements.push(self.parse_statement()?);
            self.skip_newlines();
        }

        Ok(Program {
            directives,
            statements,
        })
    }

    fn parse_directives(&mut self) -> Result<Directives, String> {
        let mut directives = Directives::default();

        self.skip_newlines();
        loop {
            if self.check(TokenKind::Remove) {
                self.advance();
                if self.match_token(TokenKind::GarbageC) {
                    directives.remove_gc = true;
                } else if self.match_token(TokenKind::Basic) {
                    directives.remove_basic = true;
                } else {
                    let tok = self.peek();
                    return Err(format!(
                        "Expected 'garbageC' or 'Basic' after 'remove' at line {}, col {}",
                        tok.line, tok.col
                    ));
                }
                self.skip_newlines();
            } else if self.check(TokenKind::Add) {
                self.advance();
                if self.match_token(TokenKind::Advanced) {
                    directives.add_advanced = true;
                } else {
                    let tok = self.peek();
                    return Err(format!(
                        "Expected 'Advanced' after 'add' at line {}, col {}",
                        tok.line, tok.col
                    ));
                }
                self.skip_newlines();
            } else {
                break;
            }
        }

        Ok(directives)
    }

    fn parse_statement(&mut self) -> Result<Stmt, String> {
        self.skip_newlines();
        let tok = self.peek().clone();

        match tok.kind {
            TokenKind::Say => {
                self.advance();
                let expr = self.parse_expression()?;
                Ok(Stmt::Say {
                    expr,
                    newline: true,
                })
            }
            TokenKind::SaySame => {
                self.advance();
                let expr = self.parse_expression()?;
                Ok(Stmt::Say {
                    expr,
                    newline: false,
                })
            }
            TokenKind::Remember => {
                self.advance();
                let name_tok = self.peek().clone();
                let name = match &name_tok.kind {
                    TokenKind::Ident(s) => s.clone(),
                    _ => {
                        return Err(format!(
                            "Expected variable name after 'remember' at line {}, col {}",
                            name_tok.line, name_tok.col
                        ))
                    }
                };
                self.advance();

                let mut explicit_type = None;
                if self.match_token(TokenKind::Colon) {
                    explicit_type = Some(self.parse_type()?);
                }

                self.consume(
                    TokenKind::Equal,
                    "Expected '=' in variable declaration",
                )?;
                let value = self.parse_expression()?;
                Ok(Stmt::Remember {
                    name,
                    explicit_type,
                    value,
                })
            }
            TokenKind::If => {
                self.advance();
                let condition = self.parse_expression()?;
                let then_branch = self.parse_block()?;

                let mut otherwise_ifs = Vec::new();
                let mut otherwise_branch = None;

                self.skip_newlines();
                while self.check(TokenKind::Otherwise) {
                    self.advance(); // consume 'otherwise'
                    if self.match_token(TokenKind::If) {
                        let elif_cond = self.parse_expression()?;
                        let elif_body = self.parse_block()?;
                        otherwise_ifs.push((elif_cond, elif_body));
                        self.skip_newlines();
                    } else {
                        // plain 'otherwise'
                        otherwise_branch = Some(self.parse_block()?);
                        break;
                    }
                }

                Ok(Stmt::If {
                    condition,
                    then_branch,
                    otherwise_ifs,
                    otherwise_branch,
                })
            }
            TokenKind::Repeat => {
                self.advance();
                let count = self.parse_expression()?;
                let mut var_name = None;
                if self.match_token(TokenKind::As) {
                    let vtok = self.peek().clone();
                    if let TokenKind::Ident(s) = vtok.kind {
                        var_name = Some(s);
                        self.advance();
                    } else {
                        return Err(format!(
                            "Expected identifier after 'as' at line {}, col {}",
                            vtok.line, vtok.col
                        ));
                    }
                }
                let body = self.parse_block()?;
                Ok(Stmt::Repeat {
                    count,
                    var_name,
                    body,
                })
            }
            TokenKind::While => {
                self.advance();
                let condition = self.parse_expression()?;
                let body = self.parse_block()?;
                Ok(Stmt::While { condition, body })
            }
            TokenKind::Stop => {
                self.advance();
                Ok(Stmt::Stop)
            }
            TokenKind::Skip => {
                self.advance();
                Ok(Stmt::Skip)
            }
            TokenKind::Action => {
                self.advance();
                let name_tok = self.peek().clone();
                let name = match name_tok.kind {
                    TokenKind::Ident(s) => s,
                    _ => {
                        return Err(format!(
                            "Expected function name after 'action' at line {}, col {}",
                            name_tok.line, name_tok.col
                        ))
                    }
                };
                self.advance();

                self.consume(TokenKind::OpenParen, "Expected '(' after action name")?;
                let mut params = Vec::new();
                if !self.check(TokenKind::CloseParen) {
                    loop {
                        let p_tok = self.peek().clone();
                        let p_name = match p_tok.kind {
                            TokenKind::Ident(s) => s,
                            _ => {
                                return Err(format!(
                                    "Expected parameter name at line {}, col {}",
                                    p_tok.line, p_tok.col
                                ))
                            }
                        };
                        self.advance();

                        let mut p_type = None;
                        if self.match_token(TokenKind::Colon) {
                            p_type = Some(self.parse_type()?);
                        }
                        params.push((p_name, p_type));

                        if self.match_token(TokenKind::Comma) {
                            continue;
                        } else {
                            break;
                        }
                    }
                }
                self.consume(TokenKind::CloseParen, "Expected ')' after parameters")?;

                let mut return_type = None;
                if self.match_token(TokenKind::Colon) {
                    return_type = Some(self.parse_type()?);
                }

                let body = self.parse_block()?;
                Ok(Stmt::ActionDef {
                    name,
                    params,
                    return_type,
                    body,
                })
            }
            TokenKind::Give => {
                self.advance();
                if self.check(TokenKind::Newline)
                    || self.check(TokenKind::CloseBrace)
                    || self.check(TokenKind::Eof)
                {
                    Ok(Stmt::Give(None))
                } else {
                    let expr = self.parse_expression()?;
                    Ok(Stmt::Give(Some(expr)))
                }
            }
            TokenKind::Run => {
                self.advance();
                let command = self.parse_expression()?;
                Ok(Stmt::RunCommand { command })
            }
            TokenKind::Use => {
                self.advance();
                let tok = self.peek().clone();
                let path = match tok.kind {
                    TokenKind::StringLit(s) => s,
                    TokenKind::Ident(s) => s,
                    _ => {
                        return Err(format!(
                            "Expected file path or module name after 'use' at line {}, col {}",
                            tok.line, tok.col
                        ))
                    }
                };
                self.advance();
                Ok(Stmt::Use(path))
            }
            TokenKind::Write => {
                self.advance();
                let data = self.parse_expression()?;
                self.consume(TokenKind::Into, "Expected 'into' after data in write statement")?;
                let target = self.parse_expression()?;
                Ok(Stmt::WriteFile { data, target })
            }
            TokenKind::Wait => {
                self.advance();
                let sec = self.parse_expression()?;
                Ok(Stmt::Wait(sec))
            }
            TokenKind::Window => {
                self.advance();
                let title = self.parse_expression()?;
                let mut width = None;
                let mut height = None;
                if self.match_token(TokenKind::Comma) {
                    width = Some(self.parse_expression()?);
                    self.consume(TokenKind::Comma, "Expected ',' between window width and height")?;
                    height = Some(self.parse_expression()?);
                }
                let body = self.parse_block()?;
                Ok(Stmt::Window {
                    title,
                    width,
                    height,
                    body,
                })
            }
            TokenKind::Button => {
                self.advance();
                let label = self.parse_expression()?;
                let action = self.parse_block()?;
                Ok(Stmt::GuiButton { label, action })
            }
            TokenKind::Label => {
                self.advance();
                let text = self.parse_expression()?;
                Ok(Stmt::GuiLabel { text })
            }
            TokenKind::Checkbox => {
                self.advance();
                let label = self.parse_expression()?;
                let mut initial_val = None;
                if self.match_token(TokenKind::Comma) {
                    initial_val = Some(self.parse_expression()?);
                }
                Ok(Stmt::GuiCheckbox { label, initial_val })
            }
            TokenKind::Ident(ref name) => {
                if name == "imrv"
                    && self.pos + 1 < self.tokens.len()
                    && self.tokens[self.pos + 1].kind == TokenKind::Draw
                {
                    self.advance(); // consume "imrv"
                    self.advance(); // consume "draw"
                    let elem_tok = self.peek().clone();
                    let elem = match elem_tok.kind {
                        TokenKind::Ident(s) => s,
                        TokenKind::Checkbox => "checkbox".to_string(),
                        TokenKind::Button => "button".to_string(),
                        _ => {
                            return Err(format!(
                                "Expected element name after 'draw' at line {}, col {}",
                                elem_tok.line, elem_tok.col
                            ))
                        }
                    };
                    self.advance();
                    let label = self.parse_expression()?;
                    let mut extra = None;
                    if self.match_token(TokenKind::Comma) {
                        extra = Some(self.parse_expression()?);
                    }
                    return Ok(Stmt::ImrvStmt {
                        element: elem,
                        label,
                        extra,
                    });
                }

                // Check if assignment: `name = expr`
                if self.pos + 1 < self.tokens.len()
                    && self.tokens[self.pos + 1].kind == TokenKind::Equal
                {
                    let var_name = name.clone();
                    self.advance(); // consume ident
                    self.advance(); // consume '='
                    let value = self.parse_expression()?;
                    Ok(Stmt::Assign {
                        name: var_name,
                        value,
                    })
                } else {
                    let expr = self.parse_expression()?;
                    Ok(Stmt::ExprStmt(expr))
                }
            }
            _ => {
                let expr = self.parse_expression()?;
                Ok(Stmt::ExprStmt(expr))
            }
        }
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>, String> {
        self.skip_newlines();
        self.consume(TokenKind::OpenBrace, "Expected '{' to start block")?;
        self.skip_newlines();

        let mut stmts = Vec::new();
        while !self.check(TokenKind::CloseBrace) && !self.is_eof() {
            stmts.push(self.parse_statement()?);
            self.skip_newlines();
        }

        self.consume(TokenKind::CloseBrace, "Expected '}' to close block")?;
        Ok(stmts)
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        let tok = self.peek().clone();
        match tok.kind {
            TokenKind::TypeInt => {
                self.advance();
                Ok(Type::Int)
            }
            TokenKind::TypeString => {
                self.advance();
                Ok(Type::String)
            }
            TokenKind::TypeBool => {
                self.advance();
                Ok(Type::Bool)
            }
            _ => Err(format!(
                "Expected type (Int, String, Bool) at line {}, col {}",
                tok.line, tok.col
            )),
        }
    }

    pub fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_and()?;
        while self.match_token(TokenKind::Or) {
            let right = self.parse_and()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::Or,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_equality()?;
        while self.match_token(TokenKind::And) {
            let right = self.parse_equality()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_comparison()?;

        loop {
            if self.match_token(TokenKind::Is) {
                if self.match_token(TokenKind::Not) {
                    let right = self.parse_comparison()?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: BinaryOp::NotEqual,
                        right: Box::new(right),
                    };
                } else {
                    let right = self.parse_comparison()?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: BinaryOp::Equal,
                        right: Box::new(right),
                    };
                }
            } else if self.match_token(TokenKind::DoubleEqual) {
                let right = self.parse_comparison()?;
                left = Expr::Binary {
                    left: Box::new(left),
                    op: BinaryOp::Equal,
                    right: Box::new(right),
                };
            } else if self.match_token(TokenKind::NotEqual) {
                let right = self.parse_comparison()?;
                left = Expr::Binary {
                    left: Box::new(left),
                    op: BinaryOp::NotEqual,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;

        loop {
            let op = if self.match_token(TokenKind::Less) {
                BinaryOp::Less
            } else if self.match_token(TokenKind::LessEqual) {
                BinaryOp::LessEqual
            } else if self.match_token(TokenKind::Greater) {
                BinaryOp::Greater
            } else if self.match_token(TokenKind::GreaterEqual) {
                BinaryOp::GreaterEqual
            } else {
                break;
            };

            let right = self.parse_term()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_factor()?;

        loop {
            let op = if self.match_token(TokenKind::Plus) {
                BinaryOp::Add
            } else if self.match_token(TokenKind::Minus) {
                BinaryOp::Sub
            } else {
                break;
            };

            let right = self.parse_factor()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;

        loop {
            let op = if self.match_token(TokenKind::Star) {
                BinaryOp::Mul
            } else if self.match_token(TokenKind::Slash) {
                BinaryOp::Div
            } else if self.match_token(TokenKind::Percent) {
                BinaryOp::Mod
            } else {
                break;
            };

            let right = self.parse_unary()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        if self.match_token(TokenKind::Not) {
            let expr = self.parse_unary()?;
            return Ok(Expr::Unary {
                op: UnaryOp::Not,
                expr: Box::new(expr),
            });
        }
        if self.match_token(TokenKind::Minus) {
            let expr = self.parse_unary()?;
            return Ok(Expr::Unary {
                op: UnaryOp::Neg,
                expr: Box::new(expr),
            });
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        let tok = self.peek().clone();

        match tok.kind {
            TokenKind::IntLit(n) => {
                self.advance();
                Ok(Expr::Int(n))
            }
            TokenKind::StringLit(s) => {
                self.advance();
                Ok(Expr::Str(s))
            }
            TokenKind::BoolLit(b) => {
                self.advance();
                Ok(Expr::Bool(b))
            }
            TokenKind::Ask => {
                self.advance();
                let prompt = self.parse_expression()?;
                Ok(Expr::Ask(Box::new(prompt)))
            }
            TokenKind::Run => {
                self.advance();
                let cmd = self.parse_expression()?;
                Ok(Expr::Run(Box::new(cmd)))
            }
            TokenKind::Read => {
                self.advance();
                let path = self.parse_expression()?;
                Ok(Expr::ReadFile(Box::new(path)))
            }
            TokenKind::Random => {
                self.advance();
                let min = self.parse_expression()?;
                self.consume(TokenKind::To, "Expected 'to' in random expression (e.g. random 1 to 100)")?;
                let max = self.parse_expression()?;
                Ok(Expr::Random {
                    min: Box::new(min),
                    max: Box::new(max),
                })
            }
            TokenKind::InterpStringBegin(first_str) => {
                self.advance();
                let mut parts = Vec::new();
                if !first_str.is_empty() {
                    parts.push(Expr::Str(first_str));
                }

                // First interpolated expression
                parts.push(self.parse_expression()?);

                // Subsequent mid parts and expressions
                while let TokenKind::InterpStringMid(mid_str) = self.peek().kind.clone() {
                    self.advance();
                    if !mid_str.is_empty() {
                        parts.push(Expr::Str(mid_str));
                    }
                    parts.push(self.parse_expression()?);
                }

                // Ending string part
                let end_tok = self.peek().clone();
                if let TokenKind::InterpStringEnd(end_str) = end_tok.kind {
                    self.advance();
                    if !end_str.is_empty() {
                        parts.push(Expr::Str(end_str));
                    }
                } else {
                    return Err(format!(
                        "Expected end of interpolated string at line {}, col {}",
                        end_tok.line, end_tok.col
                    ));
                }

                Ok(Expr::InterpolatedString(parts))
            }
            TokenKind::Ident(name) => {
                if name == "imrv"
                    && self.pos + 1 < self.tokens.len()
                    && self.tokens[self.pos + 1].kind == TokenKind::Draw
                {
                    self.advance(); // consume "imrv"
                    self.advance(); // consume "draw"
                    let elem_tok = self.peek().clone();
                    let elem = match elem_tok.kind {
                        TokenKind::Ident(s) => s,
                        TokenKind::Checkbox => "checkbox".to_string(),
                        TokenKind::Button => "button".to_string(),
                        _ => {
                            return Err(format!(
                                "Expected element name after 'draw' at line {}, col {}",
                                elem_tok.line, elem_tok.col
                            ))
                        }
                    };
                    self.advance();
                    let label = self.parse_expression()?;
                    let mut extra = None;
                    if self.match_token(TokenKind::Comma) {
                        extra = Some(self.parse_expression()?);
                    }
                    return Ok(Expr::ImrvDraw {
                        element: elem,
                        label: Box::new(label),
                        extra: extra.map(Box::new),
                    });
                }
                self.advance();
                // Check if function call: name(...)
                if self.match_token(TokenKind::OpenParen) {
                    let mut args = Vec::new();
                    if !self.check(TokenKind::CloseParen) {
                        loop {
                            args.push(self.parse_expression()?);
                            if self.match_token(TokenKind::Comma) {
                                continue;
                            } else {
                                break;
                            }
                        }
                    }
                    self.consume(TokenKind::CloseParen, "Expected ')' after function arguments")?;
                    Ok(Expr::Call {
                        callee: name,
                        args,
                    })
                } else {
                    Ok(Expr::Var(name))
                }
            }
            TokenKind::OpenParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(TokenKind::CloseParen, "Expected ')'")?;
                Ok(expr)
            }
            _ => Err(format!(
                "Unexpected token {:?} in expression at line {}, col {}",
                tok.kind, tok.line, tok.col
            )),
        }
    }

    fn skip_newlines(&mut self) {
        while self.check(TokenKind::Newline) {
            self.advance();
        }
    }

    fn peek(&self) -> &Token {
        if self.pos < self.tokens.len() {
            &self.tokens[self.pos]
        } else {
            self.tokens.last().unwrap()
        }
    }

    fn advance(&mut self) -> &Token {
        let prev = self.pos;
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        &self.tokens[prev]
    }

    fn check(&self, kind: TokenKind) -> bool {
        if self.is_eof() {
            false
        } else {
            self.peek().kind == kind
        }
    }

    fn match_token(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn consume(&mut self, kind: TokenKind, err_msg: &str) -> Result<(), String> {
        if self.check(kind) {
            self.advance();
            Ok(())
        } else {
            let tok = self.peek();
            Err(format!("{} at line {}, col {}", err_msg, tok.line, tok.col))
        }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.tokens.len() || self.peek().kind == TokenKind::Eof
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_parse_simple_program() {
        let code = r#"
            remember msg = "Hi"
            say msg
        "#;
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse().unwrap();

        assert_eq!(prog.statements.len(), 2);
    }

    #[test]
    fn test_parse_if_otherwise() {
        let code = r#"
            if x is 10 {
                say "ten"
            } otherwise if x is 20 {
                say "twenty"
            } otherwise {
                say "other"
            }
        "#;
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse().unwrap();

        assert_eq!(prog.statements.len(), 1);
        if let Stmt::If {
            otherwise_ifs,
            otherwise_branch,
            ..
        } = &prog.statements[0]
        {
            assert_eq!(otherwise_ifs.len(), 1);
            assert!(otherwise_branch.is_some());
        } else {
            panic!("Expected Stmt::If");
        }
    }

    #[test]
    fn test_parse_interpolated_string() {
        let code = r#"say "Count: {x + 1} items""#;
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse().unwrap();

        assert_eq!(prog.statements.len(), 1);
        if let Stmt::Say {
            expr: Expr::InterpolatedString(parts),
            ..
        } = &prog.statements[0]
        {
            assert_eq!(parts.len(), 3); // "Count: ", (x + 1), " items"
        } else {
            panic!("Expected Stmt::Say with InterpolatedString");
        }
    }
}
