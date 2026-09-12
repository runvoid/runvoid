pub mod ast;
pub mod codegen;
pub mod compiler;
pub mod formatter;
pub mod lexer;
pub mod optimizer;
pub mod parser;
pub mod token;
pub mod typechecker;

use clap::{Parser as ClapParser, Subcommand};
use compiler::{Compiler, CompilerOptions, TargetOs};
use std::fs;
use std::path::PathBuf;

#[derive(ClapParser)]
#[command(name = "runvoid")]
#[command(
    about = "Compiler for the simple, high-performance Runvoid programming language (Runvoid 1)",
    version = "1.3.0"
)
]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a Runvoid script directly (compiles and executes immediately)
    Run {
        /// Path to the .rv source file
        file: PathBuf,

        /// Verbose optimizer output
        #[arg(short, long)]
        verbose: bool,

        /// Target OS (linux, windows)
        #[arg(short, long, value_enum)]
        target: Option<TargetOs>,
    },

    /// Compile a Runvoid file into a standalone native binary
    Build {
        /// Path to the .rv source file
        file: PathBuf,

        /// Output executable binary path
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Verbose optimizer output
        #[arg(short, long)]
        verbose: bool,

        /// Target OS (linux, windows)
        #[arg(short, long, value_enum)]
        target: Option<TargetOs>,
    },

    /// Emit generated NASM assembly (x86_64) without compiling to binary
    EmitAsm {
        /// Path to the .rv source file
        file: PathBuf,

        /// Output .asm file path
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Target OS (linux, windows)
        #[arg(short, long, value_enum)]
        target: Option<TargetOs>,
    },

    /// Format a Runvoid source file
    Fmt {
        /// Path to the .rv source file to format
        file: PathBuf,

        /// Write formatted output directly back to the file
        #[arg(short, long)]
        write: bool,
    },

    /// Run tests in a file or directory
    Test {
        /// Path to test file or directory (defaults to current directory and tests/)
        file: Option<PathBuf>,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,

        /// Target OS (linux, windows)
        #[arg(short, long, value_enum)]
        target: Option<TargetOs>,
    },

    /// Launch the interactive Runvoid REPL
    Repl,

    /// Watch a file for changes and recompile/run automatically
    Watch {
        /// Path to the .rv source file
        file: PathBuf,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,

        /// Target OS (linux, windows)
        #[arg(short, long, value_enum)]
        target: Option<TargetOs>,
    },

    /// Create a new starter project from a template (game, gui, script)
    New {
        /// Template type: game, gui, or script
        template: String,

        /// Optional project/file name
        name: Option<String>,
    },

    /// Interactive quick-reference cheat sheet for all Runvoid syntax & features
    Cheat,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Run {
            file,
            verbose,
            target,
        } => {
            let t = target.unwrap_or_default();
            let options = CompilerOptions {
                output_path: None,
                run_after_build: true,
                emit_asm_only: false,
                verbose,
                target: t,
            };
            Compiler::compile_file(&file, &options)
        }
        Commands::Build {
            file,
            output,
            verbose,
            target,
        } => {
            let t = target.unwrap_or_default();
            let out_file = output.unwrap_or_else(|| {
                let mut p = file.clone();
                if t == TargetOs::Windows {
                    p.set_extension("exe");
                } else {
                    p.set_extension("");
                }
                p
            });
            let options = CompilerOptions {
                output_path: Some(out_file),
                run_after_build: false,
                emit_asm_only: false,
                verbose,
                target: t,
            };
            Compiler::compile_file(&file, &options)
        }
        Commands::EmitAsm {
            file,
            output,
            target,
        } => {
            let t = target.unwrap_or_default();
            let options = CompilerOptions {
                output_path: output,
                run_after_build: false,
                emit_asm_only: true,
                verbose: false,
                target: t,
            };
            Compiler::compile_file(&file, &options)
        }
        Commands::Test {
            file,
            verbose,
            target,
        } => {
            run_tests(file, verbose, target);
            return;
        }
        Commands::Repl => {
            run_repl();
            return;
        }
        Commands::Watch {
            file,
            verbose,
            target,
        } => {
            run_watch(file, verbose, target);
            return;
        }
        Commands::Fmt { file, write } => {
            let content = match fs::read_to_string(&file) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Failed to read file {:?}: {}", file, e);
                    std::process::exit(1);
                }
            };
            let formatted = formatter::format_source(&content);
            if write {
                if let Err(e) = fs::write(&file, &formatted) {
                    eprintln!("Failed to write formatted file {:?}: {}", file, e);
                    std::process::exit(1);
                }
                println!("✨ Formatted {:?}", file);
            } else {
                print!("{}", formatted);
            }
            return;
        }
        Commands::New { template, name } => {
            let file_name = name.unwrap_or_else(|| match template.to_lowercase().as_str() {
                "game" => "game.rv".to_string(),
                "gui" => "app.rv".to_string(),
                _ => "main.rv".to_string(),
            });
            let final_name = if file_name.ends_with(".rv") {
                file_name
            } else {
                format!("{}.rv", file_name)
            };

            let content = match template.to_lowercase().as_str() {
                "game" => {
                    r#"// Runvoid 2D Canvas Game Starter
say green "Launching Runvoid 2D Game..."

screen "Runvoid Adventure", 640, 480 {
    // Draw background
    draw box at 0, 0, size 640, 480, color "black"

    // Draw player and objects
    draw circle at 320, 240, size 35, color "cyan"
    draw box at 100, 150, size 80, 80, color "green"

    // Draw game UI
    draw text "Runvoid Adventure 2D - Press key to exit", at 160, 50, color "yellow"
}
"#
                }
                "gui" => {
                    r#"// Runvoid Desktop GUI Starter
say cyan "Launching Runvoid Desktop App..."

window "My Application", 500, 400 {
    label "Welcome to Runvoid Native GUI!"

    button "Click Me!" {
        say green "Button was clicked!"
        beep
    }

    checkbox "Enable Sound Effects", 1
}
"#
                }
                _ => {
                    r#"// Runvoid Conversational Starter Script
say cyan "=== Welcome to Runvoid ==="

remember backpack = "Sword", "Health Potion", "Torch"
say yellow "Items in backpack: {count backpack}"

for every item in backpack {
    say " - Found item: {item}"
}

remember hero = ask "What is your hero name? "
say green "Welcome to the adventure, {hero}!"
beep
"#
                }
            };

            if let Err(e) = fs::write(&final_name, content) {
                eprintln!("Failed to create starter project: {}", e);
                std::process::exit(1);
            }
            println!(
                "🎉 Created starter template '{}' in {}",
                template, final_name
            );
            println!("👉 Run it with: runvoid run {}", final_name);
            return;
        }
        Commands::Cheat => {
            print_cheat_sheet();
            return;
        }
    };

    match result {
        Ok(Some(code)) if code != 0 => {
            std::process::exit(code);
        }
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}

