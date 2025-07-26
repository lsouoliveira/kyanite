mod ast;
mod builtins;
mod bytecode;
mod compiler;
mod dumper;
mod errors;
mod internal;
mod interpreter;
mod lexer;
mod lock;
mod objects;
mod opcodes;
mod parser;
mod visitor;

use clap::Parser;
use std::sync::Arc;

use dumper::ASTDumper;

fn dump(input: &str) {
    let lexer = lexer::Lexer::new(input.to_string());
    let mut parser = parser::Parser::new(lexer);

    match parser.parse() {
        Ok(module) => {
            let mut dumper = ASTDumper::new();
            module.accept(&mut dumper);
            println!("{}", dumper.output);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

fn interpret(filename: &str) -> Result<(), String> {
    let input = std::fs::read_to_string(filename)
        .map_err(|_| format!("Error: Could not read file {}", filename))?;

    let _root_dir = std::path::Path::new(filename)
        .parent()
        .unwrap_or(std::path::Path::new("."))
        .to_str()
        .unwrap_or(".");

    let mut parser = parser::Parser::new(lexer::Lexer::new(input.clone()));
    let ast = Arc::new(parser.parse().unwrap_or_else(|e| {
        eprintln!("Error parsing file {}: {}", filename, e);

        std::process::exit(1);
    }));

    let mut compiler = compiler::Compiler::new(ast);
    let _ = compiler.compile().unwrap_or_else(|e| {
        eprintln!("{}", e.to_string());

        std::process::exit(1);
    });

    let mut interpreter = interpreter::Interpreter::new(".");

    let _ = interpreter
        .eval(&compiler.get_output())
        .unwrap_or_else(|e| {
            eprintln!("{}", e.to_string());

            std::process::exit(1);
        });

    Ok(())
}

fn disassemble(filename: &str) -> Result<(), String> {
    let input = std::fs::read_to_string(filename)
        .map_err(|_| format!("Error: Could not read file {}", filename))?;

    let mut parser = parser::Parser::new(lexer::Lexer::new(input));
    let ast = Arc::new(parser.parse().unwrap_or_else(|e| {
        eprintln!("Error parsing file {}: {}", filename, e);
        std::process::exit(1);
    }));

    let mut compiler = compiler::Compiler::new(ast);
    let _ = compiler.compile().unwrap_or_else(|e| {
        eprintln!("Error compiling file {}: {}", filename, e);

        std::process::exit(1);
    });

    println!("{}", compiler.get_output().dis());

    Ok(())
}

#[derive(Parser)]
struct Cli {
    #[arg(required = true)]
    file: String,

    /// Dump the AST
    #[clap(short, long)]
    dump: bool,

    /// Disassemble the bytecode
    #[clap(long)]
    disassemble: bool,
}

fn main() {
    let cli = Cli::parse();

    let input = std::fs::read_to_string(&cli.file).unwrap_or_else(|_| {
        eprintln!("Error: Could not read file {}", cli.file);
        std::process::exit(1);
    });

    if cli.dump {
        dump(&input);
    } else if cli.disassemble {
        disassemble(&cli.file).unwrap();
    } else {
        interpret(&cli.file).unwrap()
    }
}
