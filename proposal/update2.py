import json
import re

with open('instructions.json') as f:
    data = json.load(f)
    for type_i, type_data in enumerate(data):
        for opcode_i, opcode_data in enumerate(type_data["opcodes"]):
            extra = {
                    "mnemonic": "",
                    "arguments": [],
            }

            full: list[str] = opcode_data["assembly"]["full"].split(" ")
            extra["mnemonic"] = full[0]
            for i, arg in enumerate(full[1:]):
                argument = {
                    "type": "",
                    "argument": "",
                }
                if arg.startswith("R"):
                    argument["type"] = "register"
                    argument["argument"] = arg
                elif arg.startswith("F"):
                    argument["type"] = "f_register"
                    argument["argument"] = arg
                elif arg.startswith("T"):
                    argument["type"] = "timer"
                    argument["argument"] = arg
                elif arg == "[Rx+Ro<<S]":
                    argument["type"] = "data_shift_register"
                    argument["argument"] = "Rx"
                elif arg == "[Rx+I<<S]":
                    argument["type"] = "data_shift_imm"
                    argument["argument"] = "Rx"
                elif arg == "[Ry+Ro<<S]":
                    argument["type"] = "data_shift_register"
                    argument["argument"] = "Ry"
                elif arg == "[Ry+I<<S]":
                    argument["type"] = "data_shift_imm"
                    argument["argument"] = "Ry"
                elif arg == "P[Rx+Ro<<S]":
                    argument["type"] = "prog_shift_register"
                    argument["argument"] = "Rx"
                elif arg == "P[Rx+I<<S]":
                    argument["type"] = "prog_shift_imm"
                    argument["argument"] = "Rx"
                elif arg == "P[Ry+Ro<<S]":
                    argument["type"] = "prog_shift_register"
                    argument["argument"] = "Ry"
                elif arg == "P[Ry+I<<S]":
                    argument["type"] = "prog_shift_imm"
                    argument["argument"] = "Ry"
                elif arg == "_label_":
                    argument["type"] = "address"
                    argument["argument"] = "_label_"
                elif arg == "_offset_":
                    argument["type"] = "number"
                    argument["argument"] = "_offset_"
                elif arg == "_count_":
                    argument["type"] = "number"
                    argument["argument"] = "_count_"
                elif arg == "_value_":
                    argument["type"] = "number"
                    argument["argument"] = "_value_"
                else:
                    print(full[0], arg)
                
                extra["arguments"].append(argument)

            data[type_i]["opcodes"][opcode_i]["assembly"] = extra

    output_file = open("instructions2.json", "w+")
    json.dump(data, output_file)