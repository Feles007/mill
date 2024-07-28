use crate::{
	ast::{Ast, Statement},
	interpreter::state::{StackFrame, State},
};

pub fn interpret(ast: Ast) -> Result<(), String> {
	let mut state = State::new();
	for statement in ast.0 {
		interpret_statement(statement, &mut state)?;
	}
	Ok(())
}
fn interpret_statement(statement: Statement, state: &mut State) -> Result<(), String> {
	todo!();
	Ok(())
}
