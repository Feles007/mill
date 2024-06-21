#![allow(unused)]

mod error;
mod lexer;
mod expression;

use std::{fmt, io::BufRead};

type LineNumber = usize;
type Integer = i32;
// Ideally we'd just parse integer literals and have negation done at runtime when
// evaluating the expression, but signed::min.abs > signed::max so it'd overflow.
// Instead, we parse it as an unsigned which has a bigger range then do a post-process
// step on the AST after parsing.
type UInteger = u32;
type Float = f64;

fn main() {
	let source = r#"
999 *
 4.8
	"#;

	let mut lexer = lexer::Lexer::new(source);

	match expression::parse_expression(&mut lexer) {
		Ok(e) => println!("{e:#?}"),
		Err(e) => println!("{e}"),
	}
}