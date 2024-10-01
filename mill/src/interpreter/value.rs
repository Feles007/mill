use std::collections::HashMap;

use crate::ast::{Float, Identifier, Integer, Statement};

#[derive(Debug, Clone)]
pub enum Value {
	Bool(bool),
	Null,

	Integer(Integer),
	Float(Float),
	String(String),
	Array(Vec<Value>),
	Map(HashMap<HashableValue, Value>),

	Function(Vec<Identifier>, Vec<Statement>),
}
impl Value {
	pub fn try_as_hashable(self) -> Option<HashableValue> {
		Some(match self {
			Self::Null => HashableValue::Null,
			Self::Bool(b) => HashableValue::Bool(b),

			Self::Integer(i) => HashableValue::Integer(i),
			Self::String(s) => HashableValue::String(s),

			_ => return None,
		})
	}
}
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum HashableValue {
	Bool(bool),
	Null,

	Integer(Integer),
	String(String),
}
