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
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a Runvoid script or project directly (compiles and executes immediately)
    Run {
        /// Path to the .rv source file (defaults to runvoid.toml or src/main.rv)
        file: Option<PathBuf>,

        /// Verbose optimizer output
        #[arg(short, long)]
        verbose: bool,

        /// Target OS (linux, windows)
        #[arg(short, long, value_enum)]
        target: Option<TargetOs>,
    },

    /// Compile a Runvoid file or project into a standalone native binary
    Build {
        /// Path to the .rv source file (defaults to runvoid.toml or src/main.rv)
        file: Option<PathBuf>,

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
        /// Path to the .rv source file (defaults to runvoid.toml or src/main.rv)
        file: Option<PathBuf>,

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

    /// Initialize a new Runvoid project with runvoid.toml and standard layout
    Init {
        /// Project directory name (defaults to current directory)
        name: Option<String>,
    },

    /// Clean build artifacts, object files, and bin/ directory
    Clean,
}

fn main() {
    let cli = Cli::parse();

    let (result, active_file) = match cli.command {
        Commands::Init { name } => {
            handle_init(name);
            return;
        }
        Commands::Clean => {
            handle_clean();
            return;
        }
        Commands::Run {
            file,
            verbose,
            target,
        } => {
            let (target_file, project_cfg) = match resolve_source_file(file) {
                Ok(res) => res,
                Err(err) => {
                    eprintln!("\x1b[1;31merror\x1b[0m: {}", err);
                    std::process::exit(1);
                }
            };
            let t = target
                .or_else(|| project_cfg.as_ref().and_then(|c| c.target))
                .unwrap_or_default();
            let options = CompilerOptions {
                output_path: None,
                run_after_build: true,
                emit_asm_only: false,
                verbose,
                target: t,
            };
            (
                Compiler::compile_file(&target_file, &options),
                Some(target_file),
            )
        }
        Commands::Build {
            file,
            output,
            verbose,
            target,
        } => {
            let (target_file, project_cfg) = match resolve_source_file(file) {
                Ok(res) => res,
                Err(err) => {
                    eprintln!("\x1b[1;31merror\x1b[0m: {}", err);
                    std::process::exit(1);
                }
            };
            let t = target
                .or_else(|| project_cfg.as_ref().and_then(|c| c.target))
                .unwrap_or_default();
            let out_file = output
                .or_else(|| project_cfg.as_ref().and_then(|c| c.output.clone()))
                .unwrap_or_else(|| {
                    let mut p = target_file.clone();
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
            (
                Compiler::compile_file(&target_file, &options),
                Some(target_file),
            )
        }
        Commands::EmitAsm {
            file,
            output,
            target,
        } => {
            let (target_file, project_cfg) = match resolve_source_file(file) {
                Ok(res) => res,
                Err(err) => {
                    eprintln!("\x1b[1;31merror\x1b[0m: {}", err);
                    std::process::exit(1);
                }
            };
            let t = target
                .or_else(|| project_cfg.as_ref().and_then(|c| c.target))
                .unwrap_or_default();
            let options = CompilerOptions {
                output_path: output,
                run_after_build: false,
                emit_asm_only: true,
                verbose: false,
                target: t,
            };
            (
                Compiler::compile_file(&target_file, &options),
                Some(target_file),
            )
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
            format_compiler_error(&err, active_file.as_deref());
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
                if let Some(name) = p.file_name().and_then(|n| n.to_str())
                    && (name.ends_with("_test.rv")
                        || (name.starts_with("test_") && name.ends_with(".rv")))
                    && !files.contains(&p)
                {
                    files.push(p);
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
        println!(
            "\x1b[1;32mTest suite passed! ({} passed, 0 failed)\x1b[0m",
            passed
        );
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
            Ok(Some(0)) => {
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

    println!("\x1b[1;33m📌 11. PROJECT MANAGEMENT & CLI\x1b[0m");
    println!("  runvoid init [app]                    # Initialize new project with runvoid.toml");
    println!("  runvoid run                           # Run project from current directory");
    println!(
        "  runvoid build                         # Build native binary specified in runvoid.toml"
    );
    println!("  runvoid test                          # Run automated test suites");
    println!("  runvoid clean                         # Clean build artifacts and bin/ folder\n");
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    pub entry: PathBuf,
    pub output: Option<PathBuf>,
    pub target: Option<TargetOs>,
}

impl ProjectConfig {
    pub fn load_from_dir(dir: &std::path::Path) -> Option<Self> {
        let manifest_path = dir.join("runvoid.toml");
        if !manifest_path.exists() {
            return None;
        }

        let content = fs::read_to_string(&manifest_path).ok()?;
        let mut name = dir
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("app")
            .to_string();
        let mut version = "1.0.0".to_string();
        let mut entry = PathBuf::from("src/main.rv");
        let mut output = None;
        let mut target = None;

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
                continue;
            }
            if let Some((k, v)) = line.split_once('=') {
                let key = k.trim();
                let val = v.trim().trim_matches('"').trim_matches('\'').trim();
                match key {
                    "name" => name = val.to_string(),
                    "version" => version = val.to_string(),
                    "entry" => entry = PathBuf::from(val),
                    "output" => output = Some(PathBuf::from(val)),
                    "target" => {
                        if val.eq_ignore_ascii_case("windows") {
                            target = Some(TargetOs::Windows);
                        } else if val.eq_ignore_ascii_case("linux") {
                            target = Some(TargetOs::Linux);
                        }
                    }
                    _ => {}
                }
            }
        }

        Some(Self {
            name,
            version,
            entry,
            output,
            target,
        })
    }
}

fn resolve_source_file(file: Option<PathBuf>) -> Result<(PathBuf, Option<ProjectConfig>), String> {
    if let Some(f) = file {
        if !f.exists() {
            return Err(format!("Source file not found: {}", f.display()));
        }
        let cfg = ProjectConfig::load_from_dir(&PathBuf::from("."));
        return Ok((f, cfg));
    }

    if let Some(cfg) = ProjectConfig::load_from_dir(&PathBuf::from("."))
        && cfg.entry.exists()
    {
        return Ok((cfg.entry.clone(), Some(cfg)));
    }

    if PathBuf::from("src/main.rv").exists() {
        return Ok((PathBuf::from("src/main.rv"), None));
    }
    if PathBuf::from("main.rv").exists() {
        return Ok((PathBuf::from("main.rv"), None));
    }

    Err(
        "No input file specified and no 'runvoid.toml' or 'src/main.rv' found.\nRun 'runvoid init' to scaffold a new project, or pass a file: 'runvoid run <file.rv>'".to_string(),
    )
}

fn handle_init(name_opt: Option<String>) {
    let (target_dir, proj_name) = match name_opt {
        Some(name) => {
            let path = PathBuf::from(&name);
            (path, name)
        }
        None => {
            let current = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            let dir_name = current
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("runvoid_app")
                .to_string();
            (PathBuf::from("."), dir_name)
        }
    };

    if target_dir != std::path::Path::new(".")
        && !target_dir.exists()
        && let Err(e) = fs::create_dir_all(&target_dir)
    {
        eprintln!("\x1b[1;31merror\x1b[0m: Failed to create directory: {}", e);
        return;
    }

    let manifest_path = target_dir.join("runvoid.toml");
    if manifest_path.exists() {
        eprintln!(
            "\x1b[1;33mwarning\x1b[0m: 'runvoid.toml' already exists in {}.",
            target_dir.display()
        );
        return;
    }

    let src_dir = target_dir.join("src");
    let tests_dir = target_dir.join("tests");
    let _ = fs::create_dir_all(&src_dir);
    let _ = fs::create_dir_all(&tests_dir);

    // Write runvoid.toml
    let manifest_content = format!(
        r#"[project]
name = "{proj_name}"
version = "1.0.0"
entry = "src/main.rv"
description = "A modern Runvoid application"
authors = []

[build]
target = "auto"
output = "bin/{proj_name}"
"#
    );
    let _ = fs::write(&manifest_path, manifest_content);

    // Write src/main.rv
    let main_path = src_dir.join("main.rv");
    if !main_path.exists() {
        let main_content = format!(
            r#"# ==============================================================================
# {proj_name} - Main Entry Point
# ==============================================================================

say "Welcome to {proj_name} powered by Runvoid 1!"

remember lucky = random 1 to 100
say "Your lucky number today: {{lucky}}"
"#
        );
        let _ = fs::write(&main_path, main_content);
    }

    // Write tests/main_test.rv
    let test_path = tests_dir.join("main_test.rv");
    if !test_path.exists() {
        let test_content = r#"test "project sanity test" {
    remember val = 10 + 20
    verify that val is 30
}
"#;
        let _ = fs::write(&test_path, test_content);
    }

    // Write .gitignore
    let gitignore_path = target_dir.join(".gitignore");
    if !gitignore_path.exists() {
        let gitignore_content = "bin/\n*.o\n*.asm\nbuild.log\n";
        let _ = fs::write(&gitignore_path, gitignore_content);
    }

    // Write README.md
    let readme_path = target_dir.join("README.md");
    if !readme_path.exists() {
        let readme_content = format!(
            r#"# {proj_name}

A modern high-performance application built with the **Runvoid** programming language.

## Quick Start

```bash
# Run directly:
runvoid run

# Run tests:
runvoid test

# Build standalone native binary:
runvoid build

# Clean build artifacts:
runvoid clean
```
"#
        );
        let _ = fs::write(&readme_path, readme_content);
    }

    println!("\x1b[1;32m✓ Initialized new Runvoid project '{proj_name}' successfully!\x1b[0m");
    println!("Created:");
    println!("  ├── runvoid.toml");
    println!("  ├── src/main.rv");
    println!("  ├── tests/main_test.rv");
    println!("  ├── .gitignore");
    println!("  └── README.md\n");
    println!("To start running:");
    if target_dir != std::path::Path::new(".") {
        println!("  cd {proj_name}");
    }
    println!("  runvoid run");
}

fn handle_clean() {
    let mut removed = 0;
    if PathBuf::from("bin").exists() && fs::remove_dir_all("bin").is_ok() {
        println!("✓ Removed bin/ directory");
        removed += 1;
    }
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries.flatten() {
            let p = entry.path();
            if let Some(ext) = p.extension()
                && (ext == "o" || ext == "asm")
            {
                let _ = fs::remove_file(&p);
                removed += 1;
            }
        }
    }
    if PathBuf::from("build.log").exists() {
        let _ = fs::remove_file("build.log");
        removed += 1;
    }
    println!(
        "\x1b[1;32m✓ Clean complete! ({} items cleaned)\x1b[0m",
        removed
    );
}

fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let mut dp = vec![vec![0; b_bytes.len() + 1]; a_bytes.len() + 1];
    for (i, row) in dp.iter_mut().enumerate().take(a_bytes.len() + 1) {
        row[0] = i;
    }
    if let Some(first_row) = dp.first_mut() {
        for (j, cell) in first_row.iter_mut().enumerate().take(b_bytes.len() + 1) {
            *cell = j;
        }
    }
    for i in 1..=a_bytes.len() {
        for j in 1..=b_bytes.len() {
            let cost = if a_bytes[i - 1] == b_bytes[j - 1] {
                0
            } else {
                1
            };
            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + cost);
        }
    }
    dp[a_bytes.len()][b_bytes.len()]
}

fn find_did_you_mean(word: &str) -> Option<&'static str> {
    const KEYWORDS: &[&str] = &[
        "say",
        "remember",
        "ask",
        "choose",
        "alert",
        "if",
        "otherwise",
        "while",
        "repeat",
        "for",
        "every",
        "in",
        "action",
        "give",
        "define",
        "match",
        "when",
        "play",
        "synth",
        "beep",
        "speak",
        "wait",
        "write",
        "read",
        "screen",
        "window",
        "button",
        "label",
        "checkbox",
        "measure",
        "time",
        "cycles",
        "verify",
        "that",
        "test",
        "add",
        "remove",
        "use",
        "struct",
        "asm",
        "thread",
        "atomic",
    ];

    let word_lower = word.to_lowercase();
    let mut best_match = None;
    let mut best_dist = usize::MAX;

    for &kw in KEYWORDS {
        let dist = levenshtein_distance(&word_lower, kw);
        if dist < best_dist && dist <= 2 {
            best_dist = dist;
            best_match = Some(kw);
        }
    }
    best_match
}

fn format_compiler_error(err: &str, file: Option<&std::path::Path>) {
    eprintln!("\x1b[1;31merror\x1b[0m: {}", err);

    // Try extracting word after 'Ident("..."', 'undeclared variable '...', or 'Unknown function '...'
    let mut candidate_ident = None;
    if let Some(pos) = err.find("Ident(\"") {
        let after = &err[pos + 7..];
        if let Some(end) = after.find('"') {
            candidate_ident = Some(&after[..end]);
        }
    } else if let Some(pos) = err.find("undeclared variable '") {
        let after = &err[pos + 21..];
        if let Some(end) = after.find('\'') {
            candidate_ident = Some(&after[..end]);
        }
    } else if let Some(pos) = err.find("Unknown function '") {
        let after = &err[pos + 18..];
        if let Some(end) = after.find('\'') {
            candidate_ident = Some(&after[..end]);
        }
    }

    if let Some(ident) = candidate_ident
        && let Some(suggestion) = find_did_you_mean(ident)
    {
        eprintln!(
            "  \x1b[1;36m= help\x1b[0m: did you mean '\x1b[1;32m{}\x1b[0m'?",
            suggestion
        );
    }

    if let Some(f) = file
        && let Some(line_pos) = err.find("line ")
        && let rest = &err[line_pos + 5..]
        && let Some((l_str, c_rest)) = rest.split_once(',')
        && let Ok(line_num) = l_str.trim().parse::<usize>()
    {
        let col_num = if let Some(c_pos) = c_rest.find("col ") {
            c_rest[c_pos + 4..].trim().parse::<usize>().unwrap_or(1)
        } else {
            1
        };
        eprintln!(
            "  \x1b[1;34m-->\x1b[0m {}:{}:{}",
            f.display(),
            line_num,
            col_num
        );

        // Show source line if available
        if let Ok(source) = fs::read_to_string(f)
            && let Some(src_line) = source.lines().nth(line_num.saturating_sub(1))
        {
            eprintln!("   \x1b[1;34m|\x1b[0m");
            eprintln!("\x1b[1;34m{:>2} |\x1b[0m {}", line_num, src_line);
            let pad = " ".repeat(col_num.saturating_sub(1));
            eprintln!("   \x1b[1;34m|\x1b[0m \x1b[1;31m{}^\x1b[0m", pad);
        }
    }
}

#[cfg(test)]
mod project_tests {
    use super::*;

    #[test]
    fn test_did_you_mean_suggestions() {
        assert_eq!(find_did_you_mean("remembr"), Some("remember"));
        assert_eq!(find_did_you_mean("sy"), Some("say"));
        assert_eq!(find_did_you_mean("verfy"), Some("verify"));
        assert_eq!(find_did_you_mean("swiitch"), None);
    }

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
        assert_eq!(levenshtein_distance("say", "say"), 0);
        assert_eq!(levenshtein_distance("say", "sayy"), 1);
    }
}
