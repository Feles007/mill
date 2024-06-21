use crate::error::{ParseError, ParseErrorKind};
use crate::LineNumber;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier(char);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Number(char);


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
	Identifier(Identifier),
	Number(Number),
	Symbol(Symbol),
	Eof,
}

#[derive(Debug)]
pub struct Lexer<'a> {
	source: &'a [u8],
	current_index: usize,
	current_token: Option<Token>,
	line_number: LineNumber,
}

impl<'a> Lexer<'a> {
	pub fn new(input: &'a str) -> Result<Self, ParseError> {
		Ok(Lexer { source: input.as_bytes(), current_index: 0, current_token: None, line_number: 1 })
	}

	pub fn next(&mut self) -> Result<Token, ParseError> {
		if let Some(token) = self.current_token.take() {
			Ok(token)
		} else {
			self.parse_token()
		}
	}
	pub fn peek(&mut self) -> Result<Token, ParseError> {
		if let None = self.current_token {
			self.current_token = Some(self.parse_token()?);
		}
		Ok(self.current_token.as_ref().cloned().unwrap())
	}
	pub fn error(&self, kind: ParseErrorKind) -> ParseError {
		ParseError {
			source_file: None,
			line_number: self.line_number,
			kind,
		}
	}
	fn parse_token(&mut self) -> Result<Token, ParseError> {
		let mut token_start = self.current_index;
		let mut token_end = 1;

		loop {
			if token_start == self.source.len() {
				return Ok(Token::Eof);
			}
			let byte = self.source[token_start];
			if !byte.is_ascii() {
				return Err(self.error(ParseErrorKind::NonAsciiByte(byte)));
			}
			let ret = match byte as char {
				'\n' => {
					self.line_number += 1;
					token_start += 1;
					continue;
				}
				c if c.is_ascii_whitespace() => {
					token_start += 1;
					continue;
				},
				'#' => {
					token_start += 1;
					while self.source.get(token_start).map(|c| *c != b'\n').unwrap_or(false) {
						token_start += 1;
					}
					self.line_number += 1;
					continue;
				}


				c if c.is_ascii_digit() => {
					Token::Number(Number(c))
				}
				c if c.is_ascii_alphabetic() => {
					Token::Identifier(Identifier(c))
				}
				

				';' => Token::Symbol(Symbol::Semicolon),
				',' => Token::Symbol(Symbol::Comma),
				'.' => Token::Symbol(Symbol::Dot),
			
				'(' => Token::Symbol(Symbol::ParenLeft),
				')' => Token::Symbol(Symbol::ParenRight),
				'{' => Token::Symbol(Symbol::CurlyLeft),
				'}' => Token::Symbol(Symbol::CurlyRight),
				'[' => Token::Symbol(Symbol::SquareLeft),
				']' => Token::Symbol(Symbol::SquareRight),
			
				'+' => Token::Symbol(Symbol::Add),
				'-' => Token::Symbol(Symbol::Sub),
				'*' => Token::Symbol(Symbol::Mul),
				'/' => Token::Symbol(Symbol::Div),
				'%' => Token::Symbol(Symbol::Mod),
			
				'=' | '!' | '<' | '>'
				if self.source.get(token_start + 1).map(|c| *c == b'=').unwrap_or(false) => {
					token_end += 1;
					match self.source[token_start] as char {
						'=' => Token::Symbol(Symbol::EqEq),
						'!' => Token::Symbol(Symbol::NoEq),
						'<' => Token::Symbol(Symbol::LtEq),
						'>' => Token::Symbol(Symbol::GtEq),
						_ => unreachable!(),
					}
				}

				'=' => Token::Symbol(Symbol::Eq),
				'!' => Token::Symbol(Symbol::No),
				'<' => Token::Symbol(Symbol::Lt),
				'>' => Token::Symbol(Symbol::Gt),

				c => return Err(self.error(ParseErrorKind::UnexpectedCharacter(c)))
			};

			self.current_index = token_start + token_end;

			return Ok(ret);
		}
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Symbol {
	Semicolon,
	Comma,
	Dot,

	ParenLeft,
	ParenRight,
	CurlyLeft,
	CurlyRight,
	SquareLeft,
	SquareRight,

	Add,
	Sub,
	Mul,
	Div,
	Mod,

	EqEq,
	Eq,
	NoEq,
	No,
	LtEq,
	Lt,
	GtEq,
	Gt,
}
impl Symbol {
	pub fn prefix_bp(self) -> Option<((), u8)> {
		Some(match self {
			Self::Add | Self::Sub => ((), 9),
			_ => return None,
		})
	}
	pub fn postfix_bp(self) -> Option<(u8, ())> {
		Some(match self {
			Self::SquareLeft | Self::ParenLeft => (11, ()),
			_ => return None,
		})
	}
	pub fn infix_bp(self) -> Option<(u8, u8)> {
		Some(match self {
			Self::Add | Self::Sub => (5, 6),
			Self::Mul | Self::Div | Self::Mod => (7, 8),
			Self::Dot => (14, 13),
			_ => return None,
		})
	}
}