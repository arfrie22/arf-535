import json
import re

output_file = open("instructions.json", "w+")

with open('instructions.json') as f:
    data = json.load(f)
    for type_i, type_data in enumerate(data):
        for opcode_i, opcode_data in enumerate(type_data["opcodes"]):
            extra = {
                    "registers": [],
                    "f_registers": [],
                    "timers": [],
            }
            if type_data["name"] == "Branch":
                extra["registers"].append({"number": 28, "read": True, "write": False})
            
            if "Compare" in opcode_data["name"]:
                extra["registers"].append({"number": 28, "read": False, "write": True})

            data[type_i]["opcodes"][opcode_i]["extra_reg"] = extra
            for bit_i, bit_data in enumerate(opcode_data["bits"]):
                long_name = bit_data["long"]
                if "Timer" in long_name or "Register" in long_name:
                    if "Source" in long_name or "Offset" in long_name:
                        data[type_i]["opcodes"][opcode_i]["bits"][bit_i]["read"] = True
                        data[type_i]["opcodes"][opcode_i]["bits"][bit_i]["write"] = False
                    elif "Dest" in long_name:
                        data[type_i]["opcodes"][opcode_i]["bits"][bit_i]["read"] = False
                        data[type_i]["opcodes"][opcode_i]["bits"][bit_i]["write"] = True
                    else:
                        data[type_i]["opcodes"][opcode_i]["bits"][bit_i]["read"] = True
                        data[type_i]["opcodes"][opcode_i]["bits"][bit_i]["write"] = True
    json.dump(data, output_file)