use std::{cell::RefCell, rc::Rc};

use instruction::Instruction;
use memory::FrontMemory;
use pipeline::{DecodeStage, ExecuteResult, ExecuteStage, FetchStage, MemoryStage, PipelineStage, WritebackRegister, WritebackStage};

pub mod memory;
pub mod pipeline;
pub mod enums;
pub mod instruction;

#[derive(Debug, Clone)]
pub struct TimerState {
    previous_set: u32,
    value: u32,
}

#[derive(Debug, Clone)]
pub struct PipelineStages {
    fetch: Rc<RefCell<PipelineStage<FetchStage>>>,
    decode: Rc<RefCell<PipelineStage<DecodeStage>>>,
    execute: Rc<RefCell<PipelineStage<ExecuteStage>>>,
    memory: Rc<RefCell<PipelineStage<MemoryStage>>>,
    write_back: Rc<RefCell<PipelineStage<WritebackStage>>>,
}

#[derive(Debug, Clone)]
pub struct SimulatorState {
    running: bool,
    registers: [u32; 32],
    f_registers: [f32; 32],
    timers: [TimerState; 32],
    program_memory: Rc<RefCell<dyn FrontMemory>>,
    data_memory: Rc<RefCell<dyn FrontMemory>>,
    pipeline_stage: PipelineStages,
    fetch_address: Option<u32>,
    fetch_result: Option<u32>,
    decode_result: Option<Instruction>,
    execute_result: Option<ExecuteResult>,
    memory_result: Option<WritebackRegister>,
}

type SimulatorStateCell = Rc<RefCell<SimulatorState>>;


#[cfg(test)]
mod tests {
    use crate::instruction::Instruction;

    #[test]
    fn it_works() {
        let a: u32 = 0x050FC000;
        println!("{:?}", Instruction::from(a));
        let result = 2+2;
        assert_eq!(result, 4);
    }
}