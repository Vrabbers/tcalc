use std::io::stdin;

use tcalc::lexer::Lexer;

fn read() -> String {
    let mut ln = String::new();
    _ = stdin().read_line(&mut ln).unwrap();
    ln
}

fn main() {
    println!("tcalc");
    let mut lines: Vec<String> = Vec::new();

    loop {
        let x = read();
        if x.trim().is_empty() {
            break;
        }
        print!("{}", x);
        lines.push(String::from(x.trim()));
    }

    let src = lines.concat(); //already have line

    for token in Lexer::new(src, true) {
        println!("{:?}", token);
    }
}
