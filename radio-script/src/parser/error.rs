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
	// Lexer
	UnexpectedCharacter(char),
	NonAsciiByte(u8),

	// Parser
	UnexpectedToken {
		expected: &'static str,
		found: Token,
	},
}

impl Display for ParseError {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
		write!(
			f,
			"{}:{} - ",
			self.source_file
				.as_ref()
				.map(String::as_str)
				.unwrap_or("<source>"),
			self.line_number
		)?;

		let indent = "    ";

		match self.kind {
			ParseErrorKind::UnexpectedCharacter(c) => {
				writeln!(f, "Unexpected character '{c}'")?;
			},
			ParseErrorKind::NonAsciiByte(b) => {
				writeln!(f, "Non ASCII byte: 0x{b:X?}")?;
				writeln!(
					f,
					"{indent}note: This is allowed in comments and string literals"
				);
			},
			ParseErrorKind::UnexpectedToken {
				expected,
				ref found,
			} => {
				writeln!(f, "Expected {expected}, found {found}")?;
			},
			_ => todo!(),
		}

		Ok(())
	}
}

impl Error for ParseError {}
