fn main() {
	let source = std::fs::read_to_string("test.radio").unwrap();

	match radio_script::parser::parse(source) {
		Ok(e) => println!("{e:#?}"),
		Err(e) => println!("{e}"),
	}
}
