use crate::{enums::{Condition, FPRegister, Register, Timer}, raw_cast_from_i32, raw_cast_to_i32, RegisterSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Invalid(u32),
    Trap,
    PushIntegerRegister { rx: Register },
    PushFloatingPointRegister { fx: FPRegister },
    PopIntegerRegister { rx: Register },
    PopFloatingPointRegister { fx: FPRegister },
    SwapRegister { rx: Register, fy: FPRegister },
    StallImmediate { value: u32 },
    StallRegister { rx: Register },
    RegisterJump { l: bool, condition: Condition, rx: Register },
    IndirectJump { l: bool, condition: Condition, rx: Register, i: u32, s: u32 },
    IndirectwithRegisterOffsetJump { l: bool, condition: Condition, rx: Register, ro: Register, s: u32 },
    RelativeJump { l: bool, condition: Condition, rx: Register },
    ImmediateJump { l: bool, condition: Condition, label: u32 },
    ImmediateRelativeJump { l: bool, condition: Condition, offset: i32 },
    IntegerLoadLow { rx: Register, value: u32 },
    IntegerLoadHigh { rx: Register, value: u32 },
    SwapIntegerRegisters { rx: Register, ry: Register },
    CopyIntegerRegister { rx: Register, ry: Register },
    LoadIntegerRegisterIndirect { rx: Register, ry: Register, i: u32, s: u32 },
    LoadIntegerRegisterIndirectwithRegisterOffset { rx: Register, ry: Register, ro: Register, s: u32 },
    LoadIntegerRegisterIndirectProgram { rx: Register, ry: Register, i: u32, s: u32 },
    LoadIntegerRegisterIndirectwithRegisterOffsetProgram { rx: Register, ry: Register, ro: Register, s: u32 },
    StoreIntegerRegisterIndirect { rx: Register, ry: Register, i: u32, s: u32 },
    StoreIntegerRegisterIndirectwithRegisterOffsetIndirect { rx: Register, ry: Register, ro: Register, s: u32 },
    StoreIntegerRegisterIndirectProgram { rx: Register, ry: Register, i: u32, s: u32 },
    StoreIntegerRegisterIndirectwithRegisterOffsetProgram { rx: Register, ry: Register, ro: Register, s: u32 },
    IntegerLoadData { rx: Register, label: u32 },
    IntegerLoadProgram { rx: Register, label: u32 },
    IntegerStoreData { rx: Register, label: u32 },
    IntegerStoreProgram { rx: Register, label: u32 },
    IntegerLoadEffectiveDataAddress { rx: Register, label: u32 },
    IntegerLoadEffectiveProgramAddress { rx: Register, label: u32 },
    LoadIntegerEffectiveAddressRegisterIndirect { rx: Register, ry: Register, i: u32, s: u32 },
    LoadIntegerEffectiveAddressRegisterIndirectwithRegisterOffset { rx: Register, ry: Register, ro: Register, s: u32 },
    UnsignedZeroExtend { rx: Register, ry: Register, count: u32 },
    SignExtend { rx: Register, ry: Register, count: u32 },
    FloatingPointLoadLow { fx: FPRegister, value: u32 },
    FloatingPointLoadHigh { fx: FPRegister, value: u32 },
    SwapFloatingPointRegisters { fx: FPRegister, fy: FPRegister },
    CopyFloatingPointRegister { fx: FPRegister, fy: FPRegister },
    LoadFloatingPointRegisterIndirect { fx: FPRegister, ry: Register, i: u32, s: u32 },
    LoadFloatingPointRegisterIndirectwithRegisterOffset { fx: FPRegister, ry: Register, ro: Register, s: u32 },
    StoreFloatingPointRegisterIndirect { rx: Register, fy: FPRegister, i: u32, s: u32 },
    StoreFloatingPointRegisterIndirectwithRegisterOffset { rx: Register, fy: FPRegister, ro: Register, s: u32 },
    FloatingPointLoadData { fx: FPRegister, label: u32 },
    FloatingPointStoreData { fx: FPRegister, label: u32 },
    IntegerCompare { rx: Register, ry: Register },
    IntegerCompareSingleAgainstZero { rx: Register },
    IncrementIntegerRegister { c: bool, rx: Register },
    DecrementIntegerRegister { c: bool, rx: Register },
    AddUnsignedInteger { c: bool, rx: Register, ry: Register, rz: Register },
    SubtractUnsignedInteger { c: bool, rx: Register, ry: Register, rz: Register },
    MultiplyUnsignedInteger { c: bool, rx: Register, ry: Register, rz: Register },
    DivideUnsignedInteger { c: bool, rx: Register, ry: Register, rz: Register },
    ModuloUnsignedInteger { c: bool, rx: Register, ry: Register, rz: Register },
    AddSignedInteger { c: bool, rx: Register, ry: Register, rz: Register },
    SubtractSignedInteger { c: bool, rx: Register, ry: Register, rz: Register },
    MultiplySignedInteger { c: bool, rx: Register, ry: Register, rz: Register },
    DivideSignedInteger { c: bool, rx: Register, ry: Register, rz: Register },
    ModuloSignedInteger { c: bool, rx: Register, ry: Register, rz: Register },
    BitwiseAND { rx: Register, ry: Register, rz: Register },
    BitwiseOR { rx: Register, ry: Register, rz: Register },
    BitwiseNOT { rx: Register, ry: Register },
    BitwiseXOR { rx: Register, ry: Register, rz: Register },
    LogicalShiftLeft { rx: Register, ry: Register, value: u32 },
    LogicalShiftRight { rx: Register, ry: Register, value: u32 },
    ArithmeticShiftLeft { rx: Register, ry: Register, value: u32 },
    ArithmeticShiftRight { rx: Register, ry: Register, value: u32 },
    RotateRight { rx: Register, ry: Register, value: u32 },
    LogicalShiftLeftRegister { rx: Register, ry: Register, rz: Register },
    LogicalShiftRightRegister { rx: Register, ry: Register, rz: Register },
    ArithmeticShiftLeftRegister { rx: Register, ry: Register, rz: Register },
    ArithmeticShiftRightRegister { rx: Register, ry: Register, rz: Register },
    RotateRightRegister { rx: Register, ry: Register, rz: Register },
    MapUnsignedToSigned { rx: Register, ry: Register },
    MapSignedToUnsigned { rx: Register, ry: Register },
    FloatingPointCompare { fx: FPRegister, fy: FPRegister },
    FloatingPointCompareSingleAgainstZero { fx: FPRegister },
    AddFloatingPoint { c: bool, fx: FPRegister, fy: FPRegister, fz: FPRegister },
    SubtractFloatingPoint { c: bool, fx: FPRegister, fy: FPRegister, fz: FPRegister },
    MultiplyFloatingPoint { c: bool, fx: FPRegister, fy: FPRegister, fz: FPRegister },
    DivideFloatingPoint { c: bool, fx: FPRegister, fy: FPRegister, fz: FPRegister },
    CastToFloat { c: bool, fx: FPRegister, ry: Register },
    CastFromFloat { c: bool, rx: Register, fy: FPRegister },
    NegateFloatingPoint { c: bool, fx: FPRegister, fy: FPRegister },
    AbsoluteValueFloatingPoint { c: bool, fx: FPRegister, fy: FPRegister },
    RoundFloatingPoint { c: bool, fx: FPRegister, fy: FPRegister },
    RoundToZeroFloatingPoint { c: bool, fx: FPRegister, fy: FPRegister },
    RoundToInfinityFloatingPoint { c: bool, fx: FPRegister, fy: FPRegister },
    SquareRootFloatingPoint { c: bool, fx: FPRegister, fy: FPRegister },
    LogBase10FloatingPoint { c: bool, fx: FPRegister, fy: FPRegister },
    LogNatrualFloatingPoint { c: bool, fx: FPRegister, fy: FPRegister },
    ExponentialFloatingPoint { c: bool, fx: FPRegister, fy: FPRegister },
    SineFloatingPoint { c: bool, fx: FPRegister, fy: FPRegister },
    CosineFloatingPoint { c: bool, fx: FPRegister, fy: FPRegister },
    TangentFloatingPoint { c: bool, fx: FPRegister, fy: FPRegister },
    ArcsineFloatingPoint { c: bool, fx: FPRegister, fy: FPRegister },
    ArccosineFloatingPoint { c: bool, fx: FPRegister, fy: FPRegister },
    ArctangentFloatingPoint { c: bool, fx: FPRegister, fy: FPRegister },
    SetTimer { tx: Timer, ry: Register },
    GetCurrentTimer { rx: Register, ty: Timer },
    CheckTimer { tx: Timer },
    ClearTimer { tx: Timer },
    StallTimer { tx: Timer },
}

