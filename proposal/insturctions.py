import json

def binary_digits(number, count):
  """Prints a number as a binary string with 3 digits, padding with leading zeros if necessary."""
  binary_string = bin(number)[2:]  # Convert to binary, remove "0b" prefix
  padded_binary = binary_string.zfill(count) #pad with leading zeros.
  return padded_binary


def generate_table(type_bin, opcode, bits, unused):
    table_str = """table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
"""
    for b in type_bin:
        table_str += f"[{b}], "

    for b in binary_digits(opcode, 5):
        table_str += f"[{b}], "

    for data in bits:
        table_str += f"table.cell(colspan: {data["count"]})[{data["short"]}], "

    table_str += "[\\*], " * unused

    table_str += "\ntable.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], "
    
    for data in bits:
        table_str += f"table.cell(colspan: {data["count"]})[{data["long"]}], "

    if unused > 0:
        table_str += f"table.cell(colspan: {unused})[Unused]"
    
    table_str += "\n)"
    return table_str



output_file = open("instructions.typ", "w")

with open('instructions.json') as f:
    data = json.load(f)
    for type_i, type_data in enumerate(data):
        type_bin = binary_digits(type_i, 3)
        type_name = type_data["name"]
        type_description = type_data["description"]
        output_file.write(f"== Type {type_bin} ({type_name}) \\\n")
        output_file.write(type_description)
        if type_description != "":
            output_file.write("\n")
        for opcode_i, opcode_data in enumerate(type_data["opcodes"]):
            opcode_hex = "0x{:02x}".format(opcode_i)
            opcode_name = opcode_data["name"]
            opcode_pneumonic = opcode_data["assembly"]["pneumonic"]
            for arg in opcode_data["assembly"]["arguments"]:
                
                opcode_pneumonic += arg["argument"]
            opcode_description = opcode_data["description"]
            opcode_bits = opcode_data["bits"]
            opcode_bit_remaining = 32-8
            for bit_data in opcode_bits:
                opcode_bit_remaining -= bit_data["count"]

            if opcode_bit_remaining < 0:
                print(f"Instruction {opcode_name} (type:{type_bin} {opcode_hex}) has too many bits")
                exit(1)

            output_file.write("#block(")
            output_file.write("breakable: false,")
            output_file.write("{text[")
            output_file.write(f"=== OPCODE: {opcode_hex} ({opcode_name}) \\\n")
            output_file.write(opcode_pneumonic + " \\\n")
            output_file.write(opcode_description + "]\n")
            output_file.write(generate_table(type_bin, opcode_i, opcode_bits, opcode_bit_remaining) + "}) \\\n")