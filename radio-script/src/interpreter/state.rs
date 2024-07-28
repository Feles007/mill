#[derive(Debug)]
pub struct State {
	stack: Vec<StackFrame>,
}
impl State {
	pub fn new() -> Self { Self { stack: Vec::new() } }
}

#[derive(Debug)]
pub struct StackFrame {}
