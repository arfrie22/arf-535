import json
import re

output_file = open("../assembler/src/assembler.pest", "w+")
output_file_rs = open("../assembler/src/gen_functions.rs", "w+")
with open('instructions.json') as f:
    data = json.load(f)
    rule_names = []
    all_rules = []
    rules = ""
    parses = "                        Rule::EOI | Rule::label_arg => continue,\n"
    for type_i, type_data in enumerate(data):
        for opcode_i, opcode_data in enumerate(type_data["opcodes"]):
            
            pneumonic = opcode_data["assembly"]["pneumonic"]
            rule_name = "instruction_" + pneumonic + "_" + "{:02x}".format((type_i << 5) | opcode_i)
            rule_names.append(rule_name)
            all_rules.append("Rule::" + rule_name)

            rule = rule_name + " = ${ ^\"" + pneumonic + "\""
            parse = "                        Rule::" + rule_name + " => {\n"

            instruction = "                            data.instructions.push(Instruction::" + opcode_data["name"].replace(" ", "").replace("-", "")

            args = opcode_data["assembly"]["arguments"]
            if len(args) > 0:
                parse += "                            let mut iter = t.into_inner();\n"
                instruction += " { "
                for i, arg in enumerate(args):
                    if arg["type"] == "condition" or arg["type"] == "condition_bit" or arg["type"] == "link_bit":
                        rule += " ~ " + arg["type"]
                    else:
                        rule += " ~ WHITESPACE+ ~ " + arg["type"]

                    if i > 0:
                        instruction += ", "
                    arg_name: str = arg["argument"].replace("_", "").lower()
                    instruction += arg_name
                    
                    parse += "                            let " + arg_name + " = "
                    if arg["type"] == "p_address":
                        parse += "parse_prog_label(iter.next().unwrap().as_str(), &prog_labels)?;\n"
                    elif arg["type"] == "d_address":
                        parse += "parse_data_label(iter.next().unwrap().as_str(), &data_labels)?;\n"
                    elif arg["type"] == "register" or arg["type"] == "f_register" or arg["type"] == "timer" or arg["type"] == "condition" or arg["type"] == "condition":
                        parse += "iter.next().unwrap().as_str().parse()?;\n"
                    elif arg["type"] == "number":
                        parse += "parse_number(iter.next().unwrap().as_str())?;\n"
                    elif arg["type"] == "signed_number":
                        parse += "parse_signed_number(iter.next().unwrap().as_str())?;\n"
                    elif arg["type"] == "data_shift_imm" or arg["type"] == "prog_shift_imm":
                        instruction += ", i, s"
                        parse += "iter.next().unwrap().as_str().parse()?;\n"
                        parse += "                            let i = parse_number(iter.next().unwrap().as_str())?;\n"
                        parse += "                            let s = parse_number(iter.next().unwrap().as_str())?;\n"
                    elif arg["type"] == "data_shift_register" or arg["type"] == "prog_shift_register":
                        instruction += ", ro, s"
                        parse += "iter.next().unwrap().as_str().parse()?;\n"
                        parse += "                            let ro = iter.next().unwrap().as_str().parse()?;\n"
                        parse += "                            let s = parse_number(iter.next().unwrap().as_str())?;\n"
                    elif arg["type"] == "condition_bit" or arg["type"] == "link_bit":
                        parse += "iter.next().unwrap().as_str().len() > 0;\n"
                    else:
                        raise Exception("Unknown arg type " + arg["type"])
                instruction += " }"
            parse += instruction + ");\n"

            rule += " }"
            rules += rule + "\n"
            
            
            parse += "                        }\n"
            parses += parse
    output_file.write("label_arg = ${ ASCII_ALPHA ~ (ASCII_ALPHANUMERIC | \"_\" )*  }\n")

    output_file.write("digit_number = ${ ASCII_DIGIT+ }\n")
    output_file.write("hex_number = ${ \"0x\" ~ ASCII_HEX_DIGIT+ }\n")
    output_file.write("binary_number = ${ \"0b\" ~ ASCII_BIN_DIGIT+ }\n")
    output_file.write("number = ${ hex_number | binary_number | digit_number }\n")
    output_file.write("signed_number = ${ (\"+\" | \"-\" )? ~ number }\n")

    numbers = " | ".join([f"\"{i}\"" for i in range(32)])
    output_file.write("register = ${ ^\"R\" ~ (" + numbers + ") | (^\"A\" | ^\"D\") ~ ('1'..'4') | ^\"PC\" | ^\"LR\" | ^\"ST\" | ^\"SP\" }\n")
    output_file.write("f_register = ${ ^\"F\" ~ (" + numbers +  ") }\n")
    output_file.write("timer = ${ ^\"T\" ~ (" + numbers +  ") }\n")
    
    output_file.write("empty = { \"\" }\n")
    
    output_file.write("p_address = ${ \"p\" ~ number | \"p:\" ~ label_arg }\n")
    output_file.write("p_address_implicit = ${ \"p\"? ~ number | \"p:\"? ~ label_arg }\n")
    output_file.write("d_address = ${ \"d\" ~ number | \"d:\" ~ label_arg }\n")
    output_file.write("d_address_implicit = ${ \"d\"? ~ number | \"d:\"? ~ label_arg }\n")
    
    output_file.write("data_shift_register = _{ \"d\"? ~ \"[\" ~ WHITESPACE* ~ register ~ (WHITESPACE* ~ \"+\" ~ WHITESPACE* ~ register ~ ((WHITESPACE* ~ \"<<\" ~ WHITESPACE* ~ number) | empty)) ~ WHITESPACE* ~ \"]\" }\n")
    output_file.write("data_shift_imm = _{ \"d\"? ~ \"[\" ~ WHITESPACE* ~ register ~ ((WHITESPACE* ~ \"+\" ~ WHITESPACE* ~ number ~ ((WHITESPACE* ~ \"<<\" ~ WHITESPACE* ~ number) | empty)) | empty ~ empty) ~ WHITESPACE* ~ \"]\" }\n")
    output_file.write("prog_shift_register = _{ \"p\" ~ \"[\" ~ WHITESPACE* ~ register ~ (WHITESPACE* ~ \"+\" ~ WHITESPACE* ~ register ~ ((WHITESPACE* ~ \"<<\" ~ WHITESPACE* ~ number) | empty)) ~ WHITESPACE* ~ \"]\" }\n")
    output_file.write("prog_shift_imm = _{ \"p\" ~ \"[\" ~ WHITESPACE* ~ register ~ ((WHITESPACE* ~ \"+\" ~ WHITESPACE* ~ number ~ ((WHITESPACE* ~ \"<<\" ~ WHITESPACE* ~ number) | empty)) | empty ~ empty) ~ WHITESPACE* ~ \"]\" }\n")
    
    output_file.write("condition = ${ (^\"NVR\" | ^\"EQ\" | ^\"GT\" | ^\"LT\" | ^\"GE\" | ^\"LE\" | ^\"OVRF\" | ^\"UNDF\" | ^\"DIVZ\" | ^\"EVEN\" | ^\"FINF\" | ^\"FZ\" | ^\"FNAN\" | ^\"FPOS\")? }\n")
    output_file.write("condition_bit = ${ (^\"C\")? }\n")
    output_file.write("link_bit = ${ (^\"L\")? }\n")

    output_file.write(rules)

    output_file.write("comment = _{ \";\" ~ ('!'..'~' | WHITESPACE)* }\n")
    output_file.write("label = _{label_arg ~ \":\"}\n")
    output_file.write("instruction = _{ " + " | ".join(rule_names) + " }\n")

    output_file.write("value = _{ instruction | label | comment }\n")
    output_file.write("prog_line = _{ WHITESPACE* ~ value ~ WHITESPACE* }\n")
    output_file.write("prog = ${ \".prog\" ~ ((WHITESPACE* ~ NEWLINE)+ ~ prog_line)+ }\n")
    output_file.write("data_value = ${(number | signed_number) ~ \"#\" ~ number}\n")
    output_file.write("data_line = ${WHITESPACE* ~ label_arg ~ (WHITESPACE+ ~ data_value)+ ~ WHITESPACE*}\n")
    output_file.write("data = ${ \".data\" ~ ((WHITESPACE* ~ NEWLINE) ~ data_line)* }\n")
    output_file.write("file = _{ SOI ~ (WHITESPACE | NEWLINE)* ~ prog ~ (WHITESPACE | NEWLINE)* ~ data? ~ (WHITESPACE | NEWLINE)* ~ EOI }\n")
    

    output_file.write("WHITESPACE = _{\" \" | \"\\t\"}\n")

    parses += "                        _ => unreachable!(),\n"


    
    output_file_rs.write("use std::collections::HashMap;\n\n")

    output_file_rs.write("use pest::Parser;\n")
    output_file_rs.write("use simulator::{instruction::Instruction, raw_cast_from_i32};\n\n")

    output_file_rs.write("use crate::{parse_prog_label, parse_data_label, parse_number, parse_signed_number, AssembledData, AssemblerError, AssemblerParser, Rule};\n\n")

    output_file_rs.write("pub fn assemble(input: &str) -> Result<AssembledData, AssemblerError> {\n")
    output_file_rs.write("    let parsed = AssemblerParser::parse(Rule::file, input)?;\n\n")

    output_file_rs.write("    let mut data = AssembledData::default();\n")
    output_file_rs.write("    let mut prog_labels = HashMap::new();\n")
    output_file_rs.write("    let mut data_labels = HashMap::new();\n")
    output_file_rs.write("    let mut first_pass_index;\n")

    
    
    output_file_rs.write("    for p in parsed {\n")
    output_file_rs.write("        let labels;\n")
    output_file_rs.write("        let beginning;\n")
    output_file_rs.write("        match p.as_rule() {\n")
    output_file_rs.write("            Rule::EOI => continue,\n")
    output_file_rs.write("            Rule::prog => {\n")
    output_file_rs.write("                labels = &mut prog_labels;\n")
    output_file_rs.write("                beginning = \"p:\".to_owned();\n")
    output_file_rs.write("                first_pass_index = 0;\n")
    output_file_rs.write("            },\n")
    output_file_rs.write("            Rule::data => {\n")
    output_file_rs.write("                labels = &mut data_labels;\n")
    output_file_rs.write("                beginning = \"d:\".to_owned();\n")
    output_file_rs.write("                first_pass_index = 0;\n")
    output_file_rs.write("            },\n")
    output_file_rs.write("            _ => unreachable!(),\n")
    output_file_rs.write("        }\n\n")

    output_file_rs.write("        for t in p.into_inner() {\n")
    output_file_rs.write("            match t.as_rule() {\n")
    output_file_rs.write("                Rule::EOI => continue,\n")
    output_file_rs.write("                Rule::label_arg => {\n")
    output_file_rs.write("                    let label = beginning.clone() + t.as_str();\n")
    output_file_rs.write("                    labels.insert(label, first_pass_index);\n")
    output_file_rs.write("                    first_pass_index += 1;\n")
    output_file_rs.write("                },\n")
    output_file_rs.write("                Rule::data_line => {\n")
    output_file_rs.write("                    for t in t.into_inner() {\n")
    output_file_rs.write("                        match t.as_rule() {\n")
    output_file_rs.write("                            Rule::label_arg => {\n")
    output_file_rs.write("                                let label = beginning.clone() + t.as_str();\n")
    output_file_rs.write("                                labels.insert(label, first_pass_index);\n")
    output_file_rs.write("                            },\n")
    output_file_rs.write("                            Rule::data_value => {\n")
    output_file_rs.write("                                let mut iter = t.into_inner();\n")
    output_file_rs.write("                                let val = iter.next().unwrap();\n")
    output_file_rs.write("                                let val = match val.as_rule() {\n")
    output_file_rs.write("                                    Rule::number => parse_number(val.as_str())?,\n")
    output_file_rs.write("                                    Rule::signed_number => raw_cast_from_i32(parse_signed_number(val.as_str())?),\n")
    output_file_rs.write("                                    _ => unreachable!(),\n")
    output_file_rs.write("                                };\n\n")
    output_file_rs.write("                                let count = parse_number(iter.next().unwrap().as_str())?;\n\n\n")
    output_file_rs.write("                                data.data[(first_pass_index as usize)..(first_pass_index + count) as usize].fill(val);\n\n")
    output_file_rs.write("                                first_pass_index += count;\n")
    output_file_rs.write("                            },\n")
    output_file_rs.write("                            _ => unreachable!(),\n")
    output_file_rs.write("                        }\n")
    output_file_rs.write("                    }\n")
    output_file_rs.write("                },\n")
    output_file_rs.write("                Rule::instruction_TRAP_00 | Rule::instruction_PUSH_01 | Rule::instruction_PUSH_02 | Rule::instruction_POP_03 | Rule::instruction_POP_04 | Rule::instruction_SWP_05 | Rule::instruction_STALL_06 | Rule::instruction_STALL_07 | Rule::instruction_B_20 | Rule::instruction_B_21 | Rule::instruction_B_22 | Rule::instruction_BR_23 | Rule::instruction_B_24 | Rule::instruction_BO_25 | Rule::instruction_LDL_40 | Rule::instruction_LDH_41 | Rule::instruction_SWP_42 | Rule::instruction_LDR_43 | Rule::instruction_LDR_44 | Rule::instruction_LDR_45 | Rule::instruction_LDR_46 | Rule::instruction_LDR_47 | Rule::instruction_STR_48 | Rule::instruction_STR_49 | Rule::instruction_STR_4a | Rule::instruction_STR_4b | Rule::instruction_LDR_4c | Rule::instruction_LDR_4d | Rule::instruction_STR_4e | Rule::instruction_STR_4f | Rule::instruction_ZEX_50 | Rule::instruction_SEX_51 | Rule::instruction_LDL_60 | Rule::instruction_LDH_61 | Rule::instruction_SWP_62 | Rule::instruction_LDR_63 | Rule::instruction_LDR_64 | Rule::instruction_LDR_65 | Rule::instruction_STR_66 | Rule::instruction_STR_67 | Rule::instruction_LDR_68 | Rule::instruction_STR_69 | Rule::instruction_CMP_80 | Rule::instruction_CMP_81 | Rule::instruction_ADD_82 | Rule::instruction_SUB_83 | Rule::instruction_MUL_84 | Rule::instruction_DIV_85 | Rule::instruction_MOD_86 | Rule::instruction_ADDS_87 | Rule::instruction_SUBS_88 | Rule::instruction_MULS_89 | Rule::instruction_DIVS_8a | Rule::instruction_MODS_8b | Rule::instruction_AND_8c | Rule::instruction_OR_8d | Rule::instruction_NOT_8e | Rule::instruction_XOR_8f | Rule::instruction_LSL_90 | Rule::instruction_LSR_91 | Rule::instruction_ASL_92 | Rule::instruction_ASR_93 | Rule::instruction_RTR_94 | Rule::instruction_LSL_95 | Rule::instruction_LSR_96 | Rule::instruction_ASL_97 | Rule::instruction_ASR_98 | Rule::instruction_RTR_99 | Rule::instruction_MUS_9a | Rule::instruction_MSU_9b | Rule::instruction_CMP_a0 | Rule::instruction_CMP_a1 | Rule::instruction_ADD_a2 | Rule::instruction_SUB_a3 | Rule::instruction_MUL_a4 | Rule::instruction_DIV_a5 | Rule::instruction_CST_a6 | Rule::instruction_CST_a7 | Rule::instruction_SETT_c0 | Rule::instruction_GETT_c1 | Rule::instruction_CHKT_c2 | Rule::instruction_CLRT_c3 => {\n")
    output_file_rs.write("                    first_pass_index += 1;\n")
    output_file_rs.write("                },\n")
    output_file_rs.write("                _ => unreachable!(),\n")
    output_file_rs.write("            }\n")
    output_file_rs.write("        }\n")
    output_file_rs.write("    }\n")

    output_file_rs.write("    let parsed = AssemblerParser::parse(Rule::file, input)?;\n")

    output_file_rs.write("    for p in parsed {\n")
    output_file_rs.write("        match p.as_rule() {\n")
    output_file_rs.write("            Rule::EOI => continue,\n")
    output_file_rs.write("            Rule::prog => {\n")
    output_file_rs.write("                for t in p.into_inner() {\n")
    output_file_rs.write("                    match t.as_rule() {\n")
    output_file_rs.write(parses)
    output_file_rs.write("                    }\n")
    output_file_rs.write("                }\n")
    output_file_rs.write("            },\n")
    output_file_rs.write("            Rule::data => continue,\n")
    output_file_rs.write("            _ => unreachable!(),\n")
    output_file_rs.write("        }\n\n")
    output_file_rs.write("        for insturction in data.instructions.iter() {\n")
    output_file_rs.write("            if !insturction.is_valid() {\n")
    output_file_rs.write("                return Err(AssemblerError::InvalidInstruction(*insturction))\n")
    output_file_rs.write("            }\n")
    output_file_rs.write("        }\n")
    output_file_rs.write("    }\n")
    output_file_rs.write("    Ok(data)\n")
    output_file_rs.write("}\n")