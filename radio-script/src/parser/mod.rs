mod error;
mod expression;
mod lexer;
mod statement;

type LineNumber = usize;
type Integer = i32;
type Float = f64;

pub use error::ParseError;
pub use expression::Expression;
pub use lexer::Identifier;
pub use statement::Statement;

pub fn parse<Source: AsRef<str>>(source: Source) -> Result<Ast, error::ParseError> {
	let mut lexer = lexer::Lexer::new(source.as_ref());
	Ok(Ast(statement::parse_file(&mut lexer)?))
}

#[derive(Debug)]
pub struct Ast(pub(crate) Vec<statement::Statement>);
