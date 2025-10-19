use crate::scanner::Scanner;

mod scanner;

fn main() {
    let source = String::from("
        var x = 10;
        print x + 5;
        if (x > 5) {
            print true;
        } else {
            print false;
        }
    ");
    let mut scanner = Scanner::new(source);
    scanner.scan_tokens();

    let tokens = scanner.get_tokens();
    dbg!(&tokens);
}
