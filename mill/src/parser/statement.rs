use crate::{
	ast::{BinaryOperation, Expression, Lvalue, Statement},
	parser::{
		error::{ParseError, ParseErrorKind},
		expression::parse_expression,
		lexer::{Lexer, Symbol, Token},
	},
};

pub fn parse_statement(lexer: &mut Lexer) -> Result<Statement, ParseError> {
	let mut expect_semicolon = true;
	let statement = match lexer.peek()? {
		Token::Break => {
			lexer.next()?;
			Statement::Break
		},
		Token::Continue => {
			lexer.next()?;
			Statement::Continue
		},
		Token::Return => {
			lexer.next()?;
			let value = if lexer.peek()? == Token::Symbol(Symbol::Semicolon) {
				Expression::Null
			} else {
				parse_expression(lexer)?
			};
			Statement::Return(value)
		},
		Token::Loop => {
			lexer.next()?;
			expect_semicolon = false;
			Statement::Loop {
				body: parse_block(lexer)?,
			}
		},
		Token::Let => {
			lexer.next()?;

			let name = match lexer.next()? {
				Token::Identifier(i) => i,
				t => {
					return Err(lexer.error(ParseErrorKind::UnexpectedToken {
						expected: "identifier",
						found: t,
					}))
				},
			};

			let initializer = match lexer.peek()? {
				Token::Symbol(Symbol::Semicolon) => Expression::Null,
				Token::Symbol(Symbol::Eq) => {
					lexer.next()?;
					parse_expression(lexer)?
				},
				t => {
					return Err(lexer.error(ParseErrorKind::UnexpectedToken {
						expected: "semicolon or initializer",
						found: t,
					}))
				},
			};

			Statement::Declaration { name, initializer }
		},
		Token::If => {
			lexer.next()?;
			expect_semicolon = false;

			let mut branches = Vec::new();

			{
				let condition = parse_expression(lexer)?;
				let body = parse_block(lexer)?;

				branches.push((condition, body));
			}

			while lexer.peek()? == Token::Else {
				lexer.next()?;
				if lexer.peek()? == Token::If {
					lexer.next()?;
					let condition = parse_expression(lexer)?;
					let body = parse_block(lexer)?;
					branches.push((condition, body));
				} else {
					branches.push((Expression::True, parse_block(lexer)?));
					break;
				}
			}

			Statement::If { branches }
		},
		Token::For => {
			lexer.next()?;
			expect_semicolon = false;

			let loop_var = match lexer.next()? {
				Token::Identifier(i) => i,
				t => {
					return Err(lexer.error(ParseErrorKind::UnexpectedToken {
						expected: "identifier",
						found: t,
					}))
				},
			};

			match lexer.next()? {
				Token::In => {},
				t => {
					return Err(lexer.error(ParseErrorKind::UnexpectedToken {
						expected: "keyword 'in'",
						found: t,
					}))
				},
			}

			let iterator = parse_expression(lexer)?;

			let body = parse_block(lexer)?;

			Statement::For {
				loop_var,
				iterator,
				body,
			}
		},
		Token::While => {
			lexer.next()?;
			expect_semicolon = false;

			let condition = parse_expression(lexer)?;
			let body = parse_block(lexer)?;
			Statement::While { condition, body }
		},
		Token::Symbol(Symbol::CurlyLeft) => {
			expect_semicolon = false;
			Statement::Block {
				body: parse_block(lexer)?,
			}
		},
		_ => {
			let expression = parse_expression(lexer)?;
			match lexer.peek()? {
				Token::Symbol(Symbol::Semicolon) => Statement::UnusedExpression(expression),
				Token::Symbol(Symbol::Eq) => {
					let lvalue = try_as_lvalue(expression, lexer)?;
					lexer.next()?;
					Statement::Assignment {
						lvalue,
						value: parse_expression(lexer)?,
					}
				},
				t => {
					return Err(lexer.error(ParseErrorKind::UnexpectedToken {
						expected: "';' or '='",
						found: t,
					}))
				},
			}
		},
	};
	if expect_semicolon {
		match lexer.next()? {
			Token::Symbol(Symbol::Semicolon) => {},
			t => {
				return Err(lexer.error(ParseErrorKind::UnexpectedToken {
					expected: "semicolon",
					found: t,
				}))
			},
		}
	}
	Ok(statement)
}
pub fn parse_block(lexer: &mut Lexer) -> Result<Vec<Statement>, ParseError> {
	match lexer.next()? {
		Token::Symbol(Symbol::CurlyLeft) => {},
		t => {
			return Err(lexer.error(ParseErrorKind::UnexpectedToken {
				expected: "block",
				found: t,
			}))
		},
	}

	let mut statements = Vec::new();

	loop {
		match lexer.peek()? {
			Token::Symbol(Symbol::CurlyRight) => break,
			Token::Eof => {
				return Err(lexer.error(ParseErrorKind::UnexpectedToken {
					expected: "closing curly bracket",
					found: Token::Eof,
				}))
			},
			_ => {},
		}

		statements.push(parse_statement(lexer)?);
	}
	match lexer.next()? {
		Token::Symbol(Symbol::CurlyRight) => {},
		t => {
			return Err(lexer.error(ParseErrorKind::UnexpectedToken {
				expected: "closing curly bracket",
				found: t,
			}))
		},
	}

	Ok(statements)
}
pub fn parse_file(lexer: &mut Lexer) -> Result<Vec<Statement>, ParseError> {
	let mut statements = Vec::new();

	loop {
		if lexer.peek()? == Token::Eof {
			break;
		}

		statements.push(parse_statement(lexer)?);
	}

	Ok(statements)
}
fn try_as_lvalue(expression: Expression, lexer: &mut Lexer) -> Result<Lvalue, ParseError> {
	Ok(match expression {
		Expression::Identifier(i) => Lvalue::Identifier(i),
		Expression::Member(e, m) => Lvalue::Member(e, m),
		Expression::BinaryOperation(ei, BinaryOperation::Index) => Lvalue::Index(ei),
		_ => return Err(lexer.error(ParseErrorKind::ExpressionNotAssignable)),
	})
}
