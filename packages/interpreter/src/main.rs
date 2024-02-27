//! The Origami Definition Language interpreter is meant to read from an
//! `.origami` file and output a custom `.fold` file, which can then be
//! used by further backends to perform actions on the resulting model.

#![warn(clippy::all, clippy::pedantic, missing_docs)]

mod scanner;
mod macros;

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
    let scanner = Scanner::new(source);
    let tokens = match scanner.scan_tokens() {
        Ok(tokens) => tokens,
        Err(error) => {
            print_error(error);
            std::process::exit(1);
        }
    };

    tokens.iter().for_each(|t| {
        println!("{:?}", t);
    });
}

fn print_error(err: ScannerError) {
    use ansi_term::{Style, Colour::*};

    eprintln!("{}: {}\n", Style::new().bold().paint("Error"), err);

    match err {
        ScannerError::UnexpectedToken(context) => {
            let first: &str = &context.line[0..context.range.start];
            let second: &str = &context.line[context.range.start..context.range.end];
            let third: &str = &context.line[context.range.end..];

            eprintln!("   {} | {}{}{}",
                context.line_number,
                Red.paint(first),
                Red.underline().paint(second),
                Red.paint(third)
            );

            eprintln!("       {}^ -- Here", " ".repeat(context.range.start));
        },
        _ => {},
    }
}
