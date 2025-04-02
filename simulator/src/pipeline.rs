use std::{cell::RefCell, fmt, marker::PhantomData, rc::Rc};

use crate::{
    enums::{FPRegister, Register, Timer}, instruction::Instruction, memory::line_offset, InFlightRegisters, RegisterSet, SimulatorState, SimulatorStateCell
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PipelineError {
    Stalled,
}

pub trait PipelineInner: fmt::Debug {
    fn call(state: &SimulatorStateCell, blocked: bool) -> Result<(), PipelineError>;
    fn squash(state: &SimulatorStateCell) -> Result<(), PipelineError>;
}

pub trait PipelineOutter: fmt::Debug {
    fn call(&mut self, blocked: bool) -> Result<(), PipelineError>;
    fn squash(&mut self) -> Result<(), PipelineError>;
}

fn decrement_inflight(inflight: &mut InFlightRegisters, register_set: &RegisterSet) {
    for r in &register_set.registers {
        inflight.registers[*r as usize] -= 1;
    }

    for r in &register_set.f_registers {
        inflight.f_registers[*r as usize] -= 1;
    }

    for t in &register_set.timers {
        inflight.timers[*t as usize] -= 1;
    }
}

fn increment_inflight(inflight: &mut InFlightRegisters, register_set: &RegisterSet) {
    for r in &register_set.registers {
        inflight.registers[*r as usize] += 1;
    }

    for r in &register_set.f_registers {
        inflight.f_registers[*r as usize] += 1;
    }

    for t in &register_set.timers {
        inflight.timers[*t as usize] += 1;
    }
}

fn check_inflight(inflight: &InFlightRegisters, register_set: &RegisterSet) -> bool {
    for r in &register_set.registers {
        if inflight.registers[*r as usize] > 0 {
            return true;
        }
    }

    for r in &register_set.f_registers {
        if inflight.f_registers[*r as usize] > 0 {
            return true;
        }
    }

    for t in &register_set.timers {
        if inflight.timers[*t as usize] > 0 {
            return true;
        }
    }

    false
}

#[derive(Debug, Clone)]
pub struct PipelineStage<T: PipelineInner> {
    next: Option<Rc<RefCell<dyn PipelineOutter>>>,
    simulator: Option<SimulatorStateCell>,
    inner: PhantomData<T>,
}

impl<T: PipelineInner> PipelineStage<T> {
    pub fn new(next: Option<Rc<RefCell<dyn PipelineOutter>>>) -> Self {
        Self { next, simulator: None, inner: PhantomData }
    }

    pub fn initalize_simulator_cell(&mut self, simulator: SimulatorStateCell) {
        self.simulator = Some(simulator)
    }
}

impl<T: PipelineInner> PipelineOutter for PipelineStage<T> {
    fn call(&mut self, blocked: bool) -> Result<(), PipelineError> {
        let res = T::call(self.simulator.as_ref().unwrap(), blocked);

        if let Some(next) = &self.next {
            let blocked = match (blocked, res) {
                (true, _) => true,
                (false, Err(PipelineError::Stalled)) => true,
                (false, _) => false,
            };

            // if done and previous is done, take next down line
            match next.borrow_mut().call(blocked) {
                _ => {}
                // Ok(()) => todo!(),
                // Err(PipelineError::Stalled) => todo!(),
                // Err(e) => todo!(),
            }
        }

        res
    }

    fn squash(&mut self) -> Result<(), PipelineError> {
        match T::squash(self.simulator.as_ref().unwrap()) {
            Ok(v) => {
                Ok(v)
            }
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug)]
pub struct FetchStage;
impl PipelineInner for FetchStage {
    fn call(state: &SimulatorStateCell, blocked: bool) -> Result<(), PipelineError> {
        let mut state_ref = state.borrow_mut();
        if state_ref.single_instruction_pipeline && state_ref.hold_fetch {
            return Ok(());
        }

        if state_ref.fetch_state.is_none() {
            if state_ref.fetch_result.is_none() {
                let address = state_ref.registers[Register::PC as usize];
                state_ref.registers[Register::PC as usize] += 1;
                state_ref.fetch_state = Some(address);
                state_ref.squashes.fetch = false;
            } else {
                return Err(PipelineError::Stalled);
            }
        }

        if state_ref.squashes.fetch {
            return if blocked {
                Err(PipelineError::Stalled)
            } else {
                Ok(())
            }
        }

        let res = if state_ref.fetch_result.is_none() {
            let address = state_ref.fetch_state.unwrap();

            let res = state_ref
                .program_memory
                .borrow_mut()
                .fetch(1, address);

            match res {
                Ok(v) => {
                    state_ref.fetch_result = Some(v[line_offset(address as usize)]);
                    state_ref.fetch_state = None;
                    state_ref.hold_fetch = true;
                    Ok(())
                }
                Err(_) => Err(PipelineError::Stalled),
            }
        } else {
            Ok(())
        };

        if blocked {
            Err(PipelineError::Stalled)
        } else {
            res
        }
    }

    fn squash(state: &SimulatorStateCell) -> Result<(), PipelineError> {
        let mut state_ref = state.borrow_mut();
        let _ = state_ref
            .program_memory
            .borrow_mut()
            .cancel(1);
        state_ref.fetch_state = None;
        state_ref.fetch_result = None; 
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct DecodeState {
    pub instruction: Instruction,
    pub read_registers: RegisterSet,
    pub write_registers: RegisterSet,
}

#[derive(Debug, Clone)]
pub struct ExecuteState {
    pub instruction: Instruction,
    pub registers: RegisterSet,
    pub timer: usize,
}

#[derive(Debug)]
pub struct DecodeStage;
impl PipelineInner for DecodeStage {
    fn call(state: &SimulatorStateCell, blocked: bool) -> Result<(), PipelineError> {
        let mut state_ref = state.borrow_mut();
        if state_ref.decode_state.is_none() {
            if state_ref.decode_result.is_none() {
                if let Some(result) = state_ref.fetch_result.take() {
                    let instruction = Instruction::from(result);
                    let read_registers = instruction.read_registers();
                    let write_registers = instruction.write_registers();
                    state_ref.decode_state = Some(DecodeState {
                        instruction,
                        read_registers,
                        write_registers,
                    });
                    state_ref.squashes.decode = state_ref.squashes.fetch;
                } else {
                    return Ok(());
                }
            } else {
                return Err(PipelineError::Stalled);
            }
        }

        if blocked {
            Err(PipelineError::Stalled)
        } else if state_ref.squashes.decode {
            Ok(())
        } else if check_inflight(
            &state_ref.inflight,
            &state_ref.decode_state.as_ref().unwrap().read_registers,
        ) {
            Err(PipelineError::Stalled)
        } else {
            let decode_state = state_ref.decode_state.take().unwrap();
            let timer = decode_state.instruction.cycle_count(&mut state_ref);
            increment_inflight(
                &mut state_ref.inflight,
                &decode_state.write_registers,
            );
            state_ref.decode_result = Some(ExecuteState {
                instruction: decode_state.instruction,
                registers: decode_state.write_registers,
                timer,
            });

            Ok(())
        }
    }

    fn squash(state: &SimulatorStateCell) -> Result<(), PipelineError> {
        let mut state_ref = state.borrow_mut();
        let instruction = state_ref.decode_state.clone().unwrap().instruction;
        state_ref.decode_state = None;
        state_ref.decode_result = Some(ExecuteState { instruction, registers: Default::default(), timer: 1 });
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryBank {
    Data,
    Program,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MemoryAction {
    #[default]
    None,
    Read(MemoryBank, u32),
    Write(MemoryBank, u32, u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WritebackRegister {
    Standard(Register, Option<u32>),
    FloatingPoint(FPRegister, Option<u32>),
    Timer(Timer, Option<u32>),
}

#[derive(Debug, Clone, Default)]
pub struct ExecuteResult {
    pub memory: MemoryAction,
    pub writeback: Vec<WritebackRegister>,
}

impl Instruction {
    pub fn cycle_count(&self, state: &mut SimulatorState) -> usize {
        match self {
            _ => 2,
        }
    }
    pub fn execute(&self, state: &mut SimulatorState) -> ExecuteResult {
        match self {
            Instruction::Invalid(_) => Default::default(),
            Instruction::Trap => {
                state.running = false;
                Default::default()
            }
            Instruction::PushIntegerRegister { rx } => todo!(),
            Instruction::PushFloatingPointRegister { fx } => todo!(),
            Instruction::PopIntegerRegister { rx } => todo!(),
            Instruction::PopFloatingPointRegister { fx } => todo!(),
            Instruction::SwapRegister { rx, fy } => todo!(),
            Instruction::Stall { rx } => todo!(),
            Instruction::RegisterJump { condition, rx } => todo!(),
            Instruction::IndirectJump { condition, rx, i, s } => todo!(),
            Instruction::IndirectwithRegisterOffsetJump { condition, rx, ro, s } => todo!(),
            Instruction::RelativeJump { condition, rx } => {
                let pc = state.registers[Register::PC as usize];
                let val_rx = state.registers[*rx as usize];
                // TODO: Overflow bit
                // TODO: Check condition
                ExecuteResult { memory: MemoryAction::None, writeback: vec![WritebackRegister::Standard(Register::PC, Some(pc + val_rx))] }
            }
            Instruction::ImmediateJump { condition, label } => {
                // TODO: Check condition
                ExecuteResult { memory: MemoryAction::None, writeback: vec![WritebackRegister::Standard(Register::PC, Some(*label))] }
            }
            Instruction::ImmediateRelativeJump { condition, offset } => todo!(),
            Instruction::RegisterJumpwithLink { condition, rx } => todo!(),
            Instruction::IndirectJumpwithLink { condition, rx, i, s } => todo!(),
            Instruction::IndirectwithRegisterOffsetJumpwithLink { condition, rx, ro, s } => todo!(),
            Instruction::RelativeJumpwithLink { condition, rx } => todo!(),
            Instruction::ImmediateJumpwithLink { condition, label } => todo!(),
            Instruction::ImmediateRelativeJumpwithLink { condition, offset } => todo!(),
            Instruction::IntegerLoadLow { rx, value } => {
                let val_rx = state.registers[*rx as usize];
                ExecuteResult { memory: MemoryAction::None, writeback: vec![WritebackRegister::Standard(*rx, Some((val_rx & 0xFFFF0000) | *value))] }
            },
            Instruction::IntegerLoadHigh { rx, value } => {
                let val_rx = state.registers[*rx as usize];
                ExecuteResult { memory: MemoryAction::None, writeback: vec![WritebackRegister::Standard(*rx, Some((val_rx & 0x0000FFFF) | *value))] }
            },
            Instruction::SwapIntegerRegisters { rx, ry } => {
                let val_rx = state.registers[*rx as usize];
                let val_ry = state.registers[*ry as usize];
                ExecuteResult { memory: MemoryAction::None, writeback: vec![WritebackRegister::Standard(*rx, Some(val_ry)), WritebackRegister::Standard(*ry, Some(val_rx))] }
            },
            Instruction::CopyIntegerRegister { rx, ry } => {
                let val_ry = state.registers[*ry as usize];
                ExecuteResult { memory: MemoryAction::None, writeback: vec![WritebackRegister::Standard(*rx, Some(val_ry))] }
            },
            Instruction::LoadIntegerRegisterIndirect { rx, ry, i, s } => todo!(),
            Instruction::LoadIntegerRegisterIndirectwithRegisterOffset { rx, ry, ro, s } => todo!(),
            Instruction::LoadIntegerRegisterIndirectProgram { rx, ry, i, s } => todo!(),
            Instruction::LoadIntegerRegisterIndirectwithRegisterOffsetProgram { rx, ry, ro, s } => todo!(),
            Instruction::StoreIntegerRegisterIndirect { rx, ry, i, s } => todo!(),
            Instruction::StoreIntegerRegisterIndirectwithRegisterOffsetIndirect { rx, ry, ro, s } => todo!(),
            Instruction::StoreIntegerRegisterIndirectProgram { rx, ry, i, s } => todo!(),
            Instruction::StoreIntegerRegisterIndirectwithRegisterOffsetProgram { rx, ry, ro, s } => todo!(),
            Instruction::IntegerLoadData { rx, label } => {
                ExecuteResult { memory: MemoryAction::Read(MemoryBank::Data, *label), writeback: vec![WritebackRegister::Standard(*rx, None)] }
            },
            Instruction::IntegerLoadProgram { rx, label } => {
                ExecuteResult { memory: MemoryAction::Read(MemoryBank::Program, *label), writeback: vec![WritebackRegister::Standard(*rx, None)] }
            },
            Instruction::IntegerStoreData { rx, label } => {
                let val_rx = state.registers[*rx as usize];
                ExecuteResult { memory: MemoryAction::Write(MemoryBank::Data, *label, val_rx), writeback: Vec::new() }
            },
            Instruction::IntegerStoreProgram { rx, label } => {
                let val_rx = state.registers[*rx as usize];
                ExecuteResult { memory: MemoryAction::Write(MemoryBank::Program, *label, val_rx), writeback: Vec::new() }
            },
            Instruction::UnsignedZeroExtend { rx, ry, count } => todo!(),
            Instruction::SignExtend { rx, ry, count } => todo!(),
            Instruction::FloatingPointLoadLow { fx, value } => todo!(),
            Instruction::FloatingPointLoadHigh { fx, value } => todo!(),
            Instruction::SwapFloatingPointRegisters { fx, fy } => todo!(),
            Instruction::CopyFloatingPointRegister { fx, fy } => todo!(),
            Instruction::LoadFloatingPointRegisterIndirect { fx, ry, i, s } => todo!(),
            Instruction::LoadFloatingPointRegisterIndirectwithRegisterOffset { fx, ry, ro, s } => todo!(),
            Instruction::StoreFloatingPointRegisterIndirect { rx, fy, i, s } => todo!(),
            Instruction::StoreFloatingPointRegisterIndirectwithRegisterOffset { rx, fy, ro, s } => todo!(),
            Instruction::FloatingPointLoadData { rx, label } => todo!(),
            Instruction::FloatingPointStoreData { rx, label } => todo!(),
            Instruction::IntegerCompare { rx, ry } => todo!(),
            Instruction::IntegerCompareSingleAgainstZero { rx } => todo!(),
            Instruction::AddUnsignedInteger { rx, ry, rz } => {
                let val_ry = state.registers[*ry as usize];
                let val_rz = state.registers[*rz as usize];
                // TODO: Overflow bit
                ExecuteResult { memory: MemoryAction::None, writeback: vec![WritebackRegister::Standard(*rx, Some(val_ry + val_rz))] }
            },
            Instruction::SubtractUnsignedInteger { rx, ry, rz } => {
                let val_ry = state.registers[*ry as usize];
                let val_rz = state.registers[*rz as usize];
                // TODO: Underflow bit
                ExecuteResult { memory: MemoryAction::None, writeback: vec![WritebackRegister::Standard(*rx, Some(val_ry - val_rz))] }
            },
            Instruction::MultiplyUnsignedInteger { rx, ry, rz } => {
                let val_ry = state.registers[*ry as usize];
                let val_rz = state.registers[*rz as usize];
                // TODO: Overflow bit
                ExecuteResult { memory: MemoryAction::None, writeback: vec![WritebackRegister::Standard(*rx, Some(val_ry * val_rz))] }
            },
            Instruction::DivideUnsignedInteger { rx, ry, rz } => {
                let val_ry = state.registers[*ry as usize];
                let val_rz = state.registers[*rz as usize];
                // TODO: Div 0 bit
                ExecuteResult { memory: MemoryAction::None, writeback: vec![WritebackRegister::Standard(*rx, Some(val_ry / val_rz))] }
            },
            Instruction::ModuloUnsignedInteger { rx, ry, rz } => {
                let val_ry = state.registers[*ry as usize];
                let val_rz = state.registers[*rz as usize];
                // TODO: Div 0 bit
                ExecuteResult { memory: MemoryAction::None, writeback: vec![WritebackRegister::Standard(*rx, Some(val_ry % val_rz))] }
            },
            Instruction::AddSignedInteger { rx, ry, rz } => {
                let val_ry = state.registers[*ry as usize];
                let val_rz = state.registers[*rz as usize];
                // TODO: Overflow bit
                ExecuteResult { memory: MemoryAction::None, writeback: vec![WritebackRegister::Standard(*rx, Some(val_ry + val_rz))] }
            },
            Instruction::SubtractSignedInteger { rx, ry, rz } => todo!(),
            Instruction::MultiplySignedInteger { rx, ry, rz } => todo!(),
            Instruction::DivideSignedInteger { rx, ry, rz } => todo!(),
            Instruction::ModuloSignedInteger { rx, ry, rz } => todo!(),
            Instruction::BitwiseAND { rx, ry, rz } => todo!(),
            Instruction::BitwiseOR { rx, ry, rz } => todo!(),
            Instruction::BitwiseNOT { rx, ry, rz } => todo!(),
            Instruction::BitwiseXOR { rx, ry, rz } => todo!(),
            Instruction::LogicalShiftLeft { rx, ry, value } => todo!(),
            Instruction::LogicalShiftRight { rx, ry, value } => todo!(),
            Instruction::ArithmeticShiftLeft { rx, ry, value } => todo!(),
            Instruction::ArithmeticShiftRight { rx, ry, value } => todo!(),
            Instruction::RotateRight { rx, ry, value } => todo!(),
            Instruction::LogicalShiftLeftRegister { rx, ry, rz } => todo!(),
            Instruction::LogicalShiftRightRegister { rx, ry, rz } => todo!(),
            Instruction::ArithmeticShiftLeftRegister { rx, ry, rz } => todo!(),
            Instruction::ArithmeticShiftRightRegister { rx, ry, rz } => todo!(),
            Instruction::RotateRightRegister { rx, ry, rz } => todo!(),
            Instruction::MapUnsignedToSigned { rx, ry } => todo!(),
            Instruction::MapSignedToUnsigned { rx, ry } => todo!(),
            Instruction::FloatingPointCompare { fx, fy } => todo!(),
            Instruction::FloatingPointCompareSingleAgainstZero { fx } => todo!(),
            Instruction::AddFloatingPoint { fx, fy, fz } => todo!(),
            Instruction::SubtractFloatingPoint { fx, fy, fz } => todo!(),
            Instruction::MultiplyFloatingPoint { fx, fy, fz } => todo!(),
            Instruction::DivideFloatingPoint { fx, fy, fz } => todo!(),
            Instruction::CasttoFloat { fx, ry } => todo!(),
            Instruction::CastfromFloat { rx, fy } => todo!(),
            Instruction::SetTimer { tx, ry } => todo!(),
            Instruction::GetCurrentTimer { rx, ty } => todo!(),
            Instruction::CheckTimer { tx } => todo!(),
            Instruction::ClearTimer { tx } => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct ExecuteStage;
impl PipelineInner for ExecuteStage {
    fn call(state: &SimulatorStateCell, blocked: bool) -> Result<(), PipelineError> {
        let mut state_ref = state.borrow_mut();
        if state_ref.execute_state.is_none() {
            if state_ref.execute_result.is_none() {
                if let Some(result) = state_ref.decode_result.take() {
                    state_ref.execute_state = Some(result);
                    state_ref.squashes.execute = state_ref.squashes.decode;
                } else {
                    return Ok(());
                }
            } else {
                return Err(PipelineError::Stalled);
            }
        }

        if state_ref.squashes.execute {
            return if blocked {
                Err(PipelineError::Stalled)
            } else {
                Ok(())
            }
        }

        if state_ref.execute_state.as_ref().unwrap().timer > 0 {
            state_ref.execute_state.as_mut().unwrap().timer -= 1;
            Err(PipelineError::Stalled)
        } else if blocked {
            Err(PipelineError::Stalled)
        } else {
            let execute_state = state_ref.execute_state.take().unwrap();

            let execute = execute_state.instruction.execute(&mut state_ref);
            state_ref.execute_result = Some(MemoryState {
                memory: execute.memory,
                writeback: WritebackState {
                    registers: execute.writeback,
                    holds: execute_state.registers,
                },
            });

            Ok(())
        }
    }

    fn squash(state: &SimulatorStateCell) -> Result<(), PipelineError> {
        let mut state_ref = state.borrow_mut();
        let registers = state_ref.execute_state.as_ref().unwrap().registers.clone();
        decrement_inflight(
            &mut state_ref.inflight,
            &registers,
        );
        state_ref.execute_state = None;
        state_ref.execute_result = Some(MemoryState { memory: MemoryAction::None, writeback: WritebackState { registers: vec![], holds: Default::default() } });
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct MemoryState {
    pub memory: MemoryAction,
    pub writeback: WritebackState,
}

#[derive(Debug)]
pub struct MemoryStage;
impl PipelineInner for MemoryStage {
    fn call(state: &SimulatorStateCell, blocked: bool) -> Result<(), PipelineError> {
        let mut state_ref = state.borrow_mut();
        if state_ref.memory_state.is_none() {
            if state_ref.memory_result.is_none() {
                if let Some(result) = state_ref.execute_result.take() {
                    state_ref.memory_state = Some(result);
                    state_ref.squashes.memory = state_ref.squashes.execute;
                } else {
                    return Ok(());
                }
            } else {
                return Err(PipelineError::Stalled);
            }
        }

        if state_ref.squashes.memory {
            return if blocked {
                Err(PipelineError::Stalled)
            } else {
                Ok(())
            }
        }

        let res = if state_ref.memory_result.is_none() {
            match state_ref.memory_state.as_ref().unwrap().memory {
                MemoryAction::None => {
                    let wb = state_ref.memory_state.take().unwrap().writeback;
                    state_ref.memory_result = Some(wb);
                    Ok(())
                }
                MemoryAction::Read(memory_bank, address) => {
                    match match memory_bank {
                        MemoryBank::Data => state_ref
                            .data_memory
                            .borrow_mut()
                            .fetch(4, address),
                        MemoryBank::Program => state_ref
                            .program_memory
                            .borrow_mut()
                            .fetch(4, address),
                    } {
                        Ok(v) => {
                            let res = v[line_offset(address as usize)];
                            let mut wb = state_ref.memory_state.take().unwrap().writeback;
                            for r in wb.registers.iter_mut() {
                                let v = match r {
                                    WritebackRegister::Standard(_register, v) => v,
                                    WritebackRegister::FloatingPoint(_f_register, v) => v,
                                    WritebackRegister::Timer(_timer, v) => v,
                                };

                                if v.is_none() {
                                    *v = Some(res);
                                }
                            }

                            state_ref.memory_result = Some(wb);
                            Ok(())
                        }
                        Err(_) => Err(PipelineError::Stalled),
                    }
                }
                MemoryAction::Write(memory_bank, address, value) => {
                    match match memory_bank {
                        MemoryBank::Data => state_ref
                            .data_memory
                            .borrow_mut()
                            .store(4, address, value),
                        MemoryBank::Program => state_ref
                            .program_memory
                            .borrow_mut()
                            .store(4, address, value),
                    } {
                        Ok(_) => {
                            let wb = state_ref.memory_state.take().unwrap().writeback;
                            state_ref.memory_result = Some(wb);
                            Ok(())
                        }
                        Err(_) => Err(PipelineError::Stalled),
                    }
                }
            }
        } else {
            Ok(())
        };

        if blocked {
            Err(PipelineError::Stalled)
        } else {
            res
        }
    }

    fn squash(state: &SimulatorStateCell) -> Result<(), PipelineError> {
        let mut state_ref = state.borrow_mut();
        if let Some(state) = state_ref.memory_state.as_ref() {
            match state.memory {
                MemoryAction::None => {}
                MemoryAction::Read(memory_bank, _address) => match memory_bank {
                    MemoryBank::Data => state_ref
                        .data_memory
                        .borrow_mut()
                        .cancel(4)
                        .unwrap(),
                    MemoryBank::Program => state_ref
                        .program_memory
                        .borrow_mut()
                        .cancel(4)
                        .unwrap(),
                },
                MemoryAction::Write(memory_bank, _address, _value) => match memory_bank {
                    MemoryBank::Data => state_ref
                        .data_memory
                        .borrow_mut()
                        .cancel(4)
                        .unwrap(),
                    MemoryBank::Program => state_ref
                        .program_memory
                        .borrow_mut()
                        .cancel(4)
                        .unwrap(),
                },
            }

            let holds = state
            .writeback
            .holds.clone();

            decrement_inflight(
                &mut state_ref.inflight,
                &holds,
            );

            state_ref.memory_state = None;
            state_ref.memory_result = Some(WritebackState { registers: vec![], holds: Default::default() });
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct WritebackState {
    pub registers: Vec<WritebackRegister>,
    pub holds: RegisterSet,
}

#[derive(Debug)]
pub struct WritebackStage;
impl PipelineInner for WritebackStage {
    fn call(state: &SimulatorStateCell, blocked: bool) -> Result<(), PipelineError> {
        let (res, should_squash) = {
        let mut state_ref = state.borrow_mut();
        if state_ref.writeback_state.is_none() {
            if let Some(result) = state_ref.memory_result.take() {
                state_ref.writeback_state = Some(result);
                state_ref.squashes.writeback = state_ref.squashes.memory;
            } else {
                return Ok(());
            }
        }

        let mut should_squash = false;
        let res = if blocked {
            Err(PipelineError::Stalled)
        } else if state_ref.squashes.writeback {
            Ok(())
        } else {
            let wb_state = state_ref.writeback_state.take().unwrap();
            for r in wb_state.registers {
                match r {
                    WritebackRegister::Standard(register, value) => {
                        state_ref.registers[register as usize] = value.unwrap();
                        if register == Register::PC {
                            should_squash = true;
                        }
                    }
                    WritebackRegister::FloatingPoint(f_register, value) => {
                        state_ref.f_registers[f_register as usize] =
                            f32::from_ne_bytes(value.unwrap().to_ne_bytes());
                    }
                    WritebackRegister::Timer(timer, value) => {
                        state_ref.timers[timer as usize].previous_set = value.unwrap();
                        state_ref.timers[timer as usize].value = value.unwrap();
                    }
                }
            }

            decrement_inflight(&mut state_ref.inflight, &wb_state.holds);
            state_ref.hold_fetch = false;
            Ok(())
        };

        (res, should_squash)
        };

        if should_squash {
            let fetch = state.borrow().pipeline_stage.fetch.clone();
            fetch.borrow_mut().squash().unwrap();
            let decode = state.borrow().pipeline_stage.decode.clone();
            decode.borrow_mut().squash().unwrap();
            let execute = state.borrow().pipeline_stage.execute.clone();
            execute.borrow_mut().squash().unwrap();
            let memory = state.borrow().pipeline_stage.memory.clone();
            memory.borrow_mut().squash().unwrap();
        }

        res
    }

    fn squash(state: &SimulatorStateCell) -> Result<(), PipelineError> {
        let mut state_ref = state.borrow_mut();
        
        let holds = state_ref
        .writeback_state
        .as_ref()
        .unwrap()
        .holds.clone();

        decrement_inflight(
            &mut state_ref.inflight,
            &holds,
        );

        state_ref.writeback_state = None;
        state_ref.hold_fetch = false;
        Ok(())
    }
}

// For april 2 make sure you have enough
// load, store, branch, arithmatic operations
// load 2 add store, change counter, jump
// cache 1 cycle, dram 2 cycle
