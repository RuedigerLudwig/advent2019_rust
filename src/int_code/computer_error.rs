#[derive(Debug, thiserror::Error)]
pub enum ComputerError {
    #[error("Not an Int")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("IllegalOperation: {0}")]
    IllegalOperation(usize),
    #[error("Machine is not running")]
    NotRunning,
    #[error("Not an instruction {0}")]
    NotAnInstruction(i64),
    #[error("Illegale ParamMode")]
    IllegalParamMode,
    #[error("Illegal Pointer: {0}")]
    PointerMustNoBeNegative(i64),
}
