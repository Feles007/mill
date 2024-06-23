use crate::{
	error::{ParseError, ParseErrorKind},
	expression::{parse_expression, Expression},
	lexer::{Identifier, Lexer, Symbol, Token},
};

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
	If {
		condition: Expression,
		body: Vec<Statement>,
		else_ifs: Vec<(Expression, Vec<Statement>)>,
		else_body: Vec<Statement>,
	},
}
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
			let value = if lexer.peek()? != Token::Symbol(Symbol::Semicolon) {
				parse_expression(lexer)?
			} else {
				Expression::Null
			};
			Statement::Return(value)
		},
		Token::Loop => {
			lexer.next()?;
			expect_semicolon = false;
			Statement::Loop {
				body: parse_block(lexer, true)?,
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
			let condition = parse_expression(lexer)?;
			let body = parse_block(lexer, true)?;

			let mut else_ifs = Vec::new();
			let mut else_body = Vec::new();

			while lexer.peek()? == Token::Else {
				lexer.next()?;
				if lexer.peek()? == Token::If {
					lexer.next()?;
					let condition = parse_expression(lexer)?;
					let body = parse_block(lexer, true)?;
					else_ifs.push((condition, body));
				} else {
					else_body = parse_block(lexer, true)?;
					break;
				}
			}

			Statement::If {
				condition,
				body,
				else_ifs,
				else_body,
			}
		},
		Token::For => {
			lexer.next()?;
			expect_semicolon = false;

			let loop_var = match lexer.next()? {
				Token::Identifier(i) => i,
				t => return Err(lexer.error(ParseErrorKind::UnexpectedToken {
					expected: "identifier",
					found: t,
				}))
			};

			match lexer.next()? {
				Token::In => {}
				t => return Err(lexer.error(ParseErrorKind::UnexpectedToken {
					expected: "keyword 'in'",
					found: t,
				}))
			}

			let iterator = parse_expression(lexer)?;

			let body = parse_block(lexer, true)?;

			Statement::For { loop_var, iterator, body }
		}
		_ => {
			let expression = parse_expression(lexer)?;
			match lexer.peek()? {
				Token::Symbol(Symbol::Semicolon) => Statement::UnusedExpression(expression),
				Token::Symbol(Symbol::Eq) => {
					lexer.next()?;
					Statement::Assignment {
						lvalue: expression,
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
pub fn parse_block(lexer: &mut Lexer, bracketed: bool) -> Result<Vec<Statement>, ParseError> {
	if bracketed {
		match lexer.next()? {
			Token::Symbol(Symbol::CurlyLeft) => {},
			t => {
				return Err(lexer.error(ParseErrorKind::UnexpectedToken {
					expected: "block",
					found: t,
				}))
			},
		}
	}

	let mut statements = Vec::new();

	loop {
		match (bracketed, lexer.peek()?) {
			(true, Token::Symbol(Symbol::CurlyRight)) | (false, Token::Eof) => break,
			(true, Token::Eof) => {
				return Err(lexer.error(ParseErrorKind::UnexpectedToken {
					expected: "closing curly bracket",
					found: Token::Eof,
				}))
			},
			_ => {},
		}

		statements.push(parse_statement(lexer)?);
	}
	if bracketed {
		match lexer.next()? {
			Token::Symbol(Symbol::CurlyRight) => {},
			t => {
				return Err(lexer.error(ParseErrorKind::UnexpectedToken {
					expected: "closing curly bracket",
					found: t,
				}))
			},
		}
	}
	Ok(statements)
}
