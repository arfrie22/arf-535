use pest::Parser;
use pest_derive::Parser;
use simulator::instruction::Instruction;

#[derive(Parser)]
#[grammar = "assembler.pest"]
pub struct AssemblerParser;

pub fn assemble(input: &str) -> Vec<Instruction> {
    let vec = AssemblerParser::parse(Rule::file, input).unwrap();
    vec![]
}

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
