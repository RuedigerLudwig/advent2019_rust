use super::{
    computer_error::ComputerError,
    state::{State, StepResult},
};

pub fn add(state: &mut State) -> Result<StepResult, ComputerError> {
    let op1 = state.get_next()?;
    let op2 = state.get_next()?;
    let target = state.get_next()?;

    let op1 = state.get_value(op1.try_into()?)?;
    let op2 = state.get_value(op2.try_into()?)?;
    state.set_value(target.try_into()?, op1 + op2)?;
    Ok(StepResult::Continue)
}

pub fn mul(state: &mut State) -> Result<StepResult, ComputerError> {
    let op1 = state.get_next()?;
    let op2 = state.get_next()?;
    let target = state.get_next()?;

    let op1 = state.get_value(op1.try_into()?)?;
    let op2 = state.get_value(op2.try_into()?)?;
    state.set_value(target.try_into()?, op1 * op2)?;
    Ok(StepResult::Continue)
}

pub fn stop() -> Result<StepResult, ComputerError> {
    Ok(StepResult::StopRunning)
}
