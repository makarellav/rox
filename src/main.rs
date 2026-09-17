use std;

use crate::{evalutator::evaluate, parser::parse, reader::read_source, tokernizer::tokenize};

mod error;
mod evalutator;
mod parser;
mod reader;
mod tokernizer;

fn main() {
    let mut args = std::env::args().skip(1);

    let filename = args.next().unwrap_or_else(|| {
        eprintln!("Not enough arguments. Usage: rox <filename>");
        std::process::exit(1);
    });

    let source = read_source(&filename).unwrap_or_else(|err| {
        eprintln!("Failed to read source: {}", err);
        std::process::exit(1);
    });

    println!("{}", &source.raw);

    tokenize();
    parse();
    evaluate();
}
