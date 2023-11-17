use super::{
    computer_error::ComputerError,
    param_mode::ParamModeDispenser,
    state::{InternalStepResult, State},
};

pub fn run_instruction(state: &mut State) -> Result<InternalStepResult, ComputerError> {
    let (code, pd) = analyze_instruction(state.get_next()?)?;

    match code {
        1 => Add::calc(state, pd),
        2 => Mul::calc(state, pd),
        3 => Input::calc(state, pd),
        4 => Output::calc(state, pd),
        5 => JumpIfTrue::calc(state, pd),
        6 => JumpIfFalse::calc(state, pd),
        7 => LessThan::calc(state, pd),
        8 => Equals::calc(state, pd),
        99 => Stop::calc(state, pd),
        _ => Err(ComputerError::IllegalOperation(code)),
    }
}

fn analyze_instruction(instruction: i64) -> Result<(usize, ParamModeDispenser), ComputerError> {
    if !instruction.is_positive() {
        return Err(ComputerError::NotAnInstruction(instruction));
    }
    let instruction = instruction as usize;
    let code = instruction % 100;
    let pd = ParamModeDispenser::new(instruction / 100);
    Ok((code, pd))
}

trait Instruction {
    fn calc(
        state: &mut State,
        parameters: ParamModeDispenser,
    ) -> Result<InternalStepResult, ComputerError>;
}

struct Add;
impl Instruction for Add {
    fn calc(
        state: &mut State,
        parameters: ParamModeDispenser,
    ) -> Result<InternalStepResult, ComputerError> {
        let op1 = state.get_value(parameters.next())?;
        let op2 = state.get_value(parameters.next())?;
        let target = state.get_address(parameters.next())?;

        state.set_value(target, op1 + op2)?;
        Ok(InternalStepResult::Continue)
    }
}

struct Mul;
impl Instruction for Mul {
    fn calc(
        state: &mut State,
        parameters: ParamModeDispenser,
    ) -> Result<InternalStepResult, ComputerError> {
        let op1 = state.get_value(parameters.next())?;
        let op2 = state.get_value(parameters.next())?;
        let target = state.get_address(parameters.next())?;

        state.set_value(target, op1 * op2)?;
        Ok(InternalStepResult::Continue)
    }
}

struct Stop;
impl Instruction for Stop {
    fn calc(
        _state: &mut State,
        _parameters: ParamModeDispenser,
    ) -> Result<InternalStepResult, ComputerError> {
        Ok(InternalStepResult::Halted)
    }
}

struct Input;
impl Instruction for Input {
    fn calc(
        state: &mut State,
        parameters: ParamModeDispenser,
    ) -> Result<InternalStepResult, ComputerError> {
        if let Some(value) = state.get_input() {
            let target = state.get_address(parameters.next())?;
            state.set_value(target, value)?;
            Ok(InternalStepResult::Continue)
        } else {
            state.repeat();
            Ok(InternalStepResult::Waiting)
        }
    }
}

struct Output;
impl Instruction for Output {
    fn calc(
        state: &mut State,
        parameters: ParamModeDispenser,
    ) -> Result<InternalStepResult, ComputerError> {
        let op1 = state.get_value(parameters.next())?;
        Ok(InternalStepResult::Output(op1))
    }
}

struct JumpIfTrue;
impl Instruction for JumpIfTrue {
    fn calc(
        state: &mut State,
        parameters: ParamModeDispenser,
    ) -> Result<InternalStepResult, ComputerError> {
        let test = state.get_value(parameters.next())?;
        let target = state.get_value(parameters.next())?;
        if test != 0 {
            state.set_pointer(target.try_into()?);
        }
        Ok(InternalStepResult::Continue)
    }
}

struct JumpIfFalse;
impl Instruction for JumpIfFalse {
    fn calc(
        state: &mut State,
        parameters: ParamModeDispenser,
    ) -> Result<InternalStepResult, ComputerError> {
        let test = state.get_value(parameters.next())?;
        let target = state.get_value(parameters.next())?;
        if test == 0 {
            state.set_pointer(target.try_into()?);
        }
        Ok(InternalStepResult::Continue)
    }
}

struct LessThan;
impl Instruction for LessThan {
    fn calc(
        state: &mut State,
        parameters: ParamModeDispenser,
    ) -> Result<InternalStepResult, ComputerError> {
        let op1 = state.get_value(parameters.next())?;
        let op2 = state.get_value(parameters.next())?;
        let target = state.get_address(parameters.next())?;

        let result = if op1 < op2 { 1 } else { 0 };
        state.set_value(target, result)?;
        Ok(InternalStepResult::Continue)
    }
}

struct Equals;
impl Instruction for Equals {
    fn calc(
        state: &mut State,
        parameters: ParamModeDispenser,
    ) -> Result<InternalStepResult, ComputerError> {
        let op1 = state.get_value(parameters.next())?;
        let op2 = state.get_value(parameters.next())?;
        let target = state.get_address(parameters.next())?;

        let result = if op1 == op2 { 1 } else { 0 };
        state.set_value(target, result)?;
        Ok(InternalStepResult::Continue)
    }
}
