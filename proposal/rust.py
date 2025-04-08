import json
import re

output_file = open("../simulator/src/instruction.rs", "w+")

with open('instructions.json') as f:
    output_file.write("use crate::{enums::{Condition, FPRegister, Register, Timer}, raw_cast_from_i32, raw_cast_to_i32, RegisterSet};\n\n")
    output_file.write("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n")
    output_file.write("pub enum Instruction {\n")
    output_file.write("    Invalid(u32),\n")
    data = json.load(f)
    parses = ""
    intos = ""
    read_regs = ""
    write_regs = ""
    for type_i, type_data in enumerate(data):
        for opcode_i, opcode_data in enumerate(type_data["opcodes"]):
            name = opcode_data["name"].replace(" ", "").replace("-", "") + ""
            enum = name
            bits = opcode_data["bits"]
            parses += "            0x{:02x} => Self::{}".format((type_i << 5) | opcode_i, name)
            intos += "            Self::{}".format(name)
            shifts = "0x{:02x} << 24".format((type_i << 5) | opcode_i)
            read_regs += "            Self::{}".format(name)
            write_regs += "            Self::{}".format(name)
            
            read_data = {"registers": [], "f_registers": [], "timers": []}
            write_data = {"registers": [], "f_registers": [], "timers": []}
            write_extra = ""


            if len(bits) > 0:
                enum += " { "
                parses += " { "
                intos += " { "
                shifts = f"({shifts})"
                read_regs += " { "
                write_regs += " { "
                opcode_bit_remaining = 32-8
                read = False
                write = False
                read_unused = False
                write_unused = False
                for i, bit_data in enumerate(bits):
                    opcode_bit_remaining -= bit_data["count"]
                    if (i > 0):
                        enum += ", "
                        parses += ", "
                        intos += ", "
                    
                    if read:
                        read_regs += ", "
                        read = False
                    if write:
                        write_regs += ", "
                        write = False
                    arg_name = bit_data["short"].replace("_", "").lower()
                    arg_type = "u32"
                    parse_value = "value"
                    into_value = arg_name

                    if re.match("R[a-z]", bit_data["short"]):
                        #int reg
                        arg_type = "Register"
                        if bit_data["read"]:
                            read_data["registers"].append("*" + arg_name)
                            read = True
                        if bit_data["write"]:
                            write_data["registers"].append("*" + arg_name)
                            write = True
                    elif re.match("F[a-z]", bit_data["short"]):
                        #float reg
                        arg_type = "FPRegister"
                        if bit_data["read"]:
                            read_data["f_registers"].append("*" + arg_name)
                            read = True
                        if bit_data["write"]:
                            write_data["f_registers"].append("*" + arg_name)
                            write = True
                    elif re.match("T[a-z]", bit_data["short"]):
                        #timer reg
                        arg_type = "Timer"
                        if bit_data["read"]:
                            read_data["timers"].append("*" + arg_name)
                            read = True
                        if bit_data["write"]:
                            write_data["timers"].append("*" + arg_name)
                            write = True
                    elif re.match("Condition", bit_data["short"]):
                        #condition code
                        arg_type = "Condition"
                    elif re.match("c", bit_data["short"]):
                        arg_type = "bool"
                        write = True
                        write_extra += "                if *c {\n"
                        write_extra += "                    registers.push(Register::ST);\n"
                        write_extra += "                }\n"
                    elif re.match("l", bit_data["short"]):
                        arg_type = "bool"
                        write = True
                        write_extra += "                if *l {\n"
                        write_extra += "                    registers.push(Register::LR);\n"
                        write_extra += "                }\n"
                    elif re.match("_offset_", bit_data["short"]):
                        arg_type = "i32"
                    
                    enum += arg_name + ": " + arg_type
                    intos += arg_name

                    if read:
                        read_regs += arg_name
                    else:
                        read_unused = True
                    
                    if write:
                        write_regs += arg_name
                    else:
                        write_unused = True

                    if arg_type == "i32":
                       into_value = f"raw_cast_from_i32({into_value})"
                    elif arg_type != "u32":
                       parse_value = "(value as usize)"
                       into_value = f"({into_value} as u32)"
                    
                    parse = f"({parse_value} >> {opcode_bit_remaining}) & 0x{(1 << bit_data["count"]) - 1:x}"
                    shifts += f"| ({into_value} << {opcode_bit_remaining})"
                    if arg_type == "bool":
                        parse = "(" + parse + " > 0)"
                    elif arg_type == "i32":
                        parse = f"raw_cast_to_i32({parse})"
                    elif arg_type != "u32":
                        parse = arg_type + "::try_from(" + parse + ").unwrap()"

                    parses += arg_name + f": " + parse
                enum += " }"
                parses += " }"
                intos += " }"

                if read_unused:
                    if read:
                        read_regs += ", "
                    read_regs += ".."
                read_regs += " }"

                
                if write_unused:
                    read = False
                    if write:
                        write_regs += ", "
                    write_regs += ".."
                write_regs += " }"
            
            for reg in opcode_data["extra_reg"]["registers"]:
                name = f"Register::try_from({reg["number"]}).unwrap()" 
                if reg["read"]:
                    read_data["registers"].append(name)
                if reg["write"]:
                    write_data["registers"].append(name)
            
            for reg in opcode_data["extra_reg"]["f_registers"]:
                name = f"FPRegister::try_from({reg["number"]}).unwrap()" 
                if reg["read"]:
                    read_data["f_registers"].append(name)
                if reg["write"]:
                    write_data["f_registers"].append(name)
            
            for reg in opcode_data["extra_reg"]["timers"]:
                name = f"Timer::try_from({reg["number"]}).unwrap()" 
                if reg["read"]:
                    read_data["timers"].append(name)
                if reg["write"]:
                    write_data["timers"].append(name)
                
            parses += ",\n"
            intos += " => " + shifts
            intos += ",\n"
            read_regs += " => " + f"RegisterSet{{ registers: vec![{", ".join(read_data["registers"])}], f_registers: vec![{", ".join(read_data["f_registers"])}], timers: vec![{", ".join(read_data["timers"])}]  }}"
            read_regs += ",\n"
            write_regs += " => {\n"
            write_regs += f"                let {"mut " if len(write_extra) > 0 else "" }registers = vec![{", ".join(write_data["registers"])}];\n"
            write_regs += write_extra
            write_regs += f"                RegisterSet{{ registers, f_registers: vec![{", ".join(write_data["f_registers"])}], timers: vec![{", ".join(write_data["timers"])}] }}\n"
            write_regs += "            },\n"

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
    output_file.write("impl Instruction {\n")
    output_file.write("    pub fn read_registers(&self) -> RegisterSet {\n")
    output_file.write("        match self {\n")
    output_file.write(read_regs)
    output_file.write("            Self::Invalid(_value) => Default::default(),\n")
    output_file.write("        }\n")
    output_file.write("    }\n")
    output_file.write("    pub fn write_registers(&self) -> RegisterSet {\n")
    output_file.write("        match self {\n")
    output_file.write(write_regs)
    output_file.write("            Self::Invalid(_value) => Default::default(),\n")
    output_file.write("        }\n")
    output_file.write("    }\n")
    output_file.write("}\n\n")

output_file.flush()
output_file.close()