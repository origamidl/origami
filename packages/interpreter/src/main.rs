//! The Origami Definition Language interpreter is meant to read from an
//! `.origami` file and output a custom `.fold` file, which can then be
//! used by further backends to perform actions on the resulting model.

#![warn(clippy::all, clippy::pedantic, missing_docs)]

mod scanner;
mod macros;

use colored::Colorize;
use clap::Parser;
use std::fs;
use crate::scanner::{Scanner, ScannerError};

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
    let mut scanner = Scanner::new(source);
    let tokens = match scanner.scan_tokens() {
        Ok(tokens) => tokens,
        Err(error) => {
            eprintln!("{}", scanner.format_error(error));
            std::process::exit(1);
        }
    };

    tokens.iter().for_each(|t| {
        println!("{:?}", t);
    });
}
