use crate::{
	ast::{Ast, Expression, Identifier, Lvalue, Statement},
	interpreter::{
		error::InterpreterError,
		state::{Scope, State},
		value::Value,
	},
};

pub fn interpret(ast: Ast) -> Result<(), InterpreterError> {
	let mut state = State::new();
	state.push();
	for statement in ast.0 {
		interpret_statement(&mut state, statement)?;
	}
	dbg!(state);
	Ok(())
}
fn interpret_statement(state: &mut State, statement: Statement) -> Result<(), InterpreterError> {
	match statement {
		Statement::Declaration { name, initializer } => {
			if state.current_scope().variables.contains_key(&name) {
				return Err(InterpreterError::Redeclaration);
			}

			let value = evaluate_expression(state, initializer)?;

			state.current_scope().variables.insert(name, value);
		},
		Statement::Assignment { lvalue, value } => {
			let value = evaluate_expression(state, value)?;
			let lvalue: &mut Value = match lvalue {
				Lvalue::Identifier(identifier) => lookup_mut(state, &identifier)?,
				_ => todo!(),
			};
			*lvalue = value;
		},
		Statement::UnusedExpression(expression) => _ = evaluate_expression(state, expression)?,
		Statement::Return(expression) => todo!(),
		Statement::Break => todo!(),
		Statement::Continue => todo!(),
		Statement::Loop { body } => todo!(),
		Statement::For {
			loop_var,
			iterator,
			body,
		} => todo!(),
		Statement::While { condition, body } => todo!(),
		Statement::If {
			condition,
			body,
			else_ifs,
			else_body,
		} => todo!(),
		Statement::Block { body } => todo!(),
	}
	Ok(())
}
fn evaluate_expression(
	state: &mut State,
	expression: Expression,
) -> Result<Value, InterpreterError> {
	Ok(match expression {
		Expression::True => Value::True,
		Expression::False => Value::False,
		Expression::Null => Value::Null,

		Expression::Identifier(identifier) => lookup(state, &identifier)?.clone(),
		Expression::Integer(integer) => Value::Integer(integer),
		Expression::Float(float) => Value::Float(float),
		Expression::String(string) => Value::String(string),
		Expression::Array(initializer) => Value::Array(
			initializer
				.into_iter()
				.map(|e| evaluate_expression(state, e))
				.collect::<Result<_, _>>()?,
		),
		Expression::Map(initializer) => todo!(),

		Expression::Function(parameters, body) => todo!(),
		Expression::Call(function, arguments) => todo!(),
		Expression::Member(value, member) => todo!(),

		Expression::UnaryOperation(operand, unary_operation) => todo!(),
		Expression::BinaryOperation(operands, binary_operation) => todo!(),
	})
}
fn lookup<'a>(state: &'a State, identifier: &Identifier) -> Result<&'a Value, InterpreterError> {
	for scope in state.stack.iter().rev() {
		if let Some(value) = scope.variables.get(identifier) {
			return Ok(value);
		}
	}
	Err(InterpreterError::UnknownIdentifier)
}
fn lookup_mut<'a>(
	state: &'a mut State,
	identifier: &Identifier,
) -> Result<&'a mut Value, InterpreterError> {
	for scope in state.stack.iter_mut().rev() {
		if let Some(value) = scope.variables.get_mut(identifier) {
			return Ok(value);
		}
	}
	Err(InterpreterError::UnknownIdentifier)
}