impl From<u32> for Instruction {
    fn from(value: u32) -> Self {
        match value >> 24 {
            0x00 => Self::Trap,
            0x01 => Self::PushIntegerRegister { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap() },
            0x02 => Self::PushFloatingPointRegister { fx: FPRegister::try_from(((value as usize) >> 19) & 0x1f).unwrap() },
            0x03 => Self::PopIntegerRegister { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap() },
            0x04 => Self::PopFloatingPointRegister { fx: FPRegister::try_from(((value as usize) >> 19) & 0x1f).unwrap() },
            0x05 => Self::SwapRegister { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), fy: FPRegister::try_from(((value as usize) >> 14) & 0x1f).unwrap() },
            0x06 => Self::StallImmediate { value: (value >> 8) & 0xffff },
            0x07 => Self::StallRegister { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap() },
            0x20 => Self::RegisterJump { l: (((value as usize) >> 23) & 0x1 > 0), condition: Condition::try_from(((value as usize) >> 18) & 0x1f).unwrap(), rx: Register::try_from(((value as usize) >> 13) & 0x1f).unwrap() },
            0x21 => Self::IndirectJump { l: (((value as usize) >> 23) & 0x1 > 0), condition: Condition::try_from(((value as usize) >> 18) & 0x1f).unwrap(), rx: Register::try_from(((value as usize) >> 13) & 0x1f).unwrap(), i: (value >> 8) & 0x1f, s: (value >> 4) & 0xf },
            0x22 => Self::IndirectwithRegisterOffsetJump { l: (((value as usize) >> 23) & 0x1 > 0), condition: Condition::try_from(((value as usize) >> 18) & 0x1f).unwrap(), rx: Register::try_from(((value as usize) >> 13) & 0x1f).unwrap(), ro: Register::try_from(((value as usize) >> 8) & 0x1f).unwrap(), s: (value >> 4) & 0xf },
            0x23 => Self::RelativeJump { l: (((value as usize) >> 23) & 0x1 > 0), condition: Condition::try_from(((value as usize) >> 18) & 0x1f).unwrap(), rx: Register::try_from(((value as usize) >> 13) & 0x1f).unwrap() },
            0x24 => Self::ImmediateJump { l: (((value as usize) >> 23) & 0x1 > 0), condition: Condition::try_from(((value as usize) >> 18) & 0x1f).unwrap(), label: (value >> 2) & 0xffff },
            0x25 => Self::ImmediateRelativeJump { l: (((value as usize) >> 23) & 0x1 > 0), condition: Condition::try_from(((value as usize) >> 18) & 0x1f).unwrap(), offset: raw_cast_to_i32((value >> 2) & 0xffff) },
            0x40 => Self::IntegerLoadLow { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), value: (value >> 3) & 0xffff },
            0x41 => Self::IntegerLoadHigh { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), value: (value >> 3) & 0xffff },
            0x42 => Self::SwapIntegerRegisters { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap() },
            0x43 => Self::CopyIntegerRegister { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap() },
            0x44 => Self::LoadIntegerRegisterIndirect { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap(), i: (value >> 9) & 0x1f, s: (value >> 5) & 0xf },
            0x45 => Self::LoadIntegerRegisterIndirectwithRegisterOffset { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap(), ro: Register::try_from(((value as usize) >> 9) & 0x1f).unwrap(), s: (value >> 5) & 0xf },
            0x46 => Self::LoadIntegerRegisterIndirectProgram { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap(), i: (value >> 9) & 0x1f, s: (value >> 5) & 0xf },
            0x47 => Self::LoadIntegerRegisterIndirectwithRegisterOffsetProgram { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap(), ro: Register::try_from(((value as usize) >> 9) & 0x1f).unwrap(), s: (value >> 5) & 0xf },
            0x48 => Self::StoreIntegerRegisterIndirect { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap(), i: (value >> 9) & 0x1f, s: (value >> 5) & 0xf },
            0x49 => Self::StoreIntegerRegisterIndirectwithRegisterOffsetIndirect { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap(), ro: Register::try_from(((value as usize) >> 9) & 0x1f).unwrap(), s: (value >> 5) & 0xf },
            0x4a => Self::StoreIntegerRegisterIndirectProgram { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap(), i: (value >> 9) & 0x1f, s: (value >> 5) & 0xf },
            0x4b => Self::StoreIntegerRegisterIndirectwithRegisterOffsetProgram { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap(), ro: Register::try_from(((value as usize) >> 9) & 0x1f).unwrap(), s: (value >> 5) & 0xf },
            0x4c => Self::IntegerLoadData { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), label: (value >> 3) & 0xffff },
            0x4d => Self::IntegerLoadProgram { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), label: (value >> 3) & 0xffff },
            0x4e => Self::IntegerStoreData { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), label: (value >> 3) & 0xffff },
            0x4f => Self::IntegerStoreProgram { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), label: (value >> 3) & 0xffff },
            0x50 => Self::IntegerLoadEffectiveDataAddress { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), label: (value >> 3) & 0xffff },
            0x51 => Self::IntegerLoadEffectiveProgramAddress { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), label: (value >> 3) & 0xffff },
            0x52 => Self::LoadIntegerEffectiveAddressRegisterIndirect { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap(), i: (value >> 9) & 0x1f, s: (value >> 5) & 0xf },
            0x53 => Self::LoadIntegerEffectiveAddressRegisterIndirectwithRegisterOffset { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap(), ro: Register::try_from(((value as usize) >> 9) & 0x1f).unwrap(), s: (value >> 5) & 0xf },
            0x54 => Self::UnsignedZeroExtend { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap(), count: (value >> 9) & 0x1f },
            0x55 => Self::SignExtend { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap(), count: (value >> 9) & 0x1f },
            0x60 => Self::FloatingPointLoadLow { fx: FPRegister::try_from(((value as usize) >> 19) & 0x1f).unwrap(), value: (value >> 3) & 0xffff },
            0x61 => Self::FloatingPointLoadHigh { fx: FPRegister::try_from(((value as usize) >> 19) & 0x1f).unwrap(), value: (value >> 3) & 0xffff },
            0x62 => Self::SwapFloatingPointRegisters { fx: FPRegister::try_from(((value as usize) >> 19) & 0x1f).unwrap(), fy: FPRegister::try_from(((value as usize) >> 14) & 0x1f).unwrap() },
            0x63 => Self::CopyFloatingPointRegister { fx: FPRegister::try_from(((value as usize) >> 19) & 0x1f).unwrap(), fy: FPRegister::try_from(((value as usize) >> 14) & 0x1f).unwrap() },
            0x64 => Self::LoadFloatingPointRegisterIndirect { fx: FPRegister::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap(), i: (value >> 9) & 0x1f, s: (value >> 5) & 0xf },
            0x65 => Self::LoadFloatingPointRegisterIndirectwithRegisterOffset { fx: FPRegister::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap(), ro: Register::try_from(((value as usize) >> 9) & 0x1f).unwrap(), s: (value >> 5) & 0xf },
            0x66 => Self::StoreFloatingPointRegisterIndirect { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), fy: FPRegister::try_from(((value as usize) >> 14) & 0x1f).unwrap(), i: (value >> 9) & 0x1f, s: (value >> 5) & 0xf },
            0x67 => Self::StoreFloatingPointRegisterIndirectwithRegisterOffset { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), fy: FPRegister::try_from(((value as usize) >> 14) & 0x1f).unwrap(), ro: Register::try_from(((value as usize) >> 9) & 0x1f).unwrap(), s: (value >> 5) & 0xf },
            0x68 => Self::FloatingPointLoadData { fx: FPRegister::try_from(((value as usize) >> 19) & 0x1f).unwrap(), label: (value >> 3) & 0xffff },
            0x69 => Self::FloatingPointStoreData { fx: FPRegister::try_from(((value as usize) >> 19) & 0x1f).unwrap(), label: (value >> 3) & 0xffff },
            0x80 => Self::IntegerCompare { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap() },
            0x81 => Self::IntegerCompareSingleAgainstZero { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap() },
            0x82 => Self::IncrementIntegerRegister { c: (((value as usize) >> 23) & 0x1 > 0), rx: Register::try_from(((value as usize) >> 18) & 0x1f).unwrap() },
            0x83 => Self::DecrementIntegerRegister { c: (((value as usize) >> 23) & 0x1 > 0), rx: Register::try_from(((value as usize) >> 18) & 0x1f).unwrap() },
            0x84 => Self::AddUnsignedInteger { c: (((value as usize) >> 23) & 0x1 > 0), rx: Register::try_from(((value as usize) >> 18) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 13) & 0x1f).unwrap(), rz: Register::try_from(((value as usize) >> 8) & 0x1f).unwrap() },
            0x85 => Self::SubtractUnsignedInteger { c: (((value as usize) >> 23) & 0x1 > 0), rx: Register::try_from(((value as usize) >> 18) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 13) & 0x1f).unwrap(), rz: Register::try_from(((value as usize) >> 8) & 0x1f).unwrap() },
            0x86 => Self::MultiplyUnsignedInteger { c: (((value as usize) >> 23) & 0x1 > 0), rx: Register::try_from(((value as usize) >> 18) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 13) & 0x1f).unwrap(), rz: Register::try_from(((value as usize) >> 8) & 0x1f).unwrap() },
            0x87 => Self::DivideUnsignedInteger { c: (((value as usize) >> 23) & 0x1 > 0), rx: Register::try_from(((value as usize) >> 18) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 13) & 0x1f).unwrap(), rz: Register::try_from(((value as usize) >> 8) & 0x1f).unwrap() },
            0x88 => Self::ModuloUnsignedInteger { c: (((value as usize) >> 23) & 0x1 > 0), rx: Register::try_from(((value as usize) >> 18) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 13) & 0x1f).unwrap(), rz: Register::try_from(((value as usize) >> 8) & 0x1f).unwrap() },
            0x89 => Self::AddSignedInteger { c: (((value as usize) >> 23) & 0x1 > 0), rx: Register::try_from(((value as usize) >> 18) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 13) & 0x1f).unwrap(), rz: Register::try_from(((value as usize) >> 8) & 0x1f).unwrap() },
            0x8a => Self::SubtractSignedInteger { c: (((value as usize) >> 23) & 0x1 > 0), rx: Register::try_from(((value as usize) >> 18) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 13) & 0x1f).unwrap(), rz: Register::try_from(((value as usize) >> 8) & 0x1f).unwrap() },
            0x8b => Self::MultiplySignedInteger { c: (((value as usize) >> 23) & 0x1 > 0), rx: Register::try_from(((value as usize) >> 18) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 13) & 0x1f).unwrap(), rz: Register::try_from(((value as usize) >> 8) & 0x1f).unwrap() },
            0x8c => Self::DivideSignedInteger { c: (((value as usize) >> 23) & 0x1 > 0), rx: Register::try_from(((value as usize) >> 18) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 13) & 0x1f).unwrap(), rz: Register::try_from(((value as usize) >> 8) & 0x1f).unwrap() },
            0x8d => Self::ModuloSignedInteger { c: (((value as usize) >> 23) & 0x1 > 0), rx: Register::try_from(((value as usize) >> 18) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 13) & 0x1f).unwrap(), rz: Register::try_from(((value as usize) >> 8) & 0x1f).unwrap() },
            0x8e => Self::BitwiseAND { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap(), rz: Register::try_from(((value as usize) >> 9) & 0x1f).unwrap() },
            0x8f => Self::BitwiseOR { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap(), rz: Register::try_from(((value as usize) >> 9) & 0x1f).unwrap() },
            0x90 => Self::BitwiseNOT { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap() },
            0x91 => Self::BitwiseXOR { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap(), rz: Register::try_from(((value as usize) >> 9) & 0x1f).unwrap() },
            0x92 => Self::LogicalShiftLeft { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap(), value: (value >> 9) & 0x1f },
            0x93 => Self::LogicalShiftRight { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap(), value: (value >> 9) & 0x1f },
            0x94 => Self::ArithmeticShiftLeft { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap(), value: (value >> 9) & 0x1f },
            0x95 => Self::ArithmeticShiftRight { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap(), value: (value >> 9) & 0x1f },
            0x96 => Self::RotateRight { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap(), value: (value >> 9) & 0x1f },
            0x97 => Self::LogicalShiftLeftRegister { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap(), rz: Register::try_from(((value as usize) >> 9) & 0x1f).unwrap() },
            0x98 => Self::LogicalShiftRightRegister { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap(), rz: Register::try_from(((value as usize) >> 9) & 0x1f).unwrap() },
            0x99 => Self::ArithmeticShiftLeftRegister { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap(), rz: Register::try_from(((value as usize) >> 9) & 0x1f).unwrap() },
            0x9a => Self::ArithmeticShiftRightRegister { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap(), rz: Register::try_from(((value as usize) >> 9) & 0x1f).unwrap() },
            0x9b => Self::RotateRightRegister { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap(), rz: Register::try_from(((value as usize) >> 9) & 0x1f).unwrap() },
            0x9c => Self::MapUnsignedToSigned { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap() },
            0x9d => Self::MapSignedToUnsigned { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap() },
            0xa0 => Self::FloatingPointCompare { fx: FPRegister::try_from(((value as usize) >> 19) & 0x1f).unwrap(), fy: FPRegister::try_from(((value as usize) >> 14) & 0x1f).unwrap() },
            0xa1 => Self::FloatingPointCompareSingleAgainstZero { fx: FPRegister::try_from(((value as usize) >> 19) & 0x1f).unwrap() },
            0xa2 => Self::AddFloatingPoint { c: (((value as usize) >> 23) & 0x1 > 0), fx: FPRegister::try_from(((value as usize) >> 18) & 0x1f).unwrap(), fy: FPRegister::try_from(((value as usize) >> 13) & 0x1f).unwrap(), fz: FPRegister::try_from(((value as usize) >> 8) & 0x1f).unwrap() },
            0xa3 => Self::SubtractFloatingPoint { c: (((value as usize) >> 23) & 0x1 > 0), fx: FPRegister::try_from(((value as usize) >> 18) & 0x1f).unwrap(), fy: FPRegister::try_from(((value as usize) >> 13) & 0x1f).unwrap(), fz: FPRegister::try_from(((value as usize) >> 8) & 0x1f).unwrap() },
            0xa4 => Self::MultiplyFloatingPoint { c: (((value as usize) >> 23) & 0x1 > 0), fx: FPRegister::try_from(((value as usize) >> 18) & 0x1f).unwrap(), fy: FPRegister::try_from(((value as usize) >> 13) & 0x1f).unwrap(), fz: FPRegister::try_from(((value as usize) >> 8) & 0x1f).unwrap() },
            0xa5 => Self::DivideFloatingPoint { c: (((value as usize) >> 23) & 0x1 > 0), fx: FPRegister::try_from(((value as usize) >> 18) & 0x1f).unwrap(), fy: FPRegister::try_from(((value as usize) >> 13) & 0x1f).unwrap(), fz: FPRegister::try_from(((value as usize) >> 8) & 0x1f).unwrap() },
            0xa6 => Self::CastToFloat { c: (((value as usize) >> 23) & 0x1 > 0), fx: FPRegister::try_from(((value as usize) >> 18) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 13) & 0x1f).unwrap() },
            0xa7 => Self::CastFromFloat { c: (((value as usize) >> 23) & 0x1 > 0), rx: Register::try_from(((value as usize) >> 18) & 0x1f).unwrap(), fy: FPRegister::try_from(((value as usize) >> 13) & 0x1f).unwrap() },
            0xa8 => Self::NegateFloatingPoint { c: (((value as usize) >> 23) & 0x1 > 0), fx: FPRegister::try_from(((value as usize) >> 18) & 0x1f).unwrap(), fy: FPRegister::try_from(((value as usize) >> 13) & 0x1f).unwrap() },
            0xa9 => Self::AbsoluteValueFloatingPoint { c: (((value as usize) >> 23) & 0x1 > 0), fx: FPRegister::try_from(((value as usize) >> 18) & 0x1f).unwrap(), fy: FPRegister::try_from(((value as usize) >> 13) & 0x1f).unwrap() },
            0xaa => Self::RoundFloatingPoint { c: (((value as usize) >> 23) & 0x1 > 0), fx: FPRegister::try_from(((value as usize) >> 18) & 0x1f).unwrap(), fy: FPRegister::try_from(((value as usize) >> 13) & 0x1f).unwrap() },
            0xab => Self::RoundToZeroFloatingPoint { c: (((value as usize) >> 23) & 0x1 > 0), fx: FPRegister::try_from(((value as usize) >> 18) & 0x1f).unwrap(), fy: FPRegister::try_from(((value as usize) >> 13) & 0x1f).unwrap() },
            0xac => Self::RoundToInfinityFloatingPoint { c: (((value as usize) >> 23) & 0x1 > 0), fx: FPRegister::try_from(((value as usize) >> 18) & 0x1f).unwrap(), fy: FPRegister::try_from(((value as usize) >> 13) & 0x1f).unwrap() },
            0xad => Self::SquareRootFloatingPoint { c: (((value as usize) >> 23) & 0x1 > 0), fx: FPRegister::try_from(((value as usize) >> 18) & 0x1f).unwrap(), fy: FPRegister::try_from(((value as usize) >> 13) & 0x1f).unwrap() },
            0xae => Self::LogBase10FloatingPoint { c: (((value as usize) >> 23) & 0x1 > 0), fx: FPRegister::try_from(((value as usize) >> 18) & 0x1f).unwrap(), fy: FPRegister::try_from(((value as usize) >> 13) & 0x1f).unwrap() },
            0xaf => Self::LogNatrualFloatingPoint { c: (((value as usize) >> 23) & 0x1 > 0), fx: FPRegister::try_from(((value as usize) >> 18) & 0x1f).unwrap(), fy: FPRegister::try_from(((value as usize) >> 13) & 0x1f).unwrap() },
            0xb0 => Self::ExponentialFloatingPoint { c: (((value as usize) >> 23) & 0x1 > 0), fx: FPRegister::try_from(((value as usize) >> 18) & 0x1f).unwrap(), fy: FPRegister::try_from(((value as usize) >> 13) & 0x1f).unwrap() },
            0xb1 => Self::SineFloatingPoint { c: (((value as usize) >> 23) & 0x1 > 0), fx: FPRegister::try_from(((value as usize) >> 18) & 0x1f).unwrap(), fy: FPRegister::try_from(((value as usize) >> 13) & 0x1f).unwrap() },
            0xb2 => Self::CosineFloatingPoint { c: (((value as usize) >> 23) & 0x1 > 0), fx: FPRegister::try_from(((value as usize) >> 18) & 0x1f).unwrap(), fy: FPRegister::try_from(((value as usize) >> 13) & 0x1f).unwrap() },
            0xb3 => Self::TangentFloatingPoint { c: (((value as usize) >> 23) & 0x1 > 0), fx: FPRegister::try_from(((value as usize) >> 18) & 0x1f).unwrap(), fy: FPRegister::try_from(((value as usize) >> 13) & 0x1f).unwrap() },
            0xb4 => Self::ArcsineFloatingPoint { c: (((value as usize) >> 23) & 0x1 > 0), fx: FPRegister::try_from(((value as usize) >> 18) & 0x1f).unwrap(), fy: FPRegister::try_from(((value as usize) >> 13) & 0x1f).unwrap() },
            0xb5 => Self::ArccosineFloatingPoint { c: (((value as usize) >> 23) & 0x1 > 0), fx: FPRegister::try_from(((value as usize) >> 18) & 0x1f).unwrap(), fy: FPRegister::try_from(((value as usize) >> 13) & 0x1f).unwrap() },
            0xb6 => Self::ArctangentFloatingPoint { c: (((value as usize) >> 23) & 0x1 > 0), fx: FPRegister::try_from(((value as usize) >> 18) & 0x1f).unwrap(), fy: FPRegister::try_from(((value as usize) >> 13) & 0x1f).unwrap() },
            0xc0 => Self::SetTimer { tx: Timer::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ry: Register::try_from(((value as usize) >> 14) & 0x1f).unwrap() },
            0xc1 => Self::GetCurrentTimer { rx: Register::try_from(((value as usize) >> 19) & 0x1f).unwrap(), ty: Timer::try_from(((value as usize) >> 14) & 0x1f).unwrap() },
            0xc2 => Self::CheckTimer { tx: Timer::try_from(((value as usize) >> 19) & 0x1f).unwrap() },
            0xc3 => Self::ClearTimer { tx: Timer::try_from(((value as usize) >> 19) & 0x1f).unwrap() },
            0xc4 => Self::StallTimer { tx: Timer::try_from(((value as usize) >> 19) & 0x1f).unwrap() },
            _ => Self::Invalid(value),
        }
    }
}

impl Into<u32> for Instruction {
    fn into(self) -> u32 {
        match self {
            Self::Trap => 0x00 << 24,
            Self::PushIntegerRegister { rx } => (0x01 << 24)| ((rx as u32) << 19),
            Self::PushFloatingPointRegister { fx } => (0x02 << 24)| ((fx as u32) << 19),
            Self::PopIntegerRegister { rx } => (0x03 << 24)| ((rx as u32) << 19),
            Self::PopFloatingPointRegister { fx } => (0x04 << 24)| ((fx as u32) << 19),
            Self::SwapRegister { rx, fy } => (0x05 << 24)| ((rx as u32) << 19)| ((fy as u32) << 14),
            Self::StallImmediate { value } => (0x06 << 24)| (value << 8),
            Self::StallRegister { rx } => (0x07 << 24)| ((rx as u32) << 19),
            Self::RegisterJump { l, condition, rx } => (0x20 << 24)| ((l as u32) << 23)| ((condition as u32) << 18)| ((rx as u32) << 13),
            Self::IndirectJump { l, condition, rx, i, s } => (0x21 << 24)| ((l as u32) << 23)| ((condition as u32) << 18)| ((rx as u32) << 13)| (i << 8)| (s << 4),
            Self::IndirectwithRegisterOffsetJump { l, condition, rx, ro, s } => (0x22 << 24)| ((l as u32) << 23)| ((condition as u32) << 18)| ((rx as u32) << 13)| ((ro as u32) << 8)| (s << 4),
            Self::RelativeJump { l, condition, rx } => (0x23 << 24)| ((l as u32) << 23)| ((condition as u32) << 18)| ((rx as u32) << 13),
            Self::ImmediateJump { l, condition, label } => (0x24 << 24)| ((l as u32) << 23)| ((condition as u32) << 18)| (label << 2),
            Self::ImmediateRelativeJump { l, condition, offset } => (0x25 << 24)| ((l as u32) << 23)| ((condition as u32) << 18)| (raw_cast_from_i32(offset) << 2),
            Self::IntegerLoadLow { rx, value } => (0x40 << 24)| ((rx as u32) << 19)| (value << 3),
            Self::IntegerLoadHigh { rx, value } => (0x41 << 24)| ((rx as u32) << 19)| (value << 3),
            Self::SwapIntegerRegisters { rx, ry } => (0x42 << 24)| ((rx as u32) << 19)| ((ry as u32) << 14),
            Self::CopyIntegerRegister { rx, ry } => (0x43 << 24)| ((rx as u32) << 19)| ((ry as u32) << 14),
            Self::LoadIntegerRegisterIndirect { rx, ry, i, s } => (0x44 << 24)| ((rx as u32) << 19)| ((ry as u32) << 14)| (i << 9)| (s << 5),
            Self::LoadIntegerRegisterIndirectwithRegisterOffset { rx, ry, ro, s } => (0x45 << 24)| ((rx as u32) << 19)| ((ry as u32) << 14)| ((ro as u32) << 9)| (s << 5),
            Self::LoadIntegerRegisterIndirectProgram { rx, ry, i, s } => (0x46 << 24)| ((rx as u32) << 19)| ((ry as u32) << 14)| (i << 9)| (s << 5),
            Self::LoadIntegerRegisterIndirectwithRegisterOffsetProgram { rx, ry, ro, s } => (0x47 << 24)| ((rx as u32) << 19)| ((ry as u32) << 14)| ((ro as u32) << 9)| (s << 5),
            Self::StoreIntegerRegisterIndirect { rx, ry, i, s } => (0x48 << 24)| ((rx as u32) << 19)| ((ry as u32) << 14)| (i << 9)| (s << 5),
            Self::StoreIntegerRegisterIndirectwithRegisterOffsetIndirect { rx, ry, ro, s } => (0x49 << 24)| ((rx as u32) << 19)| ((ry as u32) << 14)| ((ro as u32) << 9)| (s << 5),
            Self::StoreIntegerRegisterIndirectProgram { rx, ry, i, s } => (0x4a << 24)| ((rx as u32) << 19)| ((ry as u32) << 14)| (i << 9)| (s << 5),
            Self::StoreIntegerRegisterIndirectwithRegisterOffsetProgram { rx, ry, ro, s } => (0x4b << 24)| ((rx as u32) << 19)| ((ry as u32) << 14)| ((ro as u32) << 9)| (s << 5),
            Self::IntegerLoadData { rx, label } => (0x4c << 24)| ((rx as u32) << 19)| (label << 3),
            Self::IntegerLoadProgram { rx, label } => (0x4d << 24)| ((rx as u32) << 19)| (label << 3),
            Self::IntegerStoreData { rx, label } => (0x4e << 24)| ((rx as u32) << 19)| (label << 3),
            Self::IntegerStoreProgram { rx, label } => (0x4f << 24)| ((rx as u32) << 19)| (label << 3),
            Self::IntegerLoadEffectiveDataAddress { rx, label } => (0x50 << 24)| ((rx as u32) << 19)| (label << 3),
            Self::IntegerLoadEffectiveProgramAddress { rx, label } => (0x51 << 24)| ((rx as u32) << 19)| (label << 3),
            Self::LoadIntegerEffectiveAddressRegisterIndirect { rx, ry, i, s } => (0x52 << 24)| ((rx as u32) << 19)| ((ry as u32) << 14)| (i << 9)| (s << 5),
            Self::LoadIntegerEffectiveAddressRegisterIndirectwithRegisterOffset { rx, ry, ro, s } => (0x53 << 24)| ((rx as u32) << 19)| ((ry as u32) << 14)| ((ro as u32) << 9)| (s << 5),
            Self::UnsignedZeroExtend { rx, ry, count } => (0x54 << 24)| ((rx as u32) << 19)| ((ry as u32) << 14)| (count << 9),
            Self::SignExtend { rx, ry, count } => (0x55 << 24)| ((rx as u32) << 19)| ((ry as u32) << 14)| (count << 9),
            Self::FloatingPointLoadLow { fx, value } => (0x60 << 24)| ((fx as u32) << 19)| (value << 3),
            Self::FloatingPointLoadHigh { fx, value } => (0x61 << 24)| ((fx as u32) << 19)| (value << 3),
            Self::SwapFloatingPointRegisters { fx, fy } => (0x62 << 24)| ((fx as u32) << 19)| ((fy as u32) << 14),
            Self::CopyFloatingPointRegister { fx, fy } => (0x63 << 24)| ((fx as u32) << 19)| ((fy as u32) << 14),
            Self::LoadFloatingPointRegisterIndirect { fx, ry, i, s } => (0x64 << 24)| ((fx as u32) << 19)| ((ry as u32) << 14)| (i << 9)| (s << 5),
            Self::LoadFloatingPointRegisterIndirectwithRegisterOffset { fx, ry, ro, s } => (0x65 << 24)| ((fx as u32) << 19)| ((ry as u32) << 14)| ((ro as u32) << 9)| (s << 5),
            Self::StoreFloatingPointRegisterIndirect { rx, fy, i, s } => (0x66 << 24)| ((rx as u32) << 19)| ((fy as u32) << 14)| (i << 9)| (s << 5),
            Self::StoreFloatingPointRegisterIndirectwithRegisterOffset { rx, fy, ro, s } => (0x67 << 24)| ((rx as u32) << 19)| ((fy as u32) << 14)| ((ro as u32) << 9)| (s << 5),
            Self::FloatingPointLoadData { fx, label } => (0x68 << 24)| ((fx as u32) << 19)| (label << 3),
            Self::FloatingPointStoreData { fx, label } => (0x69 << 24)| ((fx as u32) << 19)| (label << 3),
            Self::IntegerCompare { rx, ry } => (0x80 << 24)| ((rx as u32) << 19)| ((ry as u32) << 14),
            Self::IntegerCompareSingleAgainstZero { rx } => (0x81 << 24)| ((rx as u32) << 19),
            Self::IncrementIntegerRegister { c, rx } => (0x82 << 24)| ((c as u32) << 23)| ((rx as u32) << 18),
            Self::DecrementIntegerRegister { c, rx } => (0x83 << 24)| ((c as u32) << 23)| ((rx as u32) << 18),
            Self::AddUnsignedInteger { c, rx, ry, rz } => (0x84 << 24)| ((c as u32) << 23)| ((rx as u32) << 18)| ((ry as u32) << 13)| ((rz as u32) << 8),
            Self::SubtractUnsignedInteger { c, rx, ry, rz } => (0x85 << 24)| ((c as u32) << 23)| ((rx as u32) << 18)| ((ry as u32) << 13)| ((rz as u32) << 8),
            Self::MultiplyUnsignedInteger { c, rx, ry, rz } => (0x86 << 24)| ((c as u32) << 23)| ((rx as u32) << 18)| ((ry as u32) << 13)| ((rz as u32) << 8),
            Self::DivideUnsignedInteger { c, rx, ry, rz } => (0x87 << 24)| ((c as u32) << 23)| ((rx as u32) << 18)| ((ry as u32) << 13)| ((rz as u32) << 8),
            Self::ModuloUnsignedInteger { c, rx, ry, rz } => (0x88 << 24)| ((c as u32) << 23)| ((rx as u32) << 18)| ((ry as u32) << 13)| ((rz as u32) << 8),
            Self::AddSignedInteger { c, rx, ry, rz } => (0x89 << 24)| ((c as u32) << 23)| ((rx as u32) << 18)| ((ry as u32) << 13)| ((rz as u32) << 8),
            Self::SubtractSignedInteger { c, rx, ry, rz } => (0x8a << 24)| ((c as u32) << 23)| ((rx as u32) << 18)| ((ry as u32) << 13)| ((rz as u32) << 8),
            Self::MultiplySignedInteger { c, rx, ry, rz } => (0x8b << 24)| ((c as u32) << 23)| ((rx as u32) << 18)| ((ry as u32) << 13)| ((rz as u32) << 8),
            Self::DivideSignedInteger { c, rx, ry, rz } => (0x8c << 24)| ((c as u32) << 23)| ((rx as u32) << 18)| ((ry as u32) << 13)| ((rz as u32) << 8),
            Self::ModuloSignedInteger { c, rx, ry, rz } => (0x8d << 24)| ((c as u32) << 23)| ((rx as u32) << 18)| ((ry as u32) << 13)| ((rz as u32) << 8),
            Self::BitwiseAND { rx, ry, rz } => (0x8e << 24)| ((rx as u32) << 19)| ((ry as u32) << 14)| ((rz as u32) << 9),
            Self::BitwiseOR { rx, ry, rz } => (0x8f << 24)| ((rx as u32) << 19)| ((ry as u32) << 14)| ((rz as u32) << 9),
            Self::BitwiseNOT { rx, ry } => (0x90 << 24)| ((rx as u32) << 19)| ((ry as u32) << 14),
            Self::BitwiseXOR { rx, ry, rz } => (0x91 << 24)| ((rx as u32) << 19)| ((ry as u32) << 14)| ((rz as u32) << 9),
            Self::LogicalShiftLeft { rx, ry, value } => (0x92 << 24)| ((rx as u32) << 19)| ((ry as u32) << 14)| (value << 9),
            Self::LogicalShiftRight { rx, ry, value } => (0x93 << 24)| ((rx as u32) << 19)| ((ry as u32) << 14)| (value << 9),
            Self::ArithmeticShiftLeft { rx, ry, value } => (0x94 << 24)| ((rx as u32) << 19)| ((ry as u32) << 14)| (value << 9),
            Self::ArithmeticShiftRight { rx, ry, value } => (0x95 << 24)| ((rx as u32) << 19)| ((ry as u32) << 14)| (value << 9),
            Self::RotateRight { rx, ry, value } => (0x96 << 24)| ((rx as u32) << 19)| ((ry as u32) << 14)| (value << 9),
            Self::LogicalShiftLeftRegister { rx, ry, rz } => (0x97 << 24)| ((rx as u32) << 19)| ((ry as u32) << 14)| ((rz as u32) << 9),
            Self::LogicalShiftRightRegister { rx, ry, rz } => (0x98 << 24)| ((rx as u32) << 19)| ((ry as u32) << 14)| ((rz as u32) << 9),
            Self::ArithmeticShiftLeftRegister { rx, ry, rz } => (0x99 << 24)| ((rx as u32) << 19)| ((ry as u32) << 14)| ((rz as u32) << 9),
            Self::ArithmeticShiftRightRegister { rx, ry, rz } => (0x9a << 24)| ((rx as u32) << 19)| ((ry as u32) << 14)| ((rz as u32) << 9),
            Self::RotateRightRegister { rx, ry, rz } => (0x9b << 24)| ((rx as u32) << 19)| ((ry as u32) << 14)| ((rz as u32) << 9),
            Self::MapUnsignedToSigned { rx, ry } => (0x9c << 24)| ((rx as u32) << 19)| ((ry as u32) << 14),
            Self::MapSignedToUnsigned { rx, ry } => (0x9d << 24)| ((rx as u32) << 19)| ((ry as u32) << 14),
            Self::FloatingPointCompare { fx, fy } => (0xa0 << 24)| ((fx as u32) << 19)| ((fy as u32) << 14),
            Self::FloatingPointCompareSingleAgainstZero { fx } => (0xa1 << 24)| ((fx as u32) << 19),
            Self::AddFloatingPoint { c, fx, fy, fz } => (0xa2 << 24)| ((c as u32) << 23)| ((fx as u32) << 18)| ((fy as u32) << 13)| ((fz as u32) << 8),
            Self::SubtractFloatingPoint { c, fx, fy, fz } => (0xa3 << 24)| ((c as u32) << 23)| ((fx as u32) << 18)| ((fy as u32) << 13)| ((fz as u32) << 8),
            Self::MultiplyFloatingPoint { c, fx, fy, fz } => (0xa4 << 24)| ((c as u32) << 23)| ((fx as u32) << 18)| ((fy as u32) << 13)| ((fz as u32) << 8),
            Self::DivideFloatingPoint { c, fx, fy, fz } => (0xa5 << 24)| ((c as u32) << 23)| ((fx as u32) << 18)| ((fy as u32) << 13)| ((fz as u32) << 8),
            Self::CastToFloat { c, fx, ry } => (0xa6 << 24)| ((c as u32) << 23)| ((fx as u32) << 18)| ((ry as u32) << 13),
            Self::CastFromFloat { c, rx, fy } => (0xa7 << 24)| ((c as u32) << 23)| ((rx as u32) << 18)| ((fy as u32) << 13),
            Self::NegateFloatingPoint { c, fx, fy } => (0xa8 << 24)| ((c as u32) << 23)| ((fx as u32) << 18)| ((fy as u32) << 13),
            Self::AbsoluteValueFloatingPoint { c, fx, fy } => (0xa9 << 24)| ((c as u32) << 23)| ((fx as u32) << 18)| ((fy as u32) << 13),
            Self::RoundFloatingPoint { c, fx, fy } => (0xaa << 24)| ((c as u32) << 23)| ((fx as u32) << 18)| ((fy as u32) << 13),
            Self::RoundToZeroFloatingPoint { c, fx, fy } => (0xab << 24)| ((c as u32) << 23)| ((fx as u32) << 18)| ((fy as u32) << 13),
            Self::RoundToInfinityFloatingPoint { c, fx, fy } => (0xac << 24)| ((c as u32) << 23)| ((fx as u32) << 18)| ((fy as u32) << 13),
            Self::SquareRootFloatingPoint { c, fx, fy } => (0xad << 24)| ((c as u32) << 23)| ((fx as u32) << 18)| ((fy as u32) << 13),
            Self::LogBase10FloatingPoint { c, fx, fy } => (0xae << 24)| ((c as u32) << 23)| ((fx as u32) << 18)| ((fy as u32) << 13),
            Self::LogNatrualFloatingPoint { c, fx, fy } => (0xaf << 24)| ((c as u32) << 23)| ((fx as u32) << 18)| ((fy as u32) << 13),
            Self::ExponentialFloatingPoint { c, fx, fy } => (0xb0 << 24)| ((c as u32) << 23)| ((fx as u32) << 18)| ((fy as u32) << 13),
            Self::SineFloatingPoint { c, fx, fy } => (0xb1 << 24)| ((c as u32) << 23)| ((fx as u32) << 18)| ((fy as u32) << 13),
            Self::CosineFloatingPoint { c, fx, fy } => (0xb2 << 24)| ((c as u32) << 23)| ((fx as u32) << 18)| ((fy as u32) << 13),
            Self::TangentFloatingPoint { c, fx, fy } => (0xb3 << 24)| ((c as u32) << 23)| ((fx as u32) << 18)| ((fy as u32) << 13),
            Self::ArcsineFloatingPoint { c, fx, fy } => (0xb4 << 24)| ((c as u32) << 23)| ((fx as u32) << 18)| ((fy as u32) << 13),
            Self::ArccosineFloatingPoint { c, fx, fy } => (0xb5 << 24)| ((c as u32) << 23)| ((fx as u32) << 18)| ((fy as u32) << 13),
            Self::ArctangentFloatingPoint { c, fx, fy } => (0xb6 << 24)| ((c as u32) << 23)| ((fx as u32) << 18)| ((fy as u32) << 13),
            Self::SetTimer { tx, ry } => (0xc0 << 24)| ((tx as u32) << 19)| ((ry as u32) << 14),
            Self::GetCurrentTimer { rx, ty } => (0xc1 << 24)| ((rx as u32) << 19)| ((ty as u32) << 14),
            Self::CheckTimer { tx } => (0xc2 << 24)| ((tx as u32) << 19),
            Self::ClearTimer { tx } => (0xc3 << 24)| ((tx as u32) << 19),
            Self::StallTimer { tx } => (0xc4 << 24)| ((tx as u32) << 19),
            Self::Invalid(value) => value,
        }
    }
}

impl Instruction {
    pub fn read_registers(&self) -> RegisterSet {
        match self {
            Self::Trap => RegisterSet{ registers: vec![], f_registers: vec![], timers: vec![]  },
            Self::PushIntegerRegister { rx } => RegisterSet{ registers: vec![*rx, Register::try_from(31).unwrap()], f_registers: vec![], timers: vec![]  },
            Self::PushFloatingPointRegister { fx } => RegisterSet{ registers: vec![Register::try_from(31).unwrap()], f_registers: vec![*fx], timers: vec![]  },
            Self::PopIntegerRegister { .. } => RegisterSet{ registers: vec![Register::try_from(31).unwrap()], f_registers: vec![], timers: vec![]  },
            Self::PopFloatingPointRegister { .. } => RegisterSet{ registers: vec![Register::try_from(31).unwrap()], f_registers: vec![], timers: vec![]  },
            Self::SwapRegister { rx, fy } => RegisterSet{ registers: vec![*rx], f_registers: vec![*fy], timers: vec![]  },
            Self::StallImmediate { .. } => RegisterSet{ registers: vec![], f_registers: vec![], timers: vec![]  },
            Self::StallRegister { rx } => RegisterSet{ registers: vec![*rx], f_registers: vec![], timers: vec![]  },
            Self::RegisterJump { rx, .. } => RegisterSet{ registers: vec![*rx, Register::try_from(30).unwrap()], f_registers: vec![], timers: vec![]  },
            Self::IndirectJump { rx, .. } => RegisterSet{ registers: vec![*rx, Register::try_from(30).unwrap()], f_registers: vec![], timers: vec![]  },
            Self::IndirectwithRegisterOffsetJump { rx, ro, .. } => RegisterSet{ registers: vec![*rx, *ro, Register::try_from(30).unwrap()], f_registers: vec![], timers: vec![]  },
            Self::RelativeJump { rx, .. } => RegisterSet{ registers: vec![*rx, Register::try_from(28).unwrap(), Register::try_from(30).unwrap()], f_registers: vec![], timers: vec![]  },
            Self::ImmediateJump { .. } => RegisterSet{ registers: vec![Register::try_from(30).unwrap()], f_registers: vec![], timers: vec![]  },
            Self::ImmediateRelativeJump { .. } => RegisterSet{ registers: vec![Register::try_from(28).unwrap(), Register::try_from(30).unwrap()], f_registers: vec![], timers: vec![]  },
            Self::IntegerLoadLow { .. } => RegisterSet{ registers: vec![], f_registers: vec![], timers: vec![]  },
            Self::IntegerLoadHigh { .. } => RegisterSet{ registers: vec![], f_registers: vec![], timers: vec![]  },
            Self::SwapIntegerRegisters { rx, ry } => RegisterSet{ registers: vec![*rx, *ry], f_registers: vec![], timers: vec![]  },
            Self::CopyIntegerRegister { ry, .. } => RegisterSet{ registers: vec![*ry], f_registers: vec![], timers: vec![]  },
            Self::LoadIntegerRegisterIndirect { ry, .. } => RegisterSet{ registers: vec![*ry], f_registers: vec![], timers: vec![]  },
            Self::LoadIntegerRegisterIndirectwithRegisterOffset { ry, ro, .. } => RegisterSet{ registers: vec![*ry, *ro], f_registers: vec![], timers: vec![]  },
            Self::LoadIntegerRegisterIndirectProgram { ry, .. } => RegisterSet{ registers: vec![*ry], f_registers: vec![], timers: vec![]  },
            Self::LoadIntegerRegisterIndirectwithRegisterOffsetProgram { ry, ro, .. } => RegisterSet{ registers: vec![*ry, *ro], f_registers: vec![], timers: vec![]  },
            Self::StoreIntegerRegisterIndirect { ry, .. } => RegisterSet{ registers: vec![*ry], f_registers: vec![], timers: vec![]  },
            Self::StoreIntegerRegisterIndirectwithRegisterOffsetIndirect { ry, ro, .. } => RegisterSet{ registers: vec![*ry, *ro], f_registers: vec![], timers: vec![]  },
            Self::StoreIntegerRegisterIndirectProgram { ry, .. } => RegisterSet{ registers: vec![*ry], f_registers: vec![], timers: vec![]  },
            Self::StoreIntegerRegisterIndirectwithRegisterOffsetProgram { ry, ro, .. } => RegisterSet{ registers: vec![*ry, *ro], f_registers: vec![], timers: vec![]  },
            Self::IntegerLoadData { .. } => RegisterSet{ registers: vec![], f_registers: vec![], timers: vec![]  },
            Self::IntegerLoadProgram { .. } => RegisterSet{ registers: vec![], f_registers: vec![], timers: vec![]  },
            Self::IntegerStoreData { rx, .. } => RegisterSet{ registers: vec![*rx], f_registers: vec![], timers: vec![]  },
            Self::IntegerStoreProgram { rx, .. } => RegisterSet{ registers: vec![*rx], f_registers: vec![], timers: vec![]  },
            Self::IntegerLoadEffectiveDataAddress { .. } => RegisterSet{ registers: vec![], f_registers: vec![], timers: vec![]  },
            Self::IntegerLoadEffectiveProgramAddress { .. } => RegisterSet{ registers: vec![], f_registers: vec![], timers: vec![]  },
            Self::LoadIntegerEffectiveAddressRegisterIndirect { ry, .. } => RegisterSet{ registers: vec![*ry], f_registers: vec![], timers: vec![]  },
            Self::LoadIntegerEffectiveAddressRegisterIndirectwithRegisterOffset { ry, ro, .. } => RegisterSet{ registers: vec![*ry, *ro], f_registers: vec![], timers: vec![]  },
            Self::UnsignedZeroExtend { ry, .. } => RegisterSet{ registers: vec![*ry], f_registers: vec![], timers: vec![]  },
            Self::SignExtend { ry, .. } => RegisterSet{ registers: vec![*ry], f_registers: vec![], timers: vec![]  },
            Self::FloatingPointLoadLow { .. } => RegisterSet{ registers: vec![], f_registers: vec![], timers: vec![]  },
            Self::FloatingPointLoadHigh { .. } => RegisterSet{ registers: vec![], f_registers: vec![], timers: vec![]  },
            Self::SwapFloatingPointRegisters { fx, fy } => RegisterSet{ registers: vec![], f_registers: vec![*fx, *fy], timers: vec![]  },
            Self::CopyFloatingPointRegister { fy, .. } => RegisterSet{ registers: vec![], f_registers: vec![*fy], timers: vec![]  },
            Self::LoadFloatingPointRegisterIndirect { ry, .. } => RegisterSet{ registers: vec![*ry], f_registers: vec![], timers: vec![]  },
            Self::LoadFloatingPointRegisterIndirectwithRegisterOffset { ry, ro, .. } => RegisterSet{ registers: vec![*ry, *ro], f_registers: vec![], timers: vec![]  },
            Self::StoreFloatingPointRegisterIndirect { fy, .. } => RegisterSet{ registers: vec![], f_registers: vec![*fy], timers: vec![]  },
            Self::StoreFloatingPointRegisterIndirectwithRegisterOffset { fy, ro, .. } => RegisterSet{ registers: vec![*ro], f_registers: vec![*fy], timers: vec![]  },
            Self::FloatingPointLoadData { .. } => RegisterSet{ registers: vec![], f_registers: vec![], timers: vec![]  },
            Self::FloatingPointStoreData { fx, .. } => RegisterSet{ registers: vec![], f_registers: vec![*fx], timers: vec![]  },
            Self::IntegerCompare { rx, ry } => RegisterSet{ registers: vec![*rx, *ry, Register::try_from(30).unwrap()], f_registers: vec![], timers: vec![]  },
            Self::IntegerCompareSingleAgainstZero { rx } => RegisterSet{ registers: vec![*rx, Register::try_from(30).unwrap()], f_registers: vec![], timers: vec![]  },
            Self::IncrementIntegerRegister { .. } => RegisterSet{ registers: vec![], f_registers: vec![], timers: vec![]  },
            Self::DecrementIntegerRegister { .. } => RegisterSet{ registers: vec![], f_registers: vec![], timers: vec![]  },
            Self::AddUnsignedInteger { ry, rz, .. } => RegisterSet{ registers: vec![*ry, *rz], f_registers: vec![], timers: vec![]  },
            Self::SubtractUnsignedInteger { ry, rz, .. } => RegisterSet{ registers: vec![*ry, *rz], f_registers: vec![], timers: vec![]  },
            Self::MultiplyUnsignedInteger { ry, rz, .. } => RegisterSet{ registers: vec![*ry, *rz], f_registers: vec![], timers: vec![]  },
            Self::DivideUnsignedInteger { ry, rz, .. } => RegisterSet{ registers: vec![*ry, *rz], f_registers: vec![], timers: vec![]  },
            Self::ModuloUnsignedInteger { ry, rz, .. } => RegisterSet{ registers: vec![*ry, *rz], f_registers: vec![], timers: vec![]  },
            Self::AddSignedInteger { ry, rz, .. } => RegisterSet{ registers: vec![*ry, *rz], f_registers: vec![], timers: vec![]  },
            Self::SubtractSignedInteger { ry, rz, .. } => RegisterSet{ registers: vec![*ry, *rz], f_registers: vec![], timers: vec![]  },
            Self::MultiplySignedInteger { ry, rz, .. } => RegisterSet{ registers: vec![*ry, *rz], f_registers: vec![], timers: vec![]  },
            Self::DivideSignedInteger { ry, rz, .. } => RegisterSet{ registers: vec![*ry, *rz], f_registers: vec![], timers: vec![]  },
            Self::ModuloSignedInteger { ry, rz, .. } => RegisterSet{ registers: vec![*ry, *rz], f_registers: vec![], timers: vec![]  },
            Self::BitwiseAND { ry, rz, .. } => RegisterSet{ registers: vec![*ry, *rz], f_registers: vec![], timers: vec![]  },
            Self::BitwiseOR { ry, rz, .. } => RegisterSet{ registers: vec![*ry, *rz], f_registers: vec![], timers: vec![]  },
            Self::BitwiseNOT { ry, .. } => RegisterSet{ registers: vec![*ry], f_registers: vec![], timers: vec![]  },
            Self::BitwiseXOR { ry, rz, .. } => RegisterSet{ registers: vec![*ry, *rz], f_registers: vec![], timers: vec![]  },
            Self::LogicalShiftLeft { ry, .. } => RegisterSet{ registers: vec![*ry], f_registers: vec![], timers: vec![]  },
            Self::LogicalShiftRight { ry, .. } => RegisterSet{ registers: vec![*ry], f_registers: vec![], timers: vec![]  },
            Self::ArithmeticShiftLeft { ry, .. } => RegisterSet{ registers: vec![*ry], f_registers: vec![], timers: vec![]  },
            Self::ArithmeticShiftRight { ry, .. } => RegisterSet{ registers: vec![*ry], f_registers: vec![], timers: vec![]  },
            Self::RotateRight { ry, .. } => RegisterSet{ registers: vec![*ry], f_registers: vec![], timers: vec![]  },
            Self::LogicalShiftLeftRegister { ry, rz, .. } => RegisterSet{ registers: vec![*ry, *rz], f_registers: vec![], timers: vec![]  },
            Self::LogicalShiftRightRegister { ry, rz, .. } => RegisterSet{ registers: vec![*ry, *rz], f_registers: vec![], timers: vec![]  },
            Self::ArithmeticShiftLeftRegister { ry, rz, .. } => RegisterSet{ registers: vec![*ry, *rz], f_registers: vec![], timers: vec![]  },
            Self::ArithmeticShiftRightRegister { ry, rz, .. } => RegisterSet{ registers: vec![*ry, *rz], f_registers: vec![], timers: vec![]  },
            Self::RotateRightRegister { ry, rz, .. } => RegisterSet{ registers: vec![*ry, *rz], f_registers: vec![], timers: vec![]  },
            Self::MapUnsignedToSigned { ry, .. } => RegisterSet{ registers: vec![*ry], f_registers: vec![], timers: vec![]  },
            Self::MapSignedToUnsigned { ry, .. } => RegisterSet{ registers: vec![*ry], f_registers: vec![], timers: vec![]  },
            Self::FloatingPointCompare { fx, fy } => RegisterSet{ registers: vec![Register::try_from(30).unwrap()], f_registers: vec![*fx, *fy], timers: vec![]  },
            Self::FloatingPointCompareSingleAgainstZero { fx } => RegisterSet{ registers: vec![Register::try_from(30).unwrap()], f_registers: vec![*fx], timers: vec![]  },
            Self::AddFloatingPoint { fy, fz, .. } => RegisterSet{ registers: vec![], f_registers: vec![*fy, *fz], timers: vec![]  },
            Self::SubtractFloatingPoint { fy, fz, .. } => RegisterSet{ registers: vec![], f_registers: vec![*fy, *fz], timers: vec![]  },
            Self::MultiplyFloatingPoint { fy, fz, .. } => RegisterSet{ registers: vec![], f_registers: vec![*fy, *fz], timers: vec![]  },
            Self::DivideFloatingPoint { fy, fz, .. } => RegisterSet{ registers: vec![], f_registers: vec![*fy, *fz], timers: vec![]  },
            Self::CastToFloat { ry, .. } => RegisterSet{ registers: vec![*ry], f_registers: vec![], timers: vec![]  },
            Self::CastFromFloat { fy, .. } => RegisterSet{ registers: vec![], f_registers: vec![*fy], timers: vec![]  },
            Self::NegateFloatingPoint { fy, .. } => RegisterSet{ registers: vec![], f_registers: vec![*fy], timers: vec![]  },
            Self::AbsoluteValueFloatingPoint { fy, .. } => RegisterSet{ registers: vec![], f_registers: vec![*fy], timers: vec![]  },
            Self::RoundFloatingPoint { fy, .. } => RegisterSet{ registers: vec![], f_registers: vec![*fy], timers: vec![]  },
            Self::RoundToZeroFloatingPoint { fy, .. } => RegisterSet{ registers: vec![], f_registers: vec![*fy], timers: vec![]  },
            Self::RoundToInfinityFloatingPoint { fy, .. } => RegisterSet{ registers: vec![], f_registers: vec![*fy], timers: vec![]  },
            Self::SquareRootFloatingPoint { fy, .. } => RegisterSet{ registers: vec![], f_registers: vec![*fy], timers: vec![]  },
            Self::LogBase10FloatingPoint { fy, .. } => RegisterSet{ registers: vec![], f_registers: vec![*fy], timers: vec![]  },
            Self::LogNatrualFloatingPoint { fy, .. } => RegisterSet{ registers: vec![], f_registers: vec![*fy], timers: vec![]  },
            Self::ExponentialFloatingPoint { fy, .. } => RegisterSet{ registers: vec![], f_registers: vec![*fy], timers: vec![]  },
            Self::SineFloatingPoint { fy, .. } => RegisterSet{ registers: vec![], f_registers: vec![*fy], timers: vec![]  },
            Self::CosineFloatingPoint { fy, .. } => RegisterSet{ registers: vec![], f_registers: vec![*fy], timers: vec![]  },
            Self::TangentFloatingPoint { fy, .. } => RegisterSet{ registers: vec![], f_registers: vec![*fy], timers: vec![]  },
            Self::ArcsineFloatingPoint { fy, .. } => RegisterSet{ registers: vec![], f_registers: vec![*fy], timers: vec![]  },
            Self::ArccosineFloatingPoint { fy, .. } => RegisterSet{ registers: vec![], f_registers: vec![*fy], timers: vec![]  },
            Self::ArctangentFloatingPoint { fy, .. } => RegisterSet{ registers: vec![], f_registers: vec![*fy], timers: vec![]  },
            Self::SetTimer { ry, .. } => RegisterSet{ registers: vec![*ry], f_registers: vec![], timers: vec![]  },
            Self::GetCurrentTimer { ty, .. } => RegisterSet{ registers: vec![], f_registers: vec![], timers: vec![*ty]  },
            Self::CheckTimer { .. } => RegisterSet{ registers: vec![Register::try_from(30).unwrap()], f_registers: vec![], timers: vec![]  },
            Self::ClearTimer { .. } => RegisterSet{ registers: vec![], f_registers: vec![], timers: vec![]  },
            Self::StallTimer { tx } => RegisterSet{ registers: vec![], f_registers: vec![], timers: vec![*tx]  },
            Self::Invalid(_value) => Default::default(),
        }
    }
    pub fn write_registers(&self) -> RegisterSet {
        match self {
            Self::Trap => {
                let registers = vec![];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::PushIntegerRegister { .. } => {
                let registers = vec![Register::try_from(31).unwrap()];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::PushFloatingPointRegister { .. } => {
                let registers = vec![Register::try_from(31).unwrap()];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::PopIntegerRegister { rx } => {
                let registers = vec![*rx, Register::try_from(31).unwrap()];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::PopFloatingPointRegister { fx } => {
                let registers = vec![Register::try_from(31).unwrap()];
                RegisterSet{ registers, f_registers: vec![*fx], timers: vec![] }
            },
            Self::SwapRegister { rx, fy } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![*fy], timers: vec![] }
            },
            Self::StallImmediate { .. } => {
                let registers = vec![];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::StallRegister { .. } => {
                let registers = vec![];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::RegisterJump { l, .. } => {
                let mut registers = vec![Register::try_from(28).unwrap()];
                if *l {
                    registers.push(Register::LR);
                }
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::IndirectJump { l, .. } => {
                let mut registers = vec![Register::try_from(28).unwrap()];
                if *l {
                    registers.push(Register::LR);
                }
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::IndirectwithRegisterOffsetJump { l, .. } => {
                let mut registers = vec![Register::try_from(28).unwrap()];
                if *l {
                    registers.push(Register::LR);
                }
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::RelativeJump { l, .. } => {
                let mut registers = vec![Register::try_from(28).unwrap()];
                if *l {
                    registers.push(Register::LR);
                }
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::ImmediateJump { l, .. } => {
                let mut registers = vec![Register::try_from(28).unwrap()];
                if *l {
                    registers.push(Register::LR);
                }
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::ImmediateRelativeJump { l, .. } => {
                let mut registers = vec![Register::try_from(28).unwrap()];
                if *l {
                    registers.push(Register::LR);
                }
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::IntegerLoadLow { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::IntegerLoadHigh { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::SwapIntegerRegisters { rx, ry } => {
                let registers = vec![*rx, *ry];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::CopyIntegerRegister { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::LoadIntegerRegisterIndirect { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::LoadIntegerRegisterIndirectwithRegisterOffset { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::LoadIntegerRegisterIndirectProgram { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::LoadIntegerRegisterIndirectwithRegisterOffsetProgram { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::StoreIntegerRegisterIndirect { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::StoreIntegerRegisterIndirectwithRegisterOffsetIndirect { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::StoreIntegerRegisterIndirectProgram { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::StoreIntegerRegisterIndirectwithRegisterOffsetProgram { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::IntegerLoadData { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::IntegerLoadProgram { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::IntegerStoreData { .. } => {
                let registers = vec![];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::IntegerStoreProgram { .. } => {
                let registers = vec![];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::IntegerLoadEffectiveDataAddress { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::IntegerLoadEffectiveProgramAddress { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::LoadIntegerEffectiveAddressRegisterIndirect { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::LoadIntegerEffectiveAddressRegisterIndirectwithRegisterOffset { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::UnsignedZeroExtend { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::SignExtend { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::FloatingPointLoadLow { fx, .. } => {
                let registers = vec![];
                RegisterSet{ registers, f_registers: vec![*fx], timers: vec![] }
            },
            Self::FloatingPointLoadHigh { fx, .. } => {
                let registers = vec![];
                RegisterSet{ registers, f_registers: vec![*fx], timers: vec![] }
            },
            Self::SwapFloatingPointRegisters { fx, fy } => {
                let registers = vec![];
                RegisterSet{ registers, f_registers: vec![*fx, *fy], timers: vec![] }
            },
            Self::CopyFloatingPointRegister { fx, .. } => {
                let registers = vec![];
                RegisterSet{ registers, f_registers: vec![*fx], timers: vec![] }
            },
            Self::LoadFloatingPointRegisterIndirect { fx, .. } => {
                let registers = vec![];
                RegisterSet{ registers, f_registers: vec![*fx], timers: vec![] }
            },
            Self::LoadFloatingPointRegisterIndirectwithRegisterOffset { fx, .. } => {
                let registers = vec![];
                RegisterSet{ registers, f_registers: vec![*fx], timers: vec![] }
            },
            Self::StoreFloatingPointRegisterIndirect { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::StoreFloatingPointRegisterIndirectwithRegisterOffset { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::FloatingPointLoadData { fx, .. } => {
                let registers = vec![];
                RegisterSet{ registers, f_registers: vec![*fx], timers: vec![] }
            },
            Self::FloatingPointStoreData { .. } => {
                let registers = vec![];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::IntegerCompare { .. } => {
                let registers = vec![Register::try_from(30).unwrap()];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::IntegerCompareSingleAgainstZero { .. } => {
                let registers = vec![Register::try_from(30).unwrap()];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::IncrementIntegerRegister { c, rx } => {
                let mut registers = vec![*rx];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::DecrementIntegerRegister { c, rx } => {
                let mut registers = vec![*rx];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::AddUnsignedInteger { c, rx, .. } => {
                let mut registers = vec![*rx];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::SubtractUnsignedInteger { c, rx, .. } => {
                let mut registers = vec![*rx];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::MultiplyUnsignedInteger { c, rx, .. } => {
                let mut registers = vec![*rx];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::DivideUnsignedInteger { c, rx, .. } => {
                let mut registers = vec![*rx];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::ModuloUnsignedInteger { c, rx, .. } => {
                let mut registers = vec![*rx];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::AddSignedInteger { c, rx, .. } => {
                let mut registers = vec![*rx];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::SubtractSignedInteger { c, rx, .. } => {
                let mut registers = vec![*rx];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::MultiplySignedInteger { c, rx, .. } => {
                let mut registers = vec![*rx];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::DivideSignedInteger { c, rx, .. } => {
                let mut registers = vec![*rx];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::ModuloSignedInteger { c, rx, .. } => {
                let mut registers = vec![*rx];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::BitwiseAND { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::BitwiseOR { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::BitwiseNOT { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::BitwiseXOR { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::LogicalShiftLeft { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::LogicalShiftRight { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::ArithmeticShiftLeft { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::ArithmeticShiftRight { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::RotateRight { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::LogicalShiftLeftRegister { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::LogicalShiftRightRegister { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::ArithmeticShiftLeftRegister { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::ArithmeticShiftRightRegister { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::RotateRightRegister { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::MapUnsignedToSigned { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::MapSignedToUnsigned { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::FloatingPointCompare { .. } => {
                let registers = vec![Register::try_from(30).unwrap()];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::FloatingPointCompareSingleAgainstZero { .. } => {
                let registers = vec![Register::try_from(30).unwrap()];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::AddFloatingPoint { c, fx, .. } => {
                let mut registers = vec![];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![*fx], timers: vec![] }
            },
            Self::SubtractFloatingPoint { c, fx, .. } => {
                let mut registers = vec![];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![*fx], timers: vec![] }
            },
            Self::MultiplyFloatingPoint { c, fx, .. } => {
                let mut registers = vec![];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![*fx], timers: vec![] }
            },
            Self::DivideFloatingPoint { c, fx, .. } => {
                let mut registers = vec![];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![*fx], timers: vec![] }
            },
            Self::CastToFloat { c, fx, .. } => {
                let mut registers = vec![];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![*fx], timers: vec![] }
            },
            Self::CastFromFloat { c, rx, .. } => {
                let mut registers = vec![*rx];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::NegateFloatingPoint { c, fx, .. } => {
                let mut registers = vec![];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![*fx], timers: vec![] }
            },
            Self::AbsoluteValueFloatingPoint { c, fx, .. } => {
                let mut registers = vec![];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![*fx], timers: vec![] }
            },
            Self::RoundFloatingPoint { c, fx, .. } => {
                let mut registers = vec![];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![*fx], timers: vec![] }
            },
            Self::RoundToZeroFloatingPoint { c, fx, .. } => {
                let mut registers = vec![];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![*fx], timers: vec![] }
            },
            Self::RoundToInfinityFloatingPoint { c, fx, .. } => {
                let mut registers = vec![];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![*fx], timers: vec![] }
            },
            Self::SquareRootFloatingPoint { c, fx, .. } => {
                let mut registers = vec![];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![*fx], timers: vec![] }
            },
            Self::LogBase10FloatingPoint { c, fx, .. } => {
                let mut registers = vec![];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![*fx], timers: vec![] }
            },
            Self::LogNatrualFloatingPoint { c, fx, .. } => {
                let mut registers = vec![];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![*fx], timers: vec![] }
            },
            Self::ExponentialFloatingPoint { c, fx, .. } => {
                let mut registers = vec![];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![*fx], timers: vec![] }
            },
            Self::SineFloatingPoint { c, fx, .. } => {
                let mut registers = vec![];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![*fx], timers: vec![] }
            },
            Self::CosineFloatingPoint { c, fx, .. } => {
                let mut registers = vec![];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![*fx], timers: vec![] }
            },
            Self::TangentFloatingPoint { c, fx, .. } => {
                let mut registers = vec![];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![*fx], timers: vec![] }
            },
            Self::ArcsineFloatingPoint { c, fx, .. } => {
                let mut registers = vec![];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![*fx], timers: vec![] }
            },
            Self::ArccosineFloatingPoint { c, fx, .. } => {
                let mut registers = vec![];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![*fx], timers: vec![] }
            },
            Self::ArctangentFloatingPoint { c, fx, .. } => {
                let mut registers = vec![];
                if *c {
                    registers.push(Register::ST);
                }
                RegisterSet{ registers, f_registers: vec![*fx], timers: vec![] }
            },
            Self::SetTimer { tx, .. } => {
                let registers = vec![];
                RegisterSet{ registers, f_registers: vec![], timers: vec![*tx] }
            },
            Self::GetCurrentTimer { rx, .. } => {
                let registers = vec![*rx];
                RegisterSet{ registers, f_registers: vec![], timers: vec![] }
            },
            Self::CheckTimer { tx } => {
                let registers = vec![Register::try_from(30).unwrap()];
                RegisterSet{ registers, f_registers: vec![], timers: vec![*tx] }
            },
            Self::ClearTimer { tx } => {
                let registers = vec![];
                RegisterSet{ registers, f_registers: vec![], timers: vec![*tx] }
            },
            Self::StallTimer { tx } => {
                let registers = vec![];
                RegisterSet{ registers, f_registers: vec![], timers: vec![*tx] }
            },
            Self::Invalid(_value) => Default::default(),
        }
    }
    pub fn is_valid(&self) -> bool {
        match self {
            Self::Trap => true,
            Self::PushIntegerRegister { .. } => true,
            Self::PushFloatingPointRegister { .. } => true,
            Self::PopIntegerRegister { .. } => true,
            Self::PopFloatingPointRegister { .. } => true,
            Self::SwapRegister { .. } => true,
            Self::StallImmediate { value } => *value < (1 << 16),
            Self::StallRegister { .. } => true,
            Self::RegisterJump { .. } => true,
            Self::IndirectJump { i, s, .. } => *i < (1 << 5) && *s < (1 << 4),
            Self::IndirectwithRegisterOffsetJump { s, .. } => *s < (1 << 4),
            Self::RelativeJump { .. } => true,
            Self::ImmediateJump { label, .. } => *label < (1 << 16),
            Self::ImmediateRelativeJump { offset, .. } => *offset < (1 << 15) && *offset >= -(1 << 15),
            Self::IntegerLoadLow { value, .. } => *value < (1 << 16),
            Self::IntegerLoadHigh { value, .. } => *value < (1 << 16),
            Self::SwapIntegerRegisters { .. } => true,
            Self::CopyIntegerRegister { .. } => true,
            Self::LoadIntegerRegisterIndirect { i, s, .. } => *i < (1 << 5) && *s < (1 << 4),
            Self::LoadIntegerRegisterIndirectwithRegisterOffset { s, .. } => *s < (1 << 4),
            Self::LoadIntegerRegisterIndirectProgram { i, s, .. } => *i < (1 << 5) && *s < (1 << 4),
            Self::LoadIntegerRegisterIndirectwithRegisterOffsetProgram { s, .. } => *s < (1 << 4),
            Self::StoreIntegerRegisterIndirect { i, s, .. } => *i < (1 << 5) && *s < (1 << 4),
            Self::StoreIntegerRegisterIndirectwithRegisterOffsetIndirect { s, .. } => *s < (1 << 4),
            Self::StoreIntegerRegisterIndirectProgram { i, s, .. } => *i < (1 << 5) && *s < (1 << 4),
            Self::StoreIntegerRegisterIndirectwithRegisterOffsetProgram { s, .. } => *s < (1 << 4),
            Self::IntegerLoadData { label, .. } => *label < (1 << 16),
            Self::IntegerLoadProgram { label, .. } => *label < (1 << 16),
            Self::IntegerStoreData { label, .. } => *label < (1 << 16),
            Self::IntegerStoreProgram { label, .. } => *label < (1 << 16),
            Self::IntegerLoadEffectiveDataAddress { label, .. } => *label < (1 << 16),
            Self::IntegerLoadEffectiveProgramAddress { label, .. } => *label < (1 << 16),
            Self::LoadIntegerEffectiveAddressRegisterIndirect { i, s, .. } => *i < (1 << 5) && *s < (1 << 4),
            Self::LoadIntegerEffectiveAddressRegisterIndirectwithRegisterOffset { s, .. } => *s < (1 << 4),
            Self::UnsignedZeroExtend { count, .. } => *count < (1 << 5),
            Self::SignExtend { count, .. } => *count < (1 << 5),
            Self::FloatingPointLoadLow { value, .. } => *value < (1 << 16),
            Self::FloatingPointLoadHigh { value, .. } => *value < (1 << 16),
            Self::SwapFloatingPointRegisters { .. } => true,
            Self::CopyFloatingPointRegister { .. } => true,
            Self::LoadFloatingPointRegisterIndirect { i, s, .. } => *i < (1 << 5) && *s < (1 << 4),
            Self::LoadFloatingPointRegisterIndirectwithRegisterOffset { s, .. } => *s < (1 << 4),
            Self::StoreFloatingPointRegisterIndirect { i, s, .. } => *i < (1 << 5) && *s < (1 << 4),
            Self::StoreFloatingPointRegisterIndirectwithRegisterOffset { s, .. } => *s < (1 << 4),
            Self::FloatingPointLoadData { label, .. } => *label < (1 << 16),
            Self::FloatingPointStoreData { label, .. } => *label < (1 << 16),
            Self::IntegerCompare { .. } => true,
            Self::IntegerCompareSingleAgainstZero { .. } => true,
            Self::IncrementIntegerRegister { .. } => true,
            Self::DecrementIntegerRegister { .. } => true,
            Self::AddUnsignedInteger { .. } => true,
            Self::SubtractUnsignedInteger { .. } => true,
            Self::MultiplyUnsignedInteger { .. } => true,
            Self::DivideUnsignedInteger { .. } => true,
            Self::ModuloUnsignedInteger { .. } => true,
            Self::AddSignedInteger { .. } => true,
            Self::SubtractSignedInteger { .. } => true,
            Self::MultiplySignedInteger { .. } => true,
            Self::DivideSignedInteger { .. } => true,
            Self::ModuloSignedInteger { .. } => true,
            Self::BitwiseAND { .. } => true,
            Self::BitwiseOR { .. } => true,
            Self::BitwiseNOT { .. } => true,
            Self::BitwiseXOR { .. } => true,
            Self::LogicalShiftLeft { value, .. } => *value < (1 << 5),
            Self::LogicalShiftRight { value, .. } => *value < (1 << 5),
            Self::ArithmeticShiftLeft { value, .. } => *value < (1 << 5),
            Self::ArithmeticShiftRight { value, .. } => *value < (1 << 5),
            Self::RotateRight { value, .. } => *value < (1 << 5),
            Self::LogicalShiftLeftRegister { .. } => true,
            Self::LogicalShiftRightRegister { .. } => true,
            Self::ArithmeticShiftLeftRegister { .. } => true,
            Self::ArithmeticShiftRightRegister { .. } => true,
            Self::RotateRightRegister { .. } => true,
            Self::MapUnsignedToSigned { .. } => true,
            Self::MapSignedToUnsigned { .. } => true,
            Self::FloatingPointCompare { .. } => true,
            Self::FloatingPointCompareSingleAgainstZero { .. } => true,
            Self::AddFloatingPoint { .. } => true,
            Self::SubtractFloatingPoint { .. } => true,
            Self::MultiplyFloatingPoint { .. } => true,
            Self::DivideFloatingPoint { .. } => true,
            Self::CastToFloat { .. } => true,
            Self::CastFromFloat { .. } => true,
            Self::NegateFloatingPoint { .. } => true,
            Self::AbsoluteValueFloatingPoint { .. } => true,
            Self::RoundFloatingPoint { .. } => true,
            Self::RoundToZeroFloatingPoint { .. } => true,
            Self::RoundToInfinityFloatingPoint { .. } => true,
            Self::SquareRootFloatingPoint { .. } => true,
            Self::LogBase10FloatingPoint { .. } => true,
            Self::LogNatrualFloatingPoint { .. } => true,
            Self::ExponentialFloatingPoint { .. } => true,
            Self::SineFloatingPoint { .. } => true,
            Self::CosineFloatingPoint { .. } => true,
            Self::TangentFloatingPoint { .. } => true,
            Self::ArcsineFloatingPoint { .. } => true,
            Self::ArccosineFloatingPoint { .. } => true,
            Self::ArctangentFloatingPoint { .. } => true,
            Self::SetTimer { .. } => true,
            Self::GetCurrentTimer { .. } => true,
            Self::CheckTimer { .. } => true,
            Self::ClearTimer { .. } => true,
            Self::StallTimer { .. } => true,
            Self::Invalid(_value) => true,
        }
    }
}

