use std::io::stdin;

use tcalc::{lexer::Lexer, parser::Parser};

fn read() -> String {
    let mut ln = String::new();
    _ = stdin().read_line(&mut ln).unwrap();
    ln
}

fn main() {
    println!("tcalc");
    loop {
        let mut lines: Vec<String> = Vec::new();

        loop {
            let x = read();
            if x.trim().is_empty() {
                break;
            }
            print!("{x}");
            lines.push(String::from(x.trim()));
        }

        let src = lines.concat(); //already have line
        let lex = Lexer::new(src, true);
        let mut par = Parser::new(lex);

        for expr in par.by_ref() {
            println!("{expr:#?}");
        }

        for e in par.diagnostic_bag() {
            println!("{e:?}");
        }
    }
}
