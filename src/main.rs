#![allow(unused)]

mod error;
mod lexer;
mod parser;

use std::{fmt, io::BufRead};

type LineNumber = usize;

fn main() {
	for line in std::io::stdin().lock().lines() {
		let line = line.unwrap();
		let s = parser::expr(&line);
		println!("{:#?}", s);
	}
}