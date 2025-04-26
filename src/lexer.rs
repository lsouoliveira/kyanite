#[derive(Debug)]
enum TokenType {
    Identifier,
    Keyword,
}

#[derive(Debug)]
struct Token {
    kind: TokenType,
    value: String,
    line: usize,
    column: usize,
}

const KEYWORDS: [&str; 3] = ["def", "end"];

pub struct Lexer {
    input: String,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer { input }
    }

    pub fn next_token(&mut self) -> Option<Token> {}
}
