use crate::ast::{Float, Identifier, Integer, Statement};

#[derive(Debug, Clone)]
pub enum Value {
	Bool(bool),
	Null,

	Integer(Integer),
	Float(Float),
	String(String),
	Array(Vec<Value>),
	Map,

	Function(Vec<Identifier>, Vec<Statement>),
}
