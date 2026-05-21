use std::io;
use std::io::Write;

mod eval;
mod lexer;
mod parser;

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        // Read line
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("failed to read line");

        if line.chars().count() == 0 {
            return;
        }

        // tokenize input
        let tokens = match lexer::tokenize(&line) {
            Ok(result) => result,
            Err(reason) => {
                println!("Error: {:?}", reason);
                continue;
            }
        };

        // Parse input
        let expr = match parser::parse(&tokens) {
            Ok(e) => e,
            Err(err) => {
                println!("Parse error: {:?}", err);
                continue;
            }
        };

        // Evaluate
        let eval = eval::evaluate(&expr);
        match eval {
            Ok(obj) => println!("{:?}", obj.print()),
            Err(err) => println!("{:?}", err),
        }
    }
}
