use std::{
	collections::VecDeque,
	fmt::{self, Display, Formatter},
	str::{self, FromStr},
};

use crate::{
	error::{ParseError, ParseErrorKind},
	Float, Integer, LineNumber, UInteger,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier(pub String);

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
	Identifier(Identifier),
	Integer(Integer),
	Float(Float),
	String(String),

	Symbol(Symbol),

	True,
	False,
	Null,
	Fn,
	Return,
	Let,
	If,
	Else,
	Loop,
	Break,
	Continue,
	For,
	In,

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
	pub fn new(input: &'a str) -> Self {
		Lexer {
			source: input.as_bytes(),
			current_index: 0,
			current_token: None,
			line_number: 1,
		}
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
			let byte = match self.source.get(token_start) {
				Some(b) => *b,
				None => return Ok(Token::Eof),
			};
			if !byte.is_ascii() {
				return Err(self.error(ParseErrorKind::NonAsciiByte(byte)));
			}
			let ret = match byte as char {
				'\n' => {
					self.line_number += 1;
					token_start += 1;
					continue;
				},
				c if c.is_ascii_whitespace() => {
					token_start += 1;
					continue;
				},
				'#' => {
					token_start += 1;
					while self
						.source
						.get(token_start)
						.map(|c| *c != b'\n')
						.unwrap_or(false)
					{
						token_start += 1;
					}
					self.line_number += 1;
					continue;
				},

				c if c.is_ascii_digit() => {
					while self
						.source
						.get(token_start + token_end)
						.map(|c| c.is_ascii_digit())
						.unwrap_or(false)
					{
						token_end += 1;
					}
					let mut float = false;
					if self.source.get(token_start + token_end + 1).is_some()
						&& self.source[token_start + token_end] == b'.'
						&& self.source[token_start + token_end + 1].is_ascii_digit()
					{
						token_end += 2;
						while self
							.source
							.get(token_start + token_end)
							.map(|c| c.is_ascii_digit())
							.unwrap_or(false)
						{
							token_end += 1;
						}
						float = true;
					}
					let string =
						&str::from_utf8(&self.source[token_start..(token_start + token_end)])
							.unwrap();

					if float {
						Token::Float(Float::from_str(string).unwrap())
					} else {
						Token::Integer(UInteger::from_str(string).unwrap() as Integer)
					}
				},
				c if c.is_ascii_alphabetic() || c == '_' => {
					while self
						.source
						.get(token_start + token_end)
						.map(|c| c.is_ascii_alphanumeric() || *c == b'_')
						.unwrap_or(false)
					{
						token_end += 1;
					}

					let identifier =
						str::from_utf8(&self.source[token_start..(token_start + token_end)])
							.unwrap()
							.to_owned();

					match identifier.as_str() {
						"true" => Token::True,
						"false" => Token::False,
						"null" => Token::Null,
						"fn" => Token::Fn,
						"return" => Token::Return,
						"let" => Token::Let,
						"if" => Token::If,
						"else" => Token::Else,
						"loop" => Token::Loop,
						"break" => Token::Break,
						"continue" => Token::Continue,
						"for" => Token::For,
						"in" => Token::In,
						_ => Token::Identifier(Identifier(identifier)),
					}
				},
				'"' => {
					while self
						.source
						.get(token_start + token_end)
						.map(|c| *c != b'"')
						.unwrap_or(false)
					{
						token_end += 1;
					}
					let string =
						str::from_utf8(&self.source[(token_start + 1)..(token_start + token_end)])
							.unwrap();
					token_end += 1;
					Token::String(string.to_owned())
				},

				':' => Token::Symbol(Symbol::Colon),
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
					if self
						.source
						.get(token_start + 1)
						.map(|c| *c == b'=')
						.unwrap_or(false) =>
				{
					token_end += 1;
					match self.source[token_start] as char {
						'=' => Token::Symbol(Symbol::EqEq),
						'!' => Token::Symbol(Symbol::NoEq),
						'<' => Token::Symbol(Symbol::LtEq),
						'>' => Token::Symbol(Symbol::GtEq),
						_ => unreachable!(),
					}
				},

				'=' => Token::Symbol(Symbol::Eq),
				'!' => Token::Symbol(Symbol::No),
				'<' => Token::Symbol(Symbol::Lt),
				'>' => Token::Symbol(Symbol::Gt),

				'&' if self
					.source
					.get(token_start + 1)
					.map(|c| *c == b'&')
					.unwrap_or(false) =>
				{
					token_end += 1;
					Token::Symbol(Symbol::And)
				},
				'|' if self
					.source
					.get(token_start + 1)
					.map(|c| *c == b'|')
					.unwrap_or(false) =>
				{
					token_end += 1;
					Token::Symbol(Symbol::Or)
				},

				c => return Err(self.error(ParseErrorKind::UnexpectedCharacter(c))),
			};

			self.current_index = token_start + token_end;

			return Ok(ret);
		}
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Symbol {
	Colon,
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

	And,
	Or,
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
			Self::SquareLeft | Self::ParenLeft => (10, ()),
			_ => return None,
		})
	}

	pub fn infix_bp(self) -> Option<(u8, u8)> {
		Some(match self {
			Self::And | Self::Or => (1, 2),
			Self::EqEq | Self::NoEq | Self::Lt | Self::LtEq | Self::Gt | Self::GtEq => (3, 4),
			Self::Add | Self::Sub => (5, 6),
			Self::Mul | Self::Div | Self::Mod => (7, 8),
			Self::Dot => (12, 11),
			_ => return None,
		})
	}
}
impl Display for Token {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
		match self {
			Self::True => write!(f, "keyword 'true'"),
			Self::False => write!(f, "keyword 'false'"),
			Self::Null => write!(f, "keyword 'null'"),
			Self::Fn => write!(f, "keyword 'fn'"),
			Self::Return => write!(f, "keyword 'return'"),
			Self::Let => write!(f, "keyword 'let'"),
			Self::If => write!(f, "keyword 'if'"),
			Self::Else => write!(f, "keyword 'else'"),
			Self::Loop => write!(f, "keyword 'loop'"),
			Self::Break => write!(f, "keyword 'break'"),
			Self::Continue => write!(f, "keyword 'continue'"),
			Self::For => write!(f, "keyword 'for'"),
			Self::In => write!(f, "keyword 'in'"),

			Self::Identifier(i) => write!(f, "identifier '{}'", i.0),
			Self::Integer(n) => write!(f, "integer '{}'", n),
			Self::Float(n) => write!(f, "float '{}'", n),
			Self::String(s) => write!(f, "string \"{}\"", s),

			Self::Eof => write!(f, "end of file"),

			Self::Symbol(s) => write!(
				f,
				"symbol '{}'",
				match s {
					Symbol::Colon => ":",
					Symbol::Semicolon => ";",
					Symbol::Comma => ",",
					Symbol::Dot => ".",

					Symbol::ParenLeft => "(",
					Symbol::ParenRight => ")",
					Symbol::CurlyLeft => "{",
					Symbol::CurlyRight => "}",
					Symbol::SquareLeft => "[",
					Symbol::SquareRight => "]",

					Symbol::Add => "+",
					Symbol::Sub => "-",
					Symbol::Mul => "*",
					Symbol::Div => "/",
					Symbol::Mod => "%",

					Symbol::EqEq => "==",
					Symbol::Eq => "=",
					Symbol::NoEq => "!=",
					Symbol::No => "!",
					Symbol::LtEq => "<=",
					Symbol::Lt => "<",
					Symbol::GtEq => ">=",
					Symbol::Gt => ">",

					Symbol::And => "&&",
					Symbol::Or => "||",
				}
			),
		}
	}
}
