use crate::ast::*;
use std::collections::HashMap;

pub struct CodeGenerator {
    directives: Directives,
    asm_rodata: Vec<String>,
    string_literals: HashMap<String, String>,
    label_counter: usize,
    var_offsets: HashMap<String, i32>,
    var_types: HashMap<String, Type>,
    current_stack_offset: i32,
    loop_stack: Vec<(String, String)>, // (continue_label, break_label)
    in_action: bool,
    pub has_gui: bool,
}

impl CodeGenerator {
    pub fn new(directives: Directives) -> Self {
        Self {
            directives,
            asm_rodata: Vec::new(),
            string_literals: HashMap::new(),
            label_counter: 0,
            var_offsets: HashMap::new(),
            var_types: HashMap::new(),
            current_stack_offset: 0,
            loop_stack: Vec::new(),
            in_action: false,
            has_gui: false,
        }
    }

    pub fn generate(mut self, program: &Program) -> (String, bool) {
        // Collect string constants
        let mut body_code = Vec::new();

        // Generate functions (actions) first
        let mut action_code = Vec::new();
        for stmt in &program.statements {
            if let Stmt::ActionDef { .. } = stmt {
                let code = self.generate_stmt(stmt);
                action_code.extend(code);
            }
        }

        // Generate main body statements
        for stmt in &program.statements {
            if !matches!(stmt, Stmt::ActionDef { .. }) {
                let code = self.generate_stmt(stmt);
                body_code.extend(code);
            }
        }

        let mut output = String::new();
        output.push_str("default rel\n\n");
        output.push_str("global main\n");
        output.push_str("extern rv_init\n");
        output.push_str("extern rv_alloc\n");
        output.push_str("extern rv_gc_collect\n");
        output.push_str("extern rv_say_str\n");
        output.push_str("extern rv_say_same_str\n");
        output.push_str("extern rv_say_int\n");
        output.push_str("extern rv_say_same_int\n");
        output.push_str("extern rv_say_bool\n");
        output.push_str("extern rv_say_same_bool\n");
        output.push_str("extern rv_int_to_str\n");
        output.push_str("extern rv_bool_to_str\n");
        output.push_str("extern rv_str_concat\n");
        output.push_str("extern rv_str_eq\n");
        output.push_str("extern rv_ask\n");
        output.push_str("extern rv_run_cmd\n");
        output.push_str("extern rv_read_file\n");
        output.push_str("extern rv_write_file\n");
        output.push_str("extern rv_wait_sec\n");
        output.push_str("extern rv_random\n");
        output.push_str("extern rv_gui_init\n");
        output.push_str("extern rv_gui_add_label\n");
        output.push_str("extern rv_gui_add_button\n");
        output.push_str("extern rv_gui_add_checkbox\n");
        output.push_str("extern rv_gui_loop\n");
        output.push_str("extern rv_imrv_draw_checkbox\n");
        output.push_str("extern rv_imrv_draw_button\n");
        output.push_str("extern rv_imrv_draw_text\n");
        output.push_str("extern rv_exit\n\n");

        // .rodata section
        output.push_str("section .rodata\n");
        for line in &self.asm_rodata {
            output.push_str(line);
            output.push('\n');
        }
        output.push('\n');

        // .text section
        output.push_str("section .text\n");

        // Action functions
        for line in &action_code {
            output.push_str(line);
            output.push('\n');
        }

        // main function
        output.push_str("main:\n");
        output.push_str("    push rbp\n");
        output.push_str("    mov rbp, rsp\n");

        // Allocate stack space (aligned to 16 bytes)
        let stack_size = ((self.current_stack_offset.abs() + 31) / 16) * 16;
        output.push_str(&format!("    sub rsp, {}\n", stack_size));

        // Initialize runtime: rdi = 0 if Zero-GC / Advanced, 1 if GC enabled
        let has_gc = if self.directives.remove_gc || self.directives.add_advanced {
            0
        } else {
            1
        };
        output.push_str(&format!("    mov rdi, {}\n", has_gc));
        output.push_str("    call rv_init\n");

        // Main statements
        for line in &body_code {
            output.push_str(line);
            output.push('\n');
        }

        output.push_str("    xor eax, eax\n");
        output.push_str("    leave\n");
        output.push_str("    ret\n");

        let final_asm = if self.directives.add_advanced {
            self.peephole_optimize(&output)
        } else {
            output
        };

        (final_asm, self.has_gui)
    }

