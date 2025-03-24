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
pub enum FPRegister {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(usize)]
pub enum Timer {
    T0 = 0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    T10,
    T11,
    T12,
    T13,
    T14,
    T15,
    T16,
    T17,
    T18,
    T19,
    T20,
    T21,
    T22,
    T23,
    T24,
    T25,
    T26,
    T27,
    T28,
    T29,
    T30,
    T31,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(usize)]
pub enum Condition {
    // C0
    AlwaysTrue = 0,
    // C1
    NeverTrue,
    // C2
    Equal,
    // C3
    GreaterThan,
    // C4
    LessThan,
    // C5
    GreaterEqual,
    // C6
    LessEqual,
    // C7
    Overflow,
    // C8
    Underflow,
    // C9
    DivideByZero,
    // C10
    IsEven,
    // C11
    FloatingPointInfinity,
    // C12
    FloatingPointZero,
    // C13
    FloatingPointNotANumber,
    C14,
    C15,
    C16,
    C17,
    C18,
    C19,
    C20,
    C21,
    C22,
    C23,
    C24,
    C25,
    C26,
    C27,
    C28,
    C29,
    C30,
    C31,
}