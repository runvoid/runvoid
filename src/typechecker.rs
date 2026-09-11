use crate::ast::*;
use std::collections::{HashMap, HashSet};

pub struct TypeChecker {
    directives: Directives,
    symbols: HashSet<String>,
    actions: HashMap<String, (Vec<Type>, Type)>,
    structs: HashMap<String, Vec<(String, Type)>>,
    imports: HashSet<String>,
}

impl TypeChecker {
    pub fn new(directives: Directives) -> Self {
        let mut imports = HashSet::new();
        for m in &directives.modules {
            imports.insert(m.clone());
        }
        for l in &directives.libs {
            imports.insert(l.clone());
        }
        Self {
            directives,
            symbols: HashSet::new(),
            actions: HashMap::new(),
            structs: HashMap::new(),
            imports,
        }
    }

    pub fn check(&mut self, program: &Program) -> Result<(), String> {
        // 1. Validate directives
        if program.directives.add_advanced
            && (!program.directives.remove_gc || !program.directives.remove_basic)
        {
            return Err(
                "Error: Directive 'add Advanced' requires disabling basic mode and garbage collection.\n\
Please declare at the beginning of the file:\n  remove garbageC\n  remove Basic\n  add Advanced"
                    .to_string(),
            );
        }

        if (program.directives.add_freestanding || program.directives.remove_linux)
            && (!program.directives.add_advanced
                || !program.directives.remove_basic
                || !program.directives.remove_gc)
        {
            return Err(
                "Error: Freestanding mode ('remove Linux', 'add Freestanding') requires:\n  remove garbageC\n  remove Basic\n  remove Linux\n  add Advanced\n  add Freestanding".to_string()
            );
        }

        // Register actions, structs, and externs first
        for stmt in &program.statements {
            match stmt {
                Stmt::ActionDef {
                    name,
                    params,
                    return_type,
                    ..
                } => {
                    let p_types = params
                        .iter()
                        .map(|(_, t)| t.clone().unwrap_or(Type::Auto))
                        .collect();
                    let ret = return_type.clone().unwrap_or(Type::Void);
                    self.actions.insert(name.clone(), (p_types, ret));
                    self.symbols.insert(name.clone());
                }
                Stmt::StructDef { name, fields } => {
                    self.structs.insert(name.clone(), fields.clone());
                    self.symbols.insert(name.clone());
                }
                Stmt::ExternBlock { actions, .. } => {
                    for act in actions {
                        let p_types = act.params.iter().map(|(_, t)| t.clone()).collect();
                        self.actions
                            .insert(act.name.clone(), (p_types, act.return_type.clone()));
                        self.symbols.insert(act.name.clone());
                    }
                }
                Stmt::Use(path) => {
                    self.imports.insert(path.clone());
                }
                Stmt::UseLib(lib) => {
                    self.imports.insert(lib.clone());
                }
                _ => {}
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
            Stmt::Alert(msg) => self.check_expr(msg),
            Stmt::Beep => Ok(()),
            Stmt::Speak(msg) => self.check_expr(msg),
            Stmt::OpenWeb(url) => self.check_expr(url),
            Stmt::DownloadWeb { url, target } => {
                self.check_expr(url)?;
                self.check_expr(target)
            }
            Stmt::Screen {
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
            Stmt::DrawCircle {
                x,
                y,
                radius,
                color,
            } => {
                self.check_expr(x)?;
                self.check_expr(y)?;
                self.check_expr(radius)?;
                if let Some(c) = color {
                    self.check_expr(c)?;
                }
                Ok(())
            }
            Stmt::DrawRect {
                x,
                y,
                width,
                height,
                color,
            } => {
                self.check_expr(x)?;
                self.check_expr(y)?;
                self.check_expr(width)?;
                self.check_expr(height)?;
                if let Some(c) = color {
                    self.check_expr(c)?;
                }
                Ok(())
            }
            Stmt::DrawLine {
                x1,
                y1,
                x2,
                y2,
                color,
            } => {
                self.check_expr(x1)?;
                self.check_expr(y1)?;
                self.check_expr(x2)?;
                self.check_expr(y2)?;
                if let Some(c) = color {
                    self.check_expr(c)?;
                }
                Ok(())
            }
            Stmt::DrawText { text, x, y, color } => {
                self.check_expr(text)?;
                self.check_expr(x)?;
                self.check_expr(y)?;
                if let Some(c) = color {
                    self.check_expr(c)?;
                }
                Ok(())
            }
            Stmt::ClearScreen => Ok(()),
            Stmt::CursorAt { x, y } => {
                self.check_expr(x)?;
                self.check_expr(y)
            }
            Stmt::CreateFolder(path) => self.check_expr(path),
            Stmt::DeleteFile(path) => self.check_expr(path),
            Stmt::DeleteFolder(path) => self.check_expr(path),
            Stmt::CopyFile { src, dest } => {
                self.check_expr(src)?;
                self.check_expr(dest)
            }
            Stmt::AddToList { item, list } => {
                self.check_expr(item)?;
                if !self.symbols.contains(list) {
                    return Err(format!(
                        "Error: List '{}' was not declared before use",
                        list
                    ));
                }
                Ok(())
            }
            Stmt::RemoveFromList { item, list } => {
                self.check_expr(item)?;
                if !self.symbols.contains(list) {
                    return Err(format!(
                        "Error: List '{}' was not declared before use",
                        list
                    ));
                }
                Ok(())
            }
            Stmt::ForEvery {
                item_var,
                list_var,
                body,
            } => {
                if !self.symbols.contains(list_var) {
                    return Err(format!(
                        "Error: List '{}' was not declared before use",
                        list_var
                    ));
                }
                self.symbols.insert(item_var.clone());
                for s in body {
                    self.check_stmt(s)?;
                }
                Ok(())
            }
            Stmt::MakeString { var_name, .. } => {
                if !self.symbols.contains(var_name) {
                    return Err(format!(
                        "Error: Variable '{}' was not declared before use",
                        var_name
                    ));
                }
                Ok(())
            }
            Stmt::MeasureTime { body } => {
                for s in body {
                    self.check_stmt(s)?;
                }
                Ok(())
            }
            Stmt::RunCommand { command } => self.check_expr(command),
            Stmt::ExprStmt(e) => self.check_expr(e),
            Stmt::UseLib(lib) => {
                self.imports.insert(lib.clone());
                Ok(())
            }
            Stmt::InlineAsm(_) => {
                if !self.directives.add_advanced {
                    return Err(
                        "Error: Inline assembly ('asm { ... }') requires 'add Advanced' mode"
                            .to_string(),
                    );
                }
                Ok(())
            }
            Stmt::MeasureCycles { body } => {
                if !self.directives.add_advanced {
                    return Err("Error: 'measure cycles' requires 'add Advanced' mode".to_string());
                }
                for s in body {
                    self.check_stmt(s)?;
                }
                Ok(())
            }
            Stmt::DerefAssign {
                ptr_expr,
                value_expr,
            } => {
                if !self.directives.add_advanced {
                    return Err(
                        "Error: Raw pointer dereference assignment requires 'add Advanced' mode"
                            .to_string(),
                    );
                }
                self.check_expr(ptr_expr)?;
                self.check_expr(value_expr)?;
                Ok(())
            }
            Stmt::FieldAssign { target, value, .. } => {
                if !self.symbols.contains(target) {
                    return Err(format!(
                        "Error: Variable '{}' was not declared before field assignment",
                        target
                    ));
                }
                self.check_expr(value)?;
                Ok(())
            }
            Stmt::StructDef { name, .. } => {
                self.symbols.insert(name.clone());
                Ok(())
            }
            Stmt::ExternBlock { actions, .. } => {
                if !self.directives.add_advanced {
                    return Err(
                        "Error: 'extern' C FFI blocks require 'add Advanced' mode".to_string()
                    );
                }
                for act in actions {
                    self.symbols.insert(act.name.clone());
                }
                Ok(())
            }
            Stmt::ThreadSpawn { body } => {
                if self.directives.add_advanced && !self.imports.contains("thread") {
                    return Err(
                        "Error: In 'add Advanced' mode, spawning threads requires: use thread"
                            .to_string(),
                    );
                }
                for s in body {
                    self.check_stmt(s)?;
                }
                Ok(())
            }
            Stmt::AtomicAdd { var, val } => {
                if !self.symbols.contains(var) {
                    return Err(format!(
                        "Error: Variable '{}' was not declared before 'atomic add'",
                        var
                    ));
                }
                if self.directives.add_advanced && !self.imports.contains("thread") {
                    return Err(
                        "Error: In 'add Advanced' mode, 'atomic add' requires: use thread"
                            .to_string(),
                    );
                }
                self.check_expr(val)?;
                Ok(())
            }
            Stmt::AddToMap { key, value, map } => {
                if !self.symbols.contains(map) {
                    return Err(format!(
                        "Error: Variable '{}' was not declared before use",
                        map
                    ));
                }
                self.check_expr(key)?;
                self.check_expr(value)?;
                Ok(())
            }
            Stmt::RemoveFromMap { key, map } => {
                if !self.symbols.contains(map) {
                    return Err(format!(
                        "Error: Variable '{}' was not declared before use",
                        map
                    ));
                }
                self.check_expr(key)?;
                Ok(())
            }
            Stmt::IndexAssign {
                target,
                index,
                value,
            } => {
                self.check_expr(target)?;
                self.check_expr(index)?;
                self.check_expr(value)?;
                Ok(())
            }
            Stmt::Match {
                target,
                arms,
                otherwise,
            } => {
                self.check_expr(target)?;
                for arm in arms {
                    self.check_expr(&arm.pattern)?;
                    for s in &arm.body {
                        self.check_stmt(s)?;
                    }
                }
                if let Some(oth) = otherwise {
                    for s in oth {
                        self.check_stmt(s)?;
                    }
                }
                Ok(())
            }
            Stmt::Verify {
                actual, expected, ..
            } => {
                self.check_expr(actual)?;
                if let Some(exp) = expected {
                    self.check_expr(exp)?;
                }
                Ok(())
            }
            Stmt::TestBlock { body, .. } => {
                for s in body {
                    self.check_stmt(s)?;
                }
                Ok(())
            }
            Stmt::PlaySynth { freq, duration } => {
                self.check_expr(freq)?;
                self.check_expr(duration)?;
                Ok(())
            }
        }
    }

    fn check_expr(&mut self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::Int(_) | Expr::Str(_) | Expr::Bool(_) => Ok(()),
            Expr::Var(name) => {
                if !self.symbols.contains(name) {
                    return Err(format!("Error: Use of undeclared variable '{}'", name));
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
                if !self.actions.contains_key(callee) && !self.structs.contains_key(callee) {
                    let math_funcs = ["sqrt", "sin", "cos", "tan", "pow", "abs"];
                    let sys_funcs = ["exit", "getpid"];
                    let net_funcs = [
                        "tcp_listen",
                        "tcp_accept",
                        "tcp_connect",
                        "tcp_send",
                        "tcp_recv",
                        "tcp_close",
                    ];
                    let thread_funcs = ["thread_spawn", "thread_join"];

                    if math_funcs.contains(&callee.as_str()) {
                        if self.directives.add_advanced && !self.imports.contains("math") {
                            return Err(format!(
                                "Error: In 'add Advanced' mode, using '{}' requires: use math",
                                callee
                            ));
                        }
                    } else if sys_funcs.contains(&callee.as_str()) {
                        if self.directives.add_advanced && !self.imports.contains("sys") {
                            return Err(format!(
                                "Error: In 'add Advanced' mode, using '{}' requires: use sys",
                                callee
                            ));
                        }
                    } else if net_funcs.contains(&callee.as_str()) {
                        if self.directives.add_advanced && !self.imports.contains("net") {
                            return Err(format!(
                                "Error: In 'add Advanced' mode, using '{}' requires: use net",
                                callee
                            ));
                        }
                    } else if thread_funcs.contains(&callee.as_str()) {
                        if self.directives.add_advanced && !self.imports.contains("thread") {
                            return Err(format!(
                                "Error: In 'add Advanced' mode, using '{}' requires: use thread",
                                callee
                            ));
                        }
                    } else {
                        return Err(format!("Error: Call to undeclared action '{}'", callee));
                    }
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
            Expr::AskUser(p) | Expr::AskHidden(p) | Expr::ReadWeb(p) | Expr::FileExists(p) => {
                self.check_expr(p)
            }
            Expr::Choose { prompt, options } => {
                self.check_expr(prompt)?;
                for o in options {
                    self.check_expr(o)?;
                }
                Ok(())
            }
            Expr::ListLiteral(items) => {
                for it in items {
                    self.check_expr(it)?;
                }
                Ok(())
            }
            Expr::ListHas { list, item } => {
                if !self.symbols.contains(list) {
                    return Err(format!(
                        "Error: List '{}' was not declared before use",
                        list
                    ));
                }
                self.check_expr(item)
            }
            Expr::ListCount(list) => {
                if !self.symbols.contains(list) {
                    return Err(format!(
                        "Error: List '{}' was not declared before use",
                        list
                    ));
                }
                Ok(())
            }
            Expr::StrReplace {
                target,
                replacement,
                source,
            } => {
                self.check_expr(target)?;
                self.check_expr(replacement)?;
                self.check_expr(source)
            }
            Expr::StrStartsWith { source, prefix } => {
                self.check_expr(source)?;
                self.check_expr(prefix)
            }
            Expr::StrEndsWith { source, suffix } => {
                self.check_expr(source)?;
                self.check_expr(suffix)
            }
            Expr::AddrOf(name) => {
                if !self.symbols.contains(name) {
                    return Err(format!(
                        "Error: Cannot take address of undeclared variable '{}'",
                        name
                    ));
                }
                if !self.directives.add_advanced {
                    return Err("Error: 'addr' operator requires 'add Advanced' mode".to_string());
                }
                Ok(())
            }
            Expr::Deref(inner) => {
                if !self.directives.add_advanced {
                    return Err(
                        "Error: Pointer dereference '@' requires 'add Advanced' mode".to_string(),
                    );
                }
                self.check_expr(inner)
            }
            Expr::Alloc(size_expr) => {
                if !self.directives.add_advanced {
                    return Err("Error: 'alloc' requires 'add Advanced' mode".to_string());
                }
                if !self.imports.contains("mem") {
                    return Err(
                        "Error: In 'add Advanced' mode, 'alloc' requires: use mem".to_string()
                    );
                }
                self.check_expr(size_expr)
            }
            Expr::Free(ptr_expr) => {
                if !self.directives.add_advanced {
                    return Err("Error: 'free' requires 'add Advanced' mode".to_string());
                }
                if !self.imports.contains("mem") {
                    return Err(
                        "Error: In 'add Advanced' mode, 'free' requires: use mem".to_string()
                    );
                }
                self.check_expr(ptr_expr)
            }
            Expr::FieldAccess { target, .. } => self.check_expr(target),
            Expr::StructInit { name, args } => {
                if !self.structs.contains_key(name) {
                    return Err(format!("Error: Unknown struct '{}'", name));
                }
                for a in args {
                    self.check_expr(a)?;
                }
                Ok(())
            }
            Expr::MapLiteral(entries) => {
                for (k, v) in entries {
                    self.check_expr(k)?;
                    self.check_expr(v)?;
                }
                Ok(())
            }
            Expr::Index { target, index } => {
                self.check_expr(target)?;
                self.check_expr(index)?;
                Ok(())
            }
            Expr::MapKeys(map) | Expr::MapValues(map) => {
                if !self.symbols.contains(map) {
                    return Err(format!(
                        "Error: Variable '{}' was not declared before use",
                        map
                    ));
                }
                Ok(())
            }
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
