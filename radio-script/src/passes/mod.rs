use crate::parser::Ast;

pub trait AstPass {
	type Error;
	const KIND: AstPassKind;
	fn run(self, ast: Ast) -> Result<Ast, Self::Error>;
}
#[derive(Debug)]
pub enum AstPassKind {
	Validation,
	Optimization,
}
