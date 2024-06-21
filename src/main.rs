#![allow(unused)]

mod error;
mod lexer;
mod parser;

use std::{fmt, io::BufRead};

type LineNumber = usize;

fn main() {
	let source = r#"
3 + 
1 + 9 #asd
* o.f() +
g()

	"#;

	match parser::parse_expression(source) {
		Ok(e) => println!("{e:#?}"),
		Err(e) => println!("{e}"),
	}
}