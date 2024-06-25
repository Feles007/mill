use std::{
	error::Error,
	fmt::{self, Display, Formatter},
};

use crate::parser::{lexer::Token, LineNumber};

#[derive(Debug)]
pub struct ParseError {
	pub source_file: Option<String>,
	pub line_number: LineNumber,
	pub kind: ParseErrorKind,
}
#[derive(Debug)]
pub enum ParseErrorKind {
	UnexpectedCharacter(char),
	NonAsciiByte(u8),
	UnexpectedToken {
		expected: &'static str,
		found: Token,
	},
	InvalidIntegerLiteral {
		message: String,
	},
}

impl Display for ParseError {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
		write!(
			f,
			"{}:{} - ",
			self.source_file.as_deref().unwrap_or("<source>"),
			self.line_number
		)?;

		match self.kind {
			ParseErrorKind::UnexpectedCharacter(c) => {
				writeln!(f, "Unexpected character '{c}'")?;
			},
			ParseErrorKind::NonAsciiByte(b) => {
				writeln!(f, "Non ASCII byte: 0x{b:X?}")?;
				writeln!(
					f,
					"    note: This is allowed in comments and string literals"
				)?;
			},
			ParseErrorKind::UnexpectedToken {
				expected,
				ref found,
			} => {
				writeln!(f, "Expected {expected}, found {found}")?;
			},
			ParseErrorKind::InvalidIntegerLiteral { ref message } => {
				writeln!(f, "{message}")?;
				writeln!(
					f,
					"    note: integer literals must be in the range [-2147483648, 2147483647]"
				)?;
			},
		}

		Ok(())
	}
}

impl Error for ParseError {}
