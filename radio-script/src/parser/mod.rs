mod error;
mod expression;
mod lexer;
mod statement;

use crate::ast::Ast;

pub type LineNumber = usize;
pub use error::{ParseError, ParseErrorKind};

pub fn parse<Source: AsRef<str>>(source: Source) -> Result<Ast, error::ParseError> {
	let mut lexer = lexer::Lexer::new(source.as_ref());
	Ok(Ast(statement::parse_file(&mut lexer)?))
}
