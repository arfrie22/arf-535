use std::{cell::RefCell, rc::Rc};

use enums::{FPRegister, Register, Timer};
use memory::{Cache, FrontMemory, InnerMemory, Memory};
use pipeline::{
    DecodeStage, DecodeState, ExecuteStage, ExecuteState, FetchResult, FetchStage, MemoryStage, MemoryState, PipelineOutter, PipelineStage, WritebackStage, WritebackState
};
use streams::{InputStream, OutputStream};

pub mod enums;
pub mod instruction;
pub mod memory;
pub mod pipeline;
pub mod streams;

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

#[inline]
pub fn raw_cast_from_f32(input: f32) -> u32 {
    u32::from_ne_bytes(input.to_ne_bytes())
}

#[inline]
pub fn raw_cast_to_f32(input: u32) -> f32 {
    f32::from_ne_bytes(input.to_ne_bytes())
}

#[inline]
pub fn raw_cast_from_i32(input: i32) -> u32 {
    u32::from_ne_bytes(input.to_ne_bytes())
}

#[inline]
pub fn raw_cast_to_i32(input: u32) -> i32 {
    i32::from_ne_bytes(input.to_ne_bytes())
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

#[derive(Debug)]
pub struct Simulator {
    state: SimulatorStateCell,
    raw_program_memory: RawMemoryCell,
    raw_data_memory: RawMemoryCell,
    raw_program_cache: RawCacheCell,
    raw_data_cache: RawCacheCell,
    adc_streams: [Box<dyn InputStream>; 4],
    dac_streams: [Box<dyn OutputStream>; 4],
    cycle_number: usize,
}

impl Simulator {
    pub fn new(raw_program_memory: RawMemoryCell, raw_data_memory: RawMemoryCell, raw_program_cache: RawCacheCell, raw_data_cache: RawCacheCell, program_memory: MemoryCell, data_memory: MemoryCell, mut adc_streams: [Box<dyn InputStream>; 4], mut dac_streams: [Box<dyn OutputStream>; 4], clock_rate: usize) -> Self {
        adc_streams.iter_mut().for_each(|stream| {
            stream.set_clock_rate(clock_rate);
        });

        dac_streams.iter_mut().for_each(|stream| {
            stream.set_clock_rate(clock_rate);
        });

        Self {
            state: SimulatorState::new(program_memory, data_memory),
            raw_program_memory,
            raw_data_memory,
            raw_program_cache,
            raw_data_cache,
            adc_streams,
            dac_streams,
            cycle_number: 0,
        }
    }

    pub fn cycle(&mut self) {
        self.state.borrow_mut().registers[Register::A1 as usize] = self.adc_streams[0].get_next();
        self.state.borrow_mut().registers[Register::A2 as usize] = self.adc_streams[1].get_next();
        self.state.borrow_mut().registers[Register::A3 as usize] = self.adc_streams[2].get_next();
        self.state.borrow_mut().registers[Register::A4 as usize] = self.adc_streams[3].get_next();

        self.state.borrow_mut().timers.iter_mut().for_each(|v| {
            v.value = v.value.saturating_sub(1);
        });

        let stage = self.state.borrow().pipeline_stage.write_back.clone();
        stage.borrow_mut().call(false).unwrap();

        self.dac_streams[0].set_next(self.state.borrow_mut().registers[Register::D1 as usize]);
        self.dac_streams[1].set_next(self.state.borrow_mut().registers[Register::D2 as usize]);
        self.dac_streams[2].set_next(self.state.borrow_mut().registers[Register::D3 as usize]);
        self.dac_streams[3].set_next(self.state.borrow_mut().registers[Register::D4 as usize]);

        self.cycle_number += 1;
    }

    pub fn get_cycle_number(&self) -> usize {
        self.cycle_number
    }

    pub fn get_state(&self) -> SimulatorStateCell {
        self.state.clone()
    }

    pub fn get_data_memory(&self) -> RawMemoryCell {
        self.raw_data_memory.clone()
    }

    pub fn get_program_memory(&self) -> RawMemoryCell {
        self.raw_program_memory.clone()
    }

    pub fn get_data_cache(&self) -> RawCacheCell {
        self.raw_data_cache.clone()
    }

    pub fn get_program_cache(&self) -> RawCacheCell {
        self.raw_program_cache.clone()
    }
}

type SimulatorStateCell = Rc<RefCell<SimulatorState>>;
type MemoryCell = Rc<RefCell<dyn FrontMemory>>;
type RawMemoryCell = Rc<RefCell<Memory>>;
type RawCacheCell = Rc<RefCell<dyn Cache>>;
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
