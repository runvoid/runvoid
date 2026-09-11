use crate::ast::*;

pub struct Optimizer {
    advanced_mode: bool,
    pub folded_constants_count: usize,
    pub eliminated_stmts_count: usize,
}

impl Optimizer {
    pub fn new(advanced_mode: bool) -> Self {
        Self {
            advanced_mode,
            folded_constants_count: 0,
            eliminated_stmts_count: 0,
        }
    }

    pub fn optimize_program(&mut self, mut program: Program) -> Program {
        if !self.advanced_mode {
            return program;
        }

        program.statements = self.optimize_statements(program.statements);
        program
    }

    pub fn optimize_statements(&mut self, stmts: Vec<Stmt>) -> Vec<Stmt> {
        let mut optimized = Vec::new();
        let mut reached_terminal = false;

        for stmt in stmts {
            if reached_terminal {
                self.eliminated_stmts_count += 1;
                continue;
            }

            match stmt {
                Stmt::Say {
                    expr,
                    newline,
                    color,
                } => {
                    let opt_expr = self.optimize_expr(expr);
                    optimized.push(Stmt::Say {
                        expr: opt_expr,
                        newline,
                        color,
                    });
                }
                Stmt::Remember {
                    name,
                    explicit_type,
                    value,
                } => {
                    let opt_val = self.optimize_expr(value);
                    optimized.push(Stmt::Remember {
                        name,
                        explicit_type,
                        value: opt_val,
                    });
                }
                Stmt::Assign { name, value } => {
                    let opt_val = self.optimize_expr(value);
                    optimized.push(Stmt::Assign {
                        name,
                        value: opt_val,
                    });
                }
                Stmt::If {
                    condition,
                    then_branch,
                    otherwise_ifs,
                    otherwise_branch,
                } => {
                    let opt_cond = self.optimize_expr(condition);

                    // Check if condition is statically known
                    if let Expr::Bool(b) = opt_cond {
                        if b {
                            // Condition is always true, inline then_branch
                            let opt_then = self.optimize_statements(then_branch);
                            optimized.extend(opt_then);
                            continue;
                        } else {
                            // Condition is always false, check otherwise_ifs / otherwise
                            let mut resolved = false;
                            for (elif_c, elif_b) in otherwise_ifs {
                                let opt_elif_c = self.optimize_expr(elif_c);
                                if let Expr::Bool(true) = opt_elif_c {
                                    let opt_elif_body = self.optimize_statements(elif_b);
                                    optimized.extend(opt_elif_body);
                                    resolved = true;
                                    break;
                                }
                            }
                            if let (false, Some(other_b)) = (resolved, otherwise_branch) {
                                let opt_other = self.optimize_statements(other_b);
                                optimized.extend(opt_other);
                            }
                            continue;
                        }
                    }

                    let opt_then = self.optimize_statements(then_branch);
                    let mut opt_otherwise_ifs = Vec::new();
                    for (c, b) in otherwise_ifs {
                        opt_otherwise_ifs
                            .push((self.optimize_expr(c), self.optimize_statements(b)));
                    }
                    let opt_otherwise = otherwise_branch.map(|b| self.optimize_statements(b));

                    optimized.push(Stmt::If {
                        condition: opt_cond,
                        then_branch: opt_then,
                        otherwise_ifs: opt_otherwise_ifs,
                        otherwise_branch: opt_otherwise,
                    });
                }
                Stmt::Repeat {
                    count,
                    var_name,
                    body,
                } => {
                    let opt_count = self.optimize_expr(count);
                    let opt_body = self.optimize_statements(body);
                    optimized.push(Stmt::Repeat {
                        count: opt_count,
                        var_name,
                        body: opt_body,
                    });
                }
                Stmt::While { condition, body } => {
                    let opt_cond = self.optimize_expr(condition);
                    if let Expr::Bool(false) = opt_cond {
                        // While false loop never runs!
                        self.eliminated_stmts_count += 1;
                        continue;
                    }
                    let opt_body = self.optimize_statements(body);
                    optimized.push(Stmt::While {
                        condition: opt_cond,
                        body: opt_body,
                    });
                }
                Stmt::ActionDef {
                    name,
                    params,
                    return_type,
                    body,
                } => {
                    let opt_body = self.optimize_statements(body);
                    optimized.push(Stmt::ActionDef {
                        name,
                        params,
                        return_type,
                        body: opt_body,
                    });
                }
                Stmt::Give(expr_opt) => {
                    let opt_expr = expr_opt.map(|e| self.optimize_expr(e));
                    optimized.push(Stmt::Give(opt_expr));
                    reached_terminal = true;
                }
                Stmt::Stop => {
                    optimized.push(Stmt::Stop);
                    reached_terminal = true;
                }
                Stmt::Skip => {
                    optimized.push(Stmt::Skip);
                    reached_terminal = true;
                }
                Stmt::Use(name) => {
                    optimized.push(Stmt::Use(name));
                }
                Stmt::WriteFile { data, target } => {
                    let opt_data = self.optimize_expr(data);
                    let opt_target = self.optimize_expr(target);
                    optimized.push(Stmt::WriteFile {
                        data: opt_data,
                        target: opt_target,
                    });
                }
                Stmt::Wait(sec) => {
                    let opt_sec = self.optimize_expr(sec);
                    optimized.push(Stmt::Wait(opt_sec));
                }
                Stmt::Window {
                    title,
                    width,
                    height,
                    body,
                } => {
                    let opt_title = self.optimize_expr(title);
                    let opt_w = width.map(|w| self.optimize_expr(w));
                    let opt_h = height.map(|h| self.optimize_expr(h));
                    let opt_body = self.optimize_statements(body);
                    optimized.push(Stmt::Window {
                        title: opt_title,
                        width: opt_w,
                        height: opt_h,
                        body: opt_body,
                    });
                }
                Stmt::GuiButton { label, action } => {
                    let opt_label = self.optimize_expr(label);
                    let opt_action = self.optimize_statements(action);
                    optimized.push(Stmt::GuiButton {
                        label: opt_label,
                        action: opt_action,
                    });
                }
                Stmt::GuiLabel { text } => {
                    let opt_text = self.optimize_expr(text);
                    optimized.push(Stmt::GuiLabel { text: opt_text });
                }
                Stmt::GuiCheckbox { label, initial_val } => {
                    let opt_label = self.optimize_expr(label);
                    let opt_val = initial_val.map(|v| self.optimize_expr(v));
                    optimized.push(Stmt::GuiCheckbox {
                        label: opt_label,
                        initial_val: opt_val,
                    });
                }
                Stmt::ImrvStmt {
                    element,
                    label,
                    extra,
                } => {
                    let opt_label = self.optimize_expr(label);
                    let opt_extra = extra.map(|e| self.optimize_expr(e));
                    optimized.push(Stmt::ImrvStmt {
                        element,
                        label: opt_label,
                        extra: opt_extra,
                    });
                }
                Stmt::Alert(msg) => {
                    let opt_msg = self.optimize_expr(msg);
                    optimized.push(Stmt::Alert(opt_msg));
                }
                Stmt::Beep => {
                    optimized.push(Stmt::Beep);
                }
                Stmt::Speak(msg) => {
                    let opt_msg = self.optimize_expr(msg);
                    optimized.push(Stmt::Speak(opt_msg));
                }
                Stmt::OpenWeb(url) => {
                    let opt_url = self.optimize_expr(url);
                    optimized.push(Stmt::OpenWeb(opt_url));
                }
                Stmt::DownloadWeb { url, target } => {
                    let opt_url = self.optimize_expr(url);
                    let opt_target = self.optimize_expr(target);
                    optimized.push(Stmt::DownloadWeb {
                        url: opt_url,
                        target: opt_target,
                    });
                }
                Stmt::Screen {
                    title,
                    width,
                    height,
                    body,
                } => {
                    let opt_title = self.optimize_expr(title);
                    let opt_w = width.map(|w| self.optimize_expr(w));
                    let opt_h = height.map(|h| self.optimize_expr(h));
                    let opt_body = self.optimize_statements(body);
                    optimized.push(Stmt::Screen {
                        title: opt_title,
                        width: opt_w,
                        height: opt_h,
                        body: opt_body,
                    });
                }
                Stmt::DrawCircle {
                    x,
                    y,
                    radius,
                    color,
                } => {
                    let opt_x = self.optimize_expr(x);
                    let opt_y = self.optimize_expr(y);
                    let opt_r = self.optimize_expr(radius);
                    let opt_c = color.map(|c| self.optimize_expr(c));
                    optimized.push(Stmt::DrawCircle {
                        x: opt_x,
                        y: opt_y,
                        radius: opt_r,
                        color: opt_c,
                    });
                }
                Stmt::DrawRect {
                    x,
                    y,
                    width,
                    height,
                    color,
                } => {
                    let opt_x = self.optimize_expr(x);
                    let opt_y = self.optimize_expr(y);
                    let opt_w = self.optimize_expr(width);
                    let opt_h = self.optimize_expr(height);
                    let opt_c = color.map(|c| self.optimize_expr(c));
                    optimized.push(Stmt::DrawRect {
                        x: opt_x,
                        y: opt_y,
                        width: opt_w,
                        height: opt_h,
                        color: opt_c,
                    });
                }
                Stmt::DrawLine {
                    x1,
                    y1,
                    x2,
                    y2,
                    color,
                } => {
                    let opt_x1 = self.optimize_expr(x1);
                    let opt_y1 = self.optimize_expr(y1);
                    let opt_x2 = self.optimize_expr(x2);
                    let opt_y2 = self.optimize_expr(y2);
                    let opt_c = color.map(|c| self.optimize_expr(c));
                    optimized.push(Stmt::DrawLine {
                        x1: opt_x1,
                        y1: opt_y1,
                        x2: opt_x2,
                        y2: opt_y2,
                        color: opt_c,
                    });
                }
                Stmt::DrawText { text, x, y, color } => {
                    let opt_text = self.optimize_expr(text);
                    let opt_x = self.optimize_expr(x);
                    let opt_y = self.optimize_expr(y);
                    let opt_c = color.map(|c| self.optimize_expr(c));
                    optimized.push(Stmt::DrawText {
                        text: opt_text,
                        x: opt_x,
                        y: opt_y,
                        color: opt_c,
                    });
                }
                Stmt::ClearScreen => {
                    optimized.push(Stmt::ClearScreen);
                }
                Stmt::CursorAt { x, y } => {
                    let opt_x = self.optimize_expr(x);
                    let opt_y = self.optimize_expr(y);
                    optimized.push(Stmt::CursorAt { x: opt_x, y: opt_y });
                }
                Stmt::CreateFolder(p) => {
                    let opt_p = self.optimize_expr(p);
                    optimized.push(Stmt::CreateFolder(opt_p));
                }
                Stmt::DeleteFile(p) => {
                    let opt_p = self.optimize_expr(p);
                    optimized.push(Stmt::DeleteFile(opt_p));
                }
                Stmt::DeleteFolder(p) => {
                    let opt_p = self.optimize_expr(p);
                    optimized.push(Stmt::DeleteFolder(opt_p));
                }
                Stmt::CopyFile { src, dest } => {
                    let opt_src = self.optimize_expr(src);
                    let opt_dest = self.optimize_expr(dest);
                    optimized.push(Stmt::CopyFile {
                        src: opt_src,
                        dest: opt_dest,
                    });
                }
                Stmt::AddToList { item, list } => {
                    let opt_item = self.optimize_expr(item);
                    optimized.push(Stmt::AddToList {
                        item: opt_item,
                        list,
                    });
                }
                Stmt::RemoveFromList { item, list } => {
                    let opt_item = self.optimize_expr(item);
                    optimized.push(Stmt::RemoveFromList {
                        item: opt_item,
                        list,
                    });
                }
                Stmt::ForEvery {
                    item_var,
                    list_var,
                    body,
                } => {
                    let opt_body = self.optimize_statements(body);
                    optimized.push(Stmt::ForEvery {
                        item_var,
                        list_var,
                        body: opt_body,
                    });
                }
                Stmt::MakeString { var_name, op } => {
                    optimized.push(Stmt::MakeString { var_name, op });
                }
                Stmt::MeasureTime { body } => {
                    let opt_body = self.optimize_statements(body);
                    optimized.push(Stmt::MeasureTime { body: opt_body });
                }
                Stmt::RunCommand { command } => {
                    let opt_cmd = self.optimize_expr(command);
                    optimized.push(Stmt::RunCommand { command: opt_cmd });
                }
                Stmt::ExprStmt(expr) => {
                    let opt_expr = self.optimize_expr(expr);
                    optimized.push(Stmt::ExprStmt(opt_expr));
                }
                Stmt::UseLib(lib) => {
                    optimized.push(Stmt::UseLib(lib));
                }
                Stmt::InlineAsm(code) => {
                    optimized.push(Stmt::InlineAsm(code));
                }
                Stmt::MeasureCycles { body } => {
                    let opt_body = self.optimize_statements(body);
                    optimized.push(Stmt::MeasureCycles { body: opt_body });
                }
                Stmt::DerefAssign {
                    ptr_expr,
                    value_expr,
                } => {
                    let opt_ptr = self.optimize_expr(ptr_expr);
                    let opt_val = self.optimize_expr(value_expr);
                    optimized.push(Stmt::DerefAssign {
                        ptr_expr: opt_ptr,
                        value_expr: opt_val,
                    });
                }
                Stmt::FieldAssign {
                    target,
                    field,
                    value,
                } => {
                    let opt_val = self.optimize_expr(value);
                    optimized.push(Stmt::FieldAssign {
                        target,
                        field,
                        value: opt_val,
                    });
                }
                Stmt::StructDef { name, fields } => {
                    optimized.push(Stmt::StructDef { name, fields });
                }
                Stmt::ExternBlock { abi, actions } => {
                    optimized.push(Stmt::ExternBlock { abi, actions });
                }
                Stmt::ThreadSpawn { body } => {
                    let opt_body = self.optimize_statements(body);
                    optimized.push(Stmt::ThreadSpawn { body: opt_body });
                }
                Stmt::AtomicAdd { var, val } => {
                    let opt_val = self.optimize_expr(val);
                    optimized.push(Stmt::AtomicAdd { var, val: opt_val });
                }
            }
        }

        optimized
    }

