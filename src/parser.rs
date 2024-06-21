use crate::lexer::{Lexer, Token, Number, Identifier, Symbol};

#[derive(Debug)]
pub enum Expression {
	Identifier(Identifier),
	Number(Number),
	
	Identity(Box<Expression>),
	Not(Box<Expression>),

	Add(Box<Expression>, Box<Expression>),
	Sub(Box<Expression>, Box<Expression>),
	Mul(Box<Expression>, Box<Expression>),
	Div(Box<Expression>, Box<Expression>),
	Mod(Box<Expression>, Box<Expression>),

	Index(Box<Expression>, Box<Expression>),
	Call(Box<Expression>, Vec<Expression>),
	Member(Box<Expression>, Box<Expression>),
}

pub fn expr(input: &str) -> Expression {
	let mut lexer = Lexer::new(input);
	dbg!(&lexer);
	let re = expr_bp(&mut lexer, 0);
	re
}

fn expr_bp(lexer: &mut Lexer, min_bp: u8) -> Expression {
	let mut lhs = match lexer.next() {
		Token::Identifier(i) => {
			Expression::Identifier(i)
		}
		Token::Number(n) => {
			Expression::Number(n)
		}
		Token::Symbol(Symbol::ParenLeft) => {
			let lhs = expr_bp(lexer, 0);
			assert_eq!(lexer.next(), Token::Symbol(Symbol::ParenRight));
			lhs
		}
		Token::Symbol(op) => {
			let ((), r_bp) = op.prefix_bp().unwrap();
			let rhs = expr_bp(lexer, r_bp);
			match op {
				Symbol::Add => Expression::Identity(Box::new(rhs)),
				Symbol::Sub => Expression::Not(Box::new(rhs)),
				_ => unimplemented!(),
			}
		}
		t => panic!("bad token: {:?}", t),
	};

	loop {
		let op = match lexer.peek() {
			Token::Eof => break,
			Token::Symbol(op) => op,
			t => panic!("bad token: {:?}", t),
		};

		if let Some((l_bp, ())) = op.postfix_bp() {
			if l_bp < min_bp {
				break;
			}
			lexer.next();

			lhs = if op == Symbol::SquareLeft {
				let rhs = expr_bp(lexer, 0);
				assert_eq!(lexer.next(), Token::Symbol(Symbol::SquareRight));
				Expression::Index(Box::new(lhs), Box::new(rhs))
			} else if op == Symbol::ParenLeft {
				let mut arguments = Vec::new();

				loop {
					if lexer.peek() == Token::Symbol(Symbol::ParenRight) {
						break;
					}
					arguments.push(expr_bp(lexer, 0));
					if lexer.peek() != Token::Symbol(Symbol::Comma) {
						break;
					}
					lexer.next();
				}
				assert_eq!(lexer.next(), Token::Symbol(Symbol::ParenRight));

				Expression::Call(Box::new(lhs), arguments)
				
			} else {
				unimplemented!()
			};
			continue;
		}

		if let Some((l_bp, r_bp)) = op.infix_bp() {
			if l_bp < min_bp {
				break;
			}
			lexer.next();

			lhs = {
				let rhs = expr_bp(lexer, r_bp);
				let lhs = Box::new(lhs);
				let rhs = Box::new(rhs);
				match op {
					Symbol::Add => Expression::Add(lhs, rhs),
					Symbol::Sub => Expression::Sub(lhs, rhs),
					Symbol::Mul => Expression::Mul(lhs, rhs),
					Symbol::Div => Expression::Div(lhs, rhs),
					Symbol::Mod => Expression::Mod(lhs, rhs),

					Symbol::Dot => Expression::Member(lhs, rhs),

					_ => unimplemented!()
				}
			};
			continue;
		}

		break;
	}

	lhs
}