    fn peephole_optimize(&self, code: &str) -> String {
        let lines: Vec<&str> = code.lines().collect();
        let mut opt_lines: Vec<String> = Vec::new();

        let mut i = 0;
        while i < lines.len() {
            let line = lines[i].trim();

            // Check two-line peephole: push rax; pop rbx -> mov rbx, rax
            if i + 1 < lines.len() {
                let next_line = lines[i + 1].trim();
                if line == "push rax" && next_line == "pop rbx" {
                    opt_lines.push("    mov rbx, rax".to_string());
                    i += 2;
                    continue;
                }
                if line == "push rax" && next_line == "pop rax" {
                    // redundant push/pop rax
                    i += 2;
                    continue;
                }
            }

            // Single line peepholes
            if line == "mov rax, 0" {
                opt_lines.push("    xor eax, eax".to_string());
            } else if line == "mov rbx, 0" {
                opt_lines.push("    xor ebx, ebx".to_string());
            } else if line == "add rax, 1" {
                opt_lines.push("    inc rax".to_string());
            } else if line == "sub rax, 1" {
                opt_lines.push("    dec rax".to_string());
            } else if line == "add rbx, 1" {
                opt_lines.push("    inc rbx".to_string());
            } else if line == "sub rbx, 1" {
                opt_lines.push("    dec rbx".to_string());
            } else {
                opt_lines.push(lines[i].to_string());
            }

            i += 1;
        }

        opt_lines.join("\n") + "\n"
    }

    fn new_label(&mut self, prefix: &str) -> String {
        self.label_counter += 1;
        format!(".L_{}_{}", prefix, self.label_counter)
    }

    fn get_or_create_string(&mut self, s: &str) -> String {
        if let Some(lbl) = self.string_literals.get(s) {
            return lbl.clone();
        }

        let lbl = format!("_rv_str_{}", self.string_literals.len());
        let len = s.as_bytes().len();

        let mut escaped_bytes = Vec::new();
        for b in s.as_bytes() {
            escaped_bytes.push(format!("{}", b));
        }
        escaped_bytes.push("0".to_string()); // null terminator

        let rodata_line = format!(
            "{}:\n    dq {}\n    db {}",
            lbl,
            len,
            escaped_bytes.join(", ")
        );

        self.asm_rodata.push(rodata_line);
        self.string_literals.insert(s.to_string(), lbl.clone());
        lbl
    }

    fn allocate_variable(&mut self, name: &str, ty: Type) -> i32 {
        if let Some(&offset) = self.var_offsets.get(name) {
            self.var_types.insert(name.to_string(), ty);
            offset
        } else {
            self.current_stack_offset -= 8;
            let offset = self.current_stack_offset;
            self.var_offsets.insert(name.to_string(), offset);
            self.var_types.insert(name.to_string(), ty);
            offset
        }
    }

    fn infer_expr_type(&self, expr: &Expr) -> Type {
        match expr {
            Expr::Int(_) => Type::Int,
            Expr::Str(_) | Expr::InterpolatedString(_) | Expr::Ask(_) => Type::String,
            Expr::Bool(_) => Type::Bool,
            Expr::Var(name) => self
                .var_types
                .get(name)
                .cloned()
                .unwrap_or(Type::Int),
            Expr::Binary { left, op, .. } => match op {
                BinaryOp::Equal
                | BinaryOp::NotEqual
                | BinaryOp::Less
                | BinaryOp::Greater
                | BinaryOp::LessEqual
                | BinaryOp::GreaterEqual
                | BinaryOp::And
                | BinaryOp::Or => Type::Bool,
                BinaryOp::Add => {
                    let left_ty = self.infer_expr_type(left);
                    if left_ty == Type::String {
                        Type::String
                    } else {
                        Type::Int
                    }
                }
                _ => Type::Int,
            },
            Expr::Unary { op, .. } => match op {
                UnaryOp::Not => Type::Bool,
                UnaryOp::Neg => Type::Int,
            },
            Expr::Call { .. } => Type::Int,
            Expr::Run(_) => Type::String,
            Expr::ReadFile(_) => Type::String,
            Expr::Random { .. } => Type::Int,
            Expr::ImrvDraw { element, .. } => {
                if element == "checkbox" || element == "button" {
                    Type::Bool
                } else {
                    Type::Void
                }
            }
        }
    }

