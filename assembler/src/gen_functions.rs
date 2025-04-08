use std::collections::HashMap;

use pest::Parser;
use simulator::instruction::Instruction;

use crate::{parse_label, parse_number, parse_signed_number, AssemblerError, AssemblerParser, Rule};

pub fn assemble(input: &str) -> Result<Vec<Instruction>, AssemblerError> {
    let parsed = AssemblerParser::parse(Rule::file, input)?;

    let mut instructions = Vec::new();
    let mut labels = HashMap::new();
    let mut first_pass_index = 0;
    for p in parsed {
        match p.as_rule() {
            Rule::EOI => continue,
            Rule::label_arg => {
                labels.insert(p.as_str().to_owned(), first_pass_index);
                first_pass_index += 1;
            },
            Rule::instruction_TRAP_00 | Rule::instruction_PUSH_01 | Rule::instruction_PUSH_02 | Rule::instruction_POP_03 | Rule::instruction_POP_04 | Rule::instruction_SWP_05 | Rule::instruction_STALL_06 | Rule::instruction_B_20 | Rule::instruction_B_21 | Rule::instruction_B_22 | Rule::instruction_BR_23 | Rule::instruction_B_24 | Rule::instruction_BO_25 | Rule::instruction_LDL_40 | Rule::instruction_LDH_41 | Rule::instruction_SWP_42 | Rule::instruction_LDR_43 | Rule::instruction_LDR_44 | Rule::instruction_LDR_45 | Rule::instruction_LDR_46 | Rule::instruction_LDR_47 | Rule::instruction_STR_48 | Rule::instruction_STR_49 | Rule::instruction_STR_4a | Rule::instruction_STR_4b | Rule::instruction_LDR_4c | Rule::instruction_LDR_4d | Rule::instruction_STR_4e | Rule::instruction_STR_4f | Rule::instruction_ZEX_50 | Rule::instruction_SEX_51 | Rule::instruction_LDL_60 | Rule::instruction_LDH_61 | Rule::instruction_SWP_62 | Rule::instruction_LDR_63 | Rule::instruction_LDR_64 | Rule::instruction_LDR_65 | Rule::instruction_STR_66 | Rule::instruction_STR_67 | Rule::instruction_LDR_68 | Rule::instruction_STR_69 | Rule::instruction_CMP_80 | Rule::instruction_CMP_81 | Rule::instruction_ADD_82 | Rule::instruction_SUB_83 | Rule::instruction_MUL_84 | Rule::instruction_DIV_85 | Rule::instruction_MOD_86 | Rule::instruction_ADDS_87 | Rule::instruction_SUBS_88 | Rule::instruction_MULS_89 | Rule::instruction_DIVS_8a | Rule::instruction_MODS_8b | Rule::instruction_AND_8c | Rule::instruction_OR_8d | Rule::instruction_NOT_8e | Rule::instruction_XOR_8f | Rule::instruction_LSL_90 | Rule::instruction_LSR_91 | Rule::instruction_ASL_92 | Rule::instruction_ASR_93 | Rule::instruction_RTR_94 | Rule::instruction_LSL_95 | Rule::instruction_LSR_96 | Rule::instruction_ASL_97 | Rule::instruction_ASR_98 | Rule::instruction_RTR_99 | Rule::instruction_MUS_9a | Rule::instruction_MSU_9b | Rule::instruction_CMP_a0 | Rule::instruction_CMP_a1 | Rule::instruction_ADD_a2 | Rule::instruction_SUB_a3 | Rule::instruction_MUL_a4 | Rule::instruction_DIV_a5 | Rule::instruction_CST_a6 | Rule::instruction_CST_a7 | Rule::instruction_SETT_c0 | Rule::instruction_GETT_c1 | Rule::instruction_CHKT_c2 | Rule::instruction_CLRT_c3 => {
                first_pass_index += 1;
            },
            _ => unreachable!(),
        }
    }

    let parsed = AssemblerParser::parse(Rule::file, input)?;
    for p in parsed {
        match p.as_rule() {
            Rule::EOI | Rule::label_arg => continue,
            Rule::instruction_TRAP_00 => {
                instructions.push(Instruction::Trap);
            }
            Rule::instruction_PUSH_01 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::PushIntegerRegister { rx });
            }
            Rule::instruction_PUSH_02 => {
                let mut iter = p.into_inner();
                let fx = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::PushFloatingPointRegister { fx });
            }
            Rule::instruction_POP_03 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::PopIntegerRegister { rx });
            }
            Rule::instruction_POP_04 => {
                let mut iter = p.into_inner();
                let fx = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::PopFloatingPointRegister { fx });
            }
            Rule::instruction_SWP_05 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let fy = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::SwapRegister { rx, fy });
            }
            Rule::instruction_STALL_06 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::Stall { rx });
            }
            Rule::instruction_B_20 => {
                let mut iter = p.into_inner();
                let l = iter.next().unwrap().as_str().len() > 0;
                let condition = iter.next().unwrap().as_str().parse()?;
                let rx = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::RegisterJump { l, condition, rx });
            }
            Rule::instruction_B_21 => {
                let mut iter = p.into_inner();
                let l = iter.next().unwrap().as_str().len() > 0;
                let condition = iter.next().unwrap().as_str().parse()?;
                let rx = iter.next().unwrap().as_str().parse()?;
                let i = parse_number(iter.next().unwrap().as_str())?;
                let s = parse_number(iter.next().unwrap().as_str())?;
                instructions.push(Instruction::IndirectJump { l, condition, rx, i, s });
            }
            Rule::instruction_B_22 => {
                let mut iter = p.into_inner();
                let l = iter.next().unwrap().as_str().len() > 0;
                let condition = iter.next().unwrap().as_str().parse()?;
                let rx = iter.next().unwrap().as_str().parse()?;
                let ro = iter.next().unwrap().as_str().parse()?;
                let s = parse_number(iter.next().unwrap().as_str())?;
                instructions.push(Instruction::IndirectwithRegisterOffsetJump { l, condition, rx, ro, s });
            }
            Rule::instruction_BR_23 => {
                let mut iter = p.into_inner();
                let l = iter.next().unwrap().as_str().len() > 0;
                let condition = iter.next().unwrap().as_str().parse()?;
                let rx = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::RelativeJump { l, condition, rx });
            }
            Rule::instruction_B_24 => {
                let mut iter = p.into_inner();
                let l = iter.next().unwrap().as_str().len() > 0;
                let condition = iter.next().unwrap().as_str().parse()?;
                let label = parse_label(iter.next().unwrap().as_str(), &labels)?;
                instructions.push(Instruction::ImmediateJump { l, condition, label });
            }
            Rule::instruction_BO_25 => {
                let mut iter = p.into_inner();
                let l = iter.next().unwrap().as_str().len() > 0;
                let condition = iter.next().unwrap().as_str().parse()?;
                let offset = parse_signed_number(iter.next().unwrap().as_str())?;
                instructions.push(Instruction::ImmediateRelativeJump { l, condition, offset });
            }
            Rule::instruction_LDL_40 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let value = parse_number(iter.next().unwrap().as_str())?;
                instructions.push(Instruction::IntegerLoadLow { rx, value });
            }
            Rule::instruction_LDH_41 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let value = parse_number(iter.next().unwrap().as_str())?;
                instructions.push(Instruction::IntegerLoadHigh { rx, value });
            }
            Rule::instruction_SWP_42 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::SwapIntegerRegisters { rx, ry });
            }
            Rule::instruction_LDR_43 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::CopyIntegerRegister { rx, ry });
            }
            Rule::instruction_LDR_44 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                let i = parse_number(iter.next().unwrap().as_str())?;
                let s = parse_number(iter.next().unwrap().as_str())?;
                instructions.push(Instruction::LoadIntegerRegisterIndirect { rx, ry, i, s });
            }
            Rule::instruction_LDR_45 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                let ro = iter.next().unwrap().as_str().parse()?;
                let s = parse_number(iter.next().unwrap().as_str())?;
                instructions.push(Instruction::LoadIntegerRegisterIndirectwithRegisterOffset { rx, ry, ro, s });
            }
            Rule::instruction_LDR_46 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                let i = parse_number(iter.next().unwrap().as_str())?;
                let s = parse_number(iter.next().unwrap().as_str())?;
                instructions.push(Instruction::LoadIntegerRegisterIndirectProgram { rx, ry, i, s });
            }
            Rule::instruction_LDR_47 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                let ro = iter.next().unwrap().as_str().parse()?;
                let s = parse_number(iter.next().unwrap().as_str())?;
                instructions.push(Instruction::LoadIntegerRegisterIndirectwithRegisterOffsetProgram { rx, ry, ro, s });
            }
            Rule::instruction_STR_48 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let i = parse_number(iter.next().unwrap().as_str())?;
                let s = parse_number(iter.next().unwrap().as_str())?;
                let ry = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::StoreIntegerRegisterIndirect { rx, i, s, ry });
            }
            Rule::instruction_STR_49 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ro = iter.next().unwrap().as_str().parse()?;
                let s = parse_number(iter.next().unwrap().as_str())?;
                let ry = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::StoreIntegerRegisterIndirectwithRegisterOffsetIndirect { rx, ro, s, ry });
            }
            Rule::instruction_STR_4a => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let i = parse_number(iter.next().unwrap().as_str())?;
                let s = parse_number(iter.next().unwrap().as_str())?;
                let ry = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::StoreIntegerRegisterIndirectProgram { rx, i, s, ry });
            }
            Rule::instruction_STR_4b => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ro = iter.next().unwrap().as_str().parse()?;
                let s = parse_number(iter.next().unwrap().as_str())?;
                let ry = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::StoreIntegerRegisterIndirectwithRegisterOffsetProgram { rx, ro, s, ry });
            }
            Rule::instruction_LDR_4c => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let label = parse_label(iter.next().unwrap().as_str(), &labels)?;
                instructions.push(Instruction::IntegerLoadData { rx, label });
            }
            Rule::instruction_LDR_4d => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let label = parse_label(iter.next().unwrap().as_str(), &labels)?;
                instructions.push(Instruction::IntegerLoadProgram { rx, label });
            }
            Rule::instruction_STR_4e => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let label = parse_label(iter.next().unwrap().as_str(), &labels)?;
                instructions.push(Instruction::IntegerStoreData { rx, label });
            }
            Rule::instruction_STR_4f => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let label = parse_label(iter.next().unwrap().as_str(), &labels)?;
                instructions.push(Instruction::IntegerStoreProgram { rx, label });
            }
            Rule::instruction_ZEX_50 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                let count = parse_number(iter.next().unwrap().as_str())?;
                instructions.push(Instruction::UnsignedZeroExtend { rx, ry, count });
            }
            Rule::instruction_SEX_51 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                let count = parse_number(iter.next().unwrap().as_str())?;
                instructions.push(Instruction::SignExtend { rx, ry, count });
            }
            Rule::instruction_LDL_60 => {
                let mut iter = p.into_inner();
                let fx = iter.next().unwrap().as_str().parse()?;
                let value = parse_number(iter.next().unwrap().as_str())?;
                instructions.push(Instruction::FloatingPointLoadLow { fx, value });
            }
            Rule::instruction_LDH_61 => {
                let mut iter = p.into_inner();
                let fx = iter.next().unwrap().as_str().parse()?;
                let value = parse_number(iter.next().unwrap().as_str())?;
                instructions.push(Instruction::FloatingPointLoadHigh { fx, value });
            }
            Rule::instruction_SWP_62 => {
                let mut iter = p.into_inner();
                let fx = iter.next().unwrap().as_str().parse()?;
                let fy = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::SwapFloatingPointRegisters { fx, fy });
            }
            Rule::instruction_LDR_63 => {
                let mut iter = p.into_inner();
                let fx = iter.next().unwrap().as_str().parse()?;
                let fy = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::CopyFloatingPointRegister { fx, fy });
            }
            Rule::instruction_LDR_64 => {
                let mut iter = p.into_inner();
                let fx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                let i = parse_number(iter.next().unwrap().as_str())?;
                let s = parse_number(iter.next().unwrap().as_str())?;
                instructions.push(Instruction::LoadFloatingPointRegisterIndirect { fx, ry, i, s });
            }
            Rule::instruction_LDR_65 => {
                let mut iter = p.into_inner();
                let fx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                let ro = iter.next().unwrap().as_str().parse()?;
                let s = parse_number(iter.next().unwrap().as_str())?;
                instructions.push(Instruction::LoadFloatingPointRegisterIndirectwithRegisterOffset { fx, ry, ro, s });
            }
            Rule::instruction_STR_66 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let i = parse_number(iter.next().unwrap().as_str())?;
                let s = parse_number(iter.next().unwrap().as_str())?;
                let fy = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::StoreFloatingPointRegisterIndirect { rx, i, s, fy });
            }
            Rule::instruction_STR_67 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ro = iter.next().unwrap().as_str().parse()?;
                let s = parse_number(iter.next().unwrap().as_str())?;
                let fy = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::StoreFloatingPointRegisterIndirectwithRegisterOffset { rx, ro, s, fy });
            }
            Rule::instruction_LDR_68 => {
                let mut iter = p.into_inner();
                let fx = iter.next().unwrap().as_str().parse()?;
                let label = parse_label(iter.next().unwrap().as_str(), &labels)?;
                instructions.push(Instruction::FloatingPointLoadData { fx, label });
            }
            Rule::instruction_STR_69 => {
                let mut iter = p.into_inner();
                let fx = iter.next().unwrap().as_str().parse()?;
                let label = parse_label(iter.next().unwrap().as_str(), &labels)?;
                instructions.push(Instruction::FloatingPointStoreData { fx, label });
            }
            Rule::instruction_CMP_80 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::IntegerCompare { rx, ry });
            }
            Rule::instruction_CMP_81 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::IntegerCompareSingleAgainstZero { rx });
            }
            Rule::instruction_ADD_82 => {
                let mut iter = p.into_inner();
                let c = iter.next().unwrap().as_str().len() > 0;
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                let rz = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::AddUnsignedInteger { c, rx, ry, rz });
            }
            Rule::instruction_SUB_83 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                let rz = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::SubtractUnsignedInteger { rx, ry, rz });
            }
            Rule::instruction_MUL_84 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                let rz = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::MultiplyUnsignedInteger { rx, ry, rz });
            }
            Rule::instruction_DIV_85 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                let rz = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::DivideUnsignedInteger { rx, ry, rz });
            }
            Rule::instruction_MOD_86 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                let rz = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::ModuloUnsignedInteger { rx, ry, rz });
            }
            Rule::instruction_ADDS_87 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                let rz = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::AddSignedInteger { rx, ry, rz });
            }
            Rule::instruction_SUBS_88 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                let rz = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::SubtractSignedInteger { rx, ry, rz });
            }
            Rule::instruction_MULS_89 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                let rz = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::MultiplySignedInteger { rx, ry, rz });
            }
            Rule::instruction_DIVS_8a => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                let rz = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::DivideSignedInteger { rx, ry, rz });
            }
            Rule::instruction_MODS_8b => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                let rz = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::ModuloSignedInteger { rx, ry, rz });
            }
            Rule::instruction_AND_8c => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                let rz = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::BitwiseAND { rx, ry, rz });
            }
            Rule::instruction_OR_8d => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                let rz = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::BitwiseOR { rx, ry, rz });
            }
            Rule::instruction_NOT_8e => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::BitwiseNOT { rx, ry });
            }
            Rule::instruction_XOR_8f => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                let rz = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::BitwiseXOR { rx, ry, rz });
            }
            Rule::instruction_LSL_90 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                let value = parse_number(iter.next().unwrap().as_str())?;
                instructions.push(Instruction::LogicalShiftLeft { rx, ry, value });
            }
            Rule::instruction_LSR_91 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                let value = parse_number(iter.next().unwrap().as_str())?;
                instructions.push(Instruction::LogicalShiftRight { rx, ry, value });
            }
            Rule::instruction_ASL_92 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                let value = parse_number(iter.next().unwrap().as_str())?;
                instructions.push(Instruction::ArithmeticShiftLeft { rx, ry, value });
            }
            Rule::instruction_ASR_93 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                let value = parse_number(iter.next().unwrap().as_str())?;
                instructions.push(Instruction::ArithmeticShiftRight { rx, ry, value });
            }
            Rule::instruction_RTR_94 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                let value = parse_number(iter.next().unwrap().as_str())?;
                instructions.push(Instruction::RotateRight { rx, ry, value });
            }
            Rule::instruction_LSL_95 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                let rz = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::LogicalShiftLeftRegister { rx, ry, rz });
            }
            Rule::instruction_LSR_96 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                let rz = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::LogicalShiftRightRegister { rx, ry, rz });
            }
            Rule::instruction_ASL_97 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                let rz = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::ArithmeticShiftLeftRegister { rx, ry, rz });
            }
            Rule::instruction_ASR_98 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                let rz = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::ArithmeticShiftRightRegister { rx, ry, rz });
            }
            Rule::instruction_RTR_99 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                let rz = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::RotateRightRegister { rx, ry, rz });
            }
            Rule::instruction_MUS_9a => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::MapUnsignedToSigned { rx, ry });
            }
            Rule::instruction_MSU_9b => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::MapSignedToUnsigned { rx, ry });
            }
            Rule::instruction_CMP_a0 => {
                let mut iter = p.into_inner();
                let fx = iter.next().unwrap().as_str().parse()?;
                let fy = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::FloatingPointCompare { fx, fy });
            }
            Rule::instruction_CMP_a1 => {
                let mut iter = p.into_inner();
                let fx = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::FloatingPointCompareSingleAgainstZero { fx });
            }
            Rule::instruction_ADD_a2 => {
                let mut iter = p.into_inner();
                let fx = iter.next().unwrap().as_str().parse()?;
                let fy = iter.next().unwrap().as_str().parse()?;
                let fz = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::AddFloatingPoint { fx, fy, fz });
            }
            Rule::instruction_SUB_a3 => {
                let mut iter = p.into_inner();
                let fx = iter.next().unwrap().as_str().parse()?;
                let fy = iter.next().unwrap().as_str().parse()?;
                let fz = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::SubtractFloatingPoint { fx, fy, fz });
            }
            Rule::instruction_MUL_a4 => {
                let mut iter = p.into_inner();
                let fx = iter.next().unwrap().as_str().parse()?;
                let fy = iter.next().unwrap().as_str().parse()?;
                let fz = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::MultiplyFloatingPoint { fx, fy, fz });
            }
            Rule::instruction_DIV_a5 => {
                let mut iter = p.into_inner();
                let fx = iter.next().unwrap().as_str().parse()?;
                let fy = iter.next().unwrap().as_str().parse()?;
                let fz = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::DivideFloatingPoint { fx, fy, fz });
            }
            Rule::instruction_CST_a6 => {
                let mut iter = p.into_inner();
                let fx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::CasttoFloat { fx, ry });
            }
            Rule::instruction_CST_a7 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let fy = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::CastfromFloat { rx, fy });
            }
            Rule::instruction_SETT_c0 => {
                let mut iter = p.into_inner();
                let tx = iter.next().unwrap().as_str().parse()?;
                let ry = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::SetTimer { tx, ry });
            }
            Rule::instruction_GETT_c1 => {
                let mut iter = p.into_inner();
                let rx = iter.next().unwrap().as_str().parse()?;
                let ty = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::GetCurrentTimer { rx, ty });
            }
            Rule::instruction_CHKT_c2 => {
                let mut iter = p.into_inner();
                let tx = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::CheckTimer { tx });
            }
            Rule::instruction_CLRT_c3 => {
                let mut iter = p.into_inner();
                let tx = iter.next().unwrap().as_str().parse()?;
                instructions.push(Instruction::ClearTimer { tx });
            }
            _ => unreachable!(),
        }
    }
    Ok(instructions)
}
