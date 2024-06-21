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
pub struct Lexer {
	tokens: VecDeque<Token>,
}

impl Lexer {
	pub fn new(input: &str) -> Lexer {
		let mut tokens = input
			.chars()
			.filter(|it| !it.is_ascii_whitespace())
			.map(|c| match c {
				'0'..='9' => Token::Number(Number(c)),

				'a'..='z' | 'A'..='Z' => Token::Identifier(Identifier(c)),

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

				_ => todo!(),
			})
			.collect();
		Lexer { tokens }
	}

	pub fn next(&mut self) -> Token {
		self.tokens.pop_front().unwrap_or(Token::Eof)
	}
	pub fn peek(&mut self) -> Token {
		self.tokens.front().cloned().unwrap_or(Token::Eof)
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
/*
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
pub struct Lexer {
	tokens: Vec<Token>,
}

impl Lexer {
	pub fn new(input: &str) -> Lexer {
		let mut tokens = input
			.chars()
			.filter(|it| !it.is_ascii_whitespace())
			.map(|c| match c {
				'0'..='9' => Token::Number(Number(c)),

				'a'..='z' | 'A'..='Z' => Token::Identifier(Identifier(c)),

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

				_ => todo!(),
			})
			.collect::<Vec<_>>();
		tokens.reverse();
		Lexer { tokens }
	}

	pub fn next(&mut self) -> Token {
		self.tokens.pop().unwrap_or(Token::Eof)
	}
	pub fn peek(&mut self) -> Token {
		self.tokens.last().cloned().unwrap_or(Token::Eof)
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
*/