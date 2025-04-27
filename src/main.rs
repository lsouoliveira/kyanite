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
    let parser = parser::Parser::new(lexer);

    match parser.parse() {
        Ok(program) => {
            let mut dumper = ASTDumper::new();
            program.accept(&mut dumper);
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
