use std::{cell::RefCell, fmt, i32, marker::PhantomData, rc::Rc};

use crate::{
    enums::{Condition, FPRegister, Register, Timer}, instruction::Instruction, memory::line_offset, raw_cast_from_f32, raw_cast_from_i32, raw_cast_to_f32, raw_cast_to_i32, InFlightRegisters, RegisterSet, SimulatorState, SimulatorStateCell
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
        Self {
            next,
            simulator: None,
            inner: PhantomData,
        }
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
                _ => {} // Ok(()) => todo!(),
                        // Err(PipelineError::Stalled) => todo!(),
                        // Err(e) => todo!(),
            }
        }

        res
    }

    fn squash(&mut self) -> Result<(), PipelineError> {
        match T::squash(self.simulator.as_ref().unwrap()) {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FetchResult {
    pub pc: u32,
    pub value: u32,
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
            };
        }

        let res = if state_ref.fetch_result.is_none() {
            let address = state_ref.fetch_state.unwrap();

            let res = state_ref.program_memory.borrow_mut().fetch(1, address);

            match res {
                Ok(v) => {
                    state_ref.fetch_result = Some(FetchResult {pc: address, value: v[line_offset(address as usize)]});
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
        let _ = state_ref.program_memory.borrow_mut().cancel(1);
        state_ref.fetch_state = None;
        state_ref.fetch_result = None;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct DecodeState {
    pub pc: u32,
    pub instruction: Instruction,
    pub read_registers: RegisterSet,
    pub write_registers: RegisterSet,
}

#[derive(Debug, Clone)]
pub struct ExecuteState {
    pub pc: u32,
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
                    let instruction = Instruction::from(result.value);
                    let read_registers = instruction.read_registers();
                    let write_registers = instruction.write_registers();
                    state_ref.decode_state = Some(DecodeState {
                        pc: result.pc,
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
            if let Some(state) = state_ref.decode_state.take() {
                state_ref.decode_result = Some(ExecuteState {
                    pc: state.pc,
                    instruction: state.instruction,
                    registers: Default::default(),
                    timer: 1,
                });
            }
            Ok(())
        } else if check_inflight(
            &state_ref.inflight,
            &state_ref.decode_state.as_ref().unwrap().read_registers,
        ) {
            Err(PipelineError::Stalled)
        } else {
            let decode_state = state_ref.decode_state.take().unwrap();
            let timer = decode_state.instruction.cycle_count(&mut state_ref);
            increment_inflight(&mut state_ref.inflight, &decode_state.write_registers);
            state_ref.decode_result = Some(ExecuteState {
                pc: decode_state.pc,
                instruction: decode_state.instruction,
                registers: decode_state.write_registers,
                timer,
            });

            Ok(())
        }
    }

    fn squash(state: &SimulatorStateCell) -> Result<(), PipelineError> {
        let mut state_ref = state.borrow_mut();
        if let Some(state) = state_ref.decode_state.take() {
            state_ref.decode_result = Some(ExecuteState {
                pc: state.pc,
                instruction: state.instruction,
                registers: Default::default(),
                timer: 1,
            });
        }

        if state_ref.decode_result.is_some() {
            state_ref.squashes.decode = true;
        }
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WritebackRegister {
    Standard(Register, Option<u32>),
    FloatingPoint(FPRegister, Option<f32>),
    Timer(Timer, Option<u32>),
}

#[derive(Debug, Clone, Default)]
pub struct ExecuteResult {
    pub memory: MemoryAction,
    pub writeback: Vec<WritebackRegister>,
    pub end_running: bool,
}

impl Instruction {
    pub fn cycle_count(&self, state: &mut SimulatorState) -> usize {
        match self {
            Instruction::Invalid(_) => 1,
            Instruction::Trap => 1,
            Instruction::PushIntegerRegister { ..  } => 1,
            Instruction::PushFloatingPointRegister { .. } => 1,
            Instruction::PopIntegerRegister { .. } => 1,
            Instruction::PopFloatingPointRegister { .. } => 1,
            Instruction::SwapRegister { .. } => 1,
            Instruction::StallImmediate { value } => *value as usize,
            Instruction::StallRegister { rx } => state.registers[*rx as usize] as usize,
            Instruction::RegisterJump { .. } => 1,
            Instruction::IndirectJump { .. } => 1,
            Instruction::IndirectwithRegisterOffsetJump { .. } => 1,
            Instruction::RelativeJump { .. } => 1,
            Instruction::ImmediateJump { .. } => 1,
            Instruction::ImmediateRelativeJump { .. } => 1,
            Instruction::IntegerLoadLow { .. } => 1,
            Instruction::IntegerLoadHigh { .. } => 1,
            Instruction::SwapIntegerRegisters { .. } => 1,
            Instruction::CopyIntegerRegister { .. } => 1,
            Instruction::LoadIntegerRegisterIndirect { .. } => 1,
            Instruction::LoadIntegerRegisterIndirectwithRegisterOffset { .. } => 1,
            Instruction::LoadIntegerRegisterIndirectProgram { .. } => 1,
            Instruction::LoadIntegerRegisterIndirectwithRegisterOffsetProgram { .. } => 1,
            Instruction::StoreIntegerRegisterIndirect { .. } => 1,
            Instruction::StoreIntegerRegisterIndirectwithRegisterOffsetIndirect { .. } => 1,
            Instruction::StoreIntegerRegisterIndirectProgram { .. } => 1,
            Instruction::StoreIntegerRegisterIndirectwithRegisterOffsetProgram { .. } => 1,
            Instruction::IntegerLoadData { .. } => 1,
            Instruction::IntegerLoadProgram { .. } => 1,
            Instruction::IntegerStoreData { .. } => 1,
            Instruction::IntegerStoreProgram { .. } => 1,
            Instruction::UnsignedZeroExtend { .. } => 1,
            Instruction::SignExtend { .. } => 1,
            Instruction::FloatingPointLoadLow { .. } => 1,
            Instruction::FloatingPointLoadHigh { .. } => 1,
            Instruction::SwapFloatingPointRegisters { .. } => 1,
            Instruction::CopyFloatingPointRegister { .. } => 1,
            Instruction::LoadFloatingPointRegisterIndirect { .. } => 1,
            Instruction::LoadFloatingPointRegisterIndirectwithRegisterOffset { .. } => 1,
            Instruction::StoreFloatingPointRegisterIndirect { .. } => 1,
            Instruction::StoreFloatingPointRegisterIndirectwithRegisterOffset { .. } => 1,
            Instruction::FloatingPointLoadData { .. } => 1,
            Instruction::FloatingPointStoreData { .. } => 1,
            Instruction::IntegerCompare { .. } => 1,
            Instruction::IntegerCompareSingleAgainstZero { .. } => 1,
            Instruction::IncrementIntegerRegister { .. } => 1,
            Instruction::DecrementIntegerRegister { .. } => 1,
            Instruction::AddUnsignedInteger { .. } => 1,
            Instruction::SubtractUnsignedInteger { .. } => 1,
            Instruction::MultiplyUnsignedInteger { .. } => 1,
            Instruction::DivideUnsignedInteger { .. } => 6,
            Instruction::ModuloUnsignedInteger { .. } => 8,
            Instruction::AddSignedInteger { .. } => 1,
            Instruction::SubtractSignedInteger { .. } => 1,
            Instruction::MultiplySignedInteger { .. } => 1,
            Instruction::DivideSignedInteger { .. } => 6,
            Instruction::ModuloSignedInteger { .. } => 8,
            Instruction::BitwiseAND { .. } => 1,
            Instruction::BitwiseOR { .. } => 1,
            Instruction::BitwiseNOT { .. } => 1,
            Instruction::BitwiseXOR { .. } => 1,
            Instruction::LogicalShiftLeft { .. } => 1,
            Instruction::LogicalShiftRight { .. } => 1,
            Instruction::ArithmeticShiftLeft { .. } => 1,
            Instruction::ArithmeticShiftRight { .. } => 1,
            Instruction::RotateRight { .. } => 1,
            Instruction::LogicalShiftLeftRegister { .. } => 1,
            Instruction::LogicalShiftRightRegister { .. } => 1,
            Instruction::ArithmeticShiftLeftRegister { .. } => 1,
            Instruction::ArithmeticShiftRightRegister { .. } => 1,
            Instruction::RotateRightRegister { .. } => 1,
            Instruction::MapUnsignedToSigned { .. } => 1,
            Instruction::MapSignedToUnsigned { .. } => 1,
            Instruction::FloatingPointCompare { .. } => 1,
            Instruction::FloatingPointCompareSingleAgainstZero { .. } => 1,
            Instruction::AddFloatingPoint { .. } => 1,
            Instruction::SubtractFloatingPoint { .. } => 1,
            Instruction::MultiplyFloatingPoint { .. } => 1,
            Instruction::DivideFloatingPoint { .. } => 16,
            Instruction::CastToFloat { .. } => 8,
            Instruction::CastFromFloat { .. } => 10,
            Instruction::SetTimer { .. } => 1,
            Instruction::GetCurrentTimer { .. } => 1,
            Instruction::CheckTimer { tx } => state.timers[*tx as usize].value as usize,
            Instruction::ClearTimer { .. } => 1,
        }
    }
    pub fn execute(&self, state: &mut SimulatorState) -> ExecuteResult {
        match self {
            Instruction::Invalid(_) => Default::default(),
            Instruction::Trap => {
                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: Vec::new(),
                    end_running: true,
                }
            },
            Instruction::PushIntegerRegister { rx } => {
                let sp = state.registers[Register::SP as usize];
                let val_rx = state.registers[*rx as usize];
                ExecuteResult {
                    memory: MemoryAction::Write(MemoryBank::Data, sp, val_rx),
                    writeback: vec![
                        WritebackRegister::Standard(Register::SP, Some(sp.wrapping_add(1))),
                    ],
                    end_running: false,
                }
            },
            Instruction::PushFloatingPointRegister { fx } => {
                let sp = state.registers[Register::SP as usize];
                let val_fx = state.f_registers[*fx as usize];
                ExecuteResult {
                    memory: MemoryAction::Write(MemoryBank::Data, sp, raw_cast_from_f32(val_fx)),
                    writeback: vec![
                        WritebackRegister::Standard(Register::SP, Some(sp.wrapping_add(1))),
                    ],
                    end_running: false,
                }
            },
            Instruction::PopIntegerRegister { rx } => {
                let sp = state.registers[Register::SP as usize];
                ExecuteResult {
                    memory: MemoryAction::Read(MemoryBank::Data, sp),
                    writeback: vec![
                        WritebackRegister::Standard(*rx, None),
                        WritebackRegister::Standard(Register::SP, Some(sp.wrapping_sub(1))),
                    ],
                    end_running: false,
                }
            },
            Instruction::PopFloatingPointRegister { fx } => {
                let sp = state.registers[Register::SP as usize];
                ExecuteResult {
                    memory: MemoryAction::Read(MemoryBank::Data, sp),
                    writeback: vec![
                        WritebackRegister::FloatingPoint(*fx, None),
                        WritebackRegister::Standard(Register::SP, Some(sp.wrapping_sub(1))),
                    ],
                    end_running: false,
                }
            },
            Instruction::SwapRegister { rx, fy } => {
                let val_rx = state.registers[*rx as usize];
                let val_fy = state.f_registers[*fy as usize];
                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![
                        WritebackRegister::Standard(*rx, Some(raw_cast_from_f32(val_fy))),
                        WritebackRegister::FloatingPoint(*fy, Some(raw_cast_to_f32(val_rx))),
                    ],
                    end_running: false,
                }
            },
            Instruction::StallImmediate { .. } => Default::default(),
            Instruction::StallRegister { .. } => Default::default(),
            Instruction::RegisterJump { l, condition, rx } => {
                let st: u32 = state.registers[Register::ST as usize];
                if condition.check(st) {
                    let val_rx = state.registers[*rx as usize];
                    let mut writeback = vec![
                        WritebackRegister::Standard(Register::PC, Some(val_rx)),
                    ];

                    if *l {
                        let pc = state.registers[Register::PC as usize];
                        writeback.push(WritebackRegister::Standard(Register::LR, Some(pc)));
                    };

                    ExecuteResult {
                        memory: MemoryAction::None,
                        writeback,
                        end_running: false,
                    }
                } else {
                    Default::default()
                }
            },
            Instruction::IndirectJump {
                l,
                condition,
                rx,
                i,
                s,
            } => {
                let st: u32 = state.registers[Register::ST as usize];
                if condition.check(st) {
                    let val_rx = state.registers[*rx as usize];
                    let mut writeback = vec![
                        WritebackRegister::Standard(Register::PC, None),
                    ];

                    if *l {
                        let pc = state.registers[Register::PC as usize];
                        writeback.push(WritebackRegister::Standard(Register::LR, Some(pc)));
                    };

                    ExecuteResult {
                        memory: MemoryAction::Read(MemoryBank::Program, val_rx + (*i << *s)),
                        writeback,
                        end_running: false,
                    }
                } else {
                    Default::default()
                }
            },
            Instruction::IndirectwithRegisterOffsetJump {
                l,
                condition,
                rx,
                ro,
                s,
            } => {
                let st: u32 = state.registers[Register::ST as usize];
                if condition.check(st) {
                    let val_rx = state.registers[*rx as usize];
                    let val_ro = state.registers[*ro as usize];
                    let mut writeback = vec![
                        WritebackRegister::Standard(Register::PC, None),
                    ];

                    if *l {
                        let pc = state.registers[Register::PC as usize];
                        writeback.push(WritebackRegister::Standard(Register::LR, Some(pc)));
                    };

                    ExecuteResult {
                        memory: MemoryAction::Read(MemoryBank::Program, val_rx + (val_ro << *s)),
                        writeback,
                        end_running: false,
                    }
                } else {
                    Default::default()
                }
            },
            Instruction::RelativeJump { l, condition, rx } => {
                let st: u32 = state.registers[Register::ST as usize];
                if condition.check(st) {
                    let pc = state.registers[Register::PC as usize];
                    let val_rx = state.registers[*rx as usize];
                    let mut writeback = vec![
                        WritebackRegister::Standard(Register::PC, Some(pc.wrapping_add(val_rx))),
                    ];

                    if *l {
                        let pc = state.registers[Register::PC as usize];
                        writeback.push(WritebackRegister::Standard(Register::LR, Some(pc)));
                    };

                    ExecuteResult {
                        memory: MemoryAction::None,
                        writeback,
                        end_running: false,
                    }
                } else {
                    Default::default()
                }
            },
            Instruction::ImmediateJump { l, condition, label } => {
                let st: u32 = state.registers[Register::ST as usize];
                if condition.check(st) {
                    let mut writeback = vec![
                        WritebackRegister::Standard(Register::PC, Some(*label)),
                    ];

                    if *l {
                        let pc = state.registers[Register::PC as usize];
                        writeback.push(WritebackRegister::Standard(Register::LR, Some(pc)));
                    };

                    ExecuteResult {
                        memory: MemoryAction::None,
                        writeback,
                        end_running: false,
                    }
                } else {
                    Default::default()
                }
            },
            Instruction::ImmediateRelativeJump { l, condition, offset } => {
                let st: u32 = state.registers[Register::ST as usize];
                if condition.check(st) {
                    let pc = state.registers[Register::PC as usize];

                    let mut writeback = Vec::new();

                    if *offset < 0 {
                        writeback.push(WritebackRegister::Standard(Register::PC, Some(pc.wrapping_sub(raw_cast_from_i32(-(*offset))))));
                    } else {
                        writeback.push(WritebackRegister::Standard(Register::PC, Some(pc.wrapping_add(raw_cast_from_i32(*offset)))));
                    }

                    if *l {
                        let pc = state.registers[Register::PC as usize];
                        writeback.push(WritebackRegister::Standard(Register::LR, Some(pc)));
                    };

                    ExecuteResult {
                        memory: MemoryAction::None,
                        writeback,
                        end_running: false,
                    }
                } else {
                    Default::default()
                }
            },
            Instruction::IntegerLoadLow { rx, value } => {
                let val_rx = state.registers[*rx as usize];
                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![WritebackRegister::Standard(
                        *rx,
                        Some((val_rx & 0xFFFF0000) | *value),
                    )],
                    end_running: false,
                }
            },
            Instruction::IntegerLoadHigh { rx, value } => {
                let val_rx = state.registers[*rx as usize];
                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![WritebackRegister::Standard(
                        *rx,
                        Some((val_rx & 0x0000FFFF) | *value),
                    )],
                    end_running: false,
                }
            },
            Instruction::SwapIntegerRegisters { rx, ry } => {
                let val_rx = state.registers[*rx as usize];
                let val_ry = state.registers[*ry as usize];
                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![
                        WritebackRegister::Standard(*rx, Some(val_ry)),
                        WritebackRegister::Standard(*ry, Some(val_rx)),
                    ],
                    end_running: false,
                }
            },
            Instruction::CopyIntegerRegister { rx, ry } => {
                let val_ry = state.registers[*ry as usize];
                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![WritebackRegister::Standard(*rx, Some(val_ry))],
                    end_running: false,
                }
            },
            Instruction::LoadIntegerRegisterIndirect { rx, ry, i, s } => {
                let val_ry = state.registers[*ry as usize];
                ExecuteResult {
                    memory: MemoryAction::Read(MemoryBank::Data, val_ry + (*i << *s)),
                    writeback: vec![WritebackRegister::Standard(*rx, None)],
                    end_running: false,
                }
            },
            Instruction::LoadIntegerRegisterIndirectwithRegisterOffset { rx, ry, ro, s } => {
                let val_ry = state.registers[*ry as usize];
                let val_ro = state.registers[*ro as usize];
                ExecuteResult {
                    memory: MemoryAction::Read(MemoryBank::Data, val_ry + (val_ro << *s)),
                    writeback: vec![WritebackRegister::Standard(*rx, None)],
                    end_running: false,
                }
            },
            Instruction::LoadIntegerRegisterIndirectProgram { rx, ry, i, s } => {
                let val_ry = state.registers[*ry as usize];
                ExecuteResult {
                    memory: MemoryAction::Read(MemoryBank::Program, val_ry + (*i << *s)),
                    writeback: vec![WritebackRegister::Standard(*rx, None)],
                    end_running: false,
                }
            },
            Instruction::LoadIntegerRegisterIndirectwithRegisterOffsetProgram { rx, ry, ro, s } => {
                let val_ry = state.registers[*ry as usize];
                let val_ro = state.registers[*ro as usize];
                ExecuteResult {
                    memory: MemoryAction::Read(MemoryBank::Program, val_ry + (val_ro << *s)),
                    writeback: vec![WritebackRegister::Standard(*rx, None)],
                    end_running: false,
                }
            },
            Instruction::StoreIntegerRegisterIndirect { rx, ry, i, s } => {
                let val_rx = state.registers[*rx as usize];
                let val_ry = state.registers[*ry as usize];
                ExecuteResult {
                    memory: MemoryAction::Write(MemoryBank::Data, val_rx + (*i << *s), val_ry),
                    writeback: Vec::new(),
                    end_running: false,
                }
            },
            Instruction::StoreIntegerRegisterIndirectwithRegisterOffsetIndirect {
                rx,
                ry,
                ro,
                s,
            } => {
                let val_rx = state.registers[*rx as usize];
                let val_ry = state.registers[*ry as usize];
                let val_ro = state.registers[*ro as usize];
                ExecuteResult {
                    memory: MemoryAction::Write(MemoryBank::Data, val_rx + (val_ro << *s), val_ry),
                    writeback: Vec::new(),
                    end_running: false,
                }
            },
            Instruction::StoreIntegerRegisterIndirectProgram { rx, ry, i, s } => {
                let val_rx = state.registers[*rx as usize];
                let val_ry = state.registers[*ry as usize];
                ExecuteResult {
                    memory: MemoryAction::Write(MemoryBank::Program, val_rx + (*i << *s), val_ry),
                    writeback: Vec::new(),
                    end_running: false,
                }
            },
            Instruction::StoreIntegerRegisterIndirectwithRegisterOffsetProgram {
                rx,
                ry,
                ro,
                s,
            } => {
                let val_rx = state.registers[*rx as usize];
                let val_ry = state.registers[*ry as usize];
                let val_ro = state.registers[*ro as usize];
                ExecuteResult {
                    memory: MemoryAction::Write(MemoryBank::Program, val_rx + (val_ro << *s), val_ry),
                    writeback: Vec::new(),
                    end_running: false,
                }
            },
            Instruction::IntegerLoadData { rx, label } => {
                ExecuteResult {
                    memory: MemoryAction::Read(MemoryBank::Data, *label),
                    writeback: vec![WritebackRegister::Standard(*rx, None)],
                    end_running: false,
                }
            },
            Instruction::IntegerLoadProgram { rx, label } => {
                ExecuteResult {
                    memory: MemoryAction::Read(MemoryBank::Program, *label),
                    writeback: vec![WritebackRegister::Standard(*rx, None)],
                    end_running: false,
                }
            },
            Instruction::IntegerStoreData { rx, label } => {
                let val_rx = state.registers[*rx as usize];
                ExecuteResult {
                    memory: MemoryAction::Write(MemoryBank::Data, *label, val_rx),
                    writeback: Vec::new(),
                    end_running: false,
                }
            },
            Instruction::IntegerStoreProgram { rx, label } => {
                let val_rx = state.registers[*rx as usize];
                ExecuteResult {
                    memory: MemoryAction::Write(MemoryBank::Program, *label, val_rx),
                    writeback: Vec::new(),
                    end_running: false,
                }
            },
            Instruction::UnsignedZeroExtend { rx, ry, count } => {
                let val_ry = state.registers[*ry as usize];
                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![WritebackRegister::Standard(*rx, Some(val_ry & (0xFFFFFFFF >> *count)))],
                    end_running: false,
                }
            },
            Instruction::SignExtend { rx, ry, count } => {
                let val_ry = state.registers[*ry as usize];
                let mut val = val_ry & (0xFFFFFFFF >> *count);
                if (31 - *count) & 1 == 1 {
                    val |= !(0xFFFFFFFF >> *count);
                }

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![WritebackRegister::Standard(*rx, Some(val))],
                    end_running: false,
                }
            },
            Instruction::FloatingPointLoadLow { fx, value } => {
                let val_fx = raw_cast_from_f32(state.f_registers[*fx as usize]);
                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![WritebackRegister::FloatingPoint(
                        *fx,
                        Some(raw_cast_to_f32((val_fx & 0xFFFF0000) | *value)),
                    )],
                    end_running: false,
                }
            },
            Instruction::FloatingPointLoadHigh { fx, value } => {
                let val_fx = raw_cast_from_f32(state.f_registers[*fx as usize]);
                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![WritebackRegister::FloatingPoint(
                        *fx,
                        Some(raw_cast_to_f32((val_fx & 0x0000FFFF) | *value)),
                    )],
                    end_running: false,
                }
            },
            Instruction::SwapFloatingPointRegisters { fx, fy } => {
                let val_fx = state.f_registers[*fx as usize];
                let val_fy = state.f_registers[*fy as usize];
                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![
                        WritebackRegister::FloatingPoint(*fx, Some(val_fy)),
                        WritebackRegister::FloatingPoint(*fy, Some(val_fx)),
                    ],
                    end_running: false,
                }
            },
            Instruction::CopyFloatingPointRegister { fx, fy } => {
                let val_fy = state.f_registers[*fy as usize];
                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![WritebackRegister::FloatingPoint(*fx, Some(val_fy))],
                    end_running: false,
                }
            },
            Instruction::LoadFloatingPointRegisterIndirect { fx, ry, i, s } => {
                let val_ry = state.registers[*ry as usize];
                ExecuteResult {
                    memory: MemoryAction::Read(MemoryBank::Data, val_ry.wrapping_add(*i << *s)),
                    writeback: vec![WritebackRegister::FloatingPoint(*fx, None)],
                    end_running: false,
                }
            },
            Instruction::LoadFloatingPointRegisterIndirectwithRegisterOffset { fx, ry, ro, s } => {
                let val_ry = state.registers[*ry as usize];
                let val_ro = state.registers[*ro as usize];
                ExecuteResult {
                    memory: MemoryAction::Read(MemoryBank::Data, val_ry.wrapping_add(val_ro << *s)),
                    writeback: vec![WritebackRegister::FloatingPoint(*fx, None)],
                    end_running: false,
                }
            }
            Instruction::StoreFloatingPointRegisterIndirect { rx, fy, i, s } => {
                let val_rx = state.registers[*rx as usize];
                let val_fy = raw_cast_from_f32(state.f_registers[*fy as usize]);
                ExecuteResult {
                    memory: MemoryAction::Write(MemoryBank::Data, val_rx.wrapping_add(*i << *s), val_fy),
                    writeback: Vec::new(),
                    end_running: false,
                }
            },
            Instruction::StoreFloatingPointRegisterIndirectwithRegisterOffset { rx, fy, ro, s } => {
                let val_rx = state.registers[*rx as usize];
                let val_fy = raw_cast_from_f32(state.f_registers[*fy as usize]);
                let val_ro = state.registers[*ro as usize];
                ExecuteResult {
                    memory: MemoryAction::Write(MemoryBank::Data, val_rx.wrapping_add(val_ro << *s), val_fy),
                    writeback: Vec::new(),
                    end_running: false,
                }
            }
            Instruction::FloatingPointLoadData { fx, label } => {
                ExecuteResult {
                    memory: MemoryAction::Read(MemoryBank::Data, *label),
                    writeback: vec![WritebackRegister::FloatingPoint(*fx, None)],
                    end_running: false,
                }
            },
            Instruction::FloatingPointStoreData { fx, label } => {
                let val_fx = raw_cast_from_f32(state.f_registers[*fx as usize]);
                ExecuteResult {
                    memory: MemoryAction::Write(MemoryBank::Data, *label, val_fx),
                    writeback: Vec::new(),
                    end_running: false,
                }
            },
            Instruction::IntegerCompare { rx, ry } => {
                let val_rx = state.registers[*rx as usize];
                let val_ry = state.registers[*ry as usize];
                let mut st = state.registers[Register::ST as usize];

                st = Condition::Equal.set(st, val_rx == val_ry);
                st = Condition::GreaterThan.set(st, val_rx > val_ry);
                st = Condition::LessThan.set(st, val_rx < val_ry);
                st = Condition::GreaterEqual.set(st, val_rx >= val_ry);
                st = Condition::LessEqual.set(st, val_rx <= val_ry);
                st = Condition::IsEven.set(st, (val_rx.wrapping_sub(val_ry)) % 2 == 0);

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![WritebackRegister::Standard(Register::ST, Some(st))],
                    end_running: false,
                }
            },
            Instruction::IntegerCompareSingleAgainstZero { rx } => {
                let val_rx = state.registers[*rx as usize];
                let mut st = state.registers[Register::ST as usize];
                st = Condition::Equal.set(st, val_rx == 0);
                st = Condition::GreaterThan.set(st, val_rx > 0);
                st = Condition::LessThan.set(st, false);
                st = Condition::GreaterEqual.set(st, true);
                st = Condition::LessEqual.set(st, val_rx <= 0);
                st = Condition::IsEven.set(st, val_rx % 2 == 0);

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![WritebackRegister::Standard(Register::ST, Some(st))],
                    end_running: false,
                }
            },
            Instruction::IncrementIntegerRegister { c, rx } => {
                let val_rx = state.registers[*rx as usize];
                let (res, ovf) = val_rx.overflowing_add(1); 
                let mut writeback = vec![WritebackRegister::Standard(*rx, Some(res))];
                if *c {
                    let mut st = state.registers[Register::ST as usize];
                    st = Condition::Overflow.set(st, ovf);
                    st = Condition::IsEven.set(st, res & 2 == 0);
                    writeback.push(WritebackRegister::Standard(Register::ST, Some(st)));
                }

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback,
                    end_running: false,
                }
            },
            Instruction::DecrementIntegerRegister { c, rx } => {
                let val_rx = state.registers[*rx as usize];
                let (res, ovf) = val_rx.overflowing_sub(1); 
                let mut writeback = vec![WritebackRegister::Standard(*rx, Some(res))];
                if *c {
                    let mut st = state.registers[Register::ST as usize];
                    st = Condition::Underflow.set(st, ovf);
                    st = Condition::IsEven.set(st, res & 2 == 0);
                    writeback.push(WritebackRegister::Standard(Register::ST, Some(st)));
                }

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback,
                    end_running: false,
                }
            },
            Instruction::AddUnsignedInteger { c, rx, ry, rz } => {
                let val_ry = state.registers[*ry as usize];
                let val_rz = state.registers[*rz as usize];
                let (res, ovf) = val_ry.overflowing_add(val_rz); 
                let mut writeback = vec![WritebackRegister::Standard(*rx, Some(res))];
                if *c {
                    let mut st = state.registers[Register::ST as usize];
                    st = Condition::Overflow.set(st, ovf);
                    st = Condition::IsEven.set(st, res & 2 == 0);
                    writeback.push(WritebackRegister::Standard(Register::ST, Some(st)));
                }

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback,
                    end_running: false,
                }
            },
            Instruction::SubtractUnsignedInteger { c, rx, ry, rz } => {
                let val_ry = state.registers[*ry as usize];
                let val_rz = state.registers[*rz as usize];
                let (res, ovf) = val_ry.overflowing_sub(val_rz); 
                let mut writeback = vec![WritebackRegister::Standard(*rx, Some(res))];
                if *c {
                    let mut st = state.registers[Register::ST as usize];
                    st = Condition::Underflow.set(st, ovf);
                    st = Condition::IsEven.set(st, res & 2 == 0);
                    writeback.push(WritebackRegister::Standard(Register::ST, Some(st)));
                }

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback,
                    end_running: false,
                }
            },
            Instruction::MultiplyUnsignedInteger { c, rx, ry, rz } => {
                let val_ry = state.registers[*ry as usize];
                let val_rz = state.registers[*rz as usize];
                let (res, ovf) = val_ry.overflowing_mul(val_rz); 
                let mut writeback = vec![WritebackRegister::Standard(*rx, Some(res))];
                if *c {
                    let mut st = state.registers[Register::ST as usize];
                    st = Condition::Overflow.set(st, ovf);
                    st = Condition::IsEven.set(st, res & 2 == 0);
                    writeback.push(WritebackRegister::Standard(Register::ST, Some(st)));
                }

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback,
                    end_running: false,
                }
            },
            Instruction::DivideUnsignedInteger { c, rx, ry, rz } => {
                let val_ry = state.registers[*ry as usize];
                let val_rz = state.registers[*rz as usize];
                let writeback = if val_rz == 0 {
                    let mut writeback = vec![WritebackRegister::Standard(*rx, Some(0))];
                    if *c {
                        let st = state.registers[Register::ST as usize];
                        writeback.push(WritebackRegister::Standard(Register::ST, Some(Condition::DivideByZero.set(st, true))));
                    }
                    writeback
                } else {
                    let (res, ovf) = val_ry.overflowing_div(val_rz); 
                    let mut writeback = vec![WritebackRegister::Standard(*rx, Some(res))];
                    if *c {
                        let mut st = state.registers[Register::ST as usize];
                        st = Condition::Underflow.set(st, ovf);
                        st = Condition::IsEven.set(st, res & 2 == 0);
                        writeback.push(WritebackRegister::Standard(Register::ST, Some(st)));
                    }
                    writeback
                };

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback,
                    end_running: false,
                }
            },
            Instruction::ModuloUnsignedInteger { c, rx, ry, rz } => {
                let val_ry = state.registers[*ry as usize];
                let val_rz = state.registers[*rz as usize];
                let writeback = if val_rz == 0 {
                    let mut writeback = vec![WritebackRegister::Standard(*rx, Some(0))];
                    if *c {
                        let st = state.registers[Register::ST as usize];
                        writeback.push(WritebackRegister::Standard(Register::ST, Some(Condition::DivideByZero.set(st, true))));
                    }
                    writeback
                } else {
                    let (res, ovf) = val_ry.overflowing_rem(val_rz); 
                    let mut writeback = vec![WritebackRegister::Standard(*rx, Some(res))];
                    if *c {
                        let mut st = state.registers[Register::ST as usize];
                        st = Condition::Underflow.set(st, ovf);
                        st = Condition::IsEven.set(st, res & 2 == 0);
                        writeback.push(WritebackRegister::Standard(Register::ST, Some(st)));
                    }
                    writeback
                };

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback,
                    end_running: false,
                }
            },
            Instruction::AddSignedInteger { c, rx, ry, rz } => {
                let val_ry = raw_cast_to_i32(state.registers[*ry as usize]);
                let val_rz = raw_cast_to_i32(state.registers[*rz as usize]);
                let (res, ovf) = val_ry.overflowing_add(val_rz); 
                let mut writeback = vec![WritebackRegister::Standard(*rx, Some(raw_cast_from_i32(res)))];
                if *c {
                    let mut st = state.registers[Register::ST as usize];
                    st = Condition::Overflow.set(st, ovf);
                    st = Condition::IsEven.set(st, res & 2 == 0);
                    writeback.push(WritebackRegister::Standard(Register::ST, Some(st)));
                }

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback,
                    end_running: false,
                }
            },
            Instruction::SubtractSignedInteger { c, rx, ry, rz } => {
                let val_ry = raw_cast_to_i32(state.registers[*ry as usize]);
                let val_rz = raw_cast_to_i32(state.registers[*rz as usize]);
                let (res, ovf) = val_ry.overflowing_sub(val_rz); 
                let mut writeback = vec![WritebackRegister::Standard(*rx, Some(raw_cast_from_i32(res)))];
                if *c {
                    let mut st = state.registers[Register::ST as usize];
                    st = Condition::Underflow.set(st, ovf);
                    st = Condition::IsEven.set(st, res & 2 == 0);
                    writeback.push(WritebackRegister::Standard(Register::ST, Some(st)));
                }

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback,
                    end_running: false,
                }
            },
            Instruction::MultiplySignedInteger { c, rx, ry, rz } => {
                let val_ry = raw_cast_to_i32(state.registers[*ry as usize]);
                let val_rz = raw_cast_to_i32(state.registers[*rz as usize]);
                let (res, ovf) = val_ry.overflowing_mul(val_rz); 
                let mut writeback = vec![WritebackRegister::Standard(*rx, Some(raw_cast_from_i32(res)))];
                if *c {
                    let mut st = state.registers[Register::ST as usize];
                    st = Condition::Overflow.set(st, ovf);
                    st = Condition::IsEven.set(st, res & 2 == 0);
                    writeback.push(WritebackRegister::Standard(Register::ST, Some(st)));
                }

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback,
                    end_running: false,
                }
            },
            Instruction::DivideSignedInteger { c, rx, ry, rz } => {
                let val_ry = raw_cast_to_i32(state.registers[*ry as usize]);
                let val_rz = raw_cast_to_i32(state.registers[*rz as usize]);
                let writeback = if val_rz == 0 {
                    let mut writeback = vec![WritebackRegister::Standard(*rx, Some(0))];
                    if *c {
                        let st = state.registers[Register::ST as usize];
                        writeback.push(WritebackRegister::Standard(Register::ST, Some(Condition::DivideByZero.set(st, true))));
                    }
                    writeback
                } else {
                    let (res, ovf) = val_ry.overflowing_div(val_rz); 
                    let mut writeback = vec![WritebackRegister::Standard(*rx, Some(raw_cast_from_i32(res)))];
                    if *c {
                        let mut st = state.registers[Register::ST as usize];
                        st = Condition::Underflow.set(st, ovf);
                        st = Condition::IsEven.set(st, res & 2 == 0);
                        writeback.push(WritebackRegister::Standard(Register::ST, Some(st)));
                    }
                    writeback
                };

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback,
                    end_running: false,
                }
            },
            Instruction::ModuloSignedInteger { c, rx, ry, rz } => {
                let val_ry = raw_cast_to_i32(state.registers[*ry as usize]);
                let val_rz = raw_cast_to_i32(state.registers[*rz as usize]);
                let writeback = if val_rz == 0 {
                    let mut writeback = vec![WritebackRegister::Standard(*rx, Some(0))];
                    if *c {
                        let st = state.registers[Register::ST as usize];
                        writeback.push(WritebackRegister::Standard(Register::ST, Some(Condition::DivideByZero.set(st, true))));
                    }
                    writeback
                } else {
                    let (res, ovf) = val_ry.overflowing_rem(val_rz); 
                    let mut writeback = vec![WritebackRegister::Standard(*rx, Some(raw_cast_from_i32(res)))];
                    if *c {
                        let mut st = state.registers[Register::ST as usize];
                        st = Condition::Underflow.set(st, ovf);
                        st = Condition::IsEven.set(st, res & 2 == 0);
                        writeback.push(WritebackRegister::Standard(Register::ST, Some(st)));
                    }
                    writeback
                };

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback,
                    end_running: false,
                }
            },
            Instruction::BitwiseAND { rx, ry, rz } => {
                let val_ry = state.registers[*ry as usize];
                let val_rz = state.registers[*rz as usize];

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![WritebackRegister::Standard(*rx, Some(val_ry & val_rz))],
                    end_running: false,
                }  
            },
            Instruction::BitwiseOR { rx, ry, rz } => {
                let val_ry = state.registers[*ry as usize];
                let val_rz = state.registers[*rz as usize];

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![WritebackRegister::Standard(*rx, Some(val_ry | val_rz))],
                    end_running: false,
                }  
            },
            Instruction::BitwiseNOT { rx, ry } => {
                let val_ry = state.registers[*ry as usize];

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![WritebackRegister::Standard(*rx, Some(!val_ry))],
                    end_running: false,
                }  
            },
            Instruction::BitwiseXOR { rx, ry, rz } => {
                let val_ry = state.registers[*ry as usize];
                let val_rz = state.registers[*rz as usize];

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![WritebackRegister::Standard(*rx, Some(val_ry ^ val_rz))],
                    end_running: false,
                }  
            },
            Instruction::LogicalShiftLeft { rx, ry, value } => {
                let val_ry = state.registers[*ry as usize];

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![WritebackRegister::Standard(*rx, Some(val_ry << *value))],
                    end_running: false,
                }  
            },
            Instruction::LogicalShiftRight { rx, ry, value } => {
                let val_ry = state.registers[*ry as usize];

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![WritebackRegister::Standard(*rx, Some(val_ry >> *value))],
                    end_running: false,
                }  
            },
            Instruction::ArithmeticShiftLeft { rx, ry, value } => {
                let val_ry = raw_cast_to_i32(state.registers[*ry as usize]);

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![WritebackRegister::Standard(*rx, Some(raw_cast_from_i32(val_ry << *value)))],
                    end_running: false,
                }  
            },
            Instruction::ArithmeticShiftRight { rx, ry, value } => {
                let val_ry = raw_cast_to_i32(state.registers[*ry as usize]);

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![WritebackRegister::Standard(*rx, Some(raw_cast_from_i32(val_ry >> *value)))],
                    end_running: false,
                }  
            },
            Instruction::RotateRight { rx, ry, value } => {
                let val_ry = state.registers[*ry as usize];

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![WritebackRegister::Standard(*rx, Some(val_ry.rotate_right(*value)))],
                    end_running: false,
                }  
            },
            Instruction::LogicalShiftLeftRegister { rx, ry, rz } => {
                let val_ry = state.registers[*ry as usize];
                let val_rz = state.registers[*rz as usize];

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![WritebackRegister::Standard(*rx, Some(val_ry << val_rz))],
                    end_running: false,
                }  
            },
            Instruction::LogicalShiftRightRegister { rx, ry, rz } => {
                let val_ry = state.registers[*ry as usize];
                let val_rz = state.registers[*rz as usize];

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![WritebackRegister::Standard(*rx, Some(val_ry >> val_rz))],
                    end_running: false,
                }  
            },
            Instruction::ArithmeticShiftLeftRegister { rx, ry, rz } => {
                let val_ry = raw_cast_to_i32(state.registers[*ry as usize]);
                let val_rz = state.registers[*rz as usize];

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![WritebackRegister::Standard(*rx, Some(raw_cast_from_i32(val_ry << val_rz)))],
                    end_running: false,
                }  
            },
            Instruction::ArithmeticShiftRightRegister { rx, ry, rz } => {
                let val_ry = raw_cast_to_i32(state.registers[*ry as usize]);
                let val_rz = state.registers[*rz as usize];

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![WritebackRegister::Standard(*rx, Some(raw_cast_from_i32(val_ry >> val_rz)))],
                    end_running: false,
                }  
            },
            Instruction::RotateRightRegister { rx, ry, rz } => {
                let val_ry = state.registers[*ry as usize];
                let val_rz = state.registers[*rz as usize];

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![WritebackRegister::Standard(*rx, Some(val_ry.rotate_right(val_rz)))],
                    end_running: false,
                }  
            },
            Instruction::MapUnsignedToSigned { rx, ry } => {
                let val_ry = state.registers[*ry as usize];

                let res = if val_ry >= 0x80000000 {
                    val_ry & !0x80000000
                } else {
                    val_ry | 0x80000000
                };

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![WritebackRegister::Standard(*rx, Some(res))],
                    end_running: false,
                }
            },
            Instruction::MapSignedToUnsigned { rx, ry } => {
                let val_ry = state.registers[*ry as usize];

                let res = if val_ry < 0x80000000 {
                    val_ry | 0x80000000
                } else {
                    val_ry & !0x80000000
                };

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![WritebackRegister::Standard(*rx, Some(res))],
                    end_running: false,
                }  
            },
            Instruction::FloatingPointCompare { fx, fy } => {
                let val_fx = state.f_registers[*fx as usize];
                let val_fy = state.f_registers[*fy as usize];
                let mut st = state.registers[Register::ST as usize];

                st = Condition::Equal.set(st, val_fx == val_fy);
                st = Condition::GreaterThan.set(st, val_fx > val_fy);
                st = Condition::LessThan.set(st, val_fx < val_fy);
                st = Condition::GreaterEqual.set(st, val_fx >= val_fy);
                st = Condition::LessEqual.set(st, val_fx <= val_fy);
                st = Condition::FloatingPointInfinity.set(st, (val_fx - val_fy).is_infinite());
                st = Condition::FloatingPointNotANumber.set(st, (val_fx - val_fy).is_nan());
                st = Condition::FloatingPointZero.set(st, (val_fx - val_fy).is_subnormal());
                st = Condition::FloatingPointPositive.set(st, (val_fx - val_fy).is_sign_positive());

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![WritebackRegister::Standard(Register::ST, Some(st))],
                    end_running: false,
                }
            },
            Instruction::FloatingPointCompareSingleAgainstZero { fx } => {
                let val_fx = state.f_registers[*fx as usize];
                let mut st = state.registers[Register::ST as usize];

                st = Condition::Equal.set(st, val_fx == 0.0);
                st = Condition::GreaterThan.set(st, val_fx > 0.0);
                st = Condition::LessThan.set(st, val_fx < 0.0);
                st = Condition::GreaterEqual.set(st, val_fx >= 0.0);
                st = Condition::LessEqual.set(st, val_fx <= 0.0);
                st = Condition::FloatingPointInfinity.set(st, val_fx.is_infinite());
                st = Condition::FloatingPointNotANumber.set(st, val_fx.is_nan());
                st = Condition::FloatingPointZero.set(st, val_fx.is_subnormal());
                st = Condition::FloatingPointPositive.set(st, val_fx.is_sign_positive());

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![WritebackRegister::Standard(Register::ST, Some(st))],
                    end_running: false,
                }
            },
            Instruction::AddFloatingPoint { c, fx, fy, fz } => {
                let val_fy = state.f_registers[*fy as usize];
                let val_fz = state.f_registers[*fz as usize];
                let res = val_fy + val_fz;
                let mut writeback = vec![WritebackRegister::FloatingPoint(*fx, Some(res))];
                if *c {
                    let mut st = state.registers[Register::ST as usize];
                    st = Condition::FloatingPointInfinity.set(st, res.is_infinite());
                    st = Condition::FloatingPointNotANumber.set(st, res.is_nan());
                    st = Condition::FloatingPointZero.set(st, res.is_subnormal());
                    st = Condition::FloatingPointPositive.set(st, res.is_sign_positive());
                    writeback.push(WritebackRegister::Standard(Register::ST, Some(st)));
                }

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback,
                    end_running: false,
                }
            },
            Instruction::SubtractFloatingPoint { c, fx, fy, fz } => {
                let val_fy = state.f_registers[*fy as usize];
                let val_fz = state.f_registers[*fz as usize];
                let res = val_fy - val_fz;
                let mut writeback = vec![WritebackRegister::FloatingPoint(*fx, Some(res))];
                if *c {
                    let mut st = state.registers[Register::ST as usize];
                    st = Condition::FloatingPointInfinity.set(st, res.is_infinite());
                    st = Condition::FloatingPointNotANumber.set(st, res.is_nan());
                    st = Condition::FloatingPointZero.set(st, res.is_subnormal());
                    st = Condition::FloatingPointPositive.set(st, res.is_sign_positive());
                    writeback.push(WritebackRegister::Standard(Register::ST, Some(st)));
                }

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback,
                    end_running: false,
                }
            },
            Instruction::MultiplyFloatingPoint { c, fx, fy, fz } => {
                let val_fy = state.f_registers[*fy as usize];
                let val_fz = state.f_registers[*fz as usize];
                let res = val_fy * val_fz;
                let mut writeback = vec![WritebackRegister::FloatingPoint(*fx, Some(res))];
                if *c {
                    let mut st = state.registers[Register::ST as usize];
                    st = Condition::FloatingPointInfinity.set(st, res.is_infinite());
                    st = Condition::FloatingPointNotANumber.set(st, res.is_nan());
                    st = Condition::FloatingPointZero.set(st, res.is_subnormal());
                    st = Condition::FloatingPointPositive.set(st, res.is_sign_positive());
                    writeback.push(WritebackRegister::Standard(Register::ST, Some(st)));
                }

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback,
                    end_running: false,
                }
            },
            Instruction::DivideFloatingPoint { c, fx, fy, fz } => {
                let val_fy = state.f_registers[*fy as usize];
                let val_fz = state.f_registers[*fz as usize];
                let res = val_fy / val_fz;
                let mut writeback = vec![WritebackRegister::FloatingPoint(*fx, Some(res))];
                if *c {
                    let mut st = state.registers[Register::ST as usize];
                    st = Condition::FloatingPointInfinity.set(st, res.is_infinite());
                    st = Condition::FloatingPointNotANumber.set(st, res.is_nan());
                    st = Condition::FloatingPointZero.set(st, res.is_subnormal());
                    st = Condition::FloatingPointPositive.set(st, res.is_sign_positive());
                    writeback.push(WritebackRegister::Standard(Register::ST, Some(st)));
                }

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback,
                    end_running: false,
                }
            },
            Instruction::CastToFloat { c, fx, ry } => {
                let val_ry = raw_cast_to_i32(state.registers[*ry as usize]);
                let res = val_ry as f32;
                let mut writeback = vec![WritebackRegister::FloatingPoint(*fx, Some(res))];

                if *c {
                    let mut st = state.registers[Register::ST as usize];
                    st = Condition::IsEven.set(st, val_ry % 2 == 0);
                    st = Condition::FloatingPointInfinity.set(st, res.is_infinite());
                    st = Condition::FloatingPointNotANumber.set(st, res.is_nan());
                    st = Condition::FloatingPointZero.set(st, res.is_subnormal());
                    st = Condition::FloatingPointPositive.set(st, res.is_sign_positive());
                    writeback.push(WritebackRegister::Standard(Register::ST, Some(st)));
                }

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback,
                    end_running: false,
                }
            },
            Instruction::CastFromFloat { c, rx, fy } => {
                let val_fy = state.f_registers[*fy as usize];
                let res = val_fy as i32;
                let mut writeback = vec![WritebackRegister::Standard(*rx, Some(raw_cast_from_i32(res)))];

                if *c {
                    let mut st = state.registers[Register::ST as usize];
                    st = Condition::Overflow.set(st, val_fy > i32::MAX as f32);
                    st = Condition::Underflow.set(st, val_fy < i32::MIN as f32);
                    st = Condition::FloatingPointInfinity.set(st, val_fy.is_infinite());
                    st = Condition::FloatingPointNotANumber.set(st, val_fy.is_nan());
                    st = Condition::FloatingPointZero.set(st, val_fy.is_subnormal());
                    st = Condition::FloatingPointPositive.set(st, val_fy.is_sign_positive());
                    writeback.push(WritebackRegister::Standard(Register::ST, Some(st)));
                }

                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback,
                    end_running: false,
                }
            },
            Instruction::SetTimer { tx, ry } => {
                let val_ry = state.registers[*ry as usize];
                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![WritebackRegister::Timer(*tx, Some(val_ry))],
                    end_running: false,
                }
            },
            Instruction::GetCurrentTimer { rx, ty } => {
                let val_ty = state.timers[*ty as usize].value;
                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![WritebackRegister::Standard(*rx, Some(val_ty))],
                    end_running: false,
                }
            },
            Instruction::CheckTimer { .. } => Default::default(),
            Instruction::ClearTimer { tx } => {
                ExecuteResult {
                    memory: MemoryAction::None,
                    writeback: vec![WritebackRegister::Timer(*tx, Some(0))],
                    end_running: false,
                }
            },
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
            if let Some(state) = state_ref.execute_state.take() {
                state_ref.execute_result = Some(MemoryState {
                    memory: MemoryAction::None,
                    writeback: WritebackState {
                        pc: state.pc,
                        instruction: state.instruction,
                        registers: Vec::new(),
                        holds: Default::default(),
                        end_running: false,
                    },
                });
            }

            return if blocked {
                Err(PipelineError::Stalled)
            } else {
                Ok(())
            };
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
                    pc: execute_state.pc,
                    instruction: execute_state.instruction,
                    registers: execute.writeback,
                    holds: execute_state.registers,
                    end_running: execute.end_running
                },
            });

            Ok(())
        }
    }

    fn squash(state: &SimulatorStateCell) -> Result<(), PipelineError> {
        let mut state_ref = state.borrow_mut();
        if let Some(state) = state_ref.execute_state.take() {
            decrement_inflight(&mut state_ref.inflight, &state.registers);

            state_ref.execute_result = Some(MemoryState {
                memory: MemoryAction::None,
                writeback: WritebackState {
                    pc: state.pc,
                    instruction: state.instruction,
                    registers: Vec::new(),
                    holds: Default::default(),
                    end_running: false,
                },
            });
        }

        if state_ref.execute_result.is_some() {
            state_ref.squashes.execute = true;
        }
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
            if let Some(state) = state_ref.memory_state.take() {
                state_ref.memory_result = Some(WritebackState {
                    pc: state.writeback.pc,
                    instruction: state.writeback.instruction,
                    registers: Vec::new(),
                    holds: Default::default(),
                    end_running: false,
                });
            }

            return if blocked {
                Err(PipelineError::Stalled)
            } else {
                Ok(())
            };
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
                        MemoryBank::Data => state_ref.data_memory.borrow_mut().fetch(4, address),
                        MemoryBank::Program => {
                            state_ref.program_memory.borrow_mut().fetch(4, address)
                        }
                    } {
                        Ok(v) => {
                            let res = v[line_offset(address as usize)];
                            let mut wb = state_ref.memory_state.take().unwrap().writeback;
                            for r in wb.registers.iter_mut() {
                                match r {
                                    WritebackRegister::Standard(_register, v) => {
                                        if v.is_none() {
                                            *v = Some(res);
                                        }
                                    },
                                    WritebackRegister::FloatingPoint(_f_register, v) => {
                                        if v.is_none() {
                                            *v = Some(raw_cast_to_f32(res));
                                        }
                                    },
                                    WritebackRegister::Timer(_timer, v) => {
                                        if v.is_none() {
                                            *v = Some(res);
                                        }
                                    },
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
                        MemoryBank::Data => {
                            state_ref.data_memory.borrow_mut().store(4, address, value)
                        }
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
        if let Some(state) = state_ref.memory_state.take() {
            match state.memory {
                MemoryAction::None => {}
                MemoryAction::Read(memory_bank, _address) => match memory_bank {
                    MemoryBank::Data => state_ref.data_memory.borrow_mut().cancel(4).unwrap(),
                    MemoryBank::Program => state_ref.program_memory.borrow_mut().cancel(4).unwrap(),
                },
                MemoryAction::Write(memory_bank, _address, _value) => match memory_bank {
                    MemoryBank::Data => state_ref.data_memory.borrow_mut().cancel(4).unwrap(),
                    MemoryBank::Program => state_ref.program_memory.borrow_mut().cancel(4).unwrap(),
                },
            }

            let holds = state.writeback.holds.clone();

            decrement_inflight(&mut state_ref.inflight, &holds);

            state_ref.memory_result = Some(WritebackState {
                pc: state.writeback.pc,
                instruction: state.writeback.instruction,
                registers: Vec::new(),
                holds: Default::default(),
                end_running: false,
            });
        }

        if state_ref.memory_result.is_some() {
            state_ref.squashes.memory = true;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct WritebackState {
    pub pc: u32,
    pub instruction: Instruction,
    pub registers: Vec<WritebackRegister>,
    pub holds: RegisterSet,
    pub end_running: bool,
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
                state_ref.writeback_state = None;
                state_ref.hold_fetch = false;
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
                                value.unwrap();
                        }
                        WritebackRegister::Timer(timer, value) => {
                            state_ref.timers[timer as usize].previous_set = value.unwrap();
                            state_ref.timers[timer as usize].value = value.unwrap();
                        }
                    }
                }

                decrement_inflight(&mut state_ref.inflight, &wb_state.holds);
                state_ref.hold_fetch = false;

                if wb_state.end_running {
                    state_ref.running = false;
                }
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

        let holds = state_ref.writeback_state.as_ref().unwrap().holds.clone();

        decrement_inflight(&mut state_ref.inflight, &holds);

        state_ref.writeback_state = None;
        state_ref.hold_fetch = false;
        Ok(())
    }
}