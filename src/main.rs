mod ast;
mod lexer;
mod parser;
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
    if let Err(err) = main_entry() {
        panic!("{err}");
    }

    Ok(())
}

fn main_entry() -> Result<(), Box<dyn std::error::Error>> {
    let mut source = String::new();

    let args = Args::parse();
    File::open(args.input)?.read_to_string(&mut source)?;

    let tokens = Lexer::lex(&source)?;

    let expr = parser::Parser::parse(&tokens)?;

    // println!("Final: {}", expr.as_ref());
    dbg!(expr);

    Ok(())
}
