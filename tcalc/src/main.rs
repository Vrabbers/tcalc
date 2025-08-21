use std::io::stdin;

use cancellation_token::CancellationToken;
use tcalc::{evaluator::Evaluator, lexer::Lexer, parser::Parser};
use tcalc_num::real::Real;

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

        let eval = Evaluator::<Real>::new();


        for expr in par.by_ref() {
            println!("{expr:#?}");
            let res = eval.evaluate(&expr, CancellationToken::default());
            println!("{res:#?}");
        }

        for e in par.diagnostic_bag() {
            println!("{e:?}");
        }

        
    }
}