fn run_tests(file: Option<PathBuf>, verbose: bool, target: Option<TargetOs>) {
    let test_files = if let Some(f) = file {
        vec![f]
    } else {
        let mut files = Vec::new();
        if let Ok(entries) = fs::read_dir("tests") {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.extension().map(|ext| ext == "rv").unwrap_or(false) {
                    files.push(p);
                }
            }
        }
        if let Ok(entries) = fs::read_dir(".") {
            for entry in entries.flatten() {
                let p = entry.path();
                if let Some(name) = p.file_name().and_then(|n| n.to_str()) {
                    if name.ends_with("_test.rv")
                        || (name.starts_with("test_") && name.ends_with(".rv"))
                    {
                        if !files.contains(&p) {
                            files.push(p);
                        }
                    }
                }
            }
        }
        files.sort();
        files
    };

    if test_files.is_empty() {
        println!("No test files found (*.rv in tests/ or *_test.rv in current directory).");
        return;
    }

    println!("Running {} test file(s)...", test_files.len());
    let mut passed = 0;
    let mut failed = 0;
    let t = target.unwrap_or_default();

    for test_file in &test_files {
        println!("\n▶️ Running test file: {:?}", test_file);
        let options = CompilerOptions {
            output_path: None,
            run_after_build: true,
            emit_asm_only: false,
            verbose,
            target: t,
        };
        match Compiler::compile_file(test_file, &options) {
            Ok(Some(0)) | Ok(None) => {
                println!("✅ Passed: {:?}", test_file);
                passed += 1;
            }
            Ok(Some(code)) => {
                eprintln!("❌ Failed (exit code {}): {:?}", code, test_file);
                failed += 1;
            }
            Err(e) => {
                eprintln!("❌ Failed with error: {}\n  In: {:?}", e, test_file);
                failed += 1;
            }
        }
    }

    println!("\n==========================================");
    if failed == 0 {
        println!("\x1b[1;32mTest suite passed! ({} passed, 0 failed)\x1b[0m", passed);
    } else {
        eprintln!(
            "\x1b[1;31mTest suite failed! ({} passed, {} failed)\x1b[0m",
            passed, failed
        );
        std::process::exit(1);
    }
}

