mod ast;
mod lexer;
mod parser;
mod token;

use lexer::Lexer;
use std::{fs::File, io::Read};

use clap::Parser;

use crate::ast::{FlatASTFormatter, TreeASTFormatter};

#[derive(clap::Parser, Debug)]
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

    println!("{}", size_of::<String>());

    let args = Args::parse();
    File::open(args.input)?.read_to_string(&mut source)?;

    let tokens = Lexer::lex(&source)?;

    let mut parser = parser::Parser::new();

    let ast = parser.parse(&tokens)?;

    println!("{}", TreeASTFormatter::new(&ast));
    println!("{}", FlatASTFormatter::new(&ast));
    
    Ok(())
}