    fn generate_stmt(&mut self, stmt: &Stmt) -> Vec<String> {
        let mut code = Vec::new();

        match stmt {
            Stmt::Say { expr, newline } => {
                let ty = self.infer_expr_type(expr);
                code.extend(self.generate_expr(expr));
                code.push("    mov rdi, rax".to_string());

                match ty {
                    Type::String => {
                        if *newline {
                            code.push("    call rv_say_str".to_string());
                        } else {
                            code.push("    call rv_say_same_str".to_string());
                        }
                    }
                    Type::Bool => {
                        if *newline {
                            code.push("    call rv_say_bool".to_string());
                        } else {
                            code.push("    call rv_say_same_bool".to_string());
                        }
                    }
                    _ => {
                        if *newline {
                            code.push("    call rv_say_int".to_string());
                        } else {
                            code.push("    call rv_say_same_int".to_string());
                        }
                    }
                }
            }
            Stmt::Remember {
                name,
                explicit_type,
                value,
            } => {
                let ty = explicit_type
                    .clone()
                    .unwrap_or_else(|| self.infer_expr_type(value));
                let offset = self.allocate_variable(name, ty);
                code.extend(self.generate_expr(value));
                code.push(format!("    mov [rbp + ({})], rax", offset));
            }
            Stmt::Assign { name, value } => {
                let offset = *self.var_offsets.get(name).unwrap_or(&-8);
                code.extend(self.generate_expr(value));
                code.push(format!("    mov [rbp + ({})], rax", offset));
            }
            Stmt::If {
                condition,
                then_branch,
                otherwise_ifs,
                otherwise_branch,
            } => {
                let end_label = self.new_label("if_end");
                let mut next_label = self.new_label("if_next");

                // Evaluate if condition
                code.extend(self.generate_expr(condition));
                code.push("    cmp rax, 0".to_string());
                code.push(format!("    je {}", next_label));

                // Then branch
                for s in then_branch {
                    code.extend(self.generate_stmt(s));
                }
                code.push(format!("    jmp {}", end_label));

                // Otherwise ifs
                for (elif_c, elif_b) in otherwise_ifs {
                    code.push(format!("{}:", next_label));
                    next_label = self.new_label("if_next");
                    code.extend(self.generate_expr(elif_c));
                    code.push("    cmp rax, 0".to_string());
                    code.push(format!("    je {}", next_label));
                    for s in elif_b {
                        code.extend(self.generate_stmt(s));
                    }
                    code.push(format!("    jmp {}", end_label));
                }

                // Otherwise branch
                code.push(format!("{}:", next_label));
                if let Some(other_b) = otherwise_branch {
                    for s in other_b {
                        code.extend(self.generate_stmt(s));
                    }
                }

                code.push(format!("{}:", end_label));
            }
            Stmt::Repeat {
                count,
                var_name,
                body,
            } => {
                let loop_start = self.new_label("repeat_start");
                let loop_step = self.new_label("repeat_step");
                let loop_end = self.new_label("repeat_end");

                // Allocate slot for current counter and limit
                let counter_offset = self.allocate_variable(&format!("__rep_cnt_{}", loop_start), Type::Int);
                let limit_offset = self.allocate_variable(&format!("__rep_lim_{}", loop_start), Type::Int);

                // If user named the loop variable, bind it to counter_offset
                let user_var_offset = var_name.as_ref().map(|v| self.allocate_variable(v, Type::Int));

                // Evaluate count and store in limit
                code.extend(self.generate_expr(count));
                code.push(format!("    mov [rbp + ({})], rax", limit_offset));

                // Initialize counter = 1
                code.push("    mov rax, 1".to_string());
                code.push(format!("    mov [rbp + ({})], rax", counter_offset));

                code.push(format!("{}:", loop_start));
                // Compare counter with limit: if counter > limit, break
                code.push(format!("    mov rax, [rbp + ({})]", counter_offset));
                code.push(format!("    cmp rax, [rbp + ({})]", limit_offset));
                code.push(format!("    jg {}", loop_end));

                // If user variable exists, update it with counter
                if let Some(u_off) = user_var_offset {
                    code.push(format!("    mov rax, [rbp + ({})]", counter_offset));
                    code.push(format!("    mov [rbp + ({})], rax", u_off));
                }

                self.loop_stack.push((loop_step.clone(), loop_end.clone()));
                for s in body {
                    code.extend(self.generate_stmt(s));
                }
                self.loop_stack.pop();

                // Increment step
                code.push(format!("{}:", loop_step));
                code.push(format!("    mov rax, [rbp + ({})]", counter_offset));
                code.push("    add rax, 1".to_string());
                code.push(format!("    mov [rbp + ({})], rax", counter_offset));
                code.push(format!("    jmp {}", loop_start));

                code.push(format!("{}:", loop_end));
            }
            Stmt::While { condition, body } => {
                let loop_start = self.new_label("while_start");
                let loop_end = self.new_label("while_end");

                code.push(format!("{}:", loop_start));
                code.extend(self.generate_expr(condition));
                code.push("    cmp rax, 0".to_string());
                code.push(format!("    je {}", loop_end));

                self.loop_stack.push((loop_start.clone(), loop_end.clone()));
                for s in body {
                    code.extend(self.generate_stmt(s));
                }
                self.loop_stack.pop();

                code.push(format!("    jmp {}", loop_start));
                code.push(format!("{}:", loop_end));
            }
            Stmt::Stop => {
                if let Some((_, break_lbl)) = self.loop_stack.last() {
                    code.push(format!("    jmp {}", break_lbl));
                }
            }
            Stmt::Skip => {
                if let Some((cont_lbl, _)) = self.loop_stack.last() {
                    code.push(format!("    jmp {}", cont_lbl));
                }
            }
            Stmt::ActionDef {
                name,
                params,
                body,
                ..
            } => {
                let func_label = format!("rv_action_{}", name);
                code.push(format!("{}:", func_label));
                code.push("    push rbp".to_string());
                code.push("    mov rbp, rsp".to_string());

                let prev_in_action = self.in_action;
                self.in_action = true;

                // Save parameters from registers (rdi, rsi, rdx, rcx, r8, r9) to stack
                let arg_regs = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
                let mut param_code = Vec::new();
                for (idx, (p_name, p_ty)) in params.iter().enumerate() {
                    if idx < arg_regs.len() {
                        let ty = p_ty.clone().unwrap_or(Type::Int);
                        let offset = self.allocate_variable(p_name, ty);
                        param_code.push(format!("    mov [rbp + ({})], {}", offset, arg_regs[idx]));
                    }
                }

                let mut body_lines = Vec::new();
                for s in body {
                    body_lines.extend(self.generate_stmt(s));
                }

                let local_stack = ((self.current_stack_offset.abs() + 31) / 16) * 16;
                code.push(format!("    sub rsp, {}", local_stack));
                code.extend(param_code);
                code.extend(body_lines);

                code.push("    leave".to_string());
                code.push("    ret".to_string());

                self.in_action = prev_in_action;
            }
            Stmt::Give(expr_opt) => {
                if let Some(e) = expr_opt {
                    code.extend(self.generate_expr(e));
                } else {
                    code.push("    xor eax, eax".to_string());
                }
                code.push("    leave".to_string());
                code.push("    ret".to_string());
            }
            Stmt::RunCommand { command } => {
                code.extend(self.generate_expr(command));
                code.push("    mov rdi, rax".to_string());
                code.push("    call rv_run_cmd".to_string());
            }
            Stmt::Use(_) => {
                // Modules and includes handled or linked
            }
            Stmt::WriteFile { data, target } => {
                code.extend(self.generate_expr(data));
                code.push("    push rax".to_string());
                code.extend(self.generate_expr(target));
                code.push("    mov rsi, rax".to_string());
                code.push("    pop rdi".to_string());
                code.push("    call rv_write_file".to_string());
            }
            Stmt::Wait(sec) => {
                code.extend(self.generate_expr(sec));
                code.push("    mov rdi, rax".to_string());
                code.push("    call rv_wait_sec".to_string());
            }
            Stmt::Window {
                title,
                width,
                height,
                body,
            } => {
                self.has_gui = true;
                code.extend(self.generate_expr(title));
                code.push("    push rax".to_string());
                if let Some(w) = width {
                    code.extend(self.generate_expr(w));
                } else {
                    code.push("    mov rax, 500".to_string());
                }
                code.push("    push rax".to_string());
                if let Some(h) = height {
                    code.extend(self.generate_expr(h));
                } else {
                    code.push("    mov rax, 400".to_string());
                }
                code.push("    mov rdx, rax".to_string());
                code.push("    pop rsi".to_string());
                code.push("    pop rdi".to_string());
                code.push("    call rv_gui_init".to_string());

                for s in body {
                    code.extend(self.generate_stmt(s));
                }
                code.push("    call rv_gui_loop".to_string());
            }
            Stmt::GuiButton { label, action } => {
                self.has_gui = true;
                let btn_cb_lbl = self.new_label("btn_cb");
                let skip_cb_lbl = self.new_label("skip_btn_cb");
                code.push(format!("    jmp {}", skip_cb_lbl));
                code.push(format!("{}:", btn_cb_lbl));
                code.push("    push rbp".to_string());
                code.push("    mov rbp, rsp".to_string());
                for s in action {
                    code.extend(self.generate_stmt(s));
                }
                code.push("    leave".to_string());
                code.push("    ret".to_string());
                code.push(format!("{}:", skip_cb_lbl));

                code.extend(self.generate_expr(label));
                code.push("    mov rdi, rax".to_string());
                code.push(format!("    lea rsi, [{}]", btn_cb_lbl));
                code.push("    call rv_gui_add_button".to_string());
            }
            Stmt::GuiLabel { text } => {
                self.has_gui = true;
                code.extend(self.generate_expr(text));
                code.push("    mov rdi, rax".to_string());
                code.push("    call rv_gui_add_label".to_string());
            }
            Stmt::GuiCheckbox { label, initial_val } => {
                self.has_gui = true;
                code.extend(self.generate_expr(label));
                code.push("    push rax".to_string());
                if let Some(init) = initial_val {
                    code.extend(self.generate_expr(init));
                } else {
                    code.push("    mov rax, 0".to_string());
                }
                code.push("    mov rsi, rax".to_string());
                code.push("    pop rdi".to_string());
                code.push("    call rv_gui_add_checkbox".to_string());
            }
            Stmt::ImrvStmt {
                element,
                label,
                extra,
            } => {
                self.has_gui = true;
                if element == "checkbox" {
                    code.extend(self.generate_expr(label));
                    code.push("    push rax".to_string());
                    if let Some(e) = extra {
                        code.extend(self.generate_expr(e));
                    } else {
                        code.push("    mov rax, 0".to_string());
                    }
                    code.push("    mov rsi, rax".to_string());
                    code.push("    pop rdi".to_string());
                    code.push("    call rv_imrv_draw_checkbox".to_string());
                } else if element == "button" {
                    code.extend(self.generate_expr(label));
                    code.push("    mov rdi, rax".to_string());
                    code.push("    call rv_imrv_draw_button".to_string());
                } else {
                    code.extend(self.generate_expr(label));
                    code.push("    mov rdi, rax".to_string());
                    code.push("    call rv_imrv_draw_text".to_string());
                }
            }
            Stmt::ExprStmt(expr) => {
                code.extend(self.generate_expr(expr));
            }
        }

        code
    }

    fn generate_expr(&mut self, expr: &Expr) -> Vec<String> {
        let mut code = Vec::new();

        match expr {
            Expr::Int(val) => {
                code.push(format!("    mov rax, {}", val));
            }
            Expr::Bool(val) => {
                let b = if *val { 1 } else { 0 };
                code.push(format!("    mov rax, {}", b));
            }
            Expr::Str(val) => {
                let lbl = self.get_or_create_string(val);
                code.push(format!("    lea rax, [{}]", lbl));
            }
            Expr::Var(name) => {
                if let Some(&offset) = self.var_offsets.get(name) {
                    code.push(format!("    mov rax, [rbp + ({})]", offset));
                } else {
                    code.push("    xor eax, eax".to_string());
                }
            }
            Expr::Binary { left, op, right } => {
                code.extend(self.generate_expr(left));
                code.push("    push rax".to_string());
                code.extend(self.generate_expr(right));
                code.push("    pop rbx".to_string()); // rbx = left, rax = right

                match op {
                    BinaryOp::Add => {
                        let left_ty = self.infer_expr_type(left);
                        if left_ty == Type::String {
                            // String concatenation: rv_str_concat(rdi=left, rsi=right)
                            code.push("    mov rdi, rbx".to_string());
                            code.push("    mov rsi, rax".to_string());
                            code.push("    call rv_str_concat".to_string());
                        } else {
                            code.push("    add rbx, rax".to_string());
                            code.push("    mov rax, rbx".to_string());
                        }
                    }
                    BinaryOp::Sub => {
                        code.push("    sub rbx, rax".to_string());
                        code.push("    mov rax, rbx".to_string());
                    }
                    BinaryOp::Mul => {
                        code.push("    imul rbx, rax".to_string());
                        code.push("    mov rax, rbx".to_string());
                    }
                    BinaryOp::Div => {
                        code.push("    mov rcx, rax".to_string());
                        code.push("    mov rax, rbx".to_string());
                        code.push("    cqo".to_string());
                        code.push("    idiv rcx".to_string());
                    }
                    BinaryOp::Mod => {
                        code.push("    mov rcx, rax".to_string());
                        code.push("    mov rax, rbx".to_string());
                        code.push("    cqo".to_string());
                        code.push("    idiv rcx".to_string());
                        code.push("    mov rax, rdx".to_string());
                    }
                    BinaryOp::Equal => {
                        let left_ty = self.infer_expr_type(left);
                        if left_ty == Type::String {
                            code.push("    mov rdi, rbx".to_string());
                            code.push("    mov rsi, rax".to_string());
                            code.push("    call rv_str_eq".to_string());
                        } else {
                            code.push("    cmp rbx, rax".to_string());
                            code.push("    sete al".to_string());
                            code.push("    movzx rax, al".to_string());
                        }
                    }
                    BinaryOp::NotEqual => {
                        let left_ty = self.infer_expr_type(left);
                        if left_ty == Type::String {
                            code.push("    mov rdi, rbx".to_string());
                            code.push("    mov rsi, rax".to_string());
                            code.push("    call rv_str_eq".to_string());
                            code.push("    xor rax, 1".to_string());
                        } else {
                            code.push("    cmp rbx, rax".to_string());
                            code.push("    setne al".to_string());
                            code.push("    movzx rax, al".to_string());
                        }
                    }
                    BinaryOp::Less => {
                        code.push("    cmp rbx, rax".to_string());
                        code.push("    setl al".to_string());
                        code.push("    movzx rax, al".to_string());
                    }
                    BinaryOp::Greater => {
                        code.push("    cmp rbx, rax".to_string());
                        code.push("    setg al".to_string());
                        code.push("    movzx rax, al".to_string());
                    }
                    BinaryOp::LessEqual => {
                        code.push("    cmp rbx, rax".to_string());
                        code.push("    setle al".to_string());
                        code.push("    movzx rax, al".to_string());
                    }
                    BinaryOp::GreaterEqual => {
                        code.push("    cmp rbx, rax".to_string());
                        code.push("    setge al".to_string());
                        code.push("    movzx rax, al".to_string());
                    }
                    BinaryOp::And => {
                        code.push("    test rbx, rbx".to_string());
                        code.push("    setnz bl".to_string());
                        code.push("    test rax, rax".to_string());
                        code.push("    setnz al".to_string());
                        code.push("    and al, bl".to_string());
                        code.push("    movzx rax, al".to_string());
                    }
                    BinaryOp::Or => {
                        code.push("    or rbx, rax".to_string());
                        code.push("    setnz al".to_string());
                        code.push("    movzx rax, al".to_string());
                    }
                }
            }
            Expr::Unary { op, expr } => {
                code.extend(self.generate_expr(expr));
                match op {
                    UnaryOp::Neg => {
                        code.push("    neg rax".to_string());
                    }
                    UnaryOp::Not => {
                        code.push("    test rax, rax".to_string());
                        code.push("    setz al".to_string());
                        code.push("    movzx rax, al".to_string());
                    }
                }
            }
            Expr::Call { callee, args } => {
                let arg_regs = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
                // Evaluate args in reverse and push, then pop into arg registers
                for a in args.iter().rev() {
                    code.extend(self.generate_expr(a));
                    code.push("    push rax".to_string());
                }
                for idx in 0..args.len() {
                    if idx < arg_regs.len() {
                        code.push(format!("    pop {}", arg_regs[idx]));
                    }
                }
                code.push(format!("    call rv_action_{}", callee));
            }
            Expr::InterpolatedString(parts) => {
                if parts.is_empty() {
                    let lbl = self.get_or_create_string("");
                    code.push(format!("    lea rax, [{}]", lbl));
                    return code;
                }

                // Generate first part
                code.extend(self.generate_stringified_expr(&parts[0]));

                // Concat subsequent parts
                for p in parts.iter().skip(1) {
                    code.push("    push rax".to_string());
                    code.extend(self.generate_stringified_expr(p));
                    code.push("    mov rsi, rax".to_string()); // new part
                    code.push("    pop rdi".to_string());      // accumulated
                    code.push("    call rv_str_concat".to_string());
                }
            }
            Expr::Ask(prompt) => {
                code.extend(self.generate_expr(prompt));
                code.push("    mov rdi, rax".to_string());
                code.push("    call rv_ask".to_string());
            }
            Expr::Run(cmd) => {
                code.extend(self.generate_expr(cmd));
                code.push("    mov rdi, rax".to_string());
                code.push("    call rv_run_cmd".to_string());
            }
            Expr::ReadFile(path) => {
                code.extend(self.generate_expr(path));
                code.push("    mov rdi, rax".to_string());
                code.push("    call rv_read_file".to_string());
            }
            Expr::Random { min, max } => {
                code.extend(self.generate_expr(min));
                code.push("    push rax".to_string());
                code.extend(self.generate_expr(max));
                code.push("    mov rsi, rax".to_string());
                code.push("    pop rdi".to_string());
                code.push("    call rv_random".to_string());
            }
            Expr::ImrvDraw {
                element,
                label,
                extra,
            } => {
                self.has_gui = true;
                if element == "checkbox" {
                    code.extend(self.generate_expr(label));
                    code.push("    push rax".to_string());
                    if let Some(e) = extra {
                        code.extend(self.generate_expr(e));
                    } else {
                        code.push("    mov rax, 0".to_string());
                    }
                    code.push("    mov rsi, rax".to_string());
                    code.push("    pop rdi".to_string());
                    code.push("    call rv_imrv_draw_checkbox".to_string());
                } else if element == "button" {
                    code.extend(self.generate_expr(label));
                    code.push("    mov rdi, rax".to_string());
                    code.push("    call rv_imrv_draw_button".to_string());
                } else {
                    code.extend(self.generate_expr(label));
                    code.push("    mov rdi, rax".to_string());
                    code.push("    call rv_imrv_draw_text".to_string());
                    code.push("    xor eax, eax".to_string());
                }
            }
        }

        code
    }

    fn generate_stringified_expr(&mut self, expr: &Expr) -> Vec<String> {
        let mut code = Vec::new();
        let ty = self.infer_expr_type(expr);

        code.extend(self.generate_expr(expr));

        match ty {
            Type::String => {
                // Already string pointer in rax
            }
            Type::Bool => {
                code.push("    mov rdi, rax".to_string());
                code.push("    call rv_bool_to_str".to_string());
            }
            _ => {
                // Integer to string
                code.push("    mov rdi, rax".to_string());
                code.push("    call rv_int_to_str".to_string());
            }
        }

        code
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_codegen_basic() {
        let code = r#"
            remember x = 42
            say x
        "#;
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse().unwrap();

        let codegen = CodeGenerator::new(prog.directives.clone());
        let (asm, _) = codegen.generate(&prog);

        assert!(asm.contains("main:"));
        assert!(asm.contains("rv_say_int"));
    }

    #[test]
    fn test_codegen_advanced_peephole() {
        let code = r#"
            remove garbageC
            remove Basic
            add Advanced
            use ior

            remember x: Int = 0
            say x
        "#;
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse().unwrap();

        let codegen = CodeGenerator::new(prog.directives.clone());
        let (asm, _) = codegen.generate(&prog);

        assert!(asm.contains("xor eax, eax"));
    }
}
