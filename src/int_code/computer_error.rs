use super::Pointer;

#[derive(Debug, thiserror::Error)]
pub enum ComputerError {
    #[error("Not an Int")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Illegal Pointer: {0}")]
    IllegalPointerI64(i64),
    #[error("Not enogh Data")]
    NoMoreData,
    #[error("IllegalAddress: {0}")]
    IllegalAddress(Pointer),
    #[error("IllegalOperation: {0}")]
    IllegalOperation(usize),
    #[error("Machine is not running")]
    NotRunning,
    #[error("Not an instruction {0}")]
    NotAnInstruction(i64),
    #[error("Illegale ParamMode")]
    IllegalParamMode,
}
