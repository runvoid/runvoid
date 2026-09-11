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
use compiler::{Compiler, CompilerOptions};
use std::fs;
use std::path::PathBuf;

#[derive(ClapParser)]
#[command(name = "runvoid")]
#[command(
    about = "Compiler for the simple, high-performance Runvoid programming language",
    version = "0.2.0"
)]
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
    },

    /// Compile a Runvoid file into a standalone native ELF binary
    Build {
        /// Path to the .rv source file
        file: PathBuf,

        /// Output executable binary path
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Verbose optimizer output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Emit generated NASM assembly (x86_64) without compiling to binary
    EmitAsm {
        /// Path to the .rv source file
        file: PathBuf,

        /// Output .asm file path
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Format a Runvoid source file
    Fmt {
        /// Path to the .rv source file to format
        file: PathBuf,

        /// Write formatted output directly back to the file
        #[arg(short, long)]
        write: bool,
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
        Commands::Run { file, verbose } => {
            let options = CompilerOptions {
                output_path: None,
                run_after_build: true,
                emit_asm_only: false,
                verbose,
            };
            Compiler::compile_file(&file, &options)
        }
        Commands::Build {
            file,
            output,
            verbose,
        } => {
            let out_file = output.unwrap_or_else(|| {
                let mut p = file.clone();
                p.set_extension("");
                p
            });
            let options = CompilerOptions {
                output_path: Some(out_file),
                run_after_build: false,
                emit_asm_only: false,
                verbose,
            };
            Compiler::compile_file(&file, &options)
        }
        Commands::EmitAsm { file, output } => {
            let options = CompilerOptions {
                output_path: output,
                run_after_build: false,
                emit_asm_only: true,
                verbose: false,
            };
            Compiler::compile_file(&file, &options)
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

    println!("\x1b[1;33m📌 2. OUTPUT & COLORFUL TERMINAL\x1b[0m");
    println!("  say \"Hello {{name}}!\"                   # String interpolation");
    println!("  say green \"Success!\"                  # Green text");
    println!("  say red \"Error!\"                      # Red text");
    println!("  say yellow \"Warning!\"                 # Yellow text");
    println!("  say same \"Inline output \"             # Output without newline");
    println!("  clear screen                          # Clear terminal screen");
    println!("  cursor at 10, 5                       # Move terminal cursor\n");

    println!("\x1b[1;33m📌 3. USER INTERACTION & SOUND\x1b[0m");
    println!("  remember input = ask \"Your name: \"    # Console input");
    println!("  remember pass = ask hidden \"Pass: \"   # Hidden password input");
    println!("  remember ok = ask user \"Continue?\"    # Modal confirmation (true/false)");
    println!("  remember opt = choose \"Pick weapon\", \"Sword\", \"Bow\", \"Staff\"");
    println!("  alert \"Mission Accomplished!\"         # GUI popup dialog");
    println!("  beep                                  # Terminal bell sound");
    println!("  speak \"Greetings adventurer!\"         # Text-to-speech\n");

    println!("\x1b[1;33m📌 4. FILES & WEB\x1b[0m");
    println!("  create folder \"data\"");
    println!("  copy file \"src.txt\" to \"dest.txt\"");
    println!("  delete file \"temp.txt\"");
    println!("  if file \"config.rv\" exists {{ ... }}");
    println!("  open web \"https://runvoid.org\"");
    println!("  download \"https://example.com/data.json\" into \"data.json\"");
    println!("  remember content = read web \"https://api.github.com\"\n");

    println!("\x1b[1;33m📌 5. GRAPHICS & GUIS\x1b[0m");
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

    println!("\x1b[1;33m📌 6. PRO DEV MODE (DIRECTIVES)\x1b[0m");
    println!("  remove garbageC                       # Disable GC (Zero-GC)");
    println!("  remove Basic                          # Require explicit type annotations");
    println!("  add Advanced                          # Enable Pro optimizations & inline ASM");
    println!("  use ior                               # Explicit I/O library import\n");
}
