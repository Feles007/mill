use crate::{
	ast::{Ast, BinaryOperation, Expression, Identifier, Lvalue, Statement, UnaryOperation},
	interpreter::{
		error::InterpreterError,
		state::{ControlFlow, Scope, State},
		value::Value,
	},
};

pub fn interpret(ast: Ast) -> Result<(), InterpreterError> {
	let mut state = State::new();
	state.push();
	for statement in ast.0 {
		match interpret_statement(&mut state, statement)? {
			ControlFlow::Normal => {},
			c => panic!("Upward control flow reached top level"),
		}
	}
	dbg!(state);
	Ok(())
}
fn interpret_statement(
	state: &mut State,
	statement: Statement,
) -> Result<ControlFlow, InterpreterError> {
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
		Statement::Block { body } => {
			state.push();
			for statement in body {
				match interpret_statement(state, statement)? {
					ControlFlow::Normal => {},
					c => return Ok(c),
				}
			}
			state.pop();
		},
		Statement::If { branches } => {
			for (condition, body) in branches {
				let condition = match evaluate_expression(state, condition)? {
					Value::Bool(b) => b,
					_ => return Err(InterpreterError::ExpectedBool),
				};
				if condition {
					state.push();
					for statement in body {
						match interpret_statement(state, statement)? {
							ControlFlow::Normal => {},
							c => {
								state.pop();
								return Ok(c);
							},
						}
					}
					state.pop();
					break;
				}
			}
		},
		Statement::Return(expression) => return Ok(ControlFlow::Return(evaluate_expression(state, expression)?)),
		Statement::Break => return Ok(ControlFlow::Break),
		Statement::Continue => return Ok(ControlFlow::Continue),
		Statement::Loop { body } => {
			let start_len = state.stack.len();
			'main_loop: loop {
				let local_body = body.clone();
				state.push();
				for statement in local_body {
					match interpret_statement(state, statement)? {
						ControlFlow::Normal => {},
						ControlFlow::Return(e) => return Ok(ControlFlow::Return(e)),
						ControlFlow::Break => break 'main_loop,
						ControlFlow::Continue => break,
					}
				}
				state.pop();
			}
			state.pop();
			assert_eq!(start_len, state.stack.len());
		},
		Statement::While { condition, body } => {
			let start_len = state.stack.len();
			'main_loop: loop {
				let local_condition = condition.clone();
				let local_condition = match evaluate_expression(state, local_condition)? {
					Value::Bool(b) => b,
					_ => return Err(InterpreterError::ExpectedBool),
				};
				let local_body = body.clone();

				state.push();
				for statement in local_body {
					match interpret_statement(state, statement)? {
						ControlFlow::Normal => {},
						ControlFlow::Return(e) => return Ok(ControlFlow::Return(e)),
						ControlFlow::Break => break 'main_loop,
						ControlFlow::Continue => break,
					}
				}
				state.pop();
			}
			assert_eq!(start_len, state.stack.len());
		},
		Statement::For {
			loop_var,
			iterator,
			body,
		} => todo!(),
	}
	Ok(ControlFlow::Normal)
}
fn evaluate_expression(
	state: &mut State,
	expression: Expression,
) -> Result<Value, InterpreterError> {
	Ok(match expression {
		Expression::True => Value::Bool(true),
		Expression::False => Value::Bool(false),
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

		Expression::UnaryOperation(operand, operation) => {
			unary_operation(evaluate_expression(state, *operand)?, operation)?
		},
		Expression::BinaryOperation(operands, operation) => {
			let [lhs, rhs] = *operands;
			let lhs = evaluate_expression(state, lhs)?;
			let rhs = evaluate_expression(state, rhs)?;
			binary_operation(lhs, rhs, operation)?
		},
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
fn unary_operation(operand: Value, operation: UnaryOperation) -> Result<Value, InterpreterError> {
	Ok(match (operand, operation) {
		(Value::Bool(true), UnaryOperation::Not) => Value::Bool(false),
		(Value::Bool(false), UnaryOperation::Not) => Value::Bool(true),

		(Value::Integer(i), UnaryOperation::Neg) => Value::Integer(-i),
		(Value::Float(f), UnaryOperation::Neg) => Value::Float(-f),

		_ => return Err(InterpreterError::UnsupportedOperation),
	})
}
fn binary_operation(
	lhs: Value,
	rhs: Value,
	operation: BinaryOperation,
) -> Result<Value, InterpreterError> {
	use BinaryOperation as O;
	use Value as V;

	Ok(match (lhs, rhs, operation) {
		//
		// Integer ops
		//
		(V::Integer(lhs), V::Integer(rhs), O::Add) => V::Integer(lhs + rhs),
		(V::Integer(lhs), V::Integer(rhs), O::Sub) => V::Integer(lhs - rhs),
		(V::Integer(lhs), V::Integer(rhs), O::Mul) => V::Integer(lhs * rhs),
		(V::Integer(lhs), V::Integer(rhs), O::Div) => V::Integer(lhs / rhs),
		(V::Integer(lhs), V::Integer(rhs), O::Mod) => V::Integer(lhs % rhs),

		//
		// Float ops
		//
		(V::Float(lhs), V::Float(rhs), O::Add) => V::Float(lhs + rhs),
		(V::Float(lhs), V::Float(rhs), O::Sub) => V::Float(lhs - rhs),
		(V::Float(lhs), V::Float(rhs), O::Mul) => V::Float(lhs * rhs),
		(V::Float(lhs), V::Float(rhs), O::Div) => V::Float(lhs / rhs),

		//
		// Comparison ops
		//
		(V::Integer(lhs), V::Integer(rhs), O::Eq) => V::Bool(lhs == rhs),
		(V::Integer(lhs), V::Integer(rhs), O::NoEq) => V::Bool(lhs != rhs),

		(V::Integer(lhs), V::Integer(rhs), O::Lt) => V::Bool(lhs < rhs),
		(V::Integer(lhs), V::Integer(rhs), O::LtEq) => V::Bool(lhs <= rhs),
		(V::Integer(lhs), V::Integer(rhs), O::Gt) => V::Bool(lhs > rhs),
		(V::Integer(lhs), V::Integer(rhs), O::GtEq) => V::Bool(lhs >= rhs),

		(V::Float(lhs), V::Float(rhs), O::Lt) => V::Bool(lhs < rhs),
		(V::Float(lhs), V::Float(rhs), O::LtEq) => V::Bool(lhs <= rhs),
		(V::Float(lhs), V::Float(rhs), O::Gt) => V::Bool(lhs > rhs),
		(V::Float(lhs), V::Float(rhs), O::GtEq) => V::Bool(lhs >= rhs),

		//
		// Bool ops
		//
		(V::Bool(lhs), V::Bool(rhs), O::And) => V::Bool(lhs && rhs),
		(V::Bool(lhs), V::Bool(rhs), O::Or) => V::Bool(lhs || rhs),

		//
		// Index
		//
		(_, _, O::Index) => todo!(),

		_ => return Err(InterpreterError::UnsupportedOperation),
	})
}
