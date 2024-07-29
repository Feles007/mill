use crate::ast::{Float, Integer};

#[derive(Debug, Clone)]
pub enum Value {
	True,
	False,
	Null,

	Integer(Integer),
	Float(Float),
	String(String),
	Array(Vec<Value>),
	Map,

	Function,
}
