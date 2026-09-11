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
        let mut directives = self.parse_directives()?;
        let mut statements = Vec::new();

        self.skip_newlines();
        while !self.is_eof() {
            let stmt = self.parse_statement()?;
            match &stmt {
                Stmt::Use(path) => {
                    directives.modules.push(path.clone());
                    if path.ends_with(".rv") {
                        let p = std::path::Path::new(path);
                        if let Ok(src) = std::fs::read_to_string(p) {
                            let mut lex = crate::lexer::Lexer::new(&src);
                            if let Ok(toks) = lex.tokenize() {
                                let mut sub_parser = Parser::new(toks);
                                if let Ok(sub_prog) = sub_parser.parse() {
                                    for m in sub_prog.directives.modules {
                                        if !directives.modules.contains(&m) {
                                            directives.modules.push(m);
                                        }
                                    }
                                    for l in sub_prog.directives.libs {
                                        if !directives.libs.contains(&l) {
                                            directives.libs.push(l);
                                        }
                                    }
                                    for s in sub_prog.statements {
                                        statements.push(s);
                                    }
                                }
                            }
                        }
                    }
                }
                Stmt::UseLib(lib_name) => {
                    directives.libs.push(lib_name.clone());
                }
                _ => {}
            }
            statements.push(stmt);
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
                if self.pos + 1 < self.tokens.len()
                    && (self.tokens[self.pos + 1].kind == TokenKind::GarbageC
                        || self.tokens[self.pos + 1].kind == TokenKind::Basic
                        || self.tokens[self.pos + 1].kind == TokenKind::Linux)
                {
                    self.advance();
                    if self.match_token(TokenKind::GarbageC) {
                        directives.remove_gc = true;
                    } else if self.match_token(TokenKind::Basic) {
                        directives.remove_basic = true;
                    } else if self.match_token(TokenKind::Linux) {
                        directives.remove_linux = true;
                    }
                    self.skip_newlines();
                } else {
                    break;
                }
            } else if self.check(TokenKind::Add) {
                if self.pos + 1 < self.tokens.len()
                    && (self.tokens[self.pos + 1].kind == TokenKind::Advanced
                        || self.tokens[self.pos + 1].kind == TokenKind::Freestanding)
                {
                    self.advance();
                    if self.match_token(TokenKind::Advanced) {
                        directives.add_advanced = true;
                    } else if self.match_token(TokenKind::Freestanding) {
                        directives.add_freestanding = true;
                    }
                    self.skip_newlines();
                } else {
                    break;
                }
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
                let is_same = self.match_token(TokenKind::Same);
                let color = if self.match_token(TokenKind::Green) {
                    Some("green".to_string())
                } else if self.match_token(TokenKind::Red) {
                    Some("red".to_string())
                } else if self.match_token(TokenKind::Blue) {
                    Some("blue".to_string())
                } else if self.match_token(TokenKind::Yellow) {
                    Some("yellow".to_string())
                } else if self.match_token(TokenKind::Cyan) {
                    Some("cyan".to_string())
                } else if self.match_token(TokenKind::Magenta) {
                    Some("magenta".to_string())
                } else {
                    None
                };
                let expr = self.parse_expression()?;
                Ok(Stmt::Say {
                    expr,
                    newline: !is_same,
                    color,
                })
            }
            TokenKind::SaySame => {
                self.advance();
                let color = if self.match_token(TokenKind::Green) {
                    Some("green".to_string())
                } else if self.match_token(TokenKind::Red) {
                    Some("red".to_string())
                } else if self.match_token(TokenKind::Blue) {
                    Some("blue".to_string())
                } else if self.match_token(TokenKind::Yellow) {
                    Some("yellow".to_string())
                } else if self.match_token(TokenKind::Cyan) {
                    Some("cyan".to_string())
                } else if self.match_token(TokenKind::Magenta) {
                    Some("magenta".to_string())
                } else {
                    None
                };
                let expr = self.parse_expression()?;
                Ok(Stmt::Say {
                    expr,
                    newline: false,
                    color,
                })
            }
            TokenKind::Remember => {
                self.advance();
                let name_tok = self.peek().clone();
                let name = match &name_tok.kind {
                    TokenKind::Ident(s) => s.clone(),
                    TokenKind::Count => "count".to_string(),
                    TokenKind::List => "list".to_string(),
                    TokenKind::File => "file".to_string(),
                    TokenKind::Folder => "folder".to_string(),
                    TokenKind::Color => "color".to_string(),
                    TokenKind::Time => "time".to_string(),
                    TokenKind::Size => "size".to_string(),
                    TokenKind::Line => "line".to_string(),
                    TokenKind::Box => "box".to_string(),
                    TokenKind::Circle => "circle".to_string(),
                    _ => {
                        return Err(format!(
                            "Expected variable name after 'remember' at line {}, col {}",
                            name_tok.line, name_tok.col
                        ));
                    }
                };
                self.advance();

                let mut explicit_type = None;
                if self.match_token(TokenKind::Colon) {
                    explicit_type = Some(self.parse_type()?);
                }

                self.consume(TokenKind::Equal, "Expected '=' in variable declaration")?;
                let value = if (matches!(self.peek().kind, TokenKind::Ident(_))
                    || matches!(self.peek().kind, TokenKind::StringLit(_)))
                    && self.pos + 1 < self.tokens.len()
                    && self.tokens[self.pos + 1].kind == TokenKind::Colon
                {
                    let mut pairs = Vec::new();
                    loop {
                        let k_tok = self.peek().clone();
                        let k_expr = match k_tok.kind {
                            TokenKind::Ident(s) => {
                                self.advance();
                                Expr::Str(s)
                            }
                            TokenKind::StringLit(s) => {
                                self.advance();
                                Expr::Str(s)
                            }
                            _ => self.parse_expression()?,
                        };
                        self.consume(TokenKind::Colon, "Expected ':' after map key")?;
                        let v_expr = self.parse_expression()?;
                        pairs.push((k_expr, v_expr));
                        if self.match_token(TokenKind::Comma) {
                            continue;
                        } else {
                            break;
                        }
                    }
                    Expr::MapLiteral(pairs)
                } else {
                    let first = self.parse_expression()?;
                    if self.match_token(TokenKind::Comma) {
                        let mut list_items = vec![first];
                        loop {
                            list_items.push(self.parse_expression()?);
                            if self.match_token(TokenKind::Comma) {
                                continue;
                            } else {
                                break;
                            }
                        }
                        Expr::ListLiteral(list_items)
                    } else {
                        first
                    }
                };

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
                if matches!(&self.peek().kind, TokenKind::Ident(s) if s == "times") {
                    self.advance();
                }
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
                        ));
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
                                ));
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
                if self.match_token(TokenKind::Lib) {
                    let tok = self.peek().clone();
                    let lib_name = match tok.kind {
                        TokenKind::StringLit(s) => s,
                        TokenKind::Ident(s) => s,
                        _ => {
                            return Err(format!(
                                "Expected library name after 'use lib' at line {}, col {}",
                                tok.line, tok.col
                            ));
                        }
                    };
                    self.advance();
                    Ok(Stmt::UseLib(lib_name))
                } else {
                    let tok = self.peek().clone();
                    let path = match tok.kind {
                        TokenKind::StringLit(s) => s,
                        TokenKind::Ident(s) => s,
                        TokenKind::Thread => "thread".to_string(),
                        TokenKind::Time => "time".to_string(),
                        _ => {
                            return Err(format!(
                                "Expected file path or module name after 'use' at line {}, col {}",
                                tok.line, tok.col
                            ));
                        }
                    };
                    self.advance();
                    Ok(Stmt::Use(path))
                }
            }
            TokenKind::Write => {
                self.advance();
                let data = self.parse_expression()?;
                self.consume(
                    TokenKind::Into,
                    "Expected 'into' after data in write statement",
                )?;
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
                    self.consume(
                        TokenKind::Comma,
                        "Expected ',' between window width and height",
                    )?;
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
            TokenKind::Alert => {
                self.advance();
                let msg = self.parse_expression()?;
                Ok(Stmt::Alert(msg))
            }
            TokenKind::Beep => {
                self.advance();
                Ok(Stmt::Beep)
            }
            TokenKind::Speak => {
                self.advance();
                let msg = self.parse_expression()?;
                Ok(Stmt::Speak(msg))
            }
            TokenKind::Open => {
                self.advance();
                self.consume(TokenKind::Web, "Expected 'web' after 'open'")?;
                let url = self.parse_expression()?;
                Ok(Stmt::OpenWeb(url))
            }
            TokenKind::Download => {
                self.advance();
                let url = self.parse_expression()?;
                self.consume(
                    TokenKind::Into,
                    "Expected 'into' after URL in download statement",
                )?;
                let target = self.parse_expression()?;
                Ok(Stmt::DownloadWeb { url, target })
            }
            TokenKind::Screen => {
                self.advance();
                let title = self.parse_expression()?;
                let mut width = None;
                let mut height = None;
                if self.match_token(TokenKind::Comma) {
                    width = Some(self.parse_expression()?);
                    self.consume(
                        TokenKind::Comma,
                        "Expected ',' between screen width and height",
                    )?;
                    height = Some(self.parse_expression()?);
                }
                let body = self.parse_block()?;
                Ok(Stmt::Screen {
                    title,
                    width,
                    height,
                    body,
                })
            }
            TokenKind::Draw => {
                self.advance();
                if self.match_token(TokenKind::Circle) {
                    self.consume(TokenKind::At, "Expected 'at' after 'circle'")?;
                    let x = self.parse_expression()?;
                    self.consume(TokenKind::Comma, "Expected ',' between circle x and y")?;
                    let y = self.parse_expression()?;
                    self.consume(TokenKind::Comma, "Expected ',' before 'size'")?;
                    self.consume(TokenKind::Size, "Expected 'size' for circle radius")?;
                    let radius = self.parse_expression()?;
                    let mut color = None;
                    if self.match_token(TokenKind::Comma) {
                        self.consume(TokenKind::Color, "Expected 'color' in draw circle")?;
                        color = Some(self.parse_expression()?);
                    }
                    Ok(Stmt::DrawCircle {
                        x,
                        y,
                        radius,
                        color,
                    })
                } else if self.match_token(TokenKind::Box) {
                    self.consume(TokenKind::At, "Expected 'at' after 'box'")?;
                    let x = self.parse_expression()?;
                    self.consume(TokenKind::Comma, "Expected ',' between box x and y")?;
                    let y = self.parse_expression()?;
                    self.consume(TokenKind::Comma, "Expected ',' before 'size'")?;
                    self.consume(TokenKind::Size, "Expected 'size' for box width and height")?;
                    let width = self.parse_expression()?;
                    self.consume(
                        TokenKind::Comma,
                        "Expected ',' between box width and height",
                    )?;
                    let height = self.parse_expression()?;
                    let mut color = None;
                    if self.match_token(TokenKind::Comma) {
                        self.consume(TokenKind::Color, "Expected 'color' in draw box")?;
                        color = Some(self.parse_expression()?);
                    }
                    Ok(Stmt::DrawRect {
                        x,
                        y,
                        width,
                        height,
                        color,
                    })
                } else if self.match_token(TokenKind::Line) {
                    self.consume(TokenKind::From, "Expected 'from' after 'line'")?;
                    let x1 = self.parse_expression()?;
                    self.consume(TokenKind::Comma, "Expected ',' between x1 and y1")?;
                    let y1 = self.parse_expression()?;
                    self.match_token(TokenKind::Comma);
                    self.consume(TokenKind::To, "Expected 'to' before x2, y2")?;
                    let x2 = self.parse_expression()?;
                    self.consume(TokenKind::Comma, "Expected ',' between x2 and y2")?;
                    let y2 = self.parse_expression()?;
                    let mut color = None;
                    if self.match_token(TokenKind::Comma) {
                        self.consume(TokenKind::Color, "Expected 'color' in draw line")?;
                        color = Some(self.parse_expression()?);
                    }
                    Ok(Stmt::DrawLine {
                        x1,
                        y1,
                        x2,
                        y2,
                        color,
                    })
                } else {
                    if matches!(&self.peek().kind, TokenKind::Ident(s) if s == "text") {
                        self.advance();
                    }
                    let text = self.parse_expression()?;
                    self.match_token(TokenKind::Comma);
                    self.consume(TokenKind::At, "Expected 'at' for text coordinates")?;
                    let x = self.parse_expression()?;
                    self.consume(TokenKind::Comma, "Expected ',' between x and y")?;
                    let y = self.parse_expression()?;
                    let mut color = None;
                    if self.match_token(TokenKind::Comma) {
                        self.consume(TokenKind::Color, "Expected 'color' in draw text")?;
                        color = Some(self.parse_expression()?);
                    }
                    Ok(Stmt::DrawText { text, x, y, color })
                }
            }
            TokenKind::Clear => {
                self.advance();
                self.consume(TokenKind::Screen, "Expected 'screen' after 'clear'")?;
                Ok(Stmt::ClearScreen)
            }
            TokenKind::Cursor => {
                self.advance();
                self.consume(TokenKind::At, "Expected 'at' after 'cursor'")?;
                let x = self.parse_expression()?;
                self.consume(TokenKind::Comma, "Expected ',' between cursor x and y")?;
                let y = self.parse_expression()?;
                Ok(Stmt::CursorAt { x, y })
            }
            TokenKind::Create => {
                self.advance();
                self.consume(TokenKind::Folder, "Expected 'folder' after 'create'")?;
                let path = self.parse_expression()?;
                Ok(Stmt::CreateFolder(path))
            }
            TokenKind::Delete => {
                self.advance();
                if self.match_token(TokenKind::Folder) {
                    let path = self.parse_expression()?;
                    Ok(Stmt::DeleteFolder(path))
                } else {
                    self.consume(
                        TokenKind::File,
                        "Expected 'file' or 'folder' after 'delete'",
                    )?;
                    let path = self.parse_expression()?;
                    Ok(Stmt::DeleteFile(path))
                }
            }
            TokenKind::Copy => {
                self.advance();
                self.consume(TokenKind::File, "Expected 'file' after 'copy'")?;
                let src = self.parse_expression()?;
                self.consume(
                    TokenKind::To,
                    "Expected 'to' after source file in 'copy file ... to ...'",
                )?;
                let dest = self.parse_expression()?;
                Ok(Stmt::CopyFile { src, dest })
            }
            TokenKind::Add => {
                self.advance();
                let key_or_item = self.parse_expression()?;
                if self.match_token(TokenKind::Colon) {
                    let value = self.parse_expression()?;
                    self.consume(
                        TokenKind::To,
                        "Expected 'to' after value in 'add key: val to <map>'",
                    )?;
                    let map_tok = self.peek().clone();
                    let map = match map_tok.kind {
                        TokenKind::Ident(s) => s,
                        _ => {
                            return Err(format!(
                                "Expected map name after 'to' at line {}, col {}",
                                map_tok.line, map_tok.col
                            ));
                        }
                    };
                    self.advance();
                    Ok(Stmt::AddToMap {
                        key: key_or_item,
                        value,
                        map,
                    })
                } else {
                    self.consume(
                        TokenKind::To,
                        "Expected 'to' after item in 'add ... to <list>'",
                    )?;
                    let list_tok = self.peek().clone();
                    let list = match list_tok.kind {
                        TokenKind::Ident(s) => s,
                        _ => {
                            return Err(format!(
                                "Expected list name after 'to' at line {}, col {}",
                                list_tok.line, list_tok.col
                            ));
                        }
                    };
                    self.advance();
                    Ok(Stmt::AddToList {
                        item: key_or_item,
                        list,
                    })
                }
            }
            TokenKind::Remove => {
                self.advance();
                let item = self.parse_expression()?;
                self.consume(
                    TokenKind::From,
                    "Expected 'from' after item in 'remove ... from <list>'",
                )?;
                let list_tok = self.peek().clone();
                let list = match list_tok.kind {
                    TokenKind::Ident(s) => s,
                    _ => {
                        return Err(format!(
                            "Expected list name after 'from' at line {}, col {}",
                            list_tok.line, list_tok.col
                        ));
                    }
                };
                self.advance();
                Ok(Stmt::RemoveFromList { item, list })
            }
            TokenKind::For => {
                self.advance();
                self.consume(TokenKind::Every, "Expected 'every' after 'for'")?;
                let item_tok = self.peek().clone();
                let item_var = match item_tok.kind {
                    TokenKind::Ident(s) => s,
                    _ => {
                        return Err(format!(
                            "Expected item variable name after 'every' at line {}, col {}",
                            item_tok.line, item_tok.col
                        ));
                    }
                };
                self.advance();
                self.consume(
                    TokenKind::In,
                    "Expected 'in' in 'for every <item> in <list>'",
                )?;
                let list_tok = self.peek().clone();
                let list_var = match list_tok.kind {
                    TokenKind::Ident(s) => s,
                    _ => {
                        return Err(format!(
                            "Expected list variable name after 'in' at line {}, col {}",
                            list_tok.line, list_tok.col
                        ));
                    }
                };
                self.advance();
                let body = self.parse_block()?;
                Ok(Stmt::ForEvery {
                    item_var,
                    list_var,
                    body,
                })
            }
            TokenKind::Make => {
                self.advance();
                let var_tok = self.peek().clone();
                let var_name = match var_tok.kind {
                    TokenKind::Ident(s) => s,
                    _ => {
                        return Err(format!(
                            "Expected variable name after 'make' at line {}, col {}",
                            var_tok.line, var_tok.col
                        ));
                    }
                };
                self.advance();
                let op = if self.match_token(TokenKind::Uppercase) {
                    MakeStringOp::Uppercase
                } else if self.match_token(TokenKind::Lowercase) {
                    MakeStringOp::Lowercase
                } else if self.match_token(TokenKind::Trim) {
                    MakeStringOp::Trim
                } else {
                    return Err(format!(
                        "Expected 'uppercase', 'lowercase', or 'trim' after 'make <var>' at line {}",
                        var_tok.line
                    ));
                };
                Ok(Stmt::MakeString { var_name, op })
            }
            TokenKind::Trim => {
                self.advance();
                let var_tok = self.peek().clone();
                let var_name = match var_tok.kind {
                    TokenKind::Ident(s) => s,
                    _ => {
                        return Err(format!(
                            "Expected variable name after 'trim' at line {}, col {}",
                            var_tok.line, var_tok.col
                        ));
                    }
                };
                self.advance();
                Ok(Stmt::MakeString {
                    var_name,
                    op: MakeStringOp::Trim,
                })
            }
            TokenKind::Measure => {
                self.advance();
                if self.match_token(TokenKind::Time) {
                    let body = self.parse_block()?;
                    Ok(Stmt::MeasureTime { body })
                } else if self.match_token(TokenKind::Cycles) {
                    let body = self.parse_block()?;
                    Ok(Stmt::MeasureCycles { body })
                } else {
                    Err(format!(
                        "Expected 'time' or 'cycles' after 'measure' at line {}, col {}",
                        tok.line, tok.col
                    ))
                }
            }
            TokenKind::AsmBlock(code) => {
                self.advance();
                Ok(Stmt::InlineAsm(code))
            }
            TokenKind::Asm => {
                self.advance();
                if let TokenKind::StringLit(s) = self.peek().kind.clone() {
                    self.advance();
                    Ok(Stmt::InlineAsm(s))
                } else {
                    Err(
                        "Expected inline assembly code block or string literal after 'asm'"
                            .to_string(),
                    )
                }
            }
            TokenKind::AtSign => {
                self.advance();
                let ptr_expr = self.parse_primary()?;
                self.consume(
                    TokenKind::Equal,
                    "Expected '=' in pointer dereference assignment",
                )?;
                let value_expr = self.parse_expression()?;
                Ok(Stmt::DerefAssign {
                    ptr_expr,
                    value_expr,
                })
            }
            TokenKind::Struct => {
                self.advance();
                let name_tok = self.peek().clone();
                let name = match name_tok.kind {
                    TokenKind::Ident(s) => s,
                    _ => {
                        return Err(format!(
                            "Expected struct name at line {}, col {}",
                            name_tok.line, name_tok.col
                        ));
                    }
                };
                self.advance();
                self.consume(TokenKind::OpenBrace, "Expected '{' after struct name")?;
                self.skip_newlines();
                let mut fields = Vec::new();
                while !self.check(TokenKind::CloseBrace) && !self.is_eof() {
                    let f_tok = self.peek().clone();
                    let f_name = match f_tok.kind {
                        TokenKind::Ident(s) => s,
                        _ => {
                            return Err(format!(
                                "Expected field name at line {}, col {}",
                                f_tok.line, f_tok.col
                            ));
                        }
                    };
                    self.advance();
                    let f_ty = if self.match_token(TokenKind::Colon) {
                        self.parse_type()?
                    } else {
                        Type::Int
                    };
                    fields.push((f_name, f_ty));
                    let _ = self.match_token(TokenKind::Comma);
                    self.skip_newlines();
                }
                self.consume(
                    TokenKind::CloseBrace,
                    "Expected '}' after struct definition",
                )?;
                Ok(Stmt::StructDef { name, fields })
            }
            TokenKind::Extern => {
                self.advance();
                let abi = if let TokenKind::StringLit(s) = self.peek().kind.clone() {
                    self.advance();
                    s
                } else {
                    "C".to_string()
                };
                self.consume(TokenKind::OpenBrace, "Expected '{' after extern ABI")?;
                self.skip_newlines();
                let mut actions = Vec::new();
                while !self.check(TokenKind::CloseBrace) && !self.is_eof() {
                    self.consume(TokenKind::Action, "Expected 'action' inside extern block")?;
                    let act_name_tok = self.peek().clone();
                    let act_name = match act_name_tok.kind {
                        TokenKind::Ident(s) => s,
                        _ => {
                            return Err(format!(
                                "Expected action name at line {}, col {}",
                                act_name_tok.line, act_name_tok.col
                            ));
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
                                    ));
                                }
                            };
                            self.advance();
                            self.consume(TokenKind::Colon, "Expected ':' after parameter name")?;
                            let p_ty = self.parse_type()?;
                            params.push((p_name, p_ty));
                            if !self.match_token(TokenKind::Comma) {
                                break;
                            }
                        }
                    }
                    self.consume(TokenKind::CloseParen, "Expected ')'")?;
                    let return_type = if self.match_token(TokenKind::Arrow) {
                        self.parse_type()?
                    } else {
                        Type::Void
                    };
                    actions.push(ExternAction {
                        name: act_name,
                        params,
                        return_type,
                    });
                    self.skip_newlines();
                }
                self.consume(TokenKind::CloseBrace, "Expected '}' at end of extern block")?;
                Ok(Stmt::ExternBlock { abi, actions })
            }
            TokenKind::Thread => {
                self.advance();
                let body = self.parse_block()?;
                Ok(Stmt::ThreadSpawn { body })
            }
            TokenKind::Atomic => {
                self.advance();
                if self.peek().kind == TokenKind::Add
                    || self.peek().kind == TokenKind::Plus
                    || matches!(&self.peek().kind, TokenKind::Ident(s) if s == "add")
                {
                    self.advance();
                }
                let var_tok = self.peek().clone();
                let var = match var_tok.kind {
                    TokenKind::Ident(s) => s,
                    _ => {
                        return Err(format!(
                            "Expected variable name after 'atomic add' at line {}",
                            var_tok.line
                        ));
                    }
                };
                self.advance();
                self.consume(
                    TokenKind::Comma,
                    "Expected ',' after variable name in 'atomic add'",
                )?;
                let val = self.parse_expression()?;
                Ok(Stmt::AtomicAdd { var, val })
            }
            TokenKind::Count => {
                if self.pos + 1 < self.tokens.len()
                    && self.tokens[self.pos + 1].kind == TokenKind::Equal
                {
                    self.advance(); // consume count
                    self.advance(); // consume '='
                    let value = self.parse_expression()?;
                    Ok(Stmt::Assign {
                        name: "count".to_string(),
                        value,
                    })
                } else {
                    let expr = self.parse_expression()?;
                    Ok(Stmt::ExprStmt(expr))
                }
            }
            TokenKind::Match => {
                self.advance();
                let target = self.parse_expression()?;
                self.consume(TokenKind::OpenBrace, "Expected '{' after match target")?;
                self.skip_newlines();
                let mut arms = Vec::new();
                let mut otherwise = None;
                while !self.check(TokenKind::CloseBrace) && !self.is_eof() {
                    self.skip_newlines();
                    if self.check(TokenKind::CloseBrace) {
                        break;
                    }
                    if self.match_token(TokenKind::When) {
                        let pattern = self.parse_expression()?;
                        self.consume(TokenKind::Arrow, "Expected '->' after when pattern")?;
                        self.skip_newlines();
                        let body = if self.check(TokenKind::OpenBrace) {
                            self.parse_block()?
                        } else {
                            vec![self.parse_statement()?]
                        };
                        arms.push(MatchArm { pattern, body });
                    } else if self.match_token(TokenKind::Otherwise) {
                        let _ = self.match_token(TokenKind::Arrow);
                        self.skip_newlines();
                        let body = if self.check(TokenKind::OpenBrace) {
                            self.parse_block()?
                        } else {
                            vec![self.parse_statement()?]
                        };
                        otherwise = Some(body);
                    } else {
                        return Err(format!(
                            "Expected 'when' or 'otherwise' inside match at line {}",
                            self.peek().line
                        ));
                    }
                    self.skip_newlines();
                }
                self.consume(TokenKind::CloseBrace, "Expected '}' at end of match block")?;
                Ok(Stmt::Match {
                    target,
                    arms,
                    otherwise,
                })
            }
            TokenKind::Verify => {
                let line = self.peek().line;
                self.advance();
                if matches!(&self.peek().kind, TokenKind::Ident(s) if s == "that") {
                    self.advance();
                }
                let actual = self.parse_expression()?;
                let expected = if self.match_token(TokenKind::Is) {
                    Some(self.parse_expression()?)
                } else {
                    None
                };
                Ok(Stmt::Verify {
                    actual,
                    expected,
                    line,
                })
            }
            TokenKind::Test => {
                self.advance();
                let name_expr = self.parse_expression()?;
                let name = match name_expr {
                    Expr::Str(s) => s,
                    Expr::Var(s) => s,
                    _ => "unnamed_test".to_string(),
                };
                let body = self.parse_block()?;
                Ok(Stmt::TestBlock { name, body })
            }
            TokenKind::Play => {
                self.advance();
                if self.match_token(TokenKind::Synth) {
                    let freq = self.parse_expression()?;
                    self.consume(
                        TokenKind::Comma,
                        "Expected ',' between frequency and duration",
                    )?;
                    let duration = self.parse_expression()?;
                    Ok(Stmt::PlaySynth { freq, duration })
                } else {
                    return Err("Expected 'synth' after 'play'".to_string());
                }
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
                            ));
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

                // Check if optional 'set var = expr' or 'set target[index] = expr'
                if name == "set"
                    && self.pos + 2 < self.tokens.len()
                    && matches!(self.tokens[self.pos + 1].kind, TokenKind::Ident(_))
                {
                    if self.tokens[self.pos + 2].kind == TokenKind::Equal {
                        self.advance(); // consume "set"
                        let var_tok = self.peek().clone();
                        let var_name = match var_tok.kind {
                            TokenKind::Ident(s) => s,
                            _ => unreachable!(),
                        };
                        self.advance(); // consume ident
                        self.advance(); // consume '='
                        let value = self.parse_expression()?;
                        return Ok(Stmt::Assign {
                            name: var_name,
                            value,
                        });
                    }
                }

                // Check if index assignment: `target[index] = expr`
                if self.pos + 1 < self.tokens.len()
                    && self.tokens[self.pos + 1].kind == TokenKind::OpenBracket
                {
                    let target = Expr::Var(name.clone());
                    self.advance(); // consume ident
                    self.advance(); // consume '['
                    let index = self.parse_expression()?;
                    self.consume(TokenKind::CloseBracket, "Expected ']' after index")?;
                    if self.match_token(TokenKind::Equal) {
                        let value = self.parse_expression()?;
                        return Ok(Stmt::IndexAssign {
                            target,
                            index,
                            value,
                        });
                    }
                }

                // Check if struct field assignment: `target.field = expr`
                if self.pos + 2 < self.tokens.len()
                    && self.tokens[self.pos + 1].kind == TokenKind::Dot
                    && self.pos + 3 < self.tokens.len()
                    && self.tokens[self.pos + 3].kind == TokenKind::Equal
                {
                    let target = name.clone();
                    self.advance(); // consume ident
                    self.advance(); // consume '.'
                    let field = match self.peek().kind.clone() {
                        TokenKind::Ident(s) => s,
                        _ => return Err("Expected field name after '.'".to_string()),
                    };
                    self.advance(); // consume field
                    self.advance(); // consume '='
                    let value = self.parse_expression()?;
                    return Ok(Stmt::FieldAssign {
                        target,
                        field,
                        value,
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
            TokenKind::TypePtr => {
                self.advance();
                Ok(Type::Ptr)
            }
            TokenKind::Ident(name) => {
                self.advance();
                Ok(Type::Custom(name))
            }
            _ => Err(format!(
                "Expected type (Int, String, Bool, Ptr, or struct name) at line {}, col {}",
                tok.line, tok.col
            )),
        }
    }

    pub fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_pipeline()
    }

    fn parse_pipeline(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_or()?;
        while self.match_token(TokenKind::PipeGreater) {
            let next = self.parse_primary()?;
            match next {
                Expr::Call { callee, mut args } => {
                    args.insert(0, expr);
                    expr = Expr::Call { callee, args };
                }
                Expr::Var(callee) => {
                    expr = Expr::Call {
                        callee,
                        args: vec![expr],
                    };
                }
                _ => {
                    return Err(
                        "Expected function call or identifier after pipeline operator '|>'"
                            .to_string(),
                    );
                }
            }
        }
        Ok(expr)
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
        let mut left = self.parse_bitwise_or()?;
        while self.match_token(TokenKind::And) {
            let right = self.parse_bitwise_or()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_bitwise_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_bitwise_xor()?;
        loop {
            let matched = if self.match_token(TokenKind::Pipe) {
                true
            } else if self.check(TokenKind::Bit)
                && self.pos + 1 < self.tokens.len()
                && self.tokens[self.pos + 1].kind == TokenKind::Or
            {
                self.advance();
                self.advance();
                true
            } else {
                false
            };
            if !matched {
                break;
            }
            let right = self.parse_bitwise_xor()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::BitOr,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_bitwise_xor(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_bitwise_and()?;
        loop {
            let matched = if self.match_token(TokenKind::Caret) {
                true
            } else if self.check(TokenKind::Bit)
                && self.pos + 1 < self.tokens.len()
                && self.tokens[self.pos + 1].kind == TokenKind::Xor
            {
                self.advance();
                self.advance();
                true
            } else {
                false
            };
            if !matched {
                break;
            }
            let right = self.parse_bitwise_and()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::BitXor,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_bitwise_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_equality()?;
        loop {
            let matched = if self.match_token(TokenKind::Ampersand) {
                true
            } else if self.check(TokenKind::Bit)
                && self.pos + 1 < self.tokens.len()
                && self.tokens[self.pos + 1].kind == TokenKind::And
            {
                self.advance();
                self.advance();
                true
            } else {
                false
            };
            if !matched {
                break;
            }
            let right = self.parse_equality()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::BitAnd,
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
            } else if self.match_token(TokenKind::Has) {
                let right = self.parse_comparison()?;
                let list_name = match left {
                    Expr::Var(ref s) => s.clone(),
                    _ => return Err("Expected list variable before 'has'".to_string()),
                };
                left = Expr::ListHas {
                    list: list_name,
                    item: Box::new(right),
                };
            } else if self.match_token(TokenKind::Starts) {
                self.consume(TokenKind::With, "Expected 'with' after 'starts'")?;
                let right = self.parse_comparison()?;
                left = Expr::StrStartsWith {
                    source: Box::new(left),
                    prefix: Box::new(right),
                };
            } else if self.match_token(TokenKind::Ends) {
                self.consume(TokenKind::With, "Expected 'with' after 'ends'")?;
                let right = self.parse_comparison()?;
                left = Expr::StrEndsWith {
                    source: Box::new(left),
                    suffix: Box::new(right),
                };
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_shift()?;

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

            let right = self.parse_shift()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_shift(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;
        loop {
            let op = if self.match_token(TokenKind::DoubleLess) {
                BinaryOp::ShiftLeft
            } else if self.match_token(TokenKind::DoubleGreater) {
                BinaryOp::ShiftRight
            } else if self.check(TokenKind::Shift)
                && self.pos + 1 < self.tokens.len()
                && self.tokens[self.pos + 1].kind == TokenKind::Left
            {
                self.advance();
                self.advance();
                BinaryOp::ShiftLeft
            } else if self.check(TokenKind::Shift)
                && self.pos + 1 < self.tokens.len()
                && self.tokens[self.pos + 1].kind == TokenKind::Right
            {
                self.advance();
                self.advance();
                BinaryOp::ShiftRight
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
        if self.match_token(TokenKind::Tilde) {
            let expr = self.parse_unary()?;
            return Ok(Expr::Unary {
                op: UnaryOp::BitNot,
                expr: Box::new(expr),
            });
        }
        if self.check(TokenKind::Bit)
            && self.pos + 1 < self.tokens.len()
            && self.tokens[self.pos + 1].kind == TokenKind::Not
        {
            self.advance();
            self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expr::Unary {
                op: UnaryOp::BitNot,
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
        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;
        loop {
            if self.match_token(TokenKind::OpenBracket) {
                let index = self.parse_expression()?;
                self.consume(TokenKind::CloseBracket, "Expected ']' after index")?;
                expr = Expr::Index {
                    target: Box::new(expr),
                    index: Box::new(index),
                };
            } else if self.match_token(TokenKind::Dot) {
                let field_tok = self.peek().clone();
                let field = match field_tok.kind {
                    TokenKind::Ident(s) => s,
                    _ => {
                        return Err(format!(
                            "Expected field name after '.' at line {}, col {}",
                            field_tok.line, field_tok.col
                        ));
                    }
                };
                self.advance();
                expr = Expr::FieldAccess {
                    target: Box::new(expr),
                    field,
                };
            } else if self.match_token(TokenKind::ApostropheS) {
                let prop_tok = self.peek().clone();
                let prop_expr = match prop_tok.kind {
                    TokenKind::Ident(s) => {
                        self.advance();
                        Expr::Str(s)
                    }
                    TokenKind::StringLit(s) => {
                        self.advance();
                        Expr::Str(s)
                    }
                    _ => {
                        return Err(format!(
                            "Expected property name after 's at line {}, col {}",
                            prop_tok.line, prop_tok.col
                        ));
                    }
                };
                expr = Expr::Index {
                    target: Box::new(expr),
                    index: Box::new(prop_expr),
                };
            } else {
                break;
            }
        }
        Ok(expr)
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
                if matches!(&self.peek().kind, TokenKind::Ident(s) if s == "user") {
                    self.advance();
                    let prompt = self.parse_expression()?;
                    Ok(Expr::AskUser(Box::new(prompt)))
                } else if self.match_token(TokenKind::Hidden) {
                    let prompt = self.parse_expression()?;
                    Ok(Expr::AskHidden(Box::new(prompt)))
                } else {
                    let prompt = self.parse_expression()?;
                    Ok(Expr::Ask(Box::new(prompt)))
                }
            }
            TokenKind::Run => {
                self.advance();
                let cmd = self.parse_expression()?;
                Ok(Expr::Run(Box::new(cmd)))
            }
            TokenKind::Read => {
                self.advance();
                if self.match_token(TokenKind::Web) {
                    let url = self.parse_expression()?;
                    Ok(Expr::ReadWeb(Box::new(url)))
                } else {
                    let path = self.parse_expression()?;
                    Ok(Expr::ReadFile(Box::new(path)))
                }
            }
            TokenKind::Choose => {
                self.advance();
                let prompt = self.parse_expression()?;
                let mut options = Vec::new();
                while self.match_token(TokenKind::Comma) {
                    options.push(self.parse_expression()?);
                }
                Ok(Expr::Choose {
                    prompt: Box::new(prompt),
                    options,
                })
            }
            TokenKind::File => {
                self.advance();
                let path = self.parse_expression()?;
                self.consume(
                    TokenKind::Exists,
                    "Expected 'exists' after file path in 'file ... exists'",
                )?;
                Ok(Expr::FileExists(Box::new(path)))
            }
            TokenKind::Replace => {
                self.advance();
                let target = self.parse_expression()?;
                self.consume(
                    TokenKind::With,
                    "Expected 'with' after target in 'replace ... with ... in ...'",
                )?;
                let replacement = self.parse_expression()?;
                self.consume(
                    TokenKind::In,
                    "Expected 'in' in 'replace ... with ... in ...'",
                )?;
                let source = self.parse_expression()?;
                Ok(Expr::StrReplace {
                    target: Box::new(target),
                    replacement: Box::new(replacement),
                    source: Box::new(source),
                })
            }
            TokenKind::How => {
                self.advance();
                self.consume(TokenKind::Many, "Expected 'many' after 'how'")?;
                let list_tok = self.peek().clone();
                let list_name = match list_tok.kind {
                    TokenKind::Ident(s) => s,
                    _ => {
                        return Err(format!(
                            "Expected list name after 'how many' at line {}",
                            list_tok.line
                        ));
                    }
                };
                self.advance();
                Ok(Expr::ListCount(list_name))
            }
            TokenKind::Count => {
                self.advance();
                if let TokenKind::Ident(s) = &self.peek().kind {
                    let list_name = s.clone();
                    self.advance();
                    Ok(Expr::ListCount(list_name))
                } else {
                    Ok(Expr::Var("count".to_string()))
                }
            }
            TokenKind::List => {
                self.advance();
                Ok(Expr::ListLiteral(Vec::new()))
            }
            TokenKind::Random => {
                self.advance();
                let min = self.parse_expression()?;
                self.consume(
                    TokenKind::To,
                    "Expected 'to' in random expression (e.g. random 1 to 100)",
                )?;
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
            TokenKind::Addr => {
                self.advance();
                let var_tok = self.peek().clone();
                let var = match var_tok.kind {
                    TokenKind::Ident(s) => s,
                    _ => {
                        return Err(format!(
                            "Expected variable name after 'addr' at line {}, col {}",
                            var_tok.line, var_tok.col
                        ));
                    }
                };
                self.advance();
                Ok(Expr::AddrOf(var))
            }
            TokenKind::AtSign => {
                self.advance();
                let target = self.parse_primary()?;
                Ok(Expr::Deref(Box::new(target)))
            }
            TokenKind::Alloc => {
                self.advance();
                let size_expr = if self.match_token(TokenKind::OpenParen) {
                    let e = self.parse_expression()?;
                    self.consume(TokenKind::CloseParen, "Expected ')' after alloc argument")?;
                    e
                } else {
                    self.parse_unary()?
                };
                Ok(Expr::Alloc(Box::new(size_expr)))
            }
            TokenKind::Free => {
                self.advance();
                let ptr_expr = if self.match_token(TokenKind::OpenParen) {
                    let e = self.parse_expression()?;
                    self.consume(TokenKind::CloseParen, "Expected ')' after free argument")?;
                    e
                } else {
                    self.parse_unary()?
                };
                Ok(Expr::Free(Box::new(ptr_expr)))
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
                            ));
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
                let expr = if self.match_token(TokenKind::OpenParen) {
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
                    self.consume(
                        TokenKind::CloseParen,
                        "Expected ')' after function arguments",
                    )?;
                    Expr::Call { callee: name, args }
                } else {
                    Expr::Var(name)
                };

                Ok(expr)
            }
            TokenKind::OpenParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(TokenKind::CloseParen, "Expected ')'")?;
                Ok(expr)
            }
            TokenKind::Map => {
                self.advance();
                self.consume(TokenKind::OpenBrace, "Expected '{' after 'map'")?;
                self.parse_map_entries()
            }
            TokenKind::OpenBrace => {
                self.advance();
                self.skip_newlines();
                if self.check(TokenKind::CloseBrace) {
                    self.advance();
                    return Ok(Expr::MapLiteral(Vec::new()));
                }
                let is_map = if self.pos + 1 < self.tokens.len() {
                    self.tokens[self.pos + 1].kind == TokenKind::Colon
                } else {
                    false
                };

                if is_map {
                    self.parse_map_entries()
                } else {
                    let expr = self.parse_expression()?;
                    self.skip_newlines();
                    self.consume(TokenKind::CloseBrace, "Expected '}' after expression")?;
                    Ok(expr)
                }
            }
            TokenKind::Keys => {
                self.advance();
                let map_tok = self.peek().clone();
                let name = match map_tok.kind {
                    TokenKind::Ident(s) => s,
                    _ => return Err("Expected map name after 'keys'".to_string()),
                };
                self.advance();
                Ok(Expr::MapKeys(name))
            }
            TokenKind::Values => {
                self.advance();
                let map_tok = self.peek().clone();
                let name = match map_tok.kind {
                    TokenKind::Ident(s) => s,
                    _ => return Err("Expected map name after 'values'".to_string()),
                };
                self.advance();
                Ok(Expr::MapValues(name))
            }
            _ => Err(format!(
                "Unexpected token {:?} in expression at line {}, col {}",
                tok.kind, tok.line, tok.col
            )),
        }
    }

    fn parse_map_entries(&mut self) -> Result<Expr, String> {
        self.skip_newlines();
        let mut pairs = Vec::new();
        if !self.check(TokenKind::CloseBrace) {
            loop {
                self.skip_newlines();
                if self.check(TokenKind::CloseBrace) {
                    break;
                }
                let k_tok = self.peek().clone();
                let key_expr = match k_tok.kind {
                    TokenKind::Ident(s) => {
                        self.advance();
                        Expr::Str(s)
                    }
                    TokenKind::StringLit(s) => {
                        self.advance();
                        Expr::Str(s)
                    }
                    _ => self.parse_expression()?,
                };
                self.consume(TokenKind::Colon, "Expected ':' after map key")?;
                let val_expr = self.parse_expression()?;
                pairs.push((key_expr, val_expr));
                self.skip_newlines();
                if self.match_token(TokenKind::Comma) {
                    self.skip_newlines();
                    if self.check(TokenKind::CloseBrace) {
                        break;
                    }
                    continue;
                } else {
                    break;
                }
            }
        }
        self.skip_newlines();
        self.consume(TokenKind::CloseBrace, "Expected '}' at end of map literal")?;
        Ok(Expr::MapLiteral(pairs))
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
