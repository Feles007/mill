#[derive(Debug)]
pub enum InterpreterError {
	Redeclaration,
	UnknownIdentifier,
	UnsupportedOperation,
}
