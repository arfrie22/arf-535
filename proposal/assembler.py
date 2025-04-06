import json
import re

output_file = open("../assembler/src/assembler.pest", "w+")
with open('instructions.json') as f:
    data = json.load(f)
    rule_names = []
    rules = ""
    for type_i, type_data in enumerate(data):
        for opcode_i, opcode_data in enumerate(type_data["opcodes"]):
            
            pneumonic = opcode_data["assembly"]["pneumonic"]
            rule_name = "instruction_" + pneumonic + "_" + "{:02x}".format((type_i << 5) | opcode_i)
            rule = rule_name + " = ${ ^\"" + pneumonic + "\""
            if type_data["name"] == "Branch":
                rule += " ~ condition"

            for arg in opcode_data["assembly"]["arguments"]:
                rule += " ~ WHITESPACE+ ~ " + arg["type"]

            rule += " }"

            rule_names.append(rule_name)

            rules += rule + "\n"
    output_file.write("label_arg = ${ ASCII_ALPHA ~ (ASCII_ALPHANUMERIC | \"_\" )*  }\n")

    output_file.write("digit_number = ${ ASCII_DIGIT+ }\n")
    output_file.write("hex_number = ${ \"0x\" ~ ASCII_HEX_DIGIT+ }\n")
    output_file.write("binary_number = ${ \"0b\" ~ ASCII_BIN_DIGIT+ }\n")
    output_file.write("number = ${ hex_number | binary_number | digit_number }\n")

    numbers = " | ".join([f"\"{i+1}\"" for i in range(32)])
    output_file.write("register = ${ ^\"R\" ~ (" + numbers + ") | (^\"A\" | ^\"D\") ~ ('1'..'4') | ^\"PC\" | ^\"LR\" | ^\"ST\" | ^\"SP\" }\n")
    output_file.write("f_register = ${ ^\"F\" ~ (" + numbers +  ") }\n")
    output_file.write("timer = ${ ^\"T\" ~ (" + numbers +  ") }\n")

    output_file.write("p_address = ${ \"p\" ~ number | label_arg }\n")
    output_file.write("d_address = ${ \"d\"? ~ number }\n")
    output_file.write("data_shift_register = ${ \"d\"? ~ \"[\" ~ WHITESPACE* ~ register ~ (WHITESPACE* ~ \"+\" ~ WHITESPACE* ~ register ~ (WHITESPACE* ~ \"<<\" ~ WHITESPACE* ~ number)?) ~ WHITESPACE* ~ \"]\" }\n")
    output_file.write("data_shift_imm = ${ \"d\"? ~ \"[\" ~ WHITESPACE* ~ register ~ (WHITESPACE* ~ \"+\" ~ WHITESPACE* ~ number ~ (WHITESPACE* ~ \"<<\" ~ WHITESPACE* ~ number)?)? ~ WHITESPACE* ~ \"]\" }\n")
    output_file.write("prog_shift_register = ${ \"p\" ~ \"[\" ~ WHITESPACE* ~ register ~ (WHITESPACE* ~ \"+\" ~ WHITESPACE* ~ register ~ (WHITESPACE* ~ \"<<\" ~ WHITESPACE* ~ number)?) ~ WHITESPACE* ~ \"]\" }\n")
    output_file.write("prog_shift_imm = ${ \"p\" ~ \"[\" ~ WHITESPACE* ~ register ~ (WHITESPACE* ~ \"+\" ~ WHITESPACE* ~ number ~ (WHITESPACE* ~ \"<<\" ~ WHITESPACE* ~ number)?)? ~ WHITESPACE* ~ \"]\" }\n")
    
    output_file.write("condition = ${ (^\"NVR\" | ^\"EQ\" | ^\"GT\" | ^\"LT\" | ^\"GE\" | ^\"LE\" | ^\"OVRF\" | ^\"UNDF\" | ^\"DIVZ\" | ^\"EVEN\" | ^\"FINF\" | ^\"FZ\" | ^\"FNAN\")? }\n")

    output_file.write(rules)

    output_file.write("comment = ${ \";\" ~ ('!'..'~' | WHITESPACE)* }\n")
    output_file.write("label = ${label_arg ~ \":\"}\n")
    output_file.write("instruction = ${ " + " | ".join(rule_names) + " }\n")

    output_file.write("value = ${ instruction | label | comment }\n")
    output_file.write("line = ${ WHITESPACE* ~ value ~ WHITESPACE* }\n")
    output_file.write("file = ${ SOI ~ (WHITESPACE | NEWLINE)* ~ line ~ (NEWLINE+ ~ line)* ~ (WHITESPACE | NEWLINE)* ~ EOI }\n")

    output_file.write("WHITESPACE = _{\" \" | \"\\t\"}\n")