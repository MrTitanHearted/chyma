mod ast;
mod lexer;
mod parser;
mod token;

mod resolvers;
use resolvers::*;

use lexer::Lexer;
use std::{fs::File, io::Read};

use clap::Parser;

use crate::ast::{FlatASTFormatter, TreeASTFormatter};
use crate::parser::{ParserError, ParserErrorFormatter};
use crate::token::TokenStringInterner;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Path to the input file")]
    input: String,
}

fn main() -> std::io::Result<()> {
    let mut interner = TokenStringInterner::new();

    if let Err(err) = main_entry(&mut interner) {
        if let Some(err) = err.downcast_ref::<ParserError>() {
            panic!("{}", ParserErrorFormatter::new(&interner, err));
        }
        panic!("{err}");
    }

    Ok(())
}

fn main_entry(interner: &mut TokenStringInterner) -> Result<(), Box<dyn std::error::Error>> {
    let mut source = String::new();

    let args = Args::parse();
    File::open(args.input)?.read_to_string(&mut source)?;

    let tokens = Lexer::lex(interner, &source)?;

    let ast = parser::Parser::parse(interner, &tokens)?;

    println!("{}", TreeASTFormatter::new(&ast));
    println!("{}", FlatASTFormatter::new(&ast));
    
    SemanticResolver::resolve(&ast)?;

    Ok(())
}
