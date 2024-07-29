fn main() {
	let source = std::fs::read_to_string("test.radio").unwrap();

	let ast = match radio_script::parser::parse(source) {
		Ok(a) => a,
		Err(e) => {
			println!("{e}");
			return;
		},
	};

	match radio_script::interpreter::interpret(ast) {
		Ok(()) => {},
		Err(e) => {
			println!("{e:?}");
		},
	}
}
