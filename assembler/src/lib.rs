use std::{collections::HashMap, io::{self, Read, Write}, num::ParseIntError};

use pest_derive::Parser;
use simulator::{enums::ParseError, instruction::Instruction, Simulator};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssemblerError {
    ParseError(ParseError),
    PestError(pest::error::Error<Rule>),
    InvalidInstruction(Instruction),
}

impl From<ParseError> for AssemblerError {
    fn from(value: ParseError) -> Self {
        AssemblerError::ParseError(value)
    }
}

impl From<ParseIntError> for AssemblerError {
    fn from(_value: ParseIntError) -> Self {
        AssemblerError::ParseError(ParseError::InvalidInput)
    }
}

impl From<pest::error::Error<Rule>> for AssemblerError {
    fn from(value: pest::error::Error<Rule>) -> Self {
        AssemblerError::PestError(value)
    }
}

#[derive(Debug)]
pub struct AssembledData {
    instructions: Vec<Instruction>,
    data: [u32; 0xFFFF],
}

impl Default for AssembledData {
    fn default() -> Self {
        Self { instructions: Default::default(), data: [0; 0xFFFF] }
    }
}

impl AssembledData {
    pub fn to_file(&self, writer: impl Write) -> io::Result<()> {
        let mut scratch = [0; 0xFFFF * 4 * 2];
        let mut ptr = 0;

        for instruction in self.instructions.iter() {
            let v: u32 = (*instruction).into();
            scratch[ptr..ptr+4].copy_from_slice(&v.to_le_bytes());
            ptr += 4;
        }

        ptr = 0xFFFF * 4;

        for d in self.data.iter() {
            scratch[ptr..ptr+4].copy_from_slice(&d.to_le_bytes());
            ptr += 4;
        }

        zstd::stream::copy_encode(&scratch[..], writer, 22)
    }
}

pub fn load_file(reader: impl Read, simulator: &mut Simulator) -> io::Result<()> {
    let mut decoded = Vec::new();
    zstd::stream::copy_decode(reader, &mut decoded)?;

    let p = simulator.get_program_memory();
    let mut prog = p.borrow_mut();
    for i in 0..0xFFFF {
        let v = i * 4;
        let mut slice = [0; 4];
        slice.copy_from_slice(&decoded[v..v + 4]);
        prog.inner[i] = u32::from_le_bytes(slice);
    }

    let d = simulator.get_data_memory();
    let mut data = d.borrow_mut();
    for i in 0..0xFFFF {
        let v = (i * 4) + (0xFFFF * 4);
        let mut slice = [0; 4];
        slice.copy_from_slice(&decoded[v..v + 4]);
        data.inner[i] = u32::from_le_bytes(slice);
    }

    Ok(())
}

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

fn parse_signed_number(input: &str) -> Result<i32, AssemblerError> {
    Ok(if input.is_empty() {
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
    })
}

fn parse_prog_label(input: &str, prog_labels: &HashMap<String, u32>) -> Result<u32, AssemblerError> {
    if let Some(v) = prog_labels.get(input) {
        Ok(*v)
    } else {
        if input.starts_with("p") {
            parse_number(&input[1..])
        } else {
            parse_number(input)
        }
    }
}

fn parse_data_label(input: &str, data_labels: &HashMap<String, u32>) -> Result<u32, AssemblerError> {
    if let Some(v) = data_labels.get(input) {
        Ok(*v)
    } else {
        if input.starts_with("d") {
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
        .prog
        test_2: 
        SWP pc r3
        SWP R1 f1
        B p:test_4
        test_3: 
        STR [R1 ] R2
        STR [R1  + 0x4] R2
        STR [R1 + r1 << 0x2] F3
        test_4:
        B p:test_3
        BO -1
        BO 10
        .data
        a 1#10 2#0x1
        ";

        let a = assemble(input).unwrap();
        let mut v = Vec::new();
        a.to_file(&mut v).unwrap();

        println!("v: {}", v.len());
    }
}
