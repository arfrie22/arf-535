use std::{cell::RefCell, rc::Rc};

use enums::{FPRegister, Register, Timer};
use instruction::Instruction;
use memory::{FrontMemory, InnerMemory};
use pipeline::{
    DecodeStage, DecodeState, ExecuteResult, ExecuteStage, ExecuteState, FetchResult, FetchStage, MemoryStage, MemoryState, PipelineOutter, PipelineStage, WritebackRegister, WritebackStage, WritebackState
};

pub mod enums;
pub mod instruction;
pub mod memory;
pub mod pipeline;

#[derive(Debug, Default, Clone)]
pub struct TimerState {
    pub previous_set: u32,
    pub value: u32,
}

#[derive(Debug, Clone)]
pub struct PipelineStages {
    pub fetch: PipelineCell<FetchStage>,
    pub decode: PipelineCell<DecodeStage>,
    pub execute: PipelineCell<ExecuteStage>,
    pub memory: PipelineCell<MemoryStage>,
    pub write_back: PipelineCell<WritebackStage>,
}

#[derive(Debug, Clone)]
pub struct InFlightRegisters {
    pub registers: [usize; 32],
    pub f_registers: [usize; 32],
    pub timers: [usize; 32],
}

#[derive(Debug, Clone, Default)]
pub struct RegisterSet {
    pub registers: Vec<Register>,
    pub f_registers: Vec<FPRegister>,
    pub timers: Vec<Timer>,
}

#[derive(Debug, Clone, Default)]
pub struct StageBool {
    pub fetch: bool,   
    pub decode: bool,   
    pub execute: bool,   
    pub memory: bool,   
    pub writeback: bool,   
}

#[derive(Debug, Clone)]
pub struct SimulatorState {
    pub running: bool,
    pub registers: [u32; 32],
    pub f_registers: [f32; 32],
    pub timers: [TimerState; 32],
    pub program_memory: MemoryCell,
    pub data_memory: MemoryCell,
    pub pipeline_stage: PipelineStages,
    pub inflight: InFlightRegisters,
    pub squashes: StageBool,
    pub fetch_state: Option<u32>,
    pub fetch_result: Option<FetchResult>,
    pub decode_state: Option<DecodeState>,
    pub decode_result: Option<ExecuteState>,
    pub execute_state: Option<ExecuteState>,
    pub execute_result: Option<MemoryState>,
    pub memory_state: Option<MemoryState>,
    pub memory_result: Option<WritebackState>,
    pub writeback_state: Option<WritebackState>,
    pub hold_fetch: bool,
    pub single_instruction_pipeline: bool,
}

impl SimulatorState {
    pub fn new(program_memory: MemoryCell, data_memory: MemoryCell) -> Rc<RefCell<Self>> {    
        let fetch = Rc::new(RefCell::new(PipelineStage::<FetchStage>::new(None)));
        let decode = Rc::new(RefCell::new(PipelineStage::<DecodeStage>::new(Some(
            fetch.clone(),
        ))));
        let execute = Rc::new(RefCell::new(PipelineStage::<ExecuteStage>::new(Some(
            decode.clone(),
        ))));
        let memory = Rc::new(RefCell::new(PipelineStage::<MemoryStage>::new(Some(
            execute.clone(),
        ))));
        let write_back = Rc::new(RefCell::new(PipelineStage::<WritebackStage>::new(Some(
            memory.clone(),
        ))));

        let simulator = Rc::new(RefCell::new(Self {
            running: false,
            registers: [0; 32],
            f_registers: [0.0; 32],
            timers: Default::default(),
            program_memory,
            data_memory,
            pipeline_stage: PipelineStages {
                fetch,
                decode,
                execute,
                memory,
                write_back,
            },
            inflight: InFlightRegisters {
                registers: [0; 32],
                f_registers: [0; 32],
                timers: [0; 32],
            },
            squashes: Default::default(),
            fetch_state: None,
            fetch_result: None,
            decode_state: None,
            decode_result: None,
            execute_state: None,
            execute_result: None,
            memory_state: None,
            memory_result: None,
            writeback_state: None,
            hold_fetch: false,
            single_instruction_pipeline: false,
        }));

        simulator
            .borrow()
            .pipeline_stage
            .write_back
            .borrow_mut()
            .initalize_simulator_cell(simulator.clone());
        simulator
            .borrow()
            .pipeline_stage
            .memory
            .borrow_mut()
            .initalize_simulator_cell(simulator.clone());
        simulator
            .borrow()
            .pipeline_stage
            .execute
            .borrow_mut()
            .initalize_simulator_cell(simulator.clone());
        simulator
            .borrow()
            .pipeline_stage
            .decode
            .borrow_mut()
            .initalize_simulator_cell(simulator.clone());
        simulator
            .borrow()
            .pipeline_stage
            .fetch
            .borrow_mut()
            .initalize_simulator_cell(simulator.clone());

        simulator
    }
}

#[derive(Debug, Clone)]
pub struct Simulator {
    state: SimulatorStateCell,
    raw_program_memory: RawMemoryCell,
    raw_data_memory: RawMemoryCell,
}

impl Simulator {
    pub fn new(raw_program_memory: RawMemoryCell, raw_data_memory: RawMemoryCell, program_memory: MemoryCell, data_memory: MemoryCell) -> Self {
        Self {
            state: SimulatorState::new(program_memory, data_memory),
            raw_program_memory,
            raw_data_memory,
        }
    }

    pub fn cycle(&self) {
        let stage = self.state.borrow().pipeline_stage.write_back.clone();
        stage.borrow_mut().call(false).unwrap();
    }

    pub fn get_state(&self) -> SimulatorStateCell {
        self.state.clone()
    }
}

type SimulatorStateCell = Rc<RefCell<SimulatorState>>;
type MemoryCell = Rc<RefCell<dyn FrontMemory>>;
type RawMemoryCell = Rc<RefCell<dyn InnerMemory>>;
type PipelineCell<T> = Rc<RefCell<PipelineStage<T>>>;

#[cfg(test)]
mod tests {
    use crate::instruction::Instruction;

    #[test]
    fn it_works() {
        let a: u32 = 0x050FC000;
        println!("{:?}", Instruction::from(a));
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
