use crate::ast::*;
use std::collections::{HashMap, HashSet};

pub struct TypeChecker {
    directives: Directives,
    symbols: HashSet<String>,
    actions: HashMap<String, (Vec<Type>, Type)>,
    imports: HashSet<String>,
}

impl TypeChecker {
    pub fn new(directives: Directives) -> Self {
        Self {
            directives,
            symbols: HashSet::new(),
            actions: HashMap::new(),
            imports: HashSet::new(),
        }
    }

    pub fn check(&mut self, program: &Program) -> Result<(), String> {
        // 1. Validate directives
        if program.directives.add_advanced {
            if !program.directives.remove_gc || !program.directives.remove_basic {
                return Err(
                    "Error: Directive 'add Advanced' requires disabling basic mode and garbage collection.\n\
Please declare at the beginning of the file:\n  remove garbageC\n  remove Basic\n  add Advanced"
                        .to_string(),
                );
            }
        }

        // Register actions first
        for stmt in &program.statements {
            if let Stmt::ActionDef {
                name,
                params,
                return_type,
                ..
            } = stmt
            {
                let p_types = params
                    .iter()
                    .map(|(_, t)| t.clone().unwrap_or(Type::Auto))
                    .collect();
                let ret = return_type.clone().unwrap_or(Type::Void);
                self.actions.insert(name.clone(), (p_types, ret));
            }
        }

        // 2. Validate statements
        for stmt in &program.statements {
            self.check_stmt(stmt)?;
        }

        Ok(())
    }

