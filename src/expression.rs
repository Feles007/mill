use crate::{
	error::{ParseError, ParseErrorKind},
	lexer::{Identifier, Lexer, Symbol, Token},
	statement::{parse_block, parse_statement, Statement},
	Float, Integer,
};

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

	Add(Box<Expression>, Box<Expression>),
	Sub(Box<Expression>, Box<Expression>),
	Mul(Box<Expression>, Box<Expression>),
	Div(Box<Expression>, Box<Expression>),
	Mod(Box<Expression>, Box<Expression>),

	Eq(Box<Expression>, Box<Expression>),
	NotEq(Box<Expression>, Box<Expression>),
	Lt(Box<Expression>, Box<Expression>),
	LtEq(Box<Expression>, Box<Expression>),
	Gt(Box<Expression>, Box<Expression>),
	GtEq(Box<Expression>, Box<Expression>),

	Index(Box<Expression>, Box<Expression>),
	Call(Box<Expression>, Vec<Expression>),
	Member(Box<Expression>, Box<Expression>),

	And(Box<Expression>, Box<Expression>),
	Or(Box<Expression>, Box<Expression>),
}

pub fn parse_expression(lexer: &mut Lexer) -> Result<Expression, ParseError> {
	parse_expression_bp(lexer, 0)
}

fn parse_expression_bp(lexer: &mut Lexer, min_bp: u8) -> Result<Expression, ParseError> {
	let mut lhs = match lexer.next()? {
		Token::True => Expression::True,
		Token::False => Expression::False,
		Token::Null => Expression::Null,

		Token::Identifier(i) => Expression::Identifier(i),
		Token::Integer(n) => Expression::Integer(n),
		Token::Float(n) => Expression::Float(n),
		Token::String(s) => Expression::String(s),

		Token::Symbol(Symbol::ParenLeft) => {
			let lhs = parse_expression(lexer)?;
			match lexer.next()? {
				Token::Symbol(Symbol::ParenRight) => {},
				t => {
					return Err(lexer.error(ParseErrorKind::UnexpectedToken {
						expected: "closing parenthesis",
						found: t,
					}))
				},
			}
			lhs
		},
		Token::Symbol(Symbol::SquareLeft) => {
			let mut initializers = Vec::new();

			loop {
				match lexer.peek()? {
					Token::Eof | Token::Symbol(Symbol::SquareRight) => break,
					_ => {
						let value = parse_expression(lexer)?;
						initializers.push(value);
					},
				}
				match lexer.peek()? {
					Token::Symbol(Symbol::Comma) => {
						lexer.next()?;
					},
					_ => break,
				}
			}

			match lexer.next()? {
				Token::Symbol(Symbol::SquareRight) => {},
				t => {
					return Err(lexer.error(ParseErrorKind::UnexpectedToken {
						expected: "closing square bracket",
						found: t,
					}))
				},
			}

			Expression::Array(initializers)
		},
		Token::Symbol(Symbol::CurlyLeft) => {
			let mut initializers = Vec::new();

			loop {
				let key;
				match lexer.next()? {
					Token::Eof => return Err(lexer.error(ParseErrorKind::UnexpectedToken {
						expected: "closing curly bracket",
						found: Token::Eof,
					})),
					Token::Symbol(Symbol::CurlyRight) => break,
					Token::Identifier(i) => {
						key = Expression::String(i.0);
					},
					Token::Symbol(Symbol::SquareLeft) => {
						key = parse_expression(lexer)?;
						match lexer.next()? {
							Token::Symbol(Symbol::SquareRight) => {},
							t => {
								return Err(lexer.error(ParseErrorKind::UnexpectedToken {
									expected: "closing square bracket",
									found: t,
								}))
							},
						}
					},
					t => {
						return Err(lexer.error(ParseErrorKind::UnexpectedToken {
							expected: "identifier or [expression]",
							found: t,
						}))
					},
				}
				match lexer.next()? {
					Token::Symbol(Symbol::Colon) => {},
					t => {
						return Err(lexer.error(ParseErrorKind::UnexpectedToken {
							expected: "colon",
							found: t,
						}))
					},
				}
				let value = parse_expression(lexer)?;
				initializers.push((key, value));
				match lexer.peek()? {
					Token::Symbol(Symbol::Comma) => {
						lexer.next()?;
					},
					Token::Symbol(Symbol::CurlyRight) => {
						lexer.next()?;
						break;
					}
					t => return Err(lexer.error(ParseErrorKind::UnexpectedToken {
						expected: "comma or closing curly bracket",
						found: t,
					})),
				}
			}

			Expression::Map(initializers)
		},
		Token::Symbol(op) if op.prefix_bp().is_some() => {
			let ((), r_bp) = op.prefix_bp().unwrap();
			let rhs = parse_expression_bp(lexer, r_bp)?;
			match op {
				Symbol::Add => Expression::Identity(Box::new(rhs)),
				Symbol::Sub => Expression::Not(Box::new(rhs)),
				_ => unimplemented!(),
			}
		},
		Token::Fn => {
			match lexer.next()? {
				Token::Symbol(Symbol::ParenLeft) => {},
				t => {
					return Err(lexer.error(ParseErrorKind::UnexpectedToken {
						expected: "parameter list",
						found: t,
					}))
				},
			}

			let mut parameters = Vec::new();

			loop {
				if lexer.peek()? == Token::Symbol(Symbol::ParenRight) {
					break;
				}
				match lexer.next()? {
					Token::Identifier(i) => parameters.push(i),
					t => {
						return Err(lexer.error(ParseErrorKind::UnexpectedToken {
							expected: "identifier",
							found: t,
						}))
					},
				}
				if lexer.peek()? != Token::Symbol(Symbol::Comma) {
					break;
				}
				lexer.next()?;
			}
			match lexer.next()? {
				Token::Symbol(Symbol::ParenRight) => {},
				t => {
					return Err(lexer.error(ParseErrorKind::UnexpectedToken {
						expected: "closing parenthesis",
						found: t,
					}))
				},
			}

			let statements = parse_block(lexer, true)?;

			Expression::Function(parameters, statements)
		},
		t => {
			return Err(lexer.error(ParseErrorKind::UnexpectedToken {
				expected: "start of expression",
				found: t,
			}))
		},
	};

	loop {
		let op = match lexer.peek()? {
			Token::Eof => break,
			Token::Symbol(op) => op,
			t => {
				return Err(lexer.error(ParseErrorKind::UnexpectedToken {
					expected: "operator or end of expression",
					found: t,
				}))
			},
		};

		if let Some((l_bp, ())) = op.postfix_bp() {
			if l_bp < min_bp {
				break;
			}
			lexer.next()?;

			lhs = match op {
				Symbol::SquareLeft => {
					let rhs = parse_expression(lexer)?;
					match lexer.next()? {
						Token::Symbol(Symbol::SquareRight) => {},
						t => {
							return Err(lexer.error(ParseErrorKind::UnexpectedToken {
								expected: "closing square bracket",
								found: t,
							}))
						},
					}
					Expression::Index(Box::new(lhs), Box::new(rhs))
				},
				Symbol::ParenLeft => {
					let mut arguments = Vec::new();

					loop {
						if lexer.peek()? == Token::Symbol(Symbol::ParenRight) {
							break;
						}
						arguments.push(parse_expression(lexer)?);
						if lexer.peek()? != Token::Symbol(Symbol::Comma) {
							break;
						}
						lexer.next()?;
					}
					match lexer.next()? {
						Token::Symbol(Symbol::ParenRight) => {},
						t => {
							return Err(lexer.error(ParseErrorKind::UnexpectedToken {
								expected: "closing parenthesis",
								found: t,
							}))
						},
					}

					Expression::Call(Box::new(lhs), arguments)
				},
				_ => unreachable!(),
			};

			continue;
		}

		if let Some((l_bp, r_bp)) = op.infix_bp() {
			if l_bp < min_bp {
				break;
			}
			lexer.next()?;

			lhs = {
				let rhs = parse_expression_bp(lexer, r_bp)?;
				let lhs = Box::new(lhs);
				let rhs = Box::new(rhs);
				match op {
					Symbol::Add => Expression::Add(lhs, rhs),
					Symbol::Sub => Expression::Sub(lhs, rhs),
					Symbol::Mul => Expression::Mul(lhs, rhs),
					Symbol::Div => Expression::Div(lhs, rhs),
					Symbol::Mod => Expression::Mod(lhs, rhs),

					Symbol::Dot => Expression::Member(lhs, rhs),

					Symbol::EqEq => Expression::Eq(lhs, rhs),
					Symbol::NoEq => Expression::NotEq(lhs, rhs),
					Symbol::Lt => Expression::Lt(lhs, rhs),
					Symbol::LtEq => Expression::LtEq(lhs, rhs),
					Symbol::Gt => Expression::Gt(lhs, rhs),
					Symbol::GtEq => Expression::GtEq(lhs, rhs),

					Symbol::And => Expression::And(lhs, rhs),
					Symbol::Or => Expression::Or(lhs, rhs),

					_ => unimplemented!(),
				}
			};
			continue;
		}

		break;
	}

	Ok(lhs)
}
