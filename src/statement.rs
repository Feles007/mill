use crate::expression::{Expression, parse_expression};
use crate::lexer::{Lexer, Token, Identifier, Symbol};
use crate::error::ParseErrorKind;
use crate::error::ParseError;

#[derive(Debug)]
pub struct Block(Vec<Statement>);

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
}
pub fn parse_statement(lexer: &mut Lexer) -> Result<Statement, ParseError> {
	let mut expect_semicolon = true;
	let statement = match lexer.peek()? {
		Token::Break => { lexer.next()?; Statement::Break },
		Token::Continue => { lexer.next()?; Statement::Continue },
		Token::Return => {
			lexer.next()?;
			let value = if lexer.peek()? != Token::Symbol(Symbol::Semicolon) {
				parse_expression(lexer)?
			} else {
				Expression::Null
			};
			Statement::Return(value)
		}
		Token::Let => {
			lexer.next()?;

			let name = match lexer.next()? {
				Token::Identifier(i) => i,
				t => return Err(lexer.error(ParseErrorKind::UnexpectedToken {
					expected: "identifier",
					found: t,
				}))
			};

			let initializer = match lexer.peek()? {
				Token::Symbol(Symbol::Semicolon) => {
					Expression::Null
				}
				Token::Symbol(Symbol::Eq) => {
					lexer.next()?;
					parse_expression(lexer)?
				}
				t => return Err(lexer.error(ParseErrorKind::UnexpectedToken {
					expected: "semicolon or initializer",
					found: t,
				}))
			};

			Statement::Declaration { name, initializer }
		}
		Token::If => {
			todo!()
		}
		_ => {
			let expression = parse_expression(lexer)?;
			match lexer.peek()? {
				Token::Symbol(Symbol::Semicolon) => {
					Statement::UnusedExpression(expression)
				}
				Token::Symbol(Symbol::Eq) => {
					lexer.next()?;
					Statement::Assignment {
						lvalue: expression,
						value: parse_expression(lexer)?
					}
				}
				t => return Err(lexer.error(ParseErrorKind::UnexpectedToken {
					expected: "';' or '='",
					found: t,
				}))
			}
		}
	};
	if expect_semicolon {
		match lexer.next()? {
			Token::Symbol(Symbol::Semicolon) => {}
			t => return Err(lexer.error(ParseErrorKind::UnexpectedToken {
				expected: "semicolon",
				found: t,
			}))
		}
	}
	Ok(statement)
}