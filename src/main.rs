use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;

mod scanner;
mod parser;
mod interpreter;

fn main() {
    let source = String::from("
        y = 5;
        x = 89;
        x = x + 10 + 9 - y;
    ");
    let mut scanner = Scanner::new(source);
    scanner.scan_tokens();

    let tokens = scanner.get_tokens();
    dbg!(&tokens);
    let mut parser = Parser::new(tokens.clone());

    // Step 3: Parse expression
    let expression_ast= parser.parse_program();

    // Step 4: Debug print AST
    println!("{:#?}", expression_ast);

    let mut interpreter = Interpreter::new();
    interpreter.interpret(expression_ast);
}
