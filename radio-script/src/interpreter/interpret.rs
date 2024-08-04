use crate::{
	ast::{Ast, BinaryOperation, Expression, Identifier, Lvalue, Statement, UnaryOperation},
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
