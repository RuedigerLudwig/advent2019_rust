use super::{
    computer_error::ComputerError,
    param_mode::ParamModeDispenser,
    state::{State, StepResult},
};

pub fn run_instruction(state: &mut State) -> Result<StepResult, ComputerError> {
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
    fn calc(state: &mut State, parameters: ParamModeDispenser)
        -> Result<StepResult, ComputerError>;
}

struct Add;
impl Instruction for Add {
    fn calc(
        state: &mut State,
        parameters: ParamModeDispenser,
    ) -> Result<StepResult, ComputerError> {
        let op1 = state.get_value(parameters.next())?;
        let op2 = state.get_value(parameters.next())?;
        let target = state.get_address(parameters.next())?;

        state.set_value(target, op1 + op2)?;
        Ok(StepResult::Continue)
    }
}

struct Mul;
impl Instruction for Mul {
    fn calc(
        state: &mut State,
        parameters: ParamModeDispenser,
    ) -> Result<StepResult, ComputerError> {
        let op1 = state.get_value(parameters.next())?;
        let op2 = state.get_value(parameters.next())?;
        let target = state.get_address(parameters.next())?;

        state.set_value(target, op1 * op2)?;
        Ok(StepResult::Continue)
    }
}

struct Stop;
impl Instruction for Stop {
    fn calc(
        _state: &mut State,
        _parameters: ParamModeDispenser,
    ) -> Result<StepResult, ComputerError> {
        Ok(StepResult::Halted)
    }
}

struct Input;
impl Instruction for Input {
    fn calc(
        state: &mut State,
        parameters: ParamModeDispenser,
    ) -> Result<StepResult, ComputerError> {
        if let Some(value) = state.get_input() {
            let target = state.get_address(parameters.next())?;
            state.set_value(target, value)?;
            Ok(StepResult::Continue)
        } else {
            state.repeat();
            Ok(StepResult::Waiting)
        }
    }
}

struct Output;
impl Instruction for Output {
    fn calc(
        state: &mut State,
        parameters: ParamModeDispenser,
    ) -> Result<StepResult, ComputerError> {
        let op1 = state.get_value(parameters.next())?;
        Ok(StepResult::Output(op1))
    }
}

struct JumpIfTrue;
impl Instruction for JumpIfTrue {
    fn calc(
        state: &mut State,
        parameters: ParamModeDispenser,
    ) -> Result<StepResult, ComputerError> {
        let test = state.get_value(parameters.next())?;
        let target = state.get_value(parameters.next())?;
        if test != 0 {
            state.set_pointer(target.try_into()?);
        }
        Ok(StepResult::Continue)
    }
}

struct JumpIfFalse;
impl Instruction for JumpIfFalse {
    fn calc(
        state: &mut State,
        parameters: ParamModeDispenser,
    ) -> Result<StepResult, ComputerError> {
        let test = state.get_value(parameters.next())?;
        let target = state.get_value(parameters.next())?;
        if test == 0 {
            state.set_pointer(target.try_into()?);
        }
        Ok(StepResult::Continue)
    }
}

struct LessThan;
impl Instruction for LessThan {
    fn calc(
        state: &mut State,
        parameters: ParamModeDispenser,
    ) -> Result<StepResult, ComputerError> {
        let op1 = state.get_value(parameters.next())?;
        let op2 = state.get_value(parameters.next())?;
        let target = state.get_address(parameters.next())?;

        let result = if op1 < op2 { 1 } else { 0 };
        state.set_value(target, result)?;
        Ok(StepResult::Continue)
    }
}

struct Equals;
impl Instruction for Equals {
    fn calc(
        state: &mut State,
        parameters: ParamModeDispenser,
    ) -> Result<StepResult, ComputerError> {
        let op1 = state.get_value(parameters.next())?;
        let op2 = state.get_value(parameters.next())?;
        let target = state.get_address(parameters.next())?;

        let result = if op1 == op2 { 1 } else { 0 };
        state.set_value(target, result)?;
        Ok(StepResult::Continue)
    }
}
