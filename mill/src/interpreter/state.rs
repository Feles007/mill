use std::collections::HashMap;

use crate::{
	ast::{Expression, Identifier},
	interpreter::{error::InterpreterError, value::Value},
};

#[derive(Debug)]
pub struct State {
	pub stack: Vec<Scope>,
}
impl State {
	pub fn new() -> Self { Self { stack: Vec::new() } }

	pub fn push(&mut self) { self.stack.push(Scope::new()); }

	pub fn pop(&mut self) { self.stack.pop().unwrap(); }

	pub fn current_scope(&mut self) -> &mut Scope {
		let len = self.stack.len();
		&mut self.stack[len - 1]
	}
}

#[derive(Debug)]
pub struct Scope {
	pub variables: HashMap<Identifier, Value>,
}

impl Scope {
	pub fn new() -> Self {
		Self {
			variables: HashMap::new(),
		}
	}
}

#[derive(Debug)]
pub enum ControlFlow {
	Normal,
	Break,
	Continue,
	Return(Value),
}
