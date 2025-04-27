mod dumper;
mod interpreter;
mod lexer;
mod parser;
mod visitor;

use dumper::ASTDumper;
use interpreter::Interpreter;

#[allow(dead_code)]
fn dump() {
    let lexer = lexer::Lexer::new("about".to_string());
    let mut parser = parser::Parser::new(lexer);

    match parser.parse() {
        Ok(module) => {
            let mut dumper = ASTDumper::new();
            module.accept(&mut dumper);
            println!("{}", dumper.output);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

fn main() {
    let input = "my_function";
    let mut interpreter = Interpreter::new();

    interpreter.eval(input);
}
