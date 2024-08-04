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
		lvalue: Lvalue,
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
		branches: Vec<(Expression, Vec<Statement>)>,
	},
	Block {
		body: Vec<Statement>,
	},
}

#[derive(Debug)]
pub enum Expression {
	// Keyword literals
	True,
	False,
	Null,

	// Literals
	Identifier(Identifier),
	Integer(Integer),
	Float(Float),
	String(String),
	Array(Vec<Expression>),
	Map(Vec<(Expression, Expression)>),

	// Special stuff
	Function(Vec<Identifier>, Vec<Statement>),
	Call(Box<Expression>, Vec<Expression>),
	Member(Box<Expression>, Identifier),

	// Normal operations
	UnaryOperation(Box<Expression>, UnaryOperation),
	BinaryOperation(Box<[Expression; 2]>, BinaryOperation),
}

#[derive(Debug)]
pub enum Lvalue {
	Identifier(Identifier),
	Member(Box<Expression>, Identifier),
	Index(Box<[Expression; 2]>),
}

#[derive(Debug)]
pub enum UnaryOperation {
	Neg,
	Not,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(pub String);
