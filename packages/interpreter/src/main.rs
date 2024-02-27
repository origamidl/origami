//! The Origami Definition Language interpreter is meant to read from an
//! `.origami` file and output a custom `.fold` file, which can then be
//! used by further backends to perform actions on the resulting model.

#![warn(clippy::all, clippy::pedantic, missing_docs)]

mod lexer;
mod macros;

use crate::lexer::Token;
use logos::Logos;
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(about = "Interpreter of the Origami Definition Language")]
#[command(version, about, long_about = None)]
struct Args {
  /// The .origami file to interpret
  input_file: String,

  /// The path to write the output to
  #[arg(short, long)]
  output: Option<String>,
}

fn main() {
  let args = Args::parse();

  // Read from file
  let contents = fs::read_to_string(args.input_file)
    .expect("Could not locate the input file.");

  run(&contents);
}

fn run(source: &str) {
  let scanner = Token::lexer(source);

  scanner //.filter_map(|op| op.ok())
    .for_each(|token| {
      println!("{:?}", token);
    });
}
