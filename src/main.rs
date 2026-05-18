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
        let tokens = match lexer::tokenize(&line) {
            Ok(result) => result,
            Err(reason) => {
                println!("Error: {:?}", reason);
                continue;
            }
        };

        // Parse input
        let mut tokens_peekable = tokens.iter().peekable();
        match parser::parse(&mut tokens_peekable) {
            Ok(e) => println!("Parsed expr: {:?}", e.print()),
            Err(err) => {
                println!("Parse error: {:?}", err);
                continue;
            }
        };
    }
}