    pub fn optimize_expr(&mut self, expr: Expr) -> Expr {
        match expr {
            Expr::Binary { left, op, right } => {
                let opt_left = self.optimize_expr(*left);
                let opt_right = self.optimize_expr(*right);

                // Constant folding
                match (&opt_left, &op, &opt_right) {
                    (Expr::Int(a), BinaryOp::Add, Expr::Int(b)) => {
                        self.folded_constants_count += 1;
                        Expr::Int(a + b)
                    }
                    (Expr::Int(a), BinaryOp::Sub, Expr::Int(b)) => {
                        self.folded_constants_count += 1;
                        Expr::Int(a - b)
                    }
                    (Expr::Int(a), BinaryOp::Mul, Expr::Int(b)) => {
                        self.folded_constants_count += 1;
                        Expr::Int(a * b)
                    }
                    (Expr::Int(a), BinaryOp::Div, Expr::Int(b)) if *b != 0 => {
                        self.folded_constants_count += 1;
                        Expr::Int(a / b)
                    }
                    (Expr::Int(a), BinaryOp::Mod, Expr::Int(b)) if *b != 0 => {
                        self.folded_constants_count += 1;
                        Expr::Int(a % b)
                    }
                    (Expr::Int(a), BinaryOp::Equal, Expr::Int(b)) => {
                        self.folded_constants_count += 1;
                        Expr::Bool(a == b)
                    }
                    (Expr::Int(a), BinaryOp::NotEqual, Expr::Int(b)) => {
                        self.folded_constants_count += 1;
                        Expr::Bool(a != b)
                    }
                    (Expr::Int(a), BinaryOp::Less, Expr::Int(b)) => {
                        self.folded_constants_count += 1;
                        Expr::Bool(a < b)
                    }
                    (Expr::Int(a), BinaryOp::Greater, Expr::Int(b)) => {
                        self.folded_constants_count += 1;
                        Expr::Bool(a > b)
                    }
                    (Expr::Int(a), BinaryOp::LessEqual, Expr::Int(b)) => {
                        self.folded_constants_count += 1;
                        Expr::Bool(a <= b)
                    }
                    (Expr::Int(a), BinaryOp::GreaterEqual, Expr::Int(b)) => {
                        self.folded_constants_count += 1;
                        Expr::Bool(a >= b)
                    }
                    (Expr::Bool(a), BinaryOp::And, Expr::Bool(b)) => {
                        self.folded_constants_count += 1;
                        Expr::Bool(*a && *b)
                    }
                    (Expr::Bool(a), BinaryOp::Or, Expr::Bool(b)) => {
                        self.folded_constants_count += 1;
                        Expr::Bool(*a || *b)
                    }
                    (Expr::Str(a), BinaryOp::Add, Expr::Str(b)) => {
                        self.folded_constants_count += 1;
                        Expr::Str(format!("{}{}", a, b))
                    }
                    _ => Expr::Binary {
                        left: Box::new(opt_left),
                        op,
                        right: Box::new(opt_right),
                    },
                }
            }
            Expr::Unary { op, expr } => {
                let opt_inner = self.optimize_expr(*expr);
                match (&op, &opt_inner) {
                    (UnaryOp::Neg, Expr::Int(n)) => {
                        self.folded_constants_count += 1;
                        Expr::Int(-n)
                    }
                    (UnaryOp::Not, Expr::Bool(b)) => {
                        self.folded_constants_count += 1;
                        Expr::Bool(!b)
                    }
                    _ => Expr::Unary {
                        op,
                        expr: Box::new(opt_inner),
                    },
                }
            }
            Expr::InterpolatedString(parts) => {
                let mut opt_parts = Vec::new();
                for p in parts {
                    opt_parts.push(self.optimize_expr(p));
                }

                // Merge adjacent strings
                let mut merged_parts = Vec::new();
                let mut str_acc = String::new();

                for p in opt_parts {
                    if let Expr::Str(s) = p {
                        str_acc.push_str(&s);
                    } else {
                        if !str_acc.is_empty() {
                            merged_parts.push(Expr::Str(str_acc.clone()));
                            str_acc.clear();
                        }
                        merged_parts.push(p);
                    }
                }
                if !str_acc.is_empty() {
                    merged_parts.push(Expr::Str(str_acc));
                }

                if let [Expr::Str(s)] = merged_parts.as_slice() {
                    return Expr::Str(s.clone());
                }

                Expr::InterpolatedString(merged_parts)
            }
            Expr::Call { callee, args } => {
                let opt_args = args.into_iter().map(|a| self.optimize_expr(a)).collect();
                Expr::Call {
                    callee,
                    args: opt_args,
                }
            }
            Expr::Ask(prompt) => Expr::Ask(Box::new(self.optimize_expr(*prompt))),
            Expr::Run(cmd) => Expr::Run(Box::new(self.optimize_expr(*cmd))),
            Expr::ReadFile(path) => Expr::ReadFile(Box::new(self.optimize_expr(*path))),
            Expr::Random { min, max } => Expr::Random {
                min: Box::new(self.optimize_expr(*min)),
                max: Box::new(self.optimize_expr(*max)),
            },
            Expr::ImrvDraw {
                element,
                label,
                extra,
            } => Expr::ImrvDraw {
                element,
                label: Box::new(self.optimize_expr(*label)),
                extra: extra.map(|e| Box::new(self.optimize_expr(*e))),
            },
            other => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_folding() {
        let mut opt = Optimizer::new(true);

        // 100 * 2 + 50
        let expr = Expr::Binary {
            left: Box::new(Expr::Binary {
                left: Box::new(Expr::Int(100)),
                op: BinaryOp::Mul,
                right: Box::new(Expr::Int(2)),
            }),
            op: BinaryOp::Add,
            right: Box::new(Expr::Int(50)),
        };

        let result = opt.optimize_expr(expr);
        assert_eq!(result, Expr::Int(250));
        assert_eq!(opt.folded_constants_count, 2);
    }

    #[test]
    fn test_dead_code_elimination() {
        let mut opt = Optimizer::new(true);
        let stmts = vec![
            Stmt::Say {
                expr: Expr::Str("First".to_string()),
                newline: true,
                color: None,
            },
            Stmt::Give(None),
            Stmt::Say {
                expr: Expr::Str("Dead".to_string()),
                newline: true,
                color: None,
            },
        ];

        let result = opt.optimize_statements(stmts);
        assert_eq!(result.len(), 2); // Third statement eliminated!
        assert_eq!(opt.eliminated_stmts_count, 1);
    }
}
