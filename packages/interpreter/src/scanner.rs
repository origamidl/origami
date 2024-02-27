use crate::bail;
use thiserror::Error;

#[derive(Debug)]
pub struct Scanner<'a> {
  source: &'a str
}

#[derive(Debug)]
pub enum Token {
  A,
}

#[derive(Debug)]
pub struct ErrorContext<'a> {
  pub line: &'a str,
  pub range: std::ops::Range<usize>,
  pub line_number: u32,
}

/// All errors that can be thrown by the scanner, using the `anyhow` crate. Every
/// error includes its full context.
#[derive(Error, Debug)]
pub enum ScannerError<'a> {
  /// An unexpected token was encountered.
  #[error("An unexpected token was encountered")]
  UnexpectedToken(ErrorContext<'a>)
}

impl<'a> Scanner<'a> {
  pub fn new(source: &'a str) -> Self {
    Scanner {
      source: source
    }
  }

  pub fn scan_tokens(&self) -> Result<Vec<Token>, ScannerError> {
    if true {
      bail!(ScannerError::UnexpectedToken(ErrorContext {
        line: "fold .b to .d",
        range: (5..7),
        line_number: 5,
      }))
    }

    Ok(vec![Token::A])
  }
}