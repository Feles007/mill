mod error;
mod expression;
mod lexer;
mod statement;

type LineNumber = usize;
type Integer = i32;
// Ideally we'd just parse integer literals and have negation done at runtime when
// evaluating the expression, but signed::min.abs > signed::max so it'd overflow.
// Instead, we parse it as an unsigned which has a bigger range then do a post-process
// step on the AST after parsing.
type UInteger = u32;
type Float = f64;

pub use error::ParseError;
pub use statement::Statement;
pub use expression::Expression;
pub use lexer::Identifier;

pub fn parse<Source: AsRef<str>>(
	source: Source,
) -> Result<Vec<statement::Statement>, error::ParseError> {
	let mut lexer = lexer::Lexer::new(source.as_ref());
	statement::parse_file(&mut lexer)
}
