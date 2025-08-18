mod lexer;
mod token;

use std::{fs::File, io::Read};

use clap::Parser;

use lexer::Lexer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Path to the input file")]
    input: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut source = String::new();

    File::open(args.input)?.read_to_string(&mut source)?;

    let tokens = match Lexer::lex(&source) {
        Ok(tokens) => tokens,
        Err(err) => {
            panic!("\n{err}\n")
        }
    };

    for token in &tokens {
        println!("{token}");
    }

    Ok(())
}
