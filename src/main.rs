#![allow(unused)]

mod error;
mod expression;
mod lexer;
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

let x = 0;
let y = 1;
let z = x + y;

loop {
	print(x);
	x = y;
	y = z;
	z = x + y;

	if z > 255 && y != 0 {
		break;
	}
}

let m = {["bop"]: "asd"};

for _ in {} {}

for pair in std.map.kv(m) {
	let k = pair.k;
	let v = pair.v;

	assert(m[k] == v);
}

let cond = true;
while cond {
	let f = fn (a) { return a * 3; };
	if f(3) == 9 {
		cond = false;
	}
	{
		let x = 3;
	}
}

"#;

	let mut lexer = lexer::Lexer::new(source);

	match statement::parse_block(&mut lexer, false) {
		Ok(e) => println!("{e:#?}"),
		Err(e) => println!("{e}"),
	}
}
