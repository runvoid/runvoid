use crate::codegen::CodeGenerator;
use crate::lexer::Lexer;
use crate::optimizer::Optimizer;
use crate::parser::Parser;
use crate::typechecker::TypeChecker;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

const RUNTIME_ASM: &str = include_str!("../runtime/runtime.asm");
const RUNTIME_GUI_C: &str = include_str!("../runtime/gui.c");

pub struct CompilerOptions {
    pub output_path: Option<PathBuf>,
    pub run_after_build: bool,
    pub emit_asm_only: bool,
    pub verbose: bool,
}

pub struct Compiler;

impl Compiler {
    pub fn compile_source(source: &str, options: &CompilerOptions) -> Result<Option<i32>, String> {
        // 1. Lexing
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;

        // 2. Parsing
        let mut parser = Parser::new(tokens);
        let program = parser.parse()?;

        // 3. Semantic & Type Checking
        let mut typechecker = TypeChecker::new(program.directives.clone());
        typechecker.check(&program)?;

        // 4. Optimization (Constant folding & dead code elimination)
        let is_advanced = program.directives.add_advanced;
        let mut optimizer = Optimizer::new(is_advanced);
        let opt_program = optimizer.optimize_program(program);

        if options.verbose && is_advanced {
            println!(
                "⚡ [Pro Dev Optimization] Folded constants: {}, eliminated dead statements: {}",
                optimizer.folded_constants_count, optimizer.eliminated_stmts_count
            );
        }

        // 5. Code Generation
        let codegen = CodeGenerator::new(opt_program.directives.clone());
        let (generated_asm, has_gui, has_helpers) = codegen.generate(&opt_program);

        if options.emit_asm_only {
            if let Some(out_p) = &options.output_path {
                fs::write(out_p, &generated_asm)
                    .map_err(|e| format!("Failed to write assembly output to file: {}", e))?;
                println!("NASM assembly saved to {:?}", out_p);
            } else {
                println!("{}", generated_asm);
            }
            return Ok(None);
        }

        // 6. Assembling & Linking
        let count = TEMP_COUNTER.fetch_add(1, Ordering::SeqCst);
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let temp_dir = std::env::temp_dir().join(format!(
            "runvoid_{}_{}_{}",
            std::process::id(),
            nanos,
            count
        ));
        fs::create_dir_all(&temp_dir)
            .map_err(|e| format!("Failed to create temporary directory: {}", e))?;

        let runtime_asm_path = temp_dir.join("runtime.asm");
        let runtime_obj_path = temp_dir.join("runtime.o");
        let user_asm_path = temp_dir.join("user.asm");
        let user_obj_path = temp_dir.join("user.o");
        let gui_c_path = temp_dir.join("gui.c");
        let gui_obj_path = temp_dir.join("gui.o");

        let final_bin_path = if let Some(p) = &options.output_path {
            p.clone()
        } else {
            temp_dir.join("runvoid_bin")
        };

        let is_freestanding =
            opt_program.directives.remove_linux && opt_program.directives.add_freestanding;
        if is_freestanding {
            fs::write(&user_asm_path, &generated_asm)
                .map_err(|e| format!("Failed to write user.asm: {}", e))?;

            let nasm_user = Command::new("nasm")
                .args(["-f", "elf64", "-O3"])
                .arg(&user_asm_path)
                .args(["-o"])
                .arg(&user_obj_path)
                .output()
                .map_err(|e| format!("Failed to execute nasm: {}", e))?;

            if !nasm_user.status.success() {
                return Err(format!(
                    "Assembly error in freestanding user code:\n{}",
                    String::from_utf8_lossy(&nasm_user.stderr)
                ));
            }

            let ld_output = Command::new("ld")
                .args(["-s"])
                .arg(&user_obj_path)
                .arg("-o")
                .arg(&final_bin_path)
                .output()
                .map_err(|e| format!("Failed to link freestanding binary with ld: {}", e))?;

            if !ld_output.status.success() {
                return Err(format!(
                    "Linker error (ld):\n{}",
                    String::from_utf8_lossy(&ld_output.stderr)
                ));
            }

            if options.run_after_build {
                let status = Command::new(&final_bin_path)
                    .status()
                    .map_err(|e| format!("Failed to execute compiled binary: {}", e))?;
                let _ = fs::remove_dir_all(&temp_dir);
                return Ok(status.code());
            }

            println!(
                "Successfully compiled freestanding binary to {:?}",
                final_bin_path
            );
            let _ = fs::remove_file(&user_asm_path);
            let _ = fs::remove_file(&user_obj_path);
            return Ok(Some(0));
        }

        fs::write(&runtime_asm_path, RUNTIME_ASM)
            .map_err(|e| format!("Failed to write runtime.asm: {}", e))?;
        fs::write(&user_asm_path, &generated_asm)
            .map_err(|e| format!("Failed to write user.asm: {}", e))?;

        // Run nasm for runtime
        let nasm_opt = if is_advanced { "-O3" } else { "-O1" };

        let nasm_rt = Command::new("nasm")
            .args(["-f", "elf64", nasm_opt])
            .arg(&runtime_asm_path)
            .args(["-o"])
            .arg(&runtime_obj_path)
            .output()
            .map_err(|e| format!("Failed to execute nasm (ensure nasm is installed): {}", e))?;

        if !nasm_rt.status.success() {
            return Err(format!(
                "Assembly error in runtime.asm:\n{}",
                String::from_utf8_lossy(&nasm_rt.stderr)
            ));
        }

        // Run nasm for user code
        let nasm_user = Command::new("nasm")
            .args(["-f", "elf64", nasm_opt])
            .arg(&user_asm_path)
            .args(["-o"])
            .arg(&user_obj_path)
            .output()
            .map_err(|e| format!("Failed to execute nasm: {}", e))?;

        if !nasm_user.status.success() {
            return Err(format!(
                "Assembly error in user code:\n{}\nGenerated ASM was:\n{}",
                String::from_utf8_lossy(&nasm_user.stderr),
                generated_asm
            ));
        }

        // If GUI or helpers are used, compile gui.c
        let need_gui_c = has_gui || has_helpers;
        if need_gui_c {
            fs::write(&gui_c_path, RUNTIME_GUI_C)
                .map_err(|e| format!("Failed to write gui.c: {}", e))?;

            let mut gcc_gui_cmd = Command::new("gcc");
            gcc_gui_cmd.args(["-c", "-O2"]);
            if !has_gui {
                gcc_gui_cmd.arg("-DNO_GUI");
            }
            gcc_gui_cmd.arg(&gui_c_path).args(["-o"]).arg(&gui_obj_path);

            let gcc_gui = gcc_gui_cmd.output().map_err(|e| {
                format!(
                    "Failed to compile GUI/helper runtime module with gcc: {}",
                    e
                )
            })?;

            if !gcc_gui.status.success() {
                return Err(format!(
                    "Compilation error in gui.c:\n{}",
                    String::from_utf8_lossy(&gcc_gui.stderr)
                ));
            }
        }

        // Link with gcc
        let mut gcc_cmd = Command::new("gcc");
        gcc_cmd.arg("-no-pie");

        if is_advanced {
            gcc_cmd.args(["-O3", "-flto", "-s", "-Wl,--gc-sections"]);
        }

        gcc_cmd.arg(&user_obj_path).arg(&runtime_obj_path);

        if need_gui_c {
            gcc_cmd.arg(&gui_obj_path);
            if has_gui {
                gcc_cmd.arg("-lX11");
            }
            gcc_cmd.arg("-lm");
            gcc_cmd.arg("-lpthread");
        }

        for lib in &opt_program.directives.libs {
            gcc_cmd.arg(format!("-l{}", lib));
        }

        gcc_cmd.arg("-o").arg(&final_bin_path);

        let gcc_output = gcc_cmd
            .output()
            .map_err(|e| format!("Failed to link with gcc: {}", e))?;

        if !gcc_output.status.success() {
            return Err(format!(
                "Linker error (gcc):\n{}",
                String::from_utf8_lossy(&gcc_output.stderr)
            ));
        }

        // If run after build:
        if options.run_after_build {
            let status = Command::new(&final_bin_path)
                .status()
                .map_err(|e| format!("Failed to execute compiled binary: {}", e))?;

            // Cleanup temp
            let _ = fs::remove_dir_all(&temp_dir);
            return Ok(status.code());
        }

        println!("Successfully compiled to {:?}", final_bin_path);
        let _ = fs::remove_file(&runtime_asm_path);
        let _ = fs::remove_file(&runtime_obj_path);
        let _ = fs::remove_file(&user_asm_path);
        let _ = fs::remove_file(&user_obj_path);
        if need_gui_c {
            let _ = fs::remove_file(&gui_c_path);
            let _ = fs::remove_file(&gui_obj_path);
        }

        Ok(Some(0))
    }

