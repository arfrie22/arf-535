use std::{cell::RefCell, fmt, marker::PhantomData, rc::Rc};

use crate::{enums::Register, instruction::Instruction, memory::line_offset, SimulatorStateCell};

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
    fn is_squashed(&self) -> bool;
}

#[derive(Debug, Clone)]
pub struct PipelineStage<T: PipelineInner> {
    next: Option<Rc<RefCell<dyn PipelineOutter>>>,
    simulator: SimulatorStateCell,
    squashed: bool,
    inner: PhantomData<T>,
}

impl<T: PipelineInner> PipelineOutter for PipelineStage<T> {
    fn call(&mut self, blocked: bool) -> Result<(), PipelineError> {
        let res = if self.squashed { Ok(()) } else { T::call(&self.simulator, blocked) };

        if let Some(next) = &self.next {
            let blocked = match (blocked, res) {
                (true, _) => true,
                (false, Err(PipelineError::Stalled)) => true,
                (false, _) => false,
            };

            // if done and previous is done, take next down line
            match next.borrow_mut().call(blocked) {
                Ok(()) => todo!(),
                Err(PipelineError::Stalled) => todo!(),
                Err(e) => todo!(),
            }
        }

        res
    }
    
    fn squash(&mut self) -> Result<(), PipelineError> {
        match T::squash(&self.simulator) {
            Ok(v) => {
                self.squashed = true;
                Ok(v)
            },
            Err(e) => Err(e),
        }
    }
    
    fn is_squashed(&self) -> bool {
        self.squashed
    }
}

#[derive(Debug)]
pub struct FetchStage;
impl PipelineInner for FetchStage {
    fn call(state: &SimulatorStateCell, blocked: bool) -> Result<(), PipelineError> {
        if !blocked && state.borrow_mut().fetch_address.is_none() {
            let address = state.borrow_mut().registers[Register::PC as usize];    
            state.borrow_mut().fetch_address = Some(address);
        }

        if let Some(address) = state.borrow_mut().fetch_address {
            match state.borrow_mut().program_memory.borrow_mut().fetch(1, address) {
                Ok(v) => {
                    if state.borrow_mut().fetch_result.is_some() {
                        Err(PipelineError::Stalled)
                    } else { 
                        state.borrow_mut().fetch_result = Some(v[line_offset(address as usize)]);
                        state.borrow_mut().fetch_address = None;
                        Ok(())
                    }
                },
                Err(_) => Err(PipelineError::Stalled),
            }
        } else { 
            Err(PipelineError::Stalled)
        }
    }
    
    fn squash(state: &SimulatorStateCell) -> Result<(), PipelineError> {
        state.borrow_mut().program_memory.borrow_mut().cancel(1).unwrap();
        state.borrow_mut().fetch_address = None;
        Ok(())
    }
}

#[derive(Debug)]
pub struct DecodeStage;
impl PipelineInner for DecodeStage {
    fn call(state: &SimulatorStateCell, blocked: bool) -> Result<(), PipelineError> {
        if blocked {
            Err(PipelineError::Stalled)
        } else { 
            if let Some(res) = state.borrow_mut().fetch_result {
                state.borrow_mut().decode_result = Some(Instruction::from(res));
                state.borrow_mut().fetch_result = None;
                Ok(())
            } else {
                Err(PipelineError::Stalled)
            }
        }
    }
    
