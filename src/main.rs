use crate::{evalutator::evaluate, parser::parse, reader::read_source, tokernizer::tokenize};

mod evalutator;
mod parser;
mod reader;
mod tokernizer;

fn main() {
    read_source("hello.txt");
    tokenize();
    parse();
    evaluate();
}