    pub fn compile_file(
        file_path: &Path,
        options: &CompilerOptions,
    ) -> Result<Option<i32>, String> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file {:?}: {}", file_path, e))?;
        Self::compile_source(&content, options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_code(code: &str) -> Result<Option<i32>, String> {
        let options = CompilerOptions {
            output_path: None,
            run_after_build: true,
            emit_asm_only: false,
            verbose: false,
        };
        Compiler::compile_source(code, &options)
    }

    #[test]
    fn test_pro_pointers_and_memory() {
        let code = r#"
            remove garbageC
            remove Basic
            add Advanced
            use ior
            use mem

            remember val: Int = 777
            remember p: Ptr = addr val
            say @p
            @p = 999
            say @p
            say val

            remember heap_p: Ptr = alloc(16)
            @heap_p = 12345
            say @heap_p
            free(heap_p)
        "#;
        let res = run_code(code);
        assert!(res.is_ok(), "Failed: {:?}", res.err());
        assert_eq!(res.unwrap(), Some(0));
    }

    #[test]
    fn test_pro_inline_asm_and_cycles() {
        let code = r#"
            remove garbageC
            remove Basic
            add Advanced
            use ior

            measure cycles {
                remember count: Int = 0
                repeat 10 as i {
                    count = count + i
                }
            }

            asm {
                nop
                nop
            }
        "#;
        let res = run_code(code);
        assert!(res.is_ok(), "Failed: {:?}", res.err());
        assert_eq!(res.unwrap(), Some(0));
    }

    #[test]
    fn test_pro_structs() {
        let code = r#"
            remove garbageC
            remove Basic
            add Advanced
            use ior
            use mem

            struct Point {
                x: Int,
                y: Int
            }

            remember pt: Point = Point(10, 20)
            say pt.x
            say pt.y
            pt.x = 100
            say pt.x
        "#;
        let res = run_code(code);
        assert!(res.is_ok(), "Failed: {:?}", res.err());
        assert_eq!(res.unwrap(), Some(0));
    }

    #[test]
    fn test_pro_c_ffi() {
        let code = r#"
            remove garbageC
            remove Basic
            add Advanced
            use ior

            extern "C" {
                action puts(s: String) -> Int
                action abs(n: Int) -> Int
            }

            puts("Hello from C FFI puts!")
            remember neg: Int = 0 - 42
            say abs(neg)
        "#;
        let res = run_code(code);
        assert!(res.is_ok(), "Failed: {:?}", res.err());
        assert_eq!(res.unwrap(), Some(0));
    }

    #[test]
    fn test_pro_freestanding_mode() {
        let code = r#"
            remove garbageC
            remove Basic
            remove Linux
            add Advanced
            add Freestanding

            asm {
                mov rax, 60
                xor rdi, rdi
                syscall
            }
        "#;
        let res = run_code(code);
        assert!(res.is_ok(), "Failed: {:?}", res.err());
        assert_eq!(res.unwrap(), Some(0));
    }

    #[test]
    fn test_pro_math_and_threads() {
        let code = r#"
            remove garbageC
            remove Basic
            add Advanced
            use ior
            use math
            use thread

            remember s: Int = sqrt(144)
            say s

            thread {
                remember x: Int = 1 + 2
            }
        "#;
        let res = run_code(code);
        assert!(res.is_ok(), "Failed: {:?}", res.err());
        assert_eq!(res.unwrap(), Some(0));
    }
}
