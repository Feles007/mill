#[derive(Debug)]
pub enum InterpreterError {
	Redeclaration,
	UnknownIdentifier,
	UnsupportedOperation,
	ExpectedBool,
	ExpectedFunction,
	WrongArgumentCount,
	UpwardControlFlowReachedTopLevel,
	LoopControlFlowReachedFunction,
}