    fn squash(_state: &SimulatorStateCell) -> Result<(), PipelineError> {
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryBank {
    Data,
    Program
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WritebackRegister {
    Standard,
    FloatingPoint,
    Timer,
}

#[derive(Debug, Clone)]
pub struct ExecuteResult {
    pub memory: Option<(MemoryBank, usize, u32)>,
    pub writeback: Option<(WritebackRegister, usize, u32)>,
}

impl Instruction {
    pub fn cycle_count(&self, state: &SimulatorStateCell) -> usize {
        match self {
            Self::Invalid(_) => 1,
            Self::Trap => 1,
            Self::PushIntegerRegister { rx } => 1,
            Self::PushFloatingPointRegister { fx } => 1,
            Self::PopIntegerRegister { rx } => 1,
            Self::PopFloatingPointRegister { fx } => 1,
            Self::SwapRegister { rx, fy } => 1,
            Self::Stall { rx } => 1,
            Self::RegisterJump { condition, rx } => 1,
            Self::IndirectJump { condition, rx, i, s } => 1,
            Self::IndirectwithRegisterOffsetJump { condition, rx, ro, s } => 1,
            Self::RelativeJump { condition, rx } => 1,
            Self::ImmediateJump { condition, label } => 1,
            Self::ImmediateRelativeJump { condition, offset } => 1,
            Self::RegisterJumpwithLink { condition, rx } => 1,
            Self::IndirectJumpwithLink { condition, rx, i, s } => 1,
            Self::IndirectwithRegisterOffsetJumpwithLink { condition, rx, ro, s } => 1,
            Self::RelativeJumpwithLink { condition, rx } => 1,
            Self::ImmediateJumpwithLink { condition, label } => 1,
            Self::ImmediateRelativeJumpwithLink { condition, offset } => 1,
            Self::IntegerLoadLow { rx, value } => 1,
            Self::IntegerLoadHigh { rx, value } => 1,
            Self::SwapIntegerRegisters { rx, ry } => 1,
            Self::CopyIntegerRegister { rx, ry } => 1,
            Self::LoadIntegerRegisterIndirect { rx, ry, i, s } => 1,
            Self::LoadIntegerRegisterIndirectwithRegisterOffset { rx, ry, ro, s } => 1,
            Self::LoadIntegerRegisterIndirectProgram { rx, ry, i, s } => 1,
            Self::LoadIntegerRegisterIndirectwithRegisterOffsetProgram { rx, ry, ro, s } => 1,
            Self::StoreIntegerRegisterIndirect { rx, ry, i, s } => 1,
            Self::StoreIntegerRegisterIndirectwithRegisterOffsetIndirect { rx, ry, ro, s } => 1,
            Self::StoreIntegerRegisterIndirectProgram { rx, ry, i, s } => 1,
            Self::StoreIntegerRegisterIndirectwithRegisterOffsetProgram { rx, ry, ro, s } => 1,
            Self::IntegerLoadData { rx, label } => 1,
            Self::IntegerLoadProgram { rx, label } => 1,
            Self::IntegerStoreData { rx, label } => 1,
            Self::IntegerStoreProgram { rx, label } => 1,
            Self::UnsignedZeroExtend { rx, ry, count } => 1,
            Self::SignExtend { rx, ry, count } => 1,
            Self::FloatingPointLoadLow { fx, value } => 1,
            Self::FloatingPointLoadHigh { fx, value } => 1,
            Self::SwapFloatingPointRegisters { fx, fy } => 1,
            Self::CopyFloatingPointRegister { fx, fy } => 1,
            Self::LoadFloatingPointRegisterIndirect { fx, ry, i, s } => 1,
            Self::LoadFloatingPointRegisterIndirectwithRegisterOffset { fx, ry, ro, s } => 1,
            Self::StoreFloatingPointRegisterIndirect { rx, fy, i, s } => 1,
            Self::StoreFloatingPointRegisterIndirectwithRegisterOffset { rx, fy, ro, s } => 1,
            Self::FloatingPointLoadData { rx, label } => 1,
            Self::FloatingPointStoreData { rx, label } => 1,
            Self::IntegerCompare { rx, ry } => 1,
            Self::IntegerCompareSingleAgainstZero { rx } => 1,
            Self::AddUnsignedInteger { rx, ry, rz } => 1,
            Self::SubtractUnsignedInteger { rx, ry, rz } => 1,
            Self::MultiplyUnsignedInteger { rx, ry, rz } => 1,
            Self::DivideUnsignedInteger { rx, ry, rz } => 1,
            Self::ModuloUnsignedInteger { rx, ry, rz } => 1,
            Self::AddSignedInteger { rx, ry, rz } => 1,
            Self::SubtractSignedInteger { rx, ry, rz } => 1,
            Self::MultiplySignedInteger { rx, ry, rz } => 1,
            Self::DivideSignedInteger { rx, ry, rz } => 1,
            Self::ModuloSignedInteger { rx, ry, rz } => 1,
            Self::BitwiseAND { rx, ry, rz } => 1,
            Self::BitwiseOR { rx, ry, rz } => 1,
            Self::BitwiseNOT { rx, ry, rz } => 1,
            Self::BitwiseXOR { rx, ry, rz } => 1,
            Self::LogicalShiftLeft { rx, ry, value } => 1,
            Self::LogicalShiftRight { rx, ry, value } => 1,
            Self::ArithmeticShiftLeft { rx, ry, value } => 1,
            Self::ArithmeticShiftRight { rx, ry, value } => 1,
            Self::RotateRight { rx, ry, value } => 1,
            Self::LogicalShiftLeftRegister { rx, ry, rz } => 1,
            Self::LogicalShiftRightRegister { rx, ry, rz } => 1,
            Self::ArithmeticShiftLeftRegister { rx, ry, rz } => 1,
            Self::ArithmeticShiftRightRegister { rx, ry, rz } => 1,
            Self::RotateRightRegister { rx, ry, rz } => 1,
            Self::MapUnsignedToSigned { rx, ry } => 1,
            Self::MapSignedToUnsigned { rx, ry } => 1,
            Self::FloatingPointCompare { fx, fy } => 1,
            Self::FloatingPointCompareSingleAgainstZero { fx } => 1,
            Self::AddFloatingPoint { fx, fy, fz } => 1,
            Self::SubtractFloatingPoint { fx, fy, fz } => 1,
            Self::MultiplyFloatingPoint { fx, fy, fz } => 1,
            Self::DivideFloatingPoint { fx, fy, fz } => 1,
            Self::CasttoFloat { fx, ry } => 1,
            Self::CastfromFloat { rx, fy } => 1,
            Self::SetTimer { tx, ry } => 1,
            Self::GetCurrentTimer { tx, ry } => 1,
            Self::CheckTimer { tx } => 1,
            Self::ClearTimer { tx } => 1,
        }
    }
    pub fn execute(&self, state: &SimulatorStateCell) -> ExecuteResult {
        match self {
            Self::Invalid(_) => ExecuteResult { memory: None, writeback: None },
            Self::Trap => {
                state.borrow_mut().running = false;
                ExecuteResult { memory: None, writeback: None }
            },
            Self::PushIntegerRegister { rx } => ExecuteResult { memory: Some((MemoryBank::Data, state.borrow().registers[Register::SP as usize] as usize, state.borrow().registers[*rx as usize])), writeback: Some((WritebackRegister::Standard, Register::SP as usize, state.borrow().registers[Register::SP as usize] + 1)) },
            Self::PushFloatingPointRegister { fx } => ExecuteResult { memory: Some((MemoryBank::Data, state.borrow().registers[Register::SP as usize] as usize, state.borrow().f_registers[*fx as usize].to_bits())), writeback: Some((WritebackRegister::Standard, Register::SP as usize, state.borrow().registers[Register::SP as usize] + 1)) },
            Self::PopIntegerRegister { rx } => todo!(),
            Self::PopFloatingPointRegister { fx } => todo!(),
            Self::SwapRegister { rx, fy } => todo!(),
            Self::Stall { rx } => todo!(),
            Self::RegisterJump { condition, rx } => todo!(),
            Self::IndirectJump { condition, rx, i, s } => todo!(),
            Self::IndirectwithRegisterOffsetJump { condition, rx, ro, s } => todo!(),
            Self::RelativeJump { condition, rx } => todo!(),
            Self::ImmediateJump { condition, label } => todo!(),
            Self::ImmediateRelativeJump { condition, offset } => todo!(),
            Self::RegisterJumpwithLink { condition, rx } => todo!(),
            Self::IndirectJumpwithLink { condition, rx, i, s } => todo!(),
            Self::IndirectwithRegisterOffsetJumpwithLink { condition, rx, ro, s } => todo!(),
            Self::RelativeJumpwithLink { condition, rx } => todo!(),
            Self::ImmediateJumpwithLink { condition, label } => todo!(),
            Self::ImmediateRelativeJumpwithLink { condition, offset } => todo!(),
            Self::IntegerLoadLow { rx, value } => todo!(),
            Self::IntegerLoadHigh { rx, value } => todo!(),
            Self::SwapIntegerRegisters { rx, ry } => todo!(),
            Self::CopyIntegerRegister { rx, ry } => todo!(),
            Self::LoadIntegerRegisterIndirect { rx, ry, i, s } => todo!(),
            Self::LoadIntegerRegisterIndirectwithRegisterOffset { rx, ry, ro, s } => todo!(),
            Self::LoadIntegerRegisterIndirectProgram { rx, ry, i, s } => todo!(),
            Self::LoadIntegerRegisterIndirectwithRegisterOffsetProgram { rx, ry, ro, s } => todo!(),
            Self::StoreIntegerRegisterIndirect { rx, ry, i, s } => todo!(),
            Self::StoreIntegerRegisterIndirectwithRegisterOffsetIndirect { rx, ry, ro, s } => todo!(),
            Self::StoreIntegerRegisterIndirectProgram { rx, ry, i, s } => todo!(),
            Self::StoreIntegerRegisterIndirectwithRegisterOffsetProgram { rx, ry, ro, s } => todo!(),
            Self::IntegerLoadData { rx, label } => todo!(),
            Self::IntegerLoadProgram { rx, label } => todo!(),
            Self::IntegerStoreData { rx, label } => todo!(),
            Self::IntegerStoreProgram { rx, label } => todo!(),
            Self::UnsignedZeroExtend { rx, ry, count } => todo!(),
            Self::SignExtend { rx, ry, count } => todo!(),
            Self::FloatingPointLoadLow { fx, value } => todo!(),
            Self::FloatingPointLoadHigh { fx, value } => todo!(),
            Self::SwapFloatingPointRegisters { fx, fy } => todo!(),
            Self::CopyFloatingPointRegister { fx, fy } => todo!(),
            Self::LoadFloatingPointRegisterIndirect { fx, ry, i, s } => todo!(),
            Self::LoadFloatingPointRegisterIndirectwithRegisterOffset { fx, ry, ro, s } => todo!(),
            Self::StoreFloatingPointRegisterIndirect { rx, fy, i, s } => todo!(),
            Self::StoreFloatingPointRegisterIndirectwithRegisterOffset { rx, fy, ro, s } => todo!(),
            Self::FloatingPointLoadData { rx, label } => todo!(),
            Self::FloatingPointStoreData { rx, label } => todo!(),
            Self::IntegerCompare { rx, ry } => todo!(),
            Self::IntegerCompareSingleAgainstZero { rx } => todo!(),
            Self::AddUnsignedInteger { rx, ry, rz } => todo!(),
            Self::SubtractUnsignedInteger { rx, ry, rz } => todo!(),
            Self::MultiplyUnsignedInteger { rx, ry, rz } => todo!(),
            Self::DivideUnsignedInteger { rx, ry, rz } => todo!(),
            Self::ModuloUnsignedInteger { rx, ry, rz } => todo!(),
            Self::AddSignedInteger { rx, ry, rz } => todo!(),
            Self::SubtractSignedInteger { rx, ry, rz } => todo!(),
            Self::MultiplySignedInteger { rx, ry, rz } => todo!(),
            Self::DivideSignedInteger { rx, ry, rz } => todo!(),
            Self::ModuloSignedInteger { rx, ry, rz } => todo!(),
            Self::BitwiseAND { rx, ry, rz } => todo!(),
            Self::BitwiseOR { rx, ry, rz } => todo!(),
            Self::BitwiseNOT { rx, ry, rz } => todo!(),
            Self::BitwiseXOR { rx, ry, rz } => todo!(),
            Self::LogicalShiftLeft { rx, ry, value } => todo!(),
            Self::LogicalShiftRight { rx, ry, value } => todo!(),
            Self::ArithmeticShiftLeft { rx, ry, value } => todo!(),
            Self::ArithmeticShiftRight { rx, ry, value } => todo!(),
            Self::RotateRight { rx, ry, value } => todo!(),
            Self::LogicalShiftLeftRegister { rx, ry, rz } => todo!(),
            Self::LogicalShiftRightRegister { rx, ry, rz } => todo!(),
            Self::ArithmeticShiftLeftRegister { rx, ry, rz } => todo!(),
            Self::ArithmeticShiftRightRegister { rx, ry, rz } => todo!(),
            Self::RotateRightRegister { rx, ry, rz } => todo!(),
            Self::MapUnsignedToSigned { rx, ry } => todo!(),
            Self::MapSignedToUnsigned { rx, ry } => todo!(),
            Self::FloatingPointCompare { fx, fy } => todo!(),
            Self::FloatingPointCompareSingleAgainstZero { fx } => todo!(),
            Self::AddFloatingPoint { fx, fy, fz } => todo!(),
            Self::SubtractFloatingPoint { fx, fy, fz } => todo!(),
            Self::MultiplyFloatingPoint { fx, fy, fz } => todo!(),
            Self::DivideFloatingPoint { fx, fy, fz } => todo!(),
            Self::CasttoFloat { fx, ry } => todo!(),
            Self::CastfromFloat { rx, fy } => todo!(),
            Self::SetTimer { tx, ry } => todo!(),
            Self::GetCurrentTimer { tx, ry } => todo!(),
            Self::CheckTimer { tx } => todo!(),
            Self::ClearTimer { tx } => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct ExecuteStage;
impl PipelineInner for ExecuteStage {
    fn call(state: &SimulatorStateCell, blocked: bool) -> Result<(), PipelineError> {
        if let Some(res) = state.borrow_mut().decode_result {
            state.borrow_mut().execute_result = Some(ExecuteResult { memory: None, writeback: None });
            state.borrow_mut().decode_result = None;
            Ok(())
        } else {
            Err(PipelineError::Stalled)
        }
    }
    
    fn squash(_state: &SimulatorStateCell) -> Result<(), PipelineError> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct MemoryStage;
impl PipelineInner for MemoryStage {
    fn call(state: &SimulatorStateCell, blocked: bool) -> Result<(), PipelineError> {
        todo!()
    }
    
    fn squash(state: &SimulatorStateCell) -> Result<(), PipelineError> {
        todo!()
    }
}

#[derive(Debug)]
pub struct WritebackStage;
impl PipelineInner for WritebackStage {
    fn call(state: &SimulatorStateCell, blocked: bool) -> Result<(), PipelineError> {
        todo!()
    }
    
    fn squash(state: &SimulatorStateCell) -> Result<(), PipelineError> {
        todo!()
    }
}

// For april 2 make sure you have enough
// load, store, branch, arithmatic operations
// load 2 add store, change counter, jump
// cache 1 cycle, dram 2 cycle 