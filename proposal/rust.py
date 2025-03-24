import json
import re

output_file = open("../simulator/src/instruction.rs", "w+")

with open('instructions.json') as f:
    output_file.write("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n")
    output_file.write("pub enum Instruction {\n")
    output_file.write("    Invalid(u32),\n")
    data = json.load(f)
    parses = ""
    for type_i, type_data in enumerate(data):
        for opcode_i, opcode_data in enumerate(type_data["opcodes"]):
            name = opcode_data["name"].replace(" ", "").replace("-", "") + ""
            enum = name
            opcode_pneumonic = opcode_data["pneumonic"]
            bits = opcode_data["bits"]
            parses += "            0x{:02x} => Self::{}".format((type_i << 5) | opcode_i, name)

            if len(bits) > 0:
                enum += " { "
                parses += " { "
                opcode_bit_remaining = 32-8
                for i, bit_data in enumerate(bits):
                    opcode_bit_remaining -= bit_data["count"]
                    if (i > 0):
                        enum += ", "
                        parses += ", "
                    arg_name = bit_data["short"].replace("_", "").lower()
                    t = "u32"
                    v = "value"
                    if re.match("R[a-z]", bit_data["short"]):
                        #int reg
                        t = "Registers"
                    elif re.match("F[a-z]", bit_data["short"]):
                        #float reg
                        t = "FloatingPointRegisters"
                    
                    enum += arg_name + ": " + t

                    if t != "u32":
                       v = "(value as usize)"
                    p = f"({v} >> {opcode_bit_remaining}) & 0x{(1 << bit_data["count"]) - 1:x}"
                    if t != "u32":
                        p = t + "::try_from(" + p + ").unwrap()"

                    parses += arg_name + f": " + p
                enum += " }"
                parses += " }"
            parses += ",\n"

            output_file.write("    " + enum + ",\n")
    output_file.write("}\n\n")
    output_file.write("impl From<u32> for Instructions {\n")
    output_file.write("    fn from(value: u32) -> Self {\n")
    output_file.write("        match value >> 24 {\n")
    output_file.write(parses)
    output_file.write("            _ => Self::Invalid(value),\n")
    output_file.write("        }\n")
    output_file.write("    }\n")
    output_file.write("}\n\n")

output_file.flush()
output_file.close()