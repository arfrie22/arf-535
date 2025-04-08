use std::{collections::HashMap, num::ParseIntError};

use pest::Parser;
use pest_derive::Parser;
use simulator::{
    enums::{ParseError, Register},
    instruction::Instruction,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssemblerError {
    ParseError(ParseError),
    PestError(pest::error::Error<Rule>),
}

impl From<ParseError> for AssemblerError {
    fn from(value: ParseError) -> Self {
        AssemblerError::ParseError(value)
    }
}

impl From<ParseIntError> for AssemblerError {
    fn from(value: ParseIntError) -> Self {
        AssemblerError::ParseError(ParseError::InvalidInput)
    }
}

impl From<pest::error::Error<Rule>> for AssemblerError {
    fn from(value: pest::error::Error<Rule>) -> Self {
        AssemblerError::PestError(value)
    }
}

// TODO: signed number for _offset_

fn parse_number(input: &str) -> Result<u32, AssemblerError> {
    if input.is_empty() {
        Ok(0)
    } else if input.starts_with("0b") {
        Ok(u32::from_str_radix(&input[2..], 2)?)
    } else if input.starts_with("0x") {
        Ok(u32::from_str_radix(&input[2..], 16)?)
    } else {
        Ok(u32::from_str_radix(input, 10)?)
    }
}

fn parse_signed_number(input: &str) -> Result<u32, AssemblerError> {
    let i = 
    if input.is_empty() {
        0
    } else if input.starts_with("0b") {
        i32::from_str_radix(&input[2..], 2)?
    } else if input.starts_with("0x") {
        i32::from_str_radix(&input[2..], 16)?
    } else if input[1..].starts_with("0b") {
        let mut str = input[3..].to_owned();
        str.insert(0, input.chars().next().unwrap());
        i32::from_str_radix(&str, 2)?
    } else if input[1..].starts_with("0x") {
        let mut str = input[3..].to_owned();
        str.insert(0, input.chars().next().unwrap());
        i32::from_str_radix(&str, 16)?
    } else {
        i32::from_str_radix(input, 10)?
    };

    Ok(u32::from_ne_bytes(i.to_ne_bytes()))
}

fn parse_label(input: &str, labels: &HashMap<String, u32>) -> Result<u32, AssemblerError> {
    if let Some(v) = labels.get(input) {
        Ok(*v)
    } else {
        if input.starts_with("p") || input.starts_with("d") {
            parse_number(&input[1..])
        } else {
            parse_number(input)
        }
    }
}

#[derive(Parser)]
#[grammar = "assembler.pest"] 
pub struct AssemblerParser;

mod gen_functions;
pub use gen_functions::assemble;

#[cfg(test)]
mod tests {
    use crate::assemble;

    #[test]
    fn it_works() {
        let input = "
        test_2: 
        SWP pc r3
        SWP R1 f1
        STR [R1  + 0x4] R2
        STR [R1 + 0x4 << 0x2] F3
        ";

        assemble(input);
    }
}
