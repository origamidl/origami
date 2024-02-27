use logos::{Lexer, Logos, Skip};
use rug::Rational;

fn rational(lex: &mut Lexer<Token>) -> Option<Rational> {
  let slice = lex.slice();
  Rational::from_str_radix(slice, 10).ok()
}

fn newline_callback(lex: &mut Lexer<Token>) -> Skip {
  lex.extras.0 += 1;
  lex.extras.1 = lex.span().end;
  Skip
}

#[derive(Logos, Debug, PartialEq)]
#[logos(extras = (usize, usize))]
#[logos(skip r"[ \t\f\r]+")] // Skip whitespace
#[logos(skip r";.+\n?")] // Skip comments
pub enum Token {
  // We split expressions at newlines, so newlines explicitly are tokens too
  #[token("\n", newline_callback)]
  Newline,

  #[token("(")]
  LeftParen,

  #[token(")")]
  RightParen,

  #[token(",")]
  Comma,

  #[token(".")]
  PointPrefix,

  #[token("--")]
  EdgePrefix,

  #[token(":")]
  Colon,

  #[token("@")]
  AtSymbol,

  #[token("$")]
  DollarSign,

  #[token("_")]
  Underscore,

  #[token("__")]
  DoubleUnderscore,

  #[token("->")]
  Arrow,

  #[regex(r"[a-zA-Z0-9][a-zA-Z0-9_\-]*", |lex| lex.slice().to_owned(), priority = 2)]
  Identifier(String),

  #[regex(r"[0-9]+/[1-9]+", rational)]
  Rational(Rational),

  // Keywords
  #[token("fold")]
  Fold,

  #[token("unfold")]
  Unfold,

  #[token("to")]
  To,

  #[token("onto")]
  Onto,

  #[token("via")]
  Via,

  #[token("pinch")]
  Pinch,

  #[token("at")]
  At,

  #[token("apply")]
  Apply,

  #[token("flip")]
  Flip,

  #[token("along")]
  Along,

  #[token("rotate")]
  Rotate,
}