    fn check_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Remember {
                name,
                explicit_type,
                value,
            } => {
                if self.directives.remove_basic && explicit_type.is_none() {
                    return Err(format!(
                        "Error: In 'remove Basic' mode, variable '{}' must have an explicit type (e.g.: remember {}: Int = ...)",
                        name, name
                    ));
                }
                self.check_expr(value)?;
                self.symbols.insert(name.clone());
                Ok(())
            }
            Stmt::Assign { name, value } => {
                if !self.symbols.contains(name) {
                    return Err(format!(
                        "Error: Variable '{}' was not declared before use",
                        name
                    ));
                }
                self.check_expr(value)?;
                Ok(())
            }
            Stmt::If {
                condition,
                then_branch,
                otherwise_ifs,
                otherwise_branch,
            } => {
                self.check_expr(condition)?;
                for s in then_branch {
                    self.check_stmt(s)?;
                }
                for (elif_cond, elif_body) in otherwise_ifs {
                    self.check_expr(elif_cond)?;
                    for s in elif_body {
                        self.check_stmt(s)?;
                    }
                }
                if let Some(other_body) = otherwise_branch {
                    for s in other_body {
                        self.check_stmt(s)?;
                    }
                }
                Ok(())
            }
            Stmt::Repeat {
                count,
                var_name,
                body,
            } => {
                self.check_expr(count)?;
                if let Some(v) = var_name {
                    self.symbols.insert(v.clone());
                }
                for s in body {
                    self.check_stmt(s)?;
                }
                Ok(())
            }
            Stmt::While { condition, body } => {
                self.check_expr(condition)?;
                for s in body {
                    self.check_stmt(s)?;
                }
                Ok(())
            }
            Stmt::Stop | Stmt::Skip => Ok(()),
            Stmt::ActionDef {
                name,
                params,
                return_type,
                body,
            } => {
                if self.directives.remove_basic {
                    for (p_name, p_type) in params {
                        if p_type.is_none() {
                            return Err(format!(
                                "Error: In 'remove Basic' mode, parameter '{}: Type' in action '{}' must have an explicit type",
                                p_name, name
                            ));
                        }
                    }
                    if return_type.is_none() {
                        return Err(format!(
                            "Error: In 'remove Basic' mode, return type of action '{}' must be explicitly specified (e.g.: action {}(...): Int)",
                            name, name
                        ));
                    }
                }

                // Sub-scope for function params
                for (p_name, _) in params {
                    self.symbols.insert(p_name.clone());
                }
                for s in body {
                    self.check_stmt(s)?;
                }
                Ok(())
            }
            Stmt::Give(expr_opt) => {
                if let Some(e) = expr_opt {
                    self.check_expr(e)?;
                }
                Ok(())
            }
            Stmt::Use(mod_name) => {
                self.imports.insert(mod_name.clone());
                Ok(())
            }
            Stmt::Say { expr, .. } => {
                if self.directives.add_advanced && !self.imports.contains("ior") {
                    return Err(
                        "Error: In 'add Advanced' mode, using 'say' requires explicitly importing the I/O module: use ior"
                            .to_string(),
                    );
                }
                self.check_expr(expr)
            }
            Stmt::WriteFile { data, target } => {
                if self.directives.add_advanced && !self.imports.contains("ior") {
                    return Err(
                        "Error: In 'add Advanced' mode, using 'write' requires explicitly importing the I/O module: use ior"
                            .to_string(),
                    );
                }
                self.check_expr(data)?;
                self.check_expr(target)?;
                Ok(())
            }
            Stmt::Wait(sec) => self.check_expr(sec),
            Stmt::Window {
                title,
                width,
                height,
                body,
            } => {
                self.check_expr(title)?;
                if let Some(w) = width {
                    self.check_expr(w)?;
                }
                if let Some(h) = height {
                    self.check_expr(h)?;
                }
                for s in body {
                    self.check_stmt(s)?;
                }
                Ok(())
            }
            Stmt::GuiButton { label, action } => {
                self.check_expr(label)?;
                for s in action {
                    self.check_stmt(s)?;
                }
                Ok(())
            }
            Stmt::GuiLabel { text } => self.check_expr(text),
            Stmt::GuiCheckbox { label, initial_val } => {
                self.check_expr(label)?;
                if let Some(init) = initial_val {
                    self.check_expr(init)?;
                }
                Ok(())
            }
            Stmt::ImrvStmt { label, extra, .. } => {
                self.check_expr(label)?;
                if let Some(e) = extra {
                    self.check_expr(e)?;
                }
                Ok(())
            }
            Stmt::RunCommand { command } => self.check_expr(command),
            Stmt::ExprStmt(e) => self.check_expr(e),
        }
    }

    fn check_expr(&mut self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::Int(_) | Expr::Str(_) | Expr::Bool(_) => Ok(()),
            Expr::Var(name) => {
                if !self.symbols.contains(name) {
                    return Err(format!(
                        "Error: Use of undeclared variable '{}'",
                        name
                    ));
                }
                Ok(())
            }
            Expr::Binary { left, right, .. } => {
                self.check_expr(left)?;
                self.check_expr(right)?;
                Ok(())
            }
            Expr::Unary { expr, .. } => self.check_expr(expr),
            Expr::Call { callee, args } => {
                if !self.actions.contains_key(callee) {
                    return Err(format!("Error: Call to undeclared action '{}'", callee));
                }
                for a in args {
                    self.check_expr(a)?;
                }
                Ok(())
            }
            Expr::InterpolatedString(parts) => {
                for p in parts {
                    self.check_expr(p)?;
                }
                Ok(())
            }
            Expr::Ask(prompt) => {
                if self.directives.add_advanced && !self.imports.contains("ior") {
                    return Err(
                        "Error: In 'add Advanced' mode, using 'ask' requires explicitly importing the I/O module: use ior"
                            .to_string(),
                    );
                }
                self.check_expr(prompt)
            }
            Expr::ReadFile(path) => {
                if self.directives.add_advanced && !self.imports.contains("ior") {
                    return Err(
                        "Error: In 'add Advanced' mode, using 'read' requires explicitly importing the I/O module: use ior"
                            .to_string(),
                    );
                }
                self.check_expr(path)
            }
            Expr::Random { min, max } => {
                self.check_expr(min)?;
                self.check_expr(max)?;
                Ok(())
            }
            Expr::ImrvDraw { label, extra, .. } => {
                self.check_expr(label)?;
                if let Some(e) = extra {
                    self.check_expr(e)?;
                }
                Ok(())
            }
            Expr::Run(cmd) => self.check_expr(cmd),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_advanced_requires_removals() {
        let code = r#"
            add Advanced
            say "Hello"
        "#;
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse().unwrap();

        let mut checker = TypeChecker::new(prog.directives.clone());
        let res = checker.check(&prog);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("add Advanced"));
    }

    #[test]
    fn test_advanced_valid() {
        let code = r#"
            remove garbageC
            remove Basic
            add Advanced
            use ior

            remember x: Int = 42
            say x
        "#;
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse().unwrap();

        let mut checker = TypeChecker::new(prog.directives.clone());
        assert!(checker.check(&prog).is_ok());
    }

    #[test]
    fn test_advanced_requires_use_ior() {
        let code = r#"
            remove garbageC
            remove Basic
            add Advanced

            remember x: Int = 42
            say x
        "#;
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse().unwrap();

        let mut checker = TypeChecker::new(prog.directives.clone());
        let res = checker.check(&prog);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("use ior"));
    }

    #[test]
    fn test_remove_basic_requires_explicit_types() {
        let code = r#"
            remove Basic
            remember x = 42
            say x
        "#;
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse().unwrap();

        let mut checker = TypeChecker::new(prog.directives.clone());
        let res = checker.check(&prog);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("must have an explicit type"));
    }
}
