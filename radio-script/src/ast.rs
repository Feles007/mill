pub type Integer = i32;
pub type Float = f64;

#[derive(Debug)]
pub struct Ast(pub(crate) Vec<Statement>);

#[derive(Debug)]
pub enum Statement {
	Declaration {
		name: Identifier,
		initializer: Expression,
	},
	Assignment {
		lvalue: Expression,
		value: Expression,
	},
	UnusedExpression(Expression),
	Return(Expression),
	Break,
	Continue,
	Loop {
		body: Vec<Statement>,
	},
	For {
		loop_var: Identifier,
		iterator: Expression,
		body: Vec<Statement>,
	},
	While {
		condition: Expression,
		body: Vec<Statement>,
	},
	If {
		condition: Expression,
		body: Vec<Statement>,
		else_ifs: Vec<(Expression, Vec<Statement>)>,
		else_body: Vec<Statement>,
	},
	Block {
		body: Vec<Statement>,
	},
}

#[derive(Debug)]
pub enum Expression {
	True,
	False,
	Null,

	Identifier(Identifier),
	Integer(Integer),
	Float(Float),
	String(String),
	Array(Vec<Expression>),
	Map(Vec<(Expression, Expression)>),

	Function(Vec<Identifier>, Vec<Statement>),

	Identity(Box<Expression>),
	Not(Box<Expression>),

	BinaryOperation(Box<[Expression; 2]>, BinaryOperation),

	Call(Box<Expression>, Vec<Expression>),
	Member(Box<Expression>, Identifier),
}

#[derive(Debug)]
pub enum BinaryOperation {
	Add,
	Sub,
	Mul,
	Div,
	Mod,
	Eq,
	NoEq,
	Lt,
	LtEq,
	Gt,
	GtEq,
	Index,
	And,
	Or,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier(pub String);
