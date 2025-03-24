use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(usize)]
pub enum Register {
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    R16,
    R17,
    R18,
    R19,
    // (R20)
    A1,
    // (R21)
    A2,
    // (R22)
    A3,
    // (R23)
    A4,
    // (R24)
    D1,
    // (R25)
    D2,
    // (R26)
    D3,
    // (R27)
    D4,
    // (R28)
    PC,
    // (R29)
    LR,
    // (R30)
    ST,
    // (R31)
    SP,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(usize)]
pub enum FloatingPointRegister {
    F0 = 0,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
}


// 0. Always True () [Value of bit does not affect branch instruction] _This is a normal branch_
// + Never True (NVR) [Value of bit does not affect branch instruction]  _This is a NOP_
// + Equal (EQ)
// + Greater Than (GT)
// + Less Than (LT)
// + Greater Than or Equal To (GE)
// + Less Than or Equal To (LE)
// + Overflow (OVRF)
// + Underflow (UNDF)
// + Divide by zero (DIVZ)
// + Parity / Is Even (EVEN)
// + Floating Point Infinity (FINF)
// + Floating Point Zero (FZ)
// + Floating Point Not-A-Number (FNAN)