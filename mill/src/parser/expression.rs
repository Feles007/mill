use crate::{
	ast::{BinaryOperation, Expression, UnaryOperation},
	parser::{
		error::{ParseError, ParseErrorKind},
		lexer::{Lexer, Symbol, Token},
		statement::parse_block,
	},
};

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
				let key = match lexer.peek()? {
					Token::Eof => {
						lexer.next()?;
						return Err(lexer.error(ParseErrorKind::UnexpectedToken {
							expected: "closing curly bracket",
							found: Token::Eof,
						}));
					},
					Token::Symbol(Symbol::CurlyRight) => {
						lexer.next()?;
						break;
					},
					_ => parse_expression(lexer)?,
				};
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
					},
					t => {
						return Err(lexer.error(ParseErrorKind::UnexpectedToken {
							expected: "comma or closing curly bracket",
							found: t,
						}))
					},
				}
			}

			Expression::Map(initializers)
		},
		Token::Symbol(op) if op.prefix_bp().is_some() => {
			let ((), r_bp) = op.prefix_bp().unwrap();
			let rhs = parse_expression_bp(lexer, r_bp)?;
			let operator = match op {
				Symbol::No => UnaryOperation::Not,
				Symbol::Sub => UnaryOperation::Neg,
				_ => unimplemented!(),
			};
			Expression::UnaryOperation(Box::new(rhs), operator)
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

			let statements = parse_block(lexer)?;

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
					Expression::BinaryOperation(Box::new([lhs, rhs]), BinaryOperation::Index)
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
				use BinaryOperation as B;
				use Symbol as S;

				if op == S::Dot {
					let member = match lexer.next()? {
						Token::Identifier(i) => i,
						t => {
							return Err(lexer.error(ParseErrorKind::UnexpectedToken {
								expected: "identifier",
								found: t,
							}))
						},
					};
					Expression::Member(Box::new(lhs), member)
				} else {
					let rhs = parse_expression_bp(lexer, r_bp)?;
					let operands = Box::new([lhs, rhs]);
					let operator = match op {
						S::Add => B::Add,
						S::Sub => B::Sub,
						S::Mul => B::Mul,
						S::Div => B::Div,
						S::Mod => B::Mod,

						S::EqEq => B::Eq,
						S::NoEq => B::NoEq,
						S::Lt => B::Lt,
						S::LtEq => B::LtEq,
						S::Gt => B::Gt,
						S::GtEq => B::GtEq,

						S::And => B::And,
						S::Or => B::Or,

						_ => unimplemented!(),
					};
					Expression::BinaryOperation(operands, operator)
				}
			};
			continue;
		}

		break;
	}

	Ok(lhs)
}
