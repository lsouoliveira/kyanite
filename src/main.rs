mod ast;
mod builtins;
mod dumper;
mod errors;
mod internal;
mod interpreter;
mod lexer;
mod objects;
mod parser;
mod visitor;

use clap::Parser;

use dumper::ASTDumper;
use interpreter::Interpreter;

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

    let root_dir = std::path::Path::new(filename)
        .parent()
        .unwrap_or(std::path::Path::new("."))
        .to_str()
        .unwrap_or(".");

    let mut interpreter = Interpreter::new(input, root_dir.to_string());

    let _ = interpreter.init();

    match interpreter.evaluate() {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", e);

            Ok(())
        }
    }
}

#[derive(Parser)]
struct Cli {
    #[arg(required = true)]
    file: String,

    /// Dump the AST
    #[clap(short, long)]
    dump: bool,
}

fn main() {
    let cli = Cli::parse();

    let input = std::fs::read_to_string(&cli.file).unwrap_or_else(|_| {
        eprintln!("Error: Could not read file {}", cli.file);
        std::process::exit(1);
    });

    if cli.dump {
        dump(&input);
    } else {
        interpret(&cli.file).unwrap()
    }
}
