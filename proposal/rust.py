import json
import re

output_file = open("../simulator/src/instruction.rs", "w+")

with open('instructions.json') as f:
    output_file.write("use crate::enums::{Condition, FPRegister, Register, Timer};\n\n")
    output_file.write("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n")
    output_file.write("pub enum Instruction {\n")
    output_file.write("    Invalid(u32),\n")
    data = json.load(f)
    parses = ""
    intos = ""
    for type_i, type_data in enumerate(data):
        for opcode_i, opcode_data in enumerate(type_data["opcodes"]):
            name = opcode_data["name"].replace(" ", "").replace("-", "") + ""
            enum = name
            opcode_pneumonic = opcode_data["pneumonic"]
            bits = opcode_data["bits"]
            parses += "            0x{:02x} => Self::{}".format((type_i << 5) | opcode_i, name)
            intos += "            Self::{}".format(name)
            shifts = "0x{:02x} << 24".format((type_i << 5) | opcode_i)

            if len(bits) > 0:
                enum += " { "
                parses += " { "
                intos += " { "
                shifts = f"({shifts})"
                opcode_bit_remaining = 32-8
                for i, bit_data in enumerate(bits):
                    opcode_bit_remaining -= bit_data["count"]
                    if (i > 0):
                        enum += ", "
                        parses += ", "
                        intos += ", "
                    arg_name = bit_data["short"].replace("_", "").lower()
                    arg_type = "u32"
                    parse_value = "value"
                    into_value = arg_name
                    if re.match("R[a-z]", bit_data["short"]):
                        #int reg
                        arg_type = "Register"
                    elif re.match("F[a-z]", bit_data["short"]):
                        #float reg
                        arg_type = "FPRegister"
                    elif re.match("T[a-z]", bit_data["short"]):
                        #timer reg
                        arg_type = "Timer"
                    elif re.match("Condition", bit_data["short"]):
                        #condition code
                        arg_type = "Condition"
                    
                    enum += arg_name + ": " + arg_type
                    intos += arg_name

                    if arg_type != "u32":
                       parse_value = "(value as usize)"
                       into_value = f"({into_value} as u32)"
                    
                    parse = f"({parse_value} >> {opcode_bit_remaining}) & 0x{(1 << bit_data["count"]) - 1:x}"
                    shifts += f"| ({into_value} << {opcode_bit_remaining})"
                    if arg_type != "u32":
                        parse = arg_type + "::try_from(" + parse + ").unwrap()"

                    parses += arg_name + f": " + parse
                enum += " }"
                parses += " }"
                intos += " }"
            parses += ",\n"
            intos += " => " + shifts
            intos += ",\n"

            output_file.write("    " + enum + ",\n")
    output_file.write("}\n\n")
    output_file.write("impl From<u32> for Instruction {\n")
    output_file.write("    fn from(value: u32) -> Self {\n")
    output_file.write("        match value >> 24 {\n")
    output_file.write(parses)
    output_file.write("            _ => Self::Invalid(value),\n")
    output_file.write("        }\n")
    output_file.write("    }\n")
    output_file.write("}\n\n")
    output_file.write("impl Into<u32> for Instruction {\n")
    output_file.write("    fn into(self) -> u32 {\n")
    output_file.write("        match self {\n")
    output_file.write(intos)
    output_file.write("            Self::Invalid(value) => value,\n")
    output_file.write("        }\n")
    output_file.write("    }\n")
    output_file.write("}\n\n")

output_file.flush()
output_file.close()