#![feature(iter_partition_in_place)]
#![feature(slice_group_by)]
use days::{day_provider, read_string, DayTrait, DayType, PartType, ResultType, UnitResult};
use std::{env, time};

mod common;
mod days;
mod int_code;
mod macros;

fn output(day: DayType, part: PartType, result: ResultType, time: time::Duration) {
    match result {
        ResultType::Integer(value) => {
            println!(
                "Day {:02} part {}: {} ({})",
                day,
                part,
                value,
                time.as_secs_f64()
            );
        }
        ResultType::String(value) => {
            println!(
                "Day {:02} part {}: {} ({})",
                day,
                part,
                value,
                time.as_secs_f32()
            );
        }
        ResultType::Lines(value) => {
            println!(
                "Day {:02} part {}: {} ({})",
                day,
                part,
                value[0],
                time.as_secs_f32()
            );
            for line in &value[1..] {
                println!("               {line}");
            }
        }
        ResultType::Nothing => {}
    }
}

fn run_part(day: &dyn DayTrait, is_part1: bool, input: &str) -> anyhow::Result<time::Duration> {
    let now = time::Instant::now();
    let result = if is_part1 {
        day.part1(input)?
    } else {
        day.part2(input)?
    };

    if matches!(result, ResultType::Nothing) {
        Ok(time::Duration::ZERO)
    } else {
        let elapsed = now.elapsed();
        output(
            day.get_day_number(),
            if is_part1 { 1 } else { 2 },
            result,
            elapsed,
        );
        Ok(elapsed)
    }
}

fn run(day: &dyn DayTrait, part1: bool, part2: bool) -> anyhow::Result<time::Duration> {
    let input = read_string(day.get_day_number(), "input.txt")?;
    let elapsed1 = if part1 {
        run_part(day, true, &input)?
    } else {
        time::Duration::ZERO
    };
    let elapsed2 = if part2 {
        run_part(day, false, &input)?
    } else {
        time::Duration::ZERO
    };

    Ok(elapsed1 + elapsed2)
}

#[derive(Debug, thiserror::Error)]
enum ParamError {
    #[error("Too many Parameters: {0}")]
    TooManyParameters(usize),

    #[error("Unknown Part: {0}")]
    UnknownPart(PartType),
}

fn run_on_parameters(params: &[String]) -> UnitResult {
    match params.len() {
        0 => {
            let mut runtime = time::Duration::ZERO;
            for day in day_provider::get_all_days() {
                runtime += run(day.as_ref(), true, true)?;
            }
            println!();
            println!("Runtime: {}", runtime.as_secs_f32());
        }
        1 => {
            let mut parts = params[0].split('/');
            if let Some(day_str) = parts.next() {
                let day_number = day_str.parse::<DayType>()?;
                let day = day_provider::get_day(day_number)?;

                if let Some(part_str) = parts.next() {
                    match part_str.parse::<PartType>()? {
                        1 => run(day.as_ref(), true, false)?,
                        2 => run(day.as_ref(), false, true)?,
                        p => Err(ParamError::UnknownPart(p))?,
                    };
                } else {
                    let runtime = run(day.as_ref(), true, true)?;
                    println!("Runtime: {}", runtime.as_secs_f32());
                }
            }
        }
        n => Err(ParamError::TooManyParameters(n))?,
    }
    Ok(())
}

fn main() -> UnitResult {
    let params = env::args().skip(1).collect::<Vec<_>>();
    run_on_parameters(&params)
}
