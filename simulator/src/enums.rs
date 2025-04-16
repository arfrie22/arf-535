use std::str::FromStr;

use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    InvalidInput,
}


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
    // ADC Channel 1 (R20)
    A1,
    // ADC Channel 2 (R21)
    A2,
    // ADC Channel 3 (R22)
    A3,
    // ADC Channel 4 (R23)
    A4,
    // DAC Channel 1 (R24)
    D1,
    // DAC Channel 2 (R25)
    D2,
    // DAC Channel 3 (R26)
    D3,
    // DAC Channel 4 (R27)
    D4,
    // Program Counter (R28)
    PC,
    // Link Register (R29)
    LR,
    // Status Register (R30)
    ST,
    // Stack Pointer (R31)
    SP,
}

impl FromStr for Register {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "r0" => Ok(Self::R0),
            "r1" => Ok(Self::R1),
            "r2" => Ok(Self::R2),
            "r3" => Ok(Self::R3),
            "r4" => Ok(Self::R4),
            "r5" => Ok(Self::R5),
            "r6" => Ok(Self::R6),
            "r7" => Ok(Self::R7),
            "r8" => Ok(Self::R8),
            "r9" => Ok(Self::R9),
            "r10" => Ok(Self::R10),
            "r11" => Ok(Self::R11),
            "r12" => Ok(Self::R12),
            "r13" => Ok(Self::R13),
            "r14" => Ok(Self::R14),
            "r15" => Ok(Self::R15),
            "r16" => Ok(Self::R16),
            "r17" => Ok(Self::R17),
            "r18" => Ok(Self::R18),
            "r19" => Ok(Self::R19),
            "r20" | "a1" => Ok(Self::A1),
            "r21" | "a2" => Ok(Self::A2),
            "r22" | "a3" => Ok(Self::A3),
            "r23" | "a4" => Ok(Self::A4),
            "r24" | "d1" => Ok(Self::D1),
            "r25" | "d2" => Ok(Self::D2),
            "r26" | "d3" => Ok(Self::D3),
            "r27" | "d4" => Ok(Self::D4),
            "r28" | "pc" => Ok(Self::PC),
            "r29" | "lr" => Ok(Self::LR),
            "r30" | "st" => Ok(Self::ST),
            "r31" | "sp" => Ok(Self::SP),
            _ => Err(ParseError::InvalidInput)
        }
    }
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

impl FromStr for FPRegister {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "f0" => Ok(Self::F0),
            "f1" => Ok(Self::F1),
            "f2" => Ok(Self::F2),
            "f3" => Ok(Self::F3),
            "f4" => Ok(Self::F4),
            "f5" => Ok(Self::F5),
            "f6" => Ok(Self::F6),
            "f7" => Ok(Self::F7),
            "f8" => Ok(Self::F8),
            "f9" => Ok(Self::F9),
            "f10" => Ok(Self::F10),
            "f11" => Ok(Self::F11),
            "f12" => Ok(Self::F12),
            "f13" => Ok(Self::F13),
            "f14" => Ok(Self::F14),
            "f15" => Ok(Self::F15),
            "f16" => Ok(Self::F16),
            "f17" => Ok(Self::F17),
            "f18" => Ok(Self::F18),
            "f19" => Ok(Self::F19),
            "f20" => Ok(Self::F20),
            "f21" => Ok(Self::F21),
            "f22" => Ok(Self::F22),
            "f23" => Ok(Self::F23),
            "f24" => Ok(Self::F24),
            "f25" => Ok(Self::F25),
            "f26" => Ok(Self::F26),
            "f27" => Ok(Self::F27),
            "f28" => Ok(Self::F28),
            "f29" => Ok(Self::F29),
            "f30" => Ok(Self::F30),
            "f31" => Ok(Self::F31),
            _ => Err(ParseError::InvalidInput)
        }
    }
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

impl FromStr for Timer {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "t0" => Ok(Self::T0),
            "t1" => Ok(Self::T1),
            "t2" => Ok(Self::T2),
            "t3" => Ok(Self::T3),
            "t4" => Ok(Self::T4),
            "t5" => Ok(Self::T5),
            "t6" => Ok(Self::T6),
            "t7" => Ok(Self::T7),
            "t8" => Ok(Self::T8),
            "t9" => Ok(Self::T9),
            "t10" => Ok(Self::T10),
            "t11" => Ok(Self::T11),
            "t12" => Ok(Self::T12),
            "t13" => Ok(Self::T13),
            "t14" => Ok(Self::T14),
            "t15" => Ok(Self::T15),
            "t16" => Ok(Self::T16),
            "t17" => Ok(Self::T17),
            "t18" => Ok(Self::T18),
            "t19" => Ok(Self::T19),
            "t20" => Ok(Self::T20),
            "t21" => Ok(Self::T21),
            "t22" => Ok(Self::T22),
            "t23" => Ok(Self::T23),
            "t24" => Ok(Self::T24),
            "t25" => Ok(Self::T25),
            "t26" => Ok(Self::T26),
            "t27" => Ok(Self::T27),
            "t28" => Ok(Self::T28),
            "t29" => Ok(Self::T29),
            "t30" => Ok(Self::T30),
            "t31" => Ok(Self::T31),
            _ => Err(ParseError::InvalidInput)
        }
    }
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
    // C14
    FloatingPointPositive,
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

impl FromStr for Condition {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "" => Ok(Self::AlwaysTrue),
            "nvr" => Ok(Self::NeverTrue),
            "eq" => Ok(Self::Equal),
            "gt" => Ok(Self::GreaterThan),
            "lt" => Ok(Self::LessThan),
            "ge" => Ok(Self::GreaterEqual),
            "le" => Ok(Self::LessEqual),
            "ovrf" => Ok(Self::Overflow),
            "undf" => Ok(Self::Underflow),
            "divz" => Ok(Self::DivideByZero),
            "even" => Ok(Self::IsEven),
            "finf" => Ok(Self::FloatingPointInfinity),
            "fz" => Ok(Self::FloatingPointZero),
            "fnan" => Ok(Self::FloatingPointNotANumber),
            "fpos" => Ok(Self::FloatingPointPositive),
            _ => Err(ParseError::InvalidInput)
        }
    }
}

impl Condition {
    pub fn check(&self, st: u32) -> bool {
        match self {
            Condition::AlwaysTrue => true,
            Condition::NeverTrue => false,
            _ => st & (1 << (*self as u32)) == 1,
        }
    }

    pub fn set(&self, st: u32, value: bool) -> u32 {
        if value {
            st | (1 << (*self as u32))
        } else {
            st & (!(1 << (*self as u32)))
        }
    }
}