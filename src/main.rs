#![allow(unused)]

mod error;
mod lexer;
mod expression;
mod statement;

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

let inc = fn(n) { return n + 1; }(4);

"#;

	let mut lexer = lexer::Lexer::new(source);

	match statement::parse_block(&mut lexer, false) {
		Ok(e) => println!("{e:#?}"),
		Err(e) => println!("{e}"),
	}
}