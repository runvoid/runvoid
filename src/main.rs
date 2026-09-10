pub mod ast;
pub mod codegen;
pub mod compiler;
pub mod lexer;
pub mod optimizer;
pub mod parser;
pub mod token;
pub mod typechecker;

use clap::{Parser as ClapParser, Subcommand};
use compiler::{Compiler, CompilerOptions};
use std::path::PathBuf;

#[derive(ClapParser)]
#[command(name = "runvoid")]
#[command(about = "Compiler for the simple, high-performance Runvoid programming language", version = "0.1.0")]
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
    };

    match result {
        Ok(code_opt) => {
            if let Some(code) = code_opt {
                if code != 0 {
                    std::process::exit(code);
                }
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
