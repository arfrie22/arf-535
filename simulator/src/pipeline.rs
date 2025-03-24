use std::{cell::RefCell, fmt, marker::PhantomData, rc::Rc};

use crate::{enums::{Instructions, Registers}, memory::line_offset, SimulatorStateCell};

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
            let address = state.borrow_mut().registers[Registers::PC as usize];    
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
                state.borrow_mut().decode_result = Some(Instructions::from(res));
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

impl Instructions {
    pub fn cycle_count(&self, state: &SimulatorStateCell) -> usize {
        match self {
            Instructions::Invalid(_) => 1,
            Instructions::Trap => 1,
            Instructions::PushIntegerRegister { rx } => 1,
            Instructions::PushFloatingPointRegister { fx } => 1,
            Instructions::PopIntegerRegister { rx } => 1,
            Instructions::PopFloatingPointRegister { fx } => 1,
            Instructions::SwapRegister { rx, fy } => 1,
            Instructions::Stall { rx } => 1,
            Instructions::RegisterJump { condition, rx } => 1,
            Instructions::IndirectJump { condition, rx, i, s } => 1,
            Instructions::IndirectwithRegisterOffsetJump { condition, rx, ro, s } => 1,
            Instructions::RelativeJump { condition, rx } => 1,
            Instructions::ImmediateJump { condition, label } => 1,
            Instructions::ImmediateRelativeJump { condition, offset } => 1,
            Instructions::RegisterJumpwithLink { condition, rx } => 1,
            Instructions::IndirectJumpwithLink { condition, rx, i, s } => 1,
            Instructions::IndirectwithRegisterOffsetJumpwithLink { condition, rx, ro, s } => 1,
            Instructions::RelativeJumpwithLink { condition, rx } => 1,
            Instructions::ImmediateJumpwithLink { condition, label } => 1,
            Instructions::ImmediateRelativeJumpwithLink { condition, offset } => 1,
            Instructions::IntegerLoadLow { rx, value } => 1,
            Instructions::IntegerLoadHigh { rx, value } => 1,
            Instructions::SwapIntegerRegisters { rx, ry } => 1,
            Instructions::CopyIntegerRegister { rx, ry } => 1,
            Instructions::LoadIntegerRegisterIndirect { rx, ry, i, s } => 1,
            Instructions::LoadIntegerRegisterIndirectwithRegisterOffset { rx, ry, ro, s } => 1,
            Instructions::LoadIntegerRegisterIndirectProgram { rx, ry, i, s } => 1,
            Instructions::LoadIntegerRegisterIndirectwithRegisterOffsetProgram { rx, ry, ro, s } => 1,
            Instructions::StoreIntegerRegisterIndirect { rx, ry, i, s } => 1,
            Instructions::StoreIntegerRegisterIndirectwithRegisterOffsetIndirect { rx, ry, ro, s } => 1,
            Instructions::StoreIntegerRegisterIndirectProgram { rx, ry, i, s } => 1,
            Instructions::StoreIntegerRegisterIndirectwithRegisterOffsetProgram { rx, ry, ro, s } => 1,
            Instructions::IntegerLoadData { rx, label } => 1,
            Instructions::IntegerLoadProgram { rx, label } => 1,
            Instructions::IntegerStoreData { rx, label } => 1,
            Instructions::IntegerStoreProgram { rx, label } => 1,
            Instructions::UnsignedZeroExtend { rx, ry, count } => 1,
            Instructions::SignExtend { rx, ry, count } => 1,
            Instructions::FloatingPointLoadLow { fx, value } => 1,
            Instructions::FloatingPointLoadHigh { fx, value } => 1,
            Instructions::SwapFloatingPointRegisters { fx, fy } => 1,
            Instructions::CopyFloatingPointRegister { fx, fy } => 1,
            Instructions::LoadFloatingPointRegisterIndirect { fx, ry, i, s } => 1,
            Instructions::LoadFloatingPointRegisterIndirectwithRegisterOffset { fx, ry, ro, s } => 1,
            Instructions::StoreFloatingPointRegisterIndirect { rx, fy, i, s } => 1,
            Instructions::StoreFloatingPointRegisterIndirectwithRegisterOffset { rx, fy, ro, s } => 1,
            Instructions::FloatingPointLoadData { rx, label } => 1,
            Instructions::FloatingPointStoreData { rx, label } => 1,
            Instructions::IntegerCompare { rx, ry } => 1,
            Instructions::IntegerCompareSingleAgainstZero { rx } => 1,
            Instructions::AddUnsignedInteger { rx, ry, rz } => 1,
            Instructions::SubtractUnsignedInteger { rx, ry, rz } => 1,
            Instructions::MultiplyUnsignedInteger { rx, ry, rz } => 1,
            Instructions::DivideUnsignedInteger { rx, ry, rz } => 1,
            Instructions::ModuloUnsignedInteger { rx, ry, rz } => 1,
            Instructions::AddSignedInteger { rx, ry, rz } => 1,
            Instructions::SubtractSignedInteger { rx, ry, rz } => 1,
            Instructions::MultiplySignedInteger { rx, ry, rz } => 1,
            Instructions::DivideSignedInteger { rx, ry, rz } => 1,
            Instructions::ModuloSignedInteger { rx, ry, rz } => 1,
            Instructions::BitwiseAND { rx, ry, rz } => 1,
            Instructions::BitwiseOR { rx, ry, rz } => 1,
            Instructions::BitwiseNOT { rx, ry, rz } => 1,
            Instructions::BitwiseXOR { rx, ry, rz } => 1,
            Instructions::LogicalShiftLeft { rx, ry, value } => 1,
            Instructions::LogicalShiftRight { rx, ry, value } => 1,
            Instructions::ArithmeticShiftLeft { rx, ry, value } => 1,
            Instructions::ArithmeticShiftRight { rx, ry, value } => 1,
            Instructions::RotateRight { rx, ry, value } => 1,
            Instructions::LogicalShiftLeftRegister { rx, ry, rz } => 1,
            Instructions::LogicalShiftRightRegister { rx, ry, rz } => 1,
            Instructions::ArithmeticShiftLeftRegister { rx, ry, rz } => 1,
            Instructions::ArithmeticShiftRightRegister { rx, ry, rz } => 1,
            Instructions::RotateRightRegister { rx, ry, rz } => 1,
            Instructions::MapUnsignedToSigned { rx, ry } => 1,
            Instructions::MapSignedToUnsigned { rx, ry } => 1,
            Instructions::FloatingPointCompare { fx, fy } => 1,
            Instructions::FloatingPointCompareSingleAgainstZero { fx } => 1,
            Instructions::AddFloatingPoint { fx, fy, fz } => 1,
            Instructions::SubtractFloatingPoint { fx, fy, fz } => 1,
            Instructions::MultiplyFloatingPoint { fx, fy, fz } => 1,
            Instructions::DivideFloatingPoint { fx, fy, fz } => 1,
            Instructions::CasttoFloat { fx, ry } => 1,
            Instructions::CastfromFloat { rx, fy } => 1,
            Instructions::SetTimer { tx, ry } => 1,
            Instructions::GetCurrentTimer { tx, ry } => 1,
            Instructions::CheckTimer { tx } => 1,
            Instructions::ClearTimer { tx } => 1,
        }
    }
    pub fn execute(&self, state: &SimulatorStateCell) -> ExecuteResult {
        match self {
            Instructions::Invalid(_) => ExecuteResult { memory: None, writeback: None },
            Instructions::Trap => {
                state.borrow_mut().running = false;
                ExecuteResult { memory: None, writeback: None }
            },
            Instructions::PushIntegerRegister { rx } => ExecuteResult { memory: Some((MemoryBank::Data, state.borrow().registers[Registers::SP as usize] as usize, state.borrow().registers[*rx as usize])), writeback: Some((WritebackRegister::Standard, Registers::SP as usize, state.borrow().registers[Registers::SP as usize] + 1)) },
            Instructions::PushFloatingPointRegister { fx } => ExecuteResult { memory: Some((MemoryBank::Data, state.borrow().registers[Registers::SP as usize] as usize, state.borrow().f_registers[*fx as usize].to_bits())), writeback: Some((WritebackRegister::Standard, Registers::SP as usize, state.borrow().registers[Registers::SP as usize] + 1)) },
            Instructions::PopIntegerRegister { rx } => todo!(),
            Instructions::PopFloatingPointRegister { fx } => todo!(),
            Instructions::SwapRegister { rx, fy } => todo!(),
            Instructions::Stall { rx } => todo!(),
            Instructions::RegisterJump { condition, rx } => todo!(),
            Instructions::IndirectJump { condition, rx, i, s } => todo!(),
            Instructions::IndirectwithRegisterOffsetJump { condition, rx, ro, s } => todo!(),
            Instructions::RelativeJump { condition, rx } => todo!(),
            Instructions::ImmediateJump { condition, label } => todo!(),
            Instructions::ImmediateRelativeJump { condition, offset } => todo!(),
            Instructions::RegisterJumpwithLink { condition, rx } => todo!(),
            Instructions::IndirectJumpwithLink { condition, rx, i, s } => todo!(),
            Instructions::IndirectwithRegisterOffsetJumpwithLink { condition, rx, ro, s } => todo!(),
            Instructions::RelativeJumpwithLink { condition, rx } => todo!(),
            Instructions::ImmediateJumpwithLink { condition, label } => todo!(),
            Instructions::ImmediateRelativeJumpwithLink { condition, offset } => todo!(),
            Instructions::IntegerLoadLow { rx, value } => todo!(),
            Instructions::IntegerLoadHigh { rx, value } => todo!(),
            Instructions::SwapIntegerRegisters { rx, ry } => todo!(),
            Instructions::CopyIntegerRegister { rx, ry } => todo!(),
            Instructions::LoadIntegerRegisterIndirect { rx, ry, i, s } => todo!(),
            Instructions::LoadIntegerRegisterIndirectwithRegisterOffset { rx, ry, ro, s } => todo!(),
            Instructions::LoadIntegerRegisterIndirectProgram { rx, ry, i, s } => todo!(),
            Instructions::LoadIntegerRegisterIndirectwithRegisterOffsetProgram { rx, ry, ro, s } => todo!(),
            Instructions::StoreIntegerRegisterIndirect { rx, ry, i, s } => todo!(),
            Instructions::StoreIntegerRegisterIndirectwithRegisterOffsetIndirect { rx, ry, ro, s } => todo!(),
            Instructions::StoreIntegerRegisterIndirectProgram { rx, ry, i, s } => todo!(),
            Instructions::StoreIntegerRegisterIndirectwithRegisterOffsetProgram { rx, ry, ro, s } => todo!(),
            Instructions::IntegerLoadData { rx, label } => todo!(),
            Instructions::IntegerLoadProgram { rx, label } => todo!(),
            Instructions::IntegerStoreData { rx, label } => todo!(),
            Instructions::IntegerStoreProgram { rx, label } => todo!(),
            Instructions::UnsignedZeroExtend { rx, ry, count } => todo!(),
            Instructions::SignExtend { rx, ry, count } => todo!(),
            Instructions::FloatingPointLoadLow { fx, value } => todo!(),
            Instructions::FloatingPointLoadHigh { fx, value } => todo!(),
            Instructions::SwapFloatingPointRegisters { fx, fy } => todo!(),
            Instructions::CopyFloatingPointRegister { fx, fy } => todo!(),
            Instructions::LoadFloatingPointRegisterIndirect { fx, ry, i, s } => todo!(),
            Instructions::LoadFloatingPointRegisterIndirectwithRegisterOffset { fx, ry, ro, s } => todo!(),
            Instructions::StoreFloatingPointRegisterIndirect { rx, fy, i, s } => todo!(),
            Instructions::StoreFloatingPointRegisterIndirectwithRegisterOffset { rx, fy, ro, s } => todo!(),
            Instructions::FloatingPointLoadData { rx, label } => todo!(),
            Instructions::FloatingPointStoreData { rx, label } => todo!(),
            Instructions::IntegerCompare { rx, ry } => todo!(),
            Instructions::IntegerCompareSingleAgainstZero { rx } => todo!(),
            Instructions::AddUnsignedInteger { rx, ry, rz } => todo!(),
            Instructions::SubtractUnsignedInteger { rx, ry, rz } => todo!(),
            Instructions::MultiplyUnsignedInteger { rx, ry, rz } => todo!(),
            Instructions::DivideUnsignedInteger { rx, ry, rz } => todo!(),
            Instructions::ModuloUnsignedInteger { rx, ry, rz } => todo!(),
            Instructions::AddSignedInteger { rx, ry, rz } => todo!(),
            Instructions::SubtractSignedInteger { rx, ry, rz } => todo!(),
            Instructions::MultiplySignedInteger { rx, ry, rz } => todo!(),
            Instructions::DivideSignedInteger { rx, ry, rz } => todo!(),
            Instructions::ModuloSignedInteger { rx, ry, rz } => todo!(),
            Instructions::BitwiseAND { rx, ry, rz } => todo!(),
            Instructions::BitwiseOR { rx, ry, rz } => todo!(),
            Instructions::BitwiseNOT { rx, ry, rz } => todo!(),
            Instructions::BitwiseXOR { rx, ry, rz } => todo!(),
            Instructions::LogicalShiftLeft { rx, ry, value } => todo!(),
            Instructions::LogicalShiftRight { rx, ry, value } => todo!(),
            Instructions::ArithmeticShiftLeft { rx, ry, value } => todo!(),
            Instructions::ArithmeticShiftRight { rx, ry, value } => todo!(),
            Instructions::RotateRight { rx, ry, value } => todo!(),
            Instructions::LogicalShiftLeftRegister { rx, ry, rz } => todo!(),
            Instructions::LogicalShiftRightRegister { rx, ry, rz } => todo!(),
            Instructions::ArithmeticShiftLeftRegister { rx, ry, rz } => todo!(),
            Instructions::ArithmeticShiftRightRegister { rx, ry, rz } => todo!(),
            Instructions::RotateRightRegister { rx, ry, rz } => todo!(),
            Instructions::MapUnsignedToSigned { rx, ry } => todo!(),
            Instructions::MapSignedToUnsigned { rx, ry } => todo!(),
            Instructions::FloatingPointCompare { fx, fy } => todo!(),
            Instructions::FloatingPointCompareSingleAgainstZero { fx } => todo!(),
            Instructions::AddFloatingPoint { fx, fy, fz } => todo!(),
            Instructions::SubtractFloatingPoint { fx, fy, fz } => todo!(),
            Instructions::MultiplyFloatingPoint { fx, fy, fz } => todo!(),
            Instructions::DivideFloatingPoint { fx, fy, fz } => todo!(),
            Instructions::CasttoFloat { fx, ry } => todo!(),
            Instructions::CastfromFloat { rx, fy } => todo!(),
            Instructions::SetTimer { tx, ry } => todo!(),
            Instructions::GetCurrentTimer { tx, ry } => todo!(),
            Instructions::CheckTimer { tx } => todo!(),
            Instructions::ClearTimer { tx } => todo!(),
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