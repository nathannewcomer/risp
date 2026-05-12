use std::io;
use std::io::Write;

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
        let tokens = lexer::tokenize(&line);
        match tokens {
            Ok(result) => println!("Tokens: {:?}", result),
            Err(reason) => println!("Error: {:?}", reason),
        }

        // // Parse input
        // let sexpr = parser::parse(&line);
    }
}
