fn main() {
	let source = std::fs::read_to_string("test.mill").unwrap();

	let ast = match mill::parser::parse(source) {
		Ok(a) => a,
		Err(e) => {
			println!("{e}");
			return;
		},
	};

	match mill::interpreter::interpret(ast) {
		Ok(()) => {},
		Err(e) => {
			println!("{e:?}");
		},
	}
}
