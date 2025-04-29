use std::collections::HashMap;

use pest::Parser;
use simulator::{instruction::Instruction, raw_cast_from_i32};

use crate::{parse_prog_label, parse_data_label, parse_number, parse_signed_number, AssembledData, AssemblerError, AssemblerParser, Rule};

pub fn assemble(input: &str) -> Result<AssembledData, AssemblerError> {
    let parsed = AssemblerParser::parse(Rule::file, input)?;

    let mut data = AssembledData::default();
    let mut prog_labels = HashMap::new();
    let mut data_labels = HashMap::new();
    let mut first_pass_index;
    for p in parsed {
        let labels;
        let beginning;
        match p.as_rule() {
            Rule::EOI => continue,
            Rule::prog => {
                labels = &mut prog_labels;
                beginning = "p:".to_owned();
                first_pass_index = 0;
            },
            Rule::data => {
                labels = &mut data_labels;
                beginning = "d:".to_owned();
                first_pass_index = 0;
            },
            _ => unreachable!(),
        }

        for t in p.into_inner() {
            match t.as_rule() {
                Rule::EOI => continue,
                Rule::label_arg => {
                    let label = beginning.clone() + t.as_str();
                    labels.insert(label, first_pass_index);
                },
                Rule::data_line => {
                    for t in t.into_inner() {
                        match t.as_rule() {
                            Rule::label_arg => {
                                let label = beginning.clone() + t.as_str();
                                labels.insert(label, first_pass_index);
                            },
                            Rule::data_value => {
                                let mut iter = t.into_inner();
                                let val = iter.next().unwrap();
                                let val = match val.as_rule() {
                                    Rule::number => parse_number(val.as_str())?,
                                    Rule::signed_number => raw_cast_from_i32(parse_signed_number(val.as_str())?),
                                    _ => unreachable!(),
                                };

                                let count = parse_number(iter.next().unwrap().as_str())?;


                                data.data[(first_pass_index as usize)..(first_pass_index + count) as usize].fill(val);

                                first_pass_index += count;
                            },
                            _ => unreachable!(),
                        }
                    }
                },
                Rule::instruction_TRAP_00 | Rule::instruction_PUSH_01 | Rule::instruction_PUSH_02 | Rule::instruction_POP_03 | Rule::instruction_POP_04 | Rule::instruction_SWP_05 | Rule::instruction_STALL_06 | Rule::instruction_STALL_07 | Rule::instruction_B_20 | Rule::instruction_B_21 | Rule::instruction_B_22 | Rule::instruction_BR_23 | Rule::instruction_B_24 | Rule::instruction_BO_25 | Rule::instruction_LDL_40 | Rule::instruction_LDH_41 | Rule::instruction_SWP_42 | Rule::instruction_LDR_43 | Rule::instruction_LDR_44 | Rule::instruction_LDR_45 | Rule::instruction_LDR_46 | Rule::instruction_LDR_47 | Rule::instruction_STR_48 | Rule::instruction_STR_49 | Rule::instruction_STR_4a | Rule::instruction_STR_4b | Rule::instruction_LDR_4c | Rule::instruction_LDR_4d | Rule::instruction_STR_4e | Rule::instruction_STR_4f | Rule::instruction_LEA_50 | Rule::instruction_LEA_51 | Rule::instruction_LEA_52 | Rule::instruction_LEA_53 | Rule::instruction_ZEX_54 | Rule::instruction_SEX_55 | Rule::instruction_LDL_60 | Rule::instruction_LDH_61 | Rule::instruction_SWP_62 | Rule::instruction_LDR_63 | Rule::instruction_LDR_64 | Rule::instruction_LDR_65 | Rule::instruction_STR_66 | Rule::instruction_STR_67 | Rule::instruction_LDR_68 | Rule::instruction_STR_69 | Rule::instruction_CMP_80 | Rule::instruction_CMP_81 | Rule::instruction_INC_82 | Rule::instruction_DEC_83 | Rule::instruction_ADD_84 | Rule::instruction_SUB_85 | Rule::instruction_MUL_86 | Rule::instruction_DIV_87 | Rule::instruction_MOD_88 | Rule::instruction_ADDS_89 | Rule::instruction_SUBS_8a | Rule::instruction_MULS_8b | Rule::instruction_DIVS_8c | Rule::instruction_MODS_8d | Rule::instruction_AND_8e | Rule::instruction_OR_8f | Rule::instruction_NOT_90 | Rule::instruction_XOR_91 | Rule::instruction_LSL_92 | Rule::instruction_LSR_93 | Rule::instruction_ASL_94 | Rule::instruction_ASR_95 | Rule::instruction_RTR_96 | Rule::instruction_LSL_97 | Rule::instruction_LSR_98 | Rule::instruction_ASL_99 | Rule::instruction_ASR_9a | Rule::instruction_RTR_9b | Rule::instruction_MUS_9c | Rule::instruction_MSU_9d | Rule::instruction_CMP_a0 | Rule::instruction_CMP_a1 | Rule::instruction_ADD_a2 | Rule::instruction_SUB_a3 | Rule::instruction_MUL_a4 | Rule::instruction_DIV_a5 | Rule::instruction_CST_a6 | Rule::instruction_CST_a7 | Rule::instruction_NEG_a8 | Rule::instruction_ABS_a9 | Rule::instruction_RND_aa | Rule::instruction_RNDZ_ab | Rule::instruction_RNDI_ac | Rule::instruction_ABS_ad | Rule::instruction_LOG_ae | Rule::instruction_LN_af | Rule::instruction_EXP_b0 | Rule::instruction_SIN_b1 | Rule::instruction_COS_b2 | Rule::instruction_SIN_b3 | Rule::instruction_ASN_b4 | Rule::instruction_ACS_b5 | Rule::instruction_ATN_b6 | Rule::instruction_SETT_c0 | Rule::instruction_GETT_c1 | Rule::instruction_CHKT_c2 | Rule::instruction_CLRT_c3 | Rule::instruction_STALL_c4 => {
                    first_pass_index += 1;
                },
                _ => unreachable!(),
            }
        }
    }
    let parsed = AssemblerParser::parse(Rule::file, input)?;
    for p in parsed {
        match p.as_rule() {
            Rule::EOI => continue,
            Rule::prog => {
                for t in p.into_inner() {
                    match t.as_rule() {
                        Rule::EOI | Rule::label_arg => continue,
                        Rule::instruction_TRAP_00 => {
                            data.instructions.push(Instruction::Trap);
                        }
                        Rule::instruction_PUSH_01 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::PushIntegerRegister { rx });
                        }
                        Rule::instruction_PUSH_02 => {
                            let mut iter = t.into_inner();
                            let fx = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::PushFloatingPointRegister { fx });
                        }
                        Rule::instruction_POP_03 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::PopIntegerRegister { rx });
                        }
                        Rule::instruction_POP_04 => {
                            let mut iter = t.into_inner();
                            let fx = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::PopFloatingPointRegister { fx });
                        }
                        Rule::instruction_SWP_05 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let fy = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::SwapRegister { rx, fy });
                        }
                        Rule::instruction_STALL_06 => {
                            let mut iter = t.into_inner();
                            let value = parse_number(iter.next().unwrap().as_str())?;
                            data.instructions.push(Instruction::StallImmediate { value });
                        }
                        Rule::instruction_STALL_07 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::StallRegister { rx });
                        }
                        Rule::instruction_B_20 => {
                            let mut iter = t.into_inner();
                            let l = iter.next().unwrap().as_str().len() > 0;
                            let condition = iter.next().unwrap().as_str().parse()?;
                            let rx = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::RegisterJump { l, condition, rx });
                        }
                        Rule::instruction_B_21 => {
                            let mut iter = t.into_inner();
                            let l = iter.next().unwrap().as_str().len() > 0;
                            let condition = iter.next().unwrap().as_str().parse()?;
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let i = parse_number(iter.next().unwrap().as_str())?;
                            let s = parse_number(iter.next().unwrap().as_str())?;
                            data.instructions.push(Instruction::IndirectJump { l, condition, rx, i, s });
                        }
                        Rule::instruction_B_22 => {
                            let mut iter = t.into_inner();
                            let l = iter.next().unwrap().as_str().len() > 0;
                            let condition = iter.next().unwrap().as_str().parse()?;
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ro = iter.next().unwrap().as_str().parse()?;
                            let s = parse_number(iter.next().unwrap().as_str())?;
                            data.instructions.push(Instruction::IndirectwithRegisterOffsetJump { l, condition, rx, ro, s });
                        }
                        Rule::instruction_BR_23 => {
                            let mut iter = t.into_inner();
                            let l = iter.next().unwrap().as_str().len() > 0;
                            let condition = iter.next().unwrap().as_str().parse()?;
                            let rx = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::RelativeJump { l, condition, rx });
                        }
                        Rule::instruction_B_24 => {
                            let mut iter = t.into_inner();
                            let l = iter.next().unwrap().as_str().len() > 0;
                            let condition = iter.next().unwrap().as_str().parse()?;
                            let label = parse_prog_label(iter.next().unwrap().as_str(), &prog_labels)?;
                            data.instructions.push(Instruction::ImmediateJump { l, condition, label });
                        }
                        Rule::instruction_BO_25 => {
                            let mut iter = t.into_inner();
                            let l = iter.next().unwrap().as_str().len() > 0;
                            let condition = iter.next().unwrap().as_str().parse()?;
                            let offset = parse_signed_number(iter.next().unwrap().as_str())?;
                            data.instructions.push(Instruction::ImmediateRelativeJump { l, condition, offset });
                        }
                        Rule::instruction_LDL_40 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let value = parse_number(iter.next().unwrap().as_str())?;
                            data.instructions.push(Instruction::IntegerLoadLow { rx, value });
                        }
                        Rule::instruction_LDH_41 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let value = parse_number(iter.next().unwrap().as_str())?;
                            data.instructions.push(Instruction::IntegerLoadHigh { rx, value });
                        }
                        Rule::instruction_SWP_42 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::SwapIntegerRegisters { rx, ry });
                        }
                        Rule::instruction_LDR_43 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::CopyIntegerRegister { rx, ry });
                        }
                        Rule::instruction_LDR_44 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let i = parse_number(iter.next().unwrap().as_str())?;
                            let s = parse_number(iter.next().unwrap().as_str())?;
                            data.instructions.push(Instruction::LoadIntegerRegisterIndirect { rx, ry, i, s });
                        }
                        Rule::instruction_LDR_45 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let ro = iter.next().unwrap().as_str().parse()?;
                            let s = parse_number(iter.next().unwrap().as_str())?;
                            data.instructions.push(Instruction::LoadIntegerRegisterIndirectwithRegisterOffset { rx, ry, ro, s });
                        }
                        Rule::instruction_LDR_46 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let i = parse_number(iter.next().unwrap().as_str())?;
                            let s = parse_number(iter.next().unwrap().as_str())?;
                            data.instructions.push(Instruction::LoadIntegerRegisterIndirectProgram { rx, ry, i, s });
                        }
                        Rule::instruction_LDR_47 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let ro = iter.next().unwrap().as_str().parse()?;
                            let s = parse_number(iter.next().unwrap().as_str())?;
                            data.instructions.push(Instruction::LoadIntegerRegisterIndirectwithRegisterOffsetProgram { rx, ry, ro, s });
                        }
                        Rule::instruction_STR_48 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let i = parse_number(iter.next().unwrap().as_str())?;
                            let s = parse_number(iter.next().unwrap().as_str())?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::StoreIntegerRegisterIndirect { rx, i, s, ry });
                        }
                        Rule::instruction_STR_49 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ro = iter.next().unwrap().as_str().parse()?;
                            let s = parse_number(iter.next().unwrap().as_str())?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::StoreIntegerRegisterIndirectwithRegisterOffsetIndirect { rx, ro, s, ry });
                        }
                        Rule::instruction_STR_4a => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let i = parse_number(iter.next().unwrap().as_str())?;
                            let s = parse_number(iter.next().unwrap().as_str())?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::StoreIntegerRegisterIndirectProgram { rx, i, s, ry });
                        }
                        Rule::instruction_STR_4b => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ro = iter.next().unwrap().as_str().parse()?;
                            let s = parse_number(iter.next().unwrap().as_str())?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::StoreIntegerRegisterIndirectwithRegisterOffsetProgram { rx, ro, s, ry });
                        }
                        Rule::instruction_LDR_4c => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let label = parse_data_label(iter.next().unwrap().as_str(), &data_labels)?;
                            data.instructions.push(Instruction::IntegerLoadData { rx, label });
                        }
                        Rule::instruction_LDR_4d => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let label = parse_prog_label(iter.next().unwrap().as_str(), &prog_labels)?;
                            data.instructions.push(Instruction::IntegerLoadProgram { rx, label });
                        }
                        Rule::instruction_STR_4e => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let label = parse_data_label(iter.next().unwrap().as_str(), &data_labels)?;
                            data.instructions.push(Instruction::IntegerStoreData { rx, label });
                        }
                        Rule::instruction_STR_4f => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let label = parse_prog_label(iter.next().unwrap().as_str(), &prog_labels)?;
                            data.instructions.push(Instruction::IntegerStoreProgram { rx, label });
                        }
                        Rule::instruction_LEA_50 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let label = parse_data_label(iter.next().unwrap().as_str(), &data_labels)?;
                            data.instructions.push(Instruction::IntegerLoadEffectiveDataAddress { rx, label });
                        }
                        Rule::instruction_LEA_51 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let label = parse_data_label(iter.next().unwrap().as_str(), &data_labels)?;
                            data.instructions.push(Instruction::IntegerLoadEffectiveProgramAddress { rx, label });
                        }
                        Rule::instruction_LEA_52 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let i = parse_number(iter.next().unwrap().as_str())?;
                            let s = parse_number(iter.next().unwrap().as_str())?;
                            data.instructions.push(Instruction::LoadIntegerEffectiveAddressRegisterIndirect { rx, ry, i, s });
                        }
                        Rule::instruction_LEA_53 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let ro = iter.next().unwrap().as_str().parse()?;
                            let s = parse_number(iter.next().unwrap().as_str())?;
                            data.instructions.push(Instruction::LoadIntegerEffectiveAddressRegisterIndirectwithRegisterOffset { rx, ry, ro, s });
                        }
                        Rule::instruction_ZEX_54 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let count = parse_number(iter.next().unwrap().as_str())?;
                            data.instructions.push(Instruction::UnsignedZeroExtend { rx, ry, count });
                        }
                        Rule::instruction_SEX_55 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let count = parse_number(iter.next().unwrap().as_str())?;
                            data.instructions.push(Instruction::SignExtend { rx, ry, count });
                        }
                        Rule::instruction_LDL_60 => {
                            let mut iter = t.into_inner();
                            let fx = iter.next().unwrap().as_str().parse()?;
                            let value = parse_number(iter.next().unwrap().as_str())?;
                            data.instructions.push(Instruction::FloatingPointLoadLow { fx, value });
                        }
                        Rule::instruction_LDH_61 => {
                            let mut iter = t.into_inner();
                            let fx = iter.next().unwrap().as_str().parse()?;
                            let value = parse_number(iter.next().unwrap().as_str())?;
                            data.instructions.push(Instruction::FloatingPointLoadHigh { fx, value });
                        }
                        Rule::instruction_SWP_62 => {
                            let mut iter = t.into_inner();
                            let fx = iter.next().unwrap().as_str().parse()?;
                            let fy = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::SwapFloatingPointRegisters { fx, fy });
                        }
                        Rule::instruction_LDR_63 => {
                            let mut iter = t.into_inner();
                            let fx = iter.next().unwrap().as_str().parse()?;
                            let fy = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::CopyFloatingPointRegister { fx, fy });
                        }
                        Rule::instruction_LDR_64 => {
                            let mut iter = t.into_inner();
                            let fx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let i = parse_number(iter.next().unwrap().as_str())?;
                            let s = parse_number(iter.next().unwrap().as_str())?;
                            data.instructions.push(Instruction::LoadFloatingPointRegisterIndirect { fx, ry, i, s });
                        }
                        Rule::instruction_LDR_65 => {
                            let mut iter = t.into_inner();
                            let fx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let ro = iter.next().unwrap().as_str().parse()?;
                            let s = parse_number(iter.next().unwrap().as_str())?;
                            data.instructions.push(Instruction::LoadFloatingPointRegisterIndirectwithRegisterOffset { fx, ry, ro, s });
                        }
                        Rule::instruction_STR_66 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let i = parse_number(iter.next().unwrap().as_str())?;
                            let s = parse_number(iter.next().unwrap().as_str())?;
                            let fy = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::StoreFloatingPointRegisterIndirect { rx, i, s, fy });
                        }
                        Rule::instruction_STR_67 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ro = iter.next().unwrap().as_str().parse()?;
                            let s = parse_number(iter.next().unwrap().as_str())?;
                            let fy = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::StoreFloatingPointRegisterIndirectwithRegisterOffset { rx, ro, s, fy });
                        }
                        Rule::instruction_LDR_68 => {
                            let mut iter = t.into_inner();
                            let fx = iter.next().unwrap().as_str().parse()?;
                            let label = parse_data_label(iter.next().unwrap().as_str(), &data_labels)?;
                            data.instructions.push(Instruction::FloatingPointLoadData { fx, label });
                        }
                        Rule::instruction_STR_69 => {
                            let mut iter = t.into_inner();
                            let fx = iter.next().unwrap().as_str().parse()?;
                            let label = parse_data_label(iter.next().unwrap().as_str(), &data_labels)?;
                            data.instructions.push(Instruction::FloatingPointStoreData { fx, label });
                        }
                        Rule::instruction_CMP_80 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::IntegerCompare { rx, ry });
                        }
                        Rule::instruction_CMP_81 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::IntegerCompareSingleAgainstZero { rx });
                        }
                        Rule::instruction_INC_82 => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let rx = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::IncrementIntegerRegister { c, rx });
                        }
                        Rule::instruction_DEC_83 => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let rx = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::DecrementIntegerRegister { c, rx });
                        }
                        Rule::instruction_ADD_84 => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let rz = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::AddUnsignedInteger { c, rx, ry, rz });
                        }
                        Rule::instruction_SUB_85 => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let rz = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::SubtractUnsignedInteger { c, rx, ry, rz });
                        }
                        Rule::instruction_MUL_86 => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let rz = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::MultiplyUnsignedInteger { c, rx, ry, rz });
                        }
                        Rule::instruction_DIV_87 => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let rz = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::DivideUnsignedInteger { c, rx, ry, rz });
                        }
                        Rule::instruction_MOD_88 => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let rz = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::ModuloUnsignedInteger { c, rx, ry, rz });
                        }
                        Rule::instruction_ADDS_89 => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let rz = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::AddSignedInteger { c, rx, ry, rz });
                        }
                        Rule::instruction_SUBS_8a => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let rz = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::SubtractSignedInteger { c, rx, ry, rz });
                        }
                        Rule::instruction_MULS_8b => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let rz = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::MultiplySignedInteger { c, rx, ry, rz });
                        }
                        Rule::instruction_DIVS_8c => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let rz = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::DivideSignedInteger { c, rx, ry, rz });
                        }
                        Rule::instruction_MODS_8d => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let rz = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::ModuloSignedInteger { c, rx, ry, rz });
                        }
                        Rule::instruction_AND_8e => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let rz = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::BitwiseAND { rx, ry, rz });
                        }
                        Rule::instruction_OR_8f => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let rz = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::BitwiseOR { rx, ry, rz });
                        }
                        Rule::instruction_NOT_90 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::BitwiseNOT { rx, ry });
                        }
                        Rule::instruction_XOR_91 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let rz = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::BitwiseXOR { rx, ry, rz });
                        }
                        Rule::instruction_LSL_92 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let value = parse_number(iter.next().unwrap().as_str())?;
                            data.instructions.push(Instruction::LogicalShiftLeft { rx, ry, value });
                        }
                        Rule::instruction_LSR_93 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let value = parse_number(iter.next().unwrap().as_str())?;
                            data.instructions.push(Instruction::LogicalShiftRight { rx, ry, value });
                        }
                        Rule::instruction_ASL_94 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let value = parse_number(iter.next().unwrap().as_str())?;
                            data.instructions.push(Instruction::ArithmeticShiftLeft { rx, ry, value });
                        }
                        Rule::instruction_ASR_95 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let value = parse_number(iter.next().unwrap().as_str())?;
                            data.instructions.push(Instruction::ArithmeticShiftRight { rx, ry, value });
                        }
                        Rule::instruction_RTR_96 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let value = parse_number(iter.next().unwrap().as_str())?;
                            data.instructions.push(Instruction::RotateRight { rx, ry, value });
                        }
                        Rule::instruction_LSL_97 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let rz = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::LogicalShiftLeftRegister { rx, ry, rz });
                        }
                        Rule::instruction_LSR_98 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let rz = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::LogicalShiftRightRegister { rx, ry, rz });
                        }
                        Rule::instruction_ASL_99 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let rz = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::ArithmeticShiftLeftRegister { rx, ry, rz });
                        }
                        Rule::instruction_ASR_9a => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let rz = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::ArithmeticShiftRightRegister { rx, ry, rz });
                        }
                        Rule::instruction_RTR_9b => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            let rz = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::RotateRightRegister { rx, ry, rz });
                        }
                        Rule::instruction_MUS_9c => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::MapUnsignedToSigned { rx, ry });
                        }
                        Rule::instruction_MSU_9d => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::MapSignedToUnsigned { rx, ry });
                        }
                        Rule::instruction_CMP_a0 => {
                            let mut iter = t.into_inner();
                            let fx = iter.next().unwrap().as_str().parse()?;
                            let fy = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::FloatingPointCompare { fx, fy });
                        }
                        Rule::instruction_CMP_a1 => {
                            let mut iter = t.into_inner();
                            let fx = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::FloatingPointCompareSingleAgainstZero { fx });
                        }
                        Rule::instruction_ADD_a2 => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let fx = iter.next().unwrap().as_str().parse()?;
                            let fy = iter.next().unwrap().as_str().parse()?;
                            let fz = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::AddFloatingPoint { c, fx, fy, fz });
                        }
                        Rule::instruction_SUB_a3 => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let fx = iter.next().unwrap().as_str().parse()?;
                            let fy = iter.next().unwrap().as_str().parse()?;
                            let fz = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::SubtractFloatingPoint { c, fx, fy, fz });
                        }
                        Rule::instruction_MUL_a4 => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let fx = iter.next().unwrap().as_str().parse()?;
                            let fy = iter.next().unwrap().as_str().parse()?;
                            let fz = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::MultiplyFloatingPoint { c, fx, fy, fz });
                        }
                        Rule::instruction_DIV_a5 => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let fx = iter.next().unwrap().as_str().parse()?;
                            let fy = iter.next().unwrap().as_str().parse()?;
                            let fz = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::DivideFloatingPoint { c, fx, fy, fz });
                        }
                        Rule::instruction_CST_a6 => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let fx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::CastToFloat { c, fx, ry });
                        }
                        Rule::instruction_CST_a7 => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let fy = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::CastFromFloat { c, rx, fy });
                        }
                        Rule::instruction_NEG_a8 => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let fx = iter.next().unwrap().as_str().parse()?;
                            let fy = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::NegateFloatingPoint { c, fx, fy });
                        }
                        Rule::instruction_ABS_a9 => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let fx = iter.next().unwrap().as_str().parse()?;
                            let fy = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::AbsoluteValueFloatingPoint { c, fx, fy });
                        }
                        Rule::instruction_RND_aa => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let fx = iter.next().unwrap().as_str().parse()?;
                            let fy = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::RoundFloatingPoint { c, fx, fy });
                        }
                        Rule::instruction_RNDZ_ab => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let fx = iter.next().unwrap().as_str().parse()?;
                            let fy = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::RoundToZeroFloatingPoint { c, fx, fy });
                        }
                        Rule::instruction_RNDI_ac => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let fx = iter.next().unwrap().as_str().parse()?;
                            let fy = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::RoundToInfinityFloatingPoint { c, fx, fy });
                        }
                        Rule::instruction_ABS_ad => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let fx = iter.next().unwrap().as_str().parse()?;
                            let fy = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::SquareRootFloatingPoint { c, fx, fy });
                        }
                        Rule::instruction_LOG_ae => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let fx = iter.next().unwrap().as_str().parse()?;
                            let fy = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::LogBase10FloatingPoint { c, fx, fy });
                        }
                        Rule::instruction_LN_af => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let fx = iter.next().unwrap().as_str().parse()?;
                            let fy = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::LogNatrualFloatingPoint { c, fx, fy });
                        }
                        Rule::instruction_EXP_b0 => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let fx = iter.next().unwrap().as_str().parse()?;
                            let fy = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::ExponentialFloatingPoint { c, fx, fy });
                        }
                        Rule::instruction_SIN_b1 => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let fx = iter.next().unwrap().as_str().parse()?;
                            let fy = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::SineFloatingPoint { c, fx, fy });
                        }
                        Rule::instruction_COS_b2 => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let fx = iter.next().unwrap().as_str().parse()?;
                            let fy = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::CosineFloatingPoint { c, fx, fy });
                        }
                        Rule::instruction_SIN_b3 => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let fx = iter.next().unwrap().as_str().parse()?;
                            let fy = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::TangentFloatingPoint { c, fx, fy });
                        }
                        Rule::instruction_ASN_b4 => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let fx = iter.next().unwrap().as_str().parse()?;
                            let fy = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::ArcsineFloatingPoint { c, fx, fy });
                        }
                        Rule::instruction_ACS_b5 => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let fx = iter.next().unwrap().as_str().parse()?;
                            let fy = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::ArccosineFloatingPoint { c, fx, fy });
                        }
                        Rule::instruction_ATN_b6 => {
                            let mut iter = t.into_inner();
                            let c = iter.next().unwrap().as_str().len() > 0;
                            let fx = iter.next().unwrap().as_str().parse()?;
                            let fy = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::ArctangentFloatingPoint { c, fx, fy });
                        }
                        Rule::instruction_SETT_c0 => {
                            let mut iter = t.into_inner();
                            let tx = iter.next().unwrap().as_str().parse()?;
                            let ry = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::SetTimer { tx, ry });
                        }
                        Rule::instruction_GETT_c1 => {
                            let mut iter = t.into_inner();
                            let rx = iter.next().unwrap().as_str().parse()?;
                            let ty = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::GetCurrentTimer { rx, ty });
                        }
                        Rule::instruction_CHKT_c2 => {
                            let mut iter = t.into_inner();
                            let tx = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::CheckTimer { tx });
                        }
                        Rule::instruction_CLRT_c3 => {
                            let mut iter = t.into_inner();
                            let tx = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::ClearTimer { tx });
                        }
                        Rule::instruction_STALL_c4 => {
                            let mut iter = t.into_inner();
                            let tx = iter.next().unwrap().as_str().parse()?;
                            data.instructions.push(Instruction::StallTimer { tx });
                        }
                        _ => unreachable!(),
                    }
                }
            },
            Rule::data => continue,
            _ => unreachable!(),
        }

        for insturction in data.instructions.iter() {
            if !insturction.is_valid() {
                return Err(AssemblerError::InvalidInstruction(*insturction))
            }
        }
    }
    Ok(data)
}
