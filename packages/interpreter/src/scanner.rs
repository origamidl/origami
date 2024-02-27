use crate::bail;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone)]
pub struct Token {
  token_type: TokenType,
  position: std::ops::Range<usize>,
}

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
  LeftParen,
  RightParen,
}

use thiserror::Error;

#[derive(Debug)]
pub struct ErrorContext {
  pub range: std::ops::Range<usize>,
}

#[derive(Error, Debug)]
pub enum ScannerError {
  /// An unexpected token was encountered.
  #[error("An unexpected token was encountered.")]
  UnexpectedToken(ErrorContext)
}

#[derive(Debug)]
pub struct Scanner<'a> {
  token_list: Vec<Token>,
  source: Vec<&'a str>,
  start: usize,
  current: usize,
}

struct ScannerState {
  token_list: Vec<Token>,
  start: usize,
  current: usize,
}

impl<'a> Scanner<'a> {
  pub fn new(source: &'a str) -> Self {
    Scanner {
      token_list: Vec::new(),
      source: source.graphemes(true).collect(),
      start: 0,
      current: 0,
    }
  }

  pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, ScannerError> {
    // let mut current_lexeme: &mut str = "".to_ref();
    while !self.finished() {
      self.start = self.current;

      self.scan_token()?;
    }

    // bail!(ScannerError::UnexpectedToken(ErrorContext {
    //   line: "fold .b to .d",
    //   range: (5..7),
    //   line_number: 5,
    // }));

    Ok(&self.token_list)
  }

  fn scan_token(&mut self) -> Result<bool, ScannerError> {
    let c = self.advance();

    match c {
      "(" => self.add_token(TokenType::LeftParen),
      ")" => self.add_token(TokenType::RightParen),

      _ => bail!(ScannerError::UnexpectedToken(ErrorContext {
        range: self.current-1..self.current,
      })),
    };

    Ok(true)
  }

  fn add_token(&mut self, token_type: TokenType) {
    self.token_list.push(Token {
      token_type: token_type,
      position: self.start..self.current
    });
  }

  fn advance(&mut self) -> &'a str {
    self.current += 1;
    self.source[self.current - 1]
  }

  fn finished(&self) -> bool {
    self.current >= self.source.len()
  }

  pub fn format_error(&self, error: ScannerError) -> String {
    use colored::Colorize;

    let mut formatted = format!("{}: {}\n", "Error".bold(), error);

    match error {
      ScannerError::UnexpectedToken(context) => {
        let line_start = self.source[0..context.range.start].iter().rposition(|&c| c == "\n").unwrap_or(0);
        let line_end = self.source[context.range.start..].iter().position(|&c| c == "\n").unwrap_or(self.source.len());

        let first = self.source[line_start..context.range.start].concat();
        let second = self.source[context.range.start..context.range.end].concat();
        let third = self.source[context.range.end..line_end].concat();

        let line = format!("   {} | {}{}{}",
          1,
          first.red(),
          second.red().underline(),
          third.red(),
        );
        
        formatted.push_str(&line);
      }
    }
    formatted
  }
}
