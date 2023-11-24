#[derive(Debug, thiserror::Error)]
pub enum ComputerError {
    #[error("Not an Int")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("IllegalOperation: {0}")]
    IllegalOperation(usize),
    #[error("Machine was stopped after an error")]
    StoppedAfterError,
    #[error("Not an instruction {0}")]
    NotAnInstruction(i64),
    #[error("Illegale ParamMode")]
    IllegalParamMode,
    #[error("Illegal Pointer: {0}")]
    PointerMustNoBeNegative(i64),
    #[error("Premature End of Output")]
    PrematureEndOfOutput,
    #[error("Waiting for Input")]
    WaitingForInput,
    #[error("not a valid char: {0}")]
    NotAValidChar(i64),
}