fn run_repl() {
    use std::io::{self, BufRead, Write};
    println!("\x1b[1;36m╔══════════════════════════════════════════════════════════════╗\x1b[0m");
    println!("\x1b[1;36m║           Runvoid 1 Interactive REPL (v1.3)                  ║\x1b[0m");
    println!("\x1b[1;36m║   Type :help for help, :clear to reset, :exit to quit        ║\x1b[0m");
    println!("\x1b[1;36m╚══════════════════════════════════════════════════════════════╝\x1b[0m");


    let stdin = io::stdin();
    let mut history: Vec<String> = Vec::new();
    let tmp_path = std::env::temp_dir().join("runvoid_repl_session.rv");

    loop {
        print!("\x1b[1;32mrunvoid>\x1b[0m ");
        let _ = io::stdout().flush();

        let mut line = String::new();
        if stdin.lock().read_line(&mut line).unwrap_or(0) == 0 {
            println!("\nGoodbye!");
            break;
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if trimmed == ":exit" || trimmed == ":quit" || trimmed == ":q" {
            println!("Goodbye!");
            break;
        }

        if trimmed == ":clear" {
            history.clear();
            let _ = fs::remove_file(&tmp_path);
            println!("✨ Session history cleared.");
            continue;
        }

        if trimmed == ":history" {
            println!("--- Session History ---");
            for (i, h) in history.iter().enumerate() {
                println!("{:3}: {}", i + 1, h);
            }
            continue;
        }

        if trimmed == ":help" {
            println!("Available commands:");
            println!("  :clear    - Reset the REPL session state");
            println!("  :history  - View accumulated session statements");
            println!("  :help     - Display this help message");
            println!("  :exit, :q - Exit REPL");
            continue;
        }

        let is_statement = trimmed.starts_with("remember ")
            || trimmed.starts_with("say ")
            || trimmed.starts_with("define ")
            || trimmed.starts_with("if ")
            || trimmed.starts_with("while ")
            || trimmed.starts_with("for ")
            || trimmed.starts_with("thread ")
            || trimmed.starts_with("remove ")
            || trimmed.starts_with("add ")
            || trimmed.starts_with("use ")
            || trimmed.starts_with("window ")
            || trimmed.starts_with("screen ")
            || trimmed.starts_with("verify ")
            || trimmed.starts_with("test ")
            || trimmed.starts_with("match ")
            || trimmed.starts_with("play ");

        let code_to_run = if is_statement {
            trimmed.to_string()
        } else {
            format!("say ({})", trimmed)
        };

        let mut full_code = String::new();
        for h in &history {
            full_code.push_str(h);
            full_code.push('\n');
        }
        full_code.push_str(&code_to_run);
        full_code.push('\n');

        if let Err(e) = fs::write(&tmp_path, &full_code) {
            eprintln!("Failed to write REPL temp file: {}", e);
            continue;
        }

        let options = CompilerOptions {
            output_path: None,
            run_after_build: true,
            emit_asm_only: false,
            verbose: false,
            target: TargetOs::default(),
        };

        match Compiler::compile_file(&tmp_path, &options) {
            Ok(Some(code)) if code == 0 => {
                if trimmed.starts_with("remember ")
                    || trimmed.starts_with("define ")
                    || trimmed.starts_with("add ")
                    || trimmed.starts_with("remove ")
                {
                    history.push(trimmed.to_string());
                }
            }
            Ok(Some(code)) => {
                eprintln!("Process exited with status {}", code);
            }
            Ok(None) => {}
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    }

    let _ = fs::remove_file(tmp_path);
}

fn run_watch(file: PathBuf, verbose: bool, target: Option<TargetOs>) {
    if !file.exists() {
        eprintln!("Error: File '{:?}' not found", file);
        std::process::exit(1);
    }

    println!(
        "\x1b[1;36m👀 Watching {:?} for changes... (Press Ctrl+C to stop)\x1b[0m",
        file
    );

    let mut last_modified = fs::metadata(&file).and_then(|m| m.modified()).ok();
    let t = target.unwrap_or_default();

    let execute = |path: &PathBuf| {
        println!("\n\x1b[1;34m[runvoid watch]\x1b[0m Running {:?}...", path);
        let options = CompilerOptions {
            output_path: None,
            run_after_build: true,
            emit_asm_only: false,
            verbose,
            target: t,
        };
        if let Err(e) = Compiler::compile_file(path, &options) {
            eprintln!("\x1b[1;31m[Error]\x1b[0m {}", e);
        }
    };

    execute(&file);

    loop {
        std::thread::sleep(std::time::Duration::from_millis(300));
        let current_modified = fs::metadata(&file).and_then(|m| m.modified()).ok();
        if current_modified != last_modified {
            last_modified = current_modified;
            std::thread::sleep(std::time::Duration::from_millis(50));
            execute(&file);
        }
    }
}

fn print_cheat_sheet() {
    println!(
        "\x1b[1;36m╔══════════════════════════════════════════════════════════════════════╗\x1b[0m"
    );
    println!(
        "\x1b[1;36m║                  🚀 RUNVOID LANGUAGE CHEAT SHEET                    ║\x1b[0m"
    );
    println!(
        "\x1b[1;36m╚══════════════════════════════════════════════════════════════════════╝\x1b[0m\n"
    );

    println!("\x1b[1;33m📌 1. VARIABLES & COLLECTIONS\x1b[0m");
    println!("  remember x = 42                       # Automatic type inference");
    println!("  remember name = \"Alice\"");
    println!("  remember list = \"apple\", \"sword\"     # Natural word list");
    println!("  add \"shield\" to list                  # Add item to list");
    println!("  remove \"apple\" from list              # Remove item from list");
    println!("  {{count list}}                           # Number of elements in list");
    println!("  if list has \"sword\" {{ ... }}           # Membership check");
    println!("  for every item in list {{ ... }}        # Loop through list\n");

    println!("\x1b[1;33m📌 2. DICTIONARIES / MAPS (Runvoid 1.3)\x1b[0m");
    println!("  remember hero = name: \"Alex\", hp: 100 # Key-value map definition");
    println!("  say hero[\"name\"]                      # Square bracket indexing");
    println!("  say hero's hp                         # Natural possessive syntax");
    println!("  add \"shield\": 50 to hero              # Add key-value pair");
    println!("  remove \"hp\" from hero                 # Remove key");
    println!("  if hero has \"name\" {{ ... }}            # Key existence check\n");

    println!("\x1b[1;33m📌 3. PATTERN MATCHING & PIPELINES (Runvoid 1.3)\x1b[0m");
    println!("  match role {{                           # Pattern matching");
    println!("      when \"admin\" -> say \"Full access\"");
    println!("      when \"guest\" -> say \"Limited access\"");
    println!("      otherwise   -> say \"Unknown role\"");
    println!("  }}");
    println!("  5 |> square |> say                    # Pipeline operator |>\n");

    println!("\x1b[1;33m📌 4. BITWISE OPERATORS & AUDIO SYNTHESIZER (Runvoid 1.3)\x1b[0m");
    println!("  remember mask = a bit and b           # bit and / bit or / bit xor");
    println!("  remember val = 1 shift left 4         # shift left / shift right");
    println!("  remember inv = ~mask                  # bit not (~)");
    println!("  play synth 440, 200                   # Synthesizer tone (Hz, ms)\n");

    println!("\x1b[1;33m📌 5. VERIFICATION & TESTING (Runvoid 1.3)\x1b[0m");

    println!("  verify that 2 + 2 is 4                # Assertion / verification");
    println!("  test \"player level\" {{                 # Unit test block");
    println!("      verify that player's hp > 0");
    println!("  }}\n");

    println!("\x1b[1;33m📌 6. OUTPUT & COLORFUL TERMINAL\x1b[0m");
    println!("  say \"Hello {{name}}!\"                   # String interpolation");
    println!("  say green \"Success!\"                  # Green text");
    println!("  say red \"Error!\"                      # Red text");
    println!("  say yellow \"Warning!\"                 # Yellow text");
    println!("  say same \"Inline output \"             # Output without newline");
    println!("  clear screen                          # Clear terminal screen");
    println!("  cursor at 10, 5                       # Move terminal cursor\n");

    println!("\x1b[1;33m📌 7. USER INTERACTION & SOUND\x1b[0m");
    println!("  remember input = ask \"Your name: \"    # Console input");
    println!("  remember pass = ask hidden \"Pass: \"   # Hidden password input");
    println!("  remember ok = ask user \"Continue?\"    # Modal confirmation (true/false)");
    println!("  remember opt = choose \"Pick weapon\", \"Sword\", \"Bow\", \"Staff\"");
    println!("  alert \"Mission Accomplished!\"         # GUI popup dialog");
    println!("  beep                                  # Terminal bell sound");
    println!("  speak \"Greetings adventurer!\"         # Text-to-speech\n");

    println!("\x1b[1;33m📌 8. FILES & WEB\x1b[0m");
    println!("  create folder \"data\"");
    println!("  copy file \"src.txt\" to \"dest.txt\"");
    println!("  delete file \"temp.txt\"");
    println!("  if file \"config.rv\" exists {{ ... }}");
    println!("  open web \"https://runvoid.org\"");
    println!("  download \"https://example.com/data.json\" into \"data.json\"");
    println!("  remember content = read web \"https://api.github.com\"\n");

    println!("\x1b[1;33m📌 9. GRAPHICS & GUIS\x1b[0m");
    println!("  screen \"My Game\", 640, 480 {{           # 2D Hardware Canvas");
    println!("      draw box at 0, 0, size 640, 480, color \"black\"");
    println!("      draw circle at 320, 240, size 40, color \"cyan\"");
    println!("      draw line from 0, 0, to 640, 480, color \"red\"");
    println!("      draw text \"Score: 100\", at 20, 30, color \"white\"");
    println!("  }}");
    println!("  window \"My App\", 500, 400 {{            # Native Desktop GUI");
    println!("      label \"Welcome!\"");
    println!("      button \"Click\" {{ say \"Clicked!\"; beep }}");
    println!("      checkbox \"Enable Turbo\", 1");
    println!("  }}\n");

    println!("\x1b[1;33m📌 10. PRO DEV MODE (DIRECTIVES)\x1b[0m");
    println!("  remove garbageC                       # Disable GC (Zero-GC)");
    println!("  remove Basic                          # Require explicit type annotations");
    println!("  add Advanced                          # Enable Pro optimizations & inline ASM");
    println!("  use ior                               # Explicit I/O library import\n");
}
