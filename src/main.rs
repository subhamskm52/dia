use crate::parser::Parser;
use crate::scanner::Scanner;

mod scanner;
mod parser;

fn main() {
    let source = String::from("
        1 + 2 * (3 - 4)
    ");
    let mut scanner = Scanner::new(source);
    scanner.scan_tokens();

    let tokens = scanner.get_tokens();
    dbg!(&tokens);
    let mut parser = Parser::new(tokens.clone());

    // Step 3: Parse expression
    let expression_ast= parser.parse_expression();

    // Step 4: Debug print AST
    println!("{:#?}", expression_ast);
}
