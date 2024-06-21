use crate::LineNumber;

pub struct ParseError {
	pub line: LineNumber,
	pub kind: ParseErrorKind,
}
pub enum ParseErrorKind {
	NonAsciiChar,
	EndOfToken,
}