use crate::parser::Parser;
use crate::scanner::Scanner;

mod scanner;
mod parser;

fn main() {
    let source = String::from("
        var x = 10;
        var y = 5;
        var z = 0;

        if (y > 0) {
            z = z + x * (y - 2);
        } else {
            z = z + x + y;
        }
        x = x - 1;
        y = y + 1;
        print z;
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
}
