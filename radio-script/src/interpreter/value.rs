use crate::ast::{Float, Integer};

#[derive(Debug, Clone)]
pub enum Value {
	Bool(bool),
	Null,

	Integer(Integer),
	Float(Float),
	String(String),
	Array(Vec<Value>),
	Map,

	Function,
}
