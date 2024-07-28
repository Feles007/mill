mod error;
mod expression;
mod lexer;
mod statement;

type LineNumber = usize;

pub use error::ParseError;

use crate::ast::Ast;

pub fn parse<Source: AsRef<str>>(source: Source) -> Result<Ast, error::ParseError> {
	let mut lexer = lexer::Lexer::new(source.as_ref());
	Ok(Ast(statement::parse_file(&mut lexer)?))
}
