import json
import re

output_file = open("../assembler/src/assembler.pest", "w+")
output_file_rs = open("../assembler/src/gen_functions.rs", "w+")
with open('instructions.json') as f:
    data = json.load(f)
    rule_names = []
    all_rules = []
    rules = ""
    parses = "            Rule::EOI | Rule::label_arg => continue,\n"
    for type_i, type_data in enumerate(data):
        for opcode_i, opcode_data in enumerate(type_data["opcodes"]):
            
            pneumonic = opcode_data["assembly"]["pneumonic"]
            rule_name = "instruction_" + pneumonic + "_" + "{:02x}".format((type_i << 5) | opcode_i)
            rule_names.append(rule_name)
            all_rules.append("Rule::" + rule_name)

            rule = rule_name + " = ${ ^\"" + pneumonic + "\""
            parse = "            Rule::" + rule_name + " => {\n"

            instruction = "                instructions.push(Instruction::" + opcode_data["name"].replace(" ", "").replace("-", "")

            args = opcode_data["assembly"]["arguments"]
            if len(args) > 0:
                parse += "                let mut iter = p.into_inner();\n"
                instruction += " { "
                for i, arg in enumerate(args):
                    if arg["type"] == "condition":
                        rule += " ~ condition"
                    elif arg["type"] == "condition_bit":
                        rule += " ~ condition_bit"
                    else:
                        rule += " ~ WHITESPACE+ ~ " + arg["type"]

                    if i > 0:
                        instruction += ", "
                    arg_name: str = arg["argument"].replace("_", "").lower()
                    instruction += arg_name
                    
                    parse += "                let " + arg_name + " = "
                    if arg["type"] == "p_address" or arg["type"] == "d_address":
                        parse += "parse_label(iter.next().unwrap().as_str(), &labels)?;\n"
                    elif arg["type"] == "register" or arg["type"] == "f_register" or arg["type"] == "timer" or arg["type"] == "condition" or arg["type"] == "condition":
                        parse += "iter.next().unwrap().as_str().parse()?;\n"
                    elif arg["type"] == "number":
                        parse += "parse_number(iter.next().unwrap().as_str())?;\n"
                    elif arg["type"] == "signed_number":
                        parse += "parse_signed_number(iter.next().unwrap().as_str())?;\n"
                    elif arg["type"] == "data_shift_imm" or arg["type"] == "prog_shift_imm":
                        instruction += ", i, s"
                        parse += "iter.next().unwrap().as_str().parse()?;\n"
                        parse += "                let i = parse_number(iter.next().unwrap().as_str())?;\n"
                        parse += "                let s = parse_number(iter.next().unwrap().as_str())?;\n"
                    elif arg["type"] == "data_shift_register" or arg["type"] == "prog_shift_register":
                        instruction += ", ro, s"
                        parse += "iter.next().unwrap().as_str().parse()?;\n"
                        parse += "                let ro = iter.next().unwrap().as_str().parse()?;\n"
                        parse += "                let s = parse_number(iter.next().unwrap().as_str())?;\n"
                    elif arg["type"] == "condition_bit":
                        parse += "if iter.next().unwrap().as_str().len() > 0 {1} else {0};\n"
                    else:
                        raise Exception("Unknown arg type " + arg["type"])
                instruction += " }"
            parse += instruction + ");\n"

            rule += " }"
            rules += rule + "\n"
            
            
            parse += "            }\n"
            parses += parse
    output_file.write("label_arg = ${ ASCII_ALPHA ~ (ASCII_ALPHANUMERIC | \"_\" )*  }\n")

    output_file.write("digit_number = ${ ASCII_DIGIT+ }\n")
    output_file.write("hex_number = ${ \"0x\" ~ ASCII_HEX_DIGIT+ }\n")
    output_file.write("binary_number = ${ \"0b\" ~ ASCII_BIN_DIGIT+ }\n")
    output_file.write("number = ${ hex_number | binary_number | digit_number }\n")
    output_file.write("signed_number = ${ (\"+\" | \"-\" )? ~ WHITESPACE* ~ number }\n")

    numbers = " | ".join([f"\"{i}\"" for i in range(32)])
    output_file.write("register = ${ ^\"R\" ~ (" + numbers + ") | (^\"A\" | ^\"D\") ~ ('1'..'4') | ^\"PC\" | ^\"LR\" | ^\"ST\" | ^\"SP\" }\n")
    output_file.write("f_register = ${ ^\"F\" ~ (" + numbers +  ") }\n")
    output_file.write("timer = ${ ^\"T\" ~ (" + numbers +  ") }\n")
    
    output_file.write("empty = { \"\" }\n")
    
    output_file.write("p_address = ${ \"p\" ~ number | label_arg }\n")
    output_file.write("d_address = ${ \"d\"? ~ number }\n")
    output_file.write("data_shift_register = _{ \"d\"? ~ \"[\" ~ WHITESPACE* ~ register ~ (WHITESPACE* ~ \"+\" ~ WHITESPACE* ~ register ~ ((WHITESPACE* ~ \"<<\" ~ WHITESPACE* ~ number) | empty)) ~ WHITESPACE* ~ \"]\" }\n")
    output_file.write("data_shift_imm = _{ \"d\"? ~ \"[\" ~ WHITESPACE* ~ register ~ ((WHITESPACE* ~ \"+\" ~ WHITESPACE* ~ number ~ ((WHITESPACE* ~ \"<<\" ~ WHITESPACE* ~ number) | empty)) | empty ~ empty) ~ WHITESPACE* ~ \"]\" }\n")
    output_file.write("prog_shift_register = _{ \"p\" ~ \"[\" ~ WHITESPACE* ~ register ~ (WHITESPACE* ~ \"+\" ~ WHITESPACE* ~ register ~ ((WHITESPACE* ~ \"<<\" ~ WHITESPACE* ~ number) | empty)) ~ WHITESPACE* ~ \"]\" }\n")
    output_file.write("prog_shift_imm = _{ \"p\" ~ \"[\" ~ WHITESPACE* ~ register ~ ((WHITESPACE* ~ \"+\" ~ WHITESPACE* ~ number ~ ((WHITESPACE* ~ \"<<\" ~ WHITESPACE* ~ number) | empty)) | empty ~ empty) ~ WHITESPACE* ~ \"]\" }\n")
    
    output_file.write("condition = ${ (^\"NVR\" | ^\"EQ\" | ^\"GT\" | ^\"LT\" | ^\"GE\" | ^\"LE\" | ^\"OVRF\" | ^\"UNDF\" | ^\"DIVZ\" | ^\"EVEN\" | ^\"FINF\" | ^\"FZ\" | ^\"FNAN\")? }\n")
    output_file.write("condition_bit = ${ (^\".C\")? }\n")

    output_file.write(rules)

    output_file.write("comment = _{ \";\" ~ ('!'..'~' | WHITESPACE)* }\n")
    output_file.write("label = _{label_arg ~ \":\"}\n")
    output_file.write("instruction = _{ " + " | ".join(rule_names) + " }\n")

    output_file.write("value = _{ instruction | label | comment }\n")
    output_file.write("line = _{ WHITESPACE* ~ value ~ WHITESPACE* }\n")
    output_file.write("file = _{ SOI ~ (WHITESPACE | NEWLINE)* ~ line ~ (NEWLINE+ ~ line)* ~ (WHITESPACE | NEWLINE)* ~ EOI }\n")

    output_file.write("WHITESPACE = _{\" \" | \"\\t\"}\n")

    parses += "            _ => unreachable!(),\n"


    
    output_file_rs.write("use std::collections::HashMap;\n\n")

    output_file_rs.write("use pest::Parser;\n")
    output_file_rs.write("use simulator::instruction::Instruction;\n\n")

    output_file_rs.write("use crate::{parse_label, parse_number, parse_signed_number, AssemblerError, AssemblerParser, Rule};\n\n")

    output_file_rs.write("pub fn assemble(input: &str) -> Result<Vec<Instruction>, AssemblerError> {\n")
    output_file_rs.write("    let parsed = AssemblerParser::parse(Rule::file, input)?;\n\n")

    output_file_rs.write("    let mut instructions = Vec::new();\n")
    output_file_rs.write("    let mut labels = HashMap::new();\n")
    output_file_rs.write("    let mut first_pass_index = 0;\n")
    
    output_file_rs.write("    for p in parsed {\n")
    output_file_rs.write("        match p.as_rule() {\n")
    output_file_rs.write("            Rule::EOI => continue,\n")
    output_file_rs.write("            Rule::label_arg => {\n")
    output_file_rs.write("                labels.insert(p.as_str().to_owned(), first_pass_index);\n")
    output_file_rs.write("                first_pass_index += 1;\n")
    output_file_rs.write("            },\n")
    output_file_rs.write("            " + " | ".join(all_rules) + " => {\n")
    output_file_rs.write("                first_pass_index += 1;\n")
    output_file_rs.write("            },\n")
    output_file_rs.write("            _ => unreachable!(),\n")
    output_file_rs.write("        }\n")
    output_file_rs.write("    }\n\n")
    output_file_rs.write("    let parsed = AssemblerParser::parse(Rule::file, input)?;\n")

    output_file_rs.write("    for p in parsed {\n")
    output_file_rs.write("        match p.as_rule() {\n")
    output_file_rs.write(parses)
    output_file_rs.write("        }\n")
    output_file_rs.write("    }\n")
    output_file_rs.write("    Ok(instructions)\n")
    output_file_rs.write("}\n")