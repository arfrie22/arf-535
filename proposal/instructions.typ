== Type 000 (General) \
#block(breakable: false,{text[=== OPCODE: 0x00 (Trap) \
TRAP \
Used in simulator to pause execution, and return control to the simulator's UI.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [0], [0], [0], [0], [0], [0], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 24)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x01 (Push Integer Register) \
PUSH Rx \
Pushes the value of Rx onto the stack (SP) and increase SP by 1.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [0], [0], [0], [0], [0], [1], table.cell(colspan: 5)[Rx], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Source Register], table.cell(colspan: 19)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x02 (Push Floating-Point Register) \
PUSH Fx \
Pushes the value of Fx onto the stack (SP) and increase SP by 1.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [0], [0], [0], [0], [1], [0], table.cell(colspan: 5)[Fx], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Source Register], table.cell(colspan: 19)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x03 (Pop Integer Register) \
POP Rx \
Pops the top value from the stack (SP) into Rx and decrease SP by 1]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [0], [0], [0], [0], [1], [1], table.cell(colspan: 5)[Rx], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 19)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x04 (Pop Floating-Point Register) \
POP Fx \
Pops the top value from the stack (SP) into Fx and decrease SP by 1]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [0], [0], [0], [1], [0], [0], table.cell(colspan: 5)[Fx], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 19)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x05 (Swap Register) \
SWP Rx Fy \
Swaps the integer register Rx with the value of the floating-point register Fy.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [0], [0], [0], [1], [0], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Fy], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Integer Register], table.cell(colspan: 5)[Floating-Point Register], table.cell(colspan: 14)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x06 (Stall Immediate) \
STALL _value_ \
Stalls the pipeline at execute for _value_ clock cycles.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [0], [0], [0], [1], [1], [0], table.cell(colspan: 16)[_value_], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 16)[Cycle Count], table.cell(colspan: 8)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x07 (Stall Register) \
STALL Rx \
Stalls the pipeline at execute for Rx clock cycles.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [0], [0], [0], [1], [1], [1], table.cell(colspan: 5)[Rx], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Source Register], table.cell(colspan: 19)[Unused]
)}) \
== Type 001 (Branch) \
The condition code for a branch is testing a bit inside the Status Register, consult the table to see the short name. In the ASM write the instruction as (B).(COND) where (B) is the branch instruction and (COND) is the short condition code.
#block(breakable: false,{text[=== OPCODE: 0x00 (Register Jump) \
B l Condition Rx \
Sets the program counter to Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [1], [0], [0], [0], [0], [0], table.cell(colspan: 1)[l], table.cell(colspan: 5)[Condition], table.cell(colspan: 5)[Rx], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Link Register Bit], table.cell(colspan: 5)[Condition code bit], table.cell(colspan: 5)[Source Register], table.cell(colspan: 13)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x01 (Indirect Jump) \
B l Condition Rx \
Sets the program counter to the value stored at the memory location in Rx offset by I shifted by S.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [1], [0], [0], [0], [0], [1], table.cell(colspan: 1)[l], table.cell(colspan: 5)[Condition], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[I], table.cell(colspan: 4)[S], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Link Register Bit], table.cell(colspan: 5)[Condition code bit], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Offset], table.cell(colspan: 4)[Shift], table.cell(colspan: 4)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x02 (Indirect with Register Offset Jump) \
B l Condition Rx \
Sets the program counter to the value stored at the memory location in Rx offset by the value of Ro shifted by S.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [1], [0], [0], [0], [1], [0], table.cell(colspan: 1)[l], table.cell(colspan: 5)[Condition], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ro], table.cell(colspan: 4)[S], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Link Register Bit], table.cell(colspan: 5)[Condition code bit], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Offset Register], table.cell(colspan: 4)[Shift], table.cell(colspan: 4)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x03 (Relative Jump) \
BR l Condition Rx \
Sets the program counter to (PC + Rx) where Rx is interpreted as a signed 32-bit integer.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [1], [0], [0], [0], [1], [1], table.cell(colspan: 1)[l], table.cell(colspan: 5)[Condition], table.cell(colspan: 5)[Rx], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Link Register Bit], table.cell(colspan: 5)[Condition code bit], table.cell(colspan: 5)[Source Register], table.cell(colspan: 13)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x04 (Immediate Jump) \
B l Condition _label_ \
Sets the program counter to the address of the label.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [1], [0], [0], [1], [0], [0], table.cell(colspan: 1)[l], table.cell(colspan: 5)[Condition], table.cell(colspan: 16)[_label_], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Link Register Bit], table.cell(colspan: 5)[Condition code bit], table.cell(colspan: 16)[Destination Address], table.cell(colspan: 2)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x05 (Immediate Relative Jump) \
BO l Condition _offset_ \
Adds the offset sign exteneded to 32-bits to the program counter (offset treated as signed 16-bit number).]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [1], [0], [0], [1], [0], [1], table.cell(colspan: 1)[l], table.cell(colspan: 5)[Condition], table.cell(colspan: 16)[_offset_], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Link Register Bit], table.cell(colspan: 5)[Condition code bit], table.cell(colspan: 16)[Signed Offset], table.cell(colspan: 2)[Unused]
)}) \
== Type 010 (Integer Register) \
#block(breakable: false,{text[=== OPCODE: 0x00 (Integer Load Low) \
LDL Rx _value_ \
Loads the 16-bit immediate value into the lower 16 bits of register Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [0], [0], [0], [0], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 16)[_value_], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 16)[Immediate Value], table.cell(colspan: 3)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x01 (Integer Load High) \
LDH Rx _value_ \
Loads the 16-bit immediate value into the higher 16 bits of register Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [0], [0], [0], [0], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 16)[_value_], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 16)[Immediate Value], table.cell(colspan: 3)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x02 (Swap Integer Registers) \
SWP Rx Ry \
Swaps the values of Rx and Ry.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [0], [0], [0], [1], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[First Register], table.cell(colspan: 5)[Second Register], table.cell(colspan: 14)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x03 (Copy Integer Register) \
LDR Rx Ry \
Sets the value of Rx to the value of Ry.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [0], [0], [0], [1], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 14)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x04 (Load Integer Register Indirect) \
LDR Rx Ry \
Sets the value of Rx to the value of stored at the data memory location at Ry offset by I shifted by S.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [0], [0], [1], [0], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[I], table.cell(colspan: 4)[S], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Offset], table.cell(colspan: 4)[Shift], table.cell(colspan: 5)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x05 (Load Integer Register Indirect with Register Offset) \
LDR Rx Ry \
Sets the value of Rx to the value of stored at the data memory location at Ry offset by the value of Ro shifted by S.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [0], [0], [1], [0], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Ro], table.cell(colspan: 4)[S], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Offset], table.cell(colspan: 4)[Shift], table.cell(colspan: 5)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x06 (Load Integer Register Indirect Program) \
LDR Rx Ry \
Sets the value of Rx to the value of stored at the program memory location at Ry offset by I shifted by S.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [0], [0], [1], [1], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[I], table.cell(colspan: 4)[S], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Offset], table.cell(colspan: 4)[Shift], table.cell(colspan: 5)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x07 (Load Integer Register Indirect with Register Offset Program) \
LDR Rx Ry \
Sets the value of Rx to the value of stored at the program memory location at Ry offset by the value of Ro shifted by S.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [0], [0], [1], [1], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Ro], table.cell(colspan: 4)[S], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Offset Register], table.cell(colspan: 4)[Shift], table.cell(colspan: 5)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x08 (Store Integer Register Indirect) \
STR Rx Ry \
Stores the value of Ry into the data memory location at Rx offset by I shifted by S.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [0], [1], [0], [0], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[I], table.cell(colspan: 4)[S], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Offset], table.cell(colspan: 4)[Shift], table.cell(colspan: 5)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x09 (Store Integer Register Indirect with Register Offset Indirect) \
STR Rx Ry \
Stores the value of Ry into the data memory location at Rx offset by the value of Ro shifted by S.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [0], [1], [0], [0], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Ro], table.cell(colspan: 4)[S], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Offset Register], table.cell(colspan: 4)[Shift], table.cell(colspan: 5)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x0a (Store Integer Register Indirect Program) \
STR Rx Ry \
Stores the value of Ry into the program memory location at Rx offset by I shifted by S.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [0], [1], [0], [1], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[I], table.cell(colspan: 4)[S], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Offset], table.cell(colspan: 4)[Shift], table.cell(colspan: 5)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x0b (Store Integer Register Indirect with Register Offset Program) \
STR Rx Ry \
Stores the value of Ry into the program memory location at Rx offset by the value of Ro shifted by S.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [0], [1], [0], [1], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Ro], table.cell(colspan: 4)[S], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Offset Register], table.cell(colspan: 4)[Shift], table.cell(colspan: 5)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x0c (Integer Load Data) \
LDR Rx _label_ \
Loads the value at data memory address label into the register Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [0], [1], [1], [0], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 16)[_label_], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 16)[Source Data Address], table.cell(colspan: 3)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x0d (Integer Load Program) \
LDR Rx _label_ \
Loads the value at program memory address label into the register Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [0], [1], [1], [0], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 16)[_label_], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 16)[Source Program Address], table.cell(colspan: 3)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x0e (Integer Store Data) \
STR Rx _label_ \
Stores the value in Rx into the data memory address label.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [0], [1], [1], [1], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 16)[_label_], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Source Register], table.cell(colspan: 16)[Destination Data Address], table.cell(colspan: 3)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x0f (Integer Store Program) \
STR Rx _label_ \
Stores the value in Rx into the program memory address label.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [0], [1], [1], [1], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 16)[_label_], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Source Register], table.cell(colspan: 16)[Destination Program Address], table.cell(colspan: 3)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x10 (Integer Load Effective Data Address) \
LEA Rx _label_ \
Loads the address of data memory address label into the register Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [1], [0], [0], [0], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 16)[_label_], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 16)[Address in data], table.cell(colspan: 3)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x11 (Integer Load Effective Program Address) \
LEA Rx _label_ \
Loads the address of program memory address label into the register Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [1], [0], [0], [0], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 16)[_label_], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 16)[Address in program], table.cell(colspan: 3)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x12 (Load Integer Effective Address Register Indirect) \
LEA Rx Ry \
Sets the value of Rx to the memory location at Ry offset by I shifted by S.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [1], [0], [0], [1], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[I], table.cell(colspan: 4)[S], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Offset], table.cell(colspan: 4)[Shift], table.cell(colspan: 5)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x13 (Load Integer Effective Address Register Indirect with Register Offset) \
LEA Rx Ry \
Sets the value of Rx to the memory location at Ry offset by the value of Ro shifted by S.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [1], [0], [0], [1], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Ro], table.cell(colspan: 4)[S], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Offset], table.cell(colspan: 4)[Shift], table.cell(colspan: 5)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x14 (Unsigned Zero Extend) \
ZEX Rx Ry _count_ \
Stores the value of Ry into Rx where the top _count_ bits are zero.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [1], [0], [1], [0], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[_count_], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Bit Count], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x15 (Sign Extend) \
SEX Rx Ry _count_ \
Stores the value of Ry into Rx where the top _count_ bits are sign extended.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [1], [0], [1], [0], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[_count_], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Bit Count], table.cell(colspan: 9)[Unused]
)}) \
== Type 011 (Floating-Point Register) \
#block(breakable: false,{text[=== OPCODE: 0x00 (Floating-Point Load Low) \
LDL Fx _value_ \
Loads the 16-bit immediate value into the lower 16 bits of register Fx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [1], [0], [0], [0], [0], [0], table.cell(colspan: 5)[Fx], table.cell(colspan: 16)[_value_], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 16)[Immediate Value], table.cell(colspan: 3)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x01 (Floating-Point Load High) \
LDH Fx _value_ \
Loads the 16-bit immediate value into the higher 16 bits of register Fx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [1], [0], [0], [0], [0], [1], table.cell(colspan: 5)[Fx], table.cell(colspan: 16)[_value_], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 16)[Immediate Value], table.cell(colspan: 3)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x02 (Swap Floating-Point Registers) \
SWP Fx Fy \
Swaps the values of Fx and Fy.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [1], [0], [0], [0], [1], [0], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Fy], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[First Register], table.cell(colspan: 5)[Second Register], table.cell(colspan: 14)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x03 (Copy Floating-Point Register) \
LDR Fx Fy \
Sets the value of Fx to the value of Fy.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [1], [0], [0], [0], [1], [1], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Fy], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 14)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x04 (Load Floating-Point Register Indirect) \
LDR Fx Ry \
Sets the value of Fx to the value of stored at the data memory location at Ry offset by I shifted by S.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [1], [0], [0], [1], [0], [0], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[I], table.cell(colspan: 4)[S], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Offset], table.cell(colspan: 4)[Shift], table.cell(colspan: 5)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x05 (Load Floating-Point Register Indirect with Register Offset) \
LDR Fx Ry \
Sets the value of Fx to the value of stored at the data memory location at Ry offset by the value of Ro shifted by S.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [1], [0], [0], [1], [0], [1], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Ro], table.cell(colspan: 4)[S], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Offset], table.cell(colspan: 4)[Shift], table.cell(colspan: 5)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x06 (Store Floating-Point Register Indirect) \
STR Rx Fy \
Stores the value of Fy into the data memory location at Rx offset by I shifted by S.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [1], [0], [0], [1], [1], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Fy], table.cell(colspan: 5)[I], table.cell(colspan: 4)[S], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Offset], table.cell(colspan: 4)[Shift], table.cell(colspan: 5)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x07 (Store Floating-Point Register Indirect with Register Offset) \
STR Rx Fy \
Stores the value of Fy into the data memory location at Rx offset by the value of Ro shifted by S.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [1], [0], [0], [1], [1], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Fy], table.cell(colspan: 5)[Ro], table.cell(colspan: 4)[S], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Offset Register], table.cell(colspan: 4)[Shift], table.cell(colspan: 5)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x08 (Floating-Point Load Data) \
LDR Fx _label_ \
Loads the value at data memory address label into the register Fx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [1], [0], [1], [0], [0], [0], table.cell(colspan: 5)[Fx], table.cell(colspan: 16)[_label_], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 16)[Destination Address in data], table.cell(colspan: 3)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x09 (Floating-Point Store Data) \
STR Fx _label_ \
Stores the value in Fx into the data memory address label.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [1], [0], [1], [0], [0], [1], table.cell(colspan: 5)[Fx], table.cell(colspan: 16)[_label_], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Source Register], table.cell(colspan: 16)[Destination Address in data], table.cell(colspan: 3)[Unused]
)}) \
== Type 100 (Integer ALU) \
#block(breakable: false,{text[=== OPCODE: 0x00 (Integer Compare) \
CMP Rx Ry \
Stores the condition variables.
- Sets EQ if $"Rx" = "Ry"$
- Sets GT if $"Rx" > "Ry"$
- Sets LT if $"Rx" < "Ry"$
- Sets GE if $"Rx" >= "Ry"$
- Sets LE if $"Rx" <= "Ry"$]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [0], [0], [0], [0], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 14)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x01 (Integer Compare Single Against Zero) \
CMP Rx \
Stores the condition variables.
- Sets EQ if $"Rx" = 0$
- Sets GT if $"Rx" > 0$
- Sets LT if $"Rx" < 0$
- Sets GE if $"Rx" >= 0$
- Sets LE if $"Rx" <= 0$
- Sets EVEN if $"Rx" % 2 = 0$]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [0], [0], [0], [0], [1], table.cell(colspan: 5)[Rx], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Source Register], table.cell(colspan: 19)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x02 (Increment Integer Register) \
INC c Rx \
Sets Rx := Rx + 1.\
Sets overflow bit if $"Rx" + "1" >= 2^32$]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [0], [0], [0], [1], [0], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Rx], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Register], table.cell(colspan: 18)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x03 (Decrement Integer Register) \
DEC c Rx \
Sets Rx := Rx - 1.\
Sets underflow bit if $"Rx" - "1" < 0$]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [0], [0], [0], [1], [1], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Rx], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Register], table.cell(colspan: 18)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x04 (Add Unsigned Integer) \
ADD c Rx Ry Rz \
Sets Rx := Ry + Rz.\
Sets overflow bit if $"Ry" + "Rz" >= 2^32$]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [0], [0], [1], [0], [0], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 8)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x05 (Subtract Unsigned Integer) \
SUB c Rx Ry Rz \
Sets Rx := Ry - Rz.\
Sets underflow bit if $"Ry" - "Rz" < 0$]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [0], [0], [1], [0], [1], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 8)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x06 (Multiply Unsigned Integer) \
MUL c Rx Ry Rz \
Sets Rx := Ry \* Rz.\
Sets overflow bit if $"Ry" times "Rz" >= 2^32$]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [0], [0], [1], [1], [0], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 8)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x07 (Divide Unsigned Integer) \
DIV c Rx Ry Rz \
Sets Rx := Ry / Rz.\
Sets divide by zero bit if $"Rz" = 0$]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [0], [0], [1], [1], [1], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 8)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x08 (Modulo Unsigned Integer) \
MOD c Rx Ry Rz \
Sets Rx := Ry % Rz.\
Sets divide by zero bit if $"Rz" = 0$]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [0], [1], [0], [0], [0], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 8)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x09 (Add Signed Integer) \
ADDS c Rx Ry Rz \
Sets Rx := Ry + Rz (treats all registers as signed).\
Sets overflow bit if $"Ry" + "Rz" >= 2^31 - 1$. Sets underflow bit if $"Ry" + "Rz" < -2^31$.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [0], [1], [0], [0], [1], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 8)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x0a (Subtract Signed Integer) \
SUBS c Rx Ry Rz \
Sets Rx := Ry - Rz (treats all registers as signed).\
Sets overflow bit if $"Ry" - "Rz" >= 2^31 - 1$. Sets underflow bit if $"Ry" - "Rz" < -2^31$.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [0], [1], [0], [1], [0], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 8)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x0b (Multiply Signed Integer) \
MULS c Rx Ry Rz \
Sets Rx := Ry \* Rz (treats all registers as signed).\
Sets overflow bit if $"Ry" times "Rz" >= 2^31 - 1$. Sets underflow bit if $"Ry" times "Rz" < -2^31$.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [0], [1], [0], [1], [1], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 8)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x0c (Divide Signed Integer) \
DIVS c Rx Ry Rz \
Sets Rx := Ry / Rz (treats all registers as signed).\
Sets divide by zero bit if $"Rz" = 0$]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [0], [1], [1], [0], [0], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 8)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x0d (Modulo Signed Integer) \
MODS c Rx Ry Rz \
Sets Rx := Ry % Rz (treats all registers as signed).\
Sets divide by zero bit if $"Rz" = 0$.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [0], [1], [1], [0], [1], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 8)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x0e (Bitwise AND) \
AND Rx Ry Rz \
Sets Rx := Ry & Rz.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [0], [1], [1], [1], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x0f (Bitwise OR) \
OR Rx Ry Rz \
Sets Rx := Ry | Rz]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [0], [1], [1], [1], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x10 (Bitwise NOT) \
NOT Rx Ry \
Sets Rx := ~Ry]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [1], [0], [0], [0], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 14)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x11 (Bitwise XOR) \
XOR Rx Ry Rz \
Sets Rx := Ry ^ Rz]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [1], [0], [0], [0], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x12 (Logical Shift Left) \
LSL Rx Ry _value_ \
Shifts Ry left by value bits, where 0 is the new value on the right and the extra bit falls off. Value stored in Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [1], [0], [0], [1], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[_value_], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Shift Amount], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x13 (Logical Shift Right) \
LSR Rx Ry _value_ \
Shifts Ry right by value bits, where 0 is the new value on the left and the extra bit falls off. Value stored in Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [1], [0], [0], [1], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[_value_], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Shift Amount], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x14 (Arithmetic Shift Left) \
ASL Rx Ry _value_ \
Shifts Ry left by value bits, where 0 is the new value on the right and only up to the 31st bit, the 32nd bit remains untouched. Value stored in Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [1], [0], [1], [0], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[_value_], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Shift Amount], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x15 (Arithmetic Shift Right) \
ASR Rx Ry _value_ \
Shifts Ry right by value bits, where the 32nd bit is now the value of what it was previously and the extra bit falls off. Value stored in Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [1], [0], [1], [0], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[_value_], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Shift Amount], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x16 (Rotate Right) \
RTR Rx Ry _value_ \
Shifts Ry right by value bits, the bit that falls off gets shifted back in. Value stored in Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [1], [0], [1], [1], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[_value_], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Shift Amount], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x17 (Logical Shift Left Register) \
LSL Rx Ry Rz \
Shifts Ry left by value bits, where 0 is the new value on the right and the extra bit falls off. Value stored in Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [1], [0], [1], [1], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x18 (Logical Shift Right Register) \
LSR Rx Ry Rz \
Shifts Ry right by value bits, where 0 is the new value on the left and the extra bit falls off. Value stored in Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [1], [1], [0], [0], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x19 (Arithmetic Shift Left Register) \
ASL Rx Ry Rz \
Shifts Ry left by value bits, where 0 is the new value on the right and only up to the 31st bit, the 32nd bit remains untouched. Value stored in Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [1], [1], [0], [0], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x1a (Arithmetic Shift Right Register) \
ASR Rx Ry Rz \
Shifts Ry right by value bits, where the 32nd bit is now the value of what it was previously and the extra bit falls off. Value stored in Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [1], [1], [0], [1], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x1b (Rotate Right Register) \
RTR Rx Ry Rz \
Shifts Ry right by value bits, the bit that falls off gets shifted back in. Value stored in Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [1], [1], [0], [1], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x1c (Map Unsigned To Signed) \
MUS Rx Ry \
Equivalent of storing (Ry-0x80000000) into Rx without loosing any bits due to casting due to signed numbers having one less bit.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [1], [1], [1], [0], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 14)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x1d (Map Signed To Unsigned) \
MSU Rx Ry \
Equivalent of storing (Ry+0x80000000) into Rx without loosing any bits due to casting due to signed numbers having one less bit.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [1], [1], [1], [0], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 14)[Unused]
)}) \
== Type 101 (Floating-Point ALU) \
#block(breakable: false,{text[=== OPCODE: 0x00 (Floating-Point Compare) \
CMP Fx Fy \
Stores the condition variables.
- Sets EQ if $"Fx" = "Fy"$
- Sets GT if $"Fx" > "Fy"$
- Sets LT if $"Fx" < "Fy"$
- Sets GE if $"Fx" >= "Fy"$
- Sets LE if $"Fx" <= "Fy"$]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [1], [0], [0], [0], [0], [0], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Fy], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 14)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x01 (Floating-Point Compare Single Against Zero) \
CMP Fx \
Stores the condition variables.
- Sets EQ if $"Fx" = 0$
- Sets GT if $"Fx" > 0$
- Sets LT if $"Fx" < 0$
- Sets GE if $"Fx" >= 0$
- Sets LE if $"Fx" <= 0$
- Sets FINF if $"Fx" = plus.minus infinity$
- Sets FZ if $"Fx" = plus.minus 0$
- Sets FNAN if $"Fx" = "NAN"$]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [1], [0], [0], [0], [0], [1], table.cell(colspan: 5)[Fx], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Source Register], table.cell(colspan: 19)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x02 (Add Floating-Point) \
ADD c Fx Fy Fz \
Sets Fx := Fy + Fz.\
Sets overflow bit if result is outside of floating-point range. Sets underflow bit if result is subnormal. Sets FZ bit if result $= plus.minus 0$. Sets FINF bit if result $= plus.minus infinity$. Sets FNAN bit if result is not-a-number.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [1], [0], [0], [0], [1], [0], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Fy], table.cell(colspan: 5)[Fz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 8)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x03 (Subtract Floating-Point) \
SUB c Fx Fy Fz \
Sets Fx := Fy - Fz.\
Sets overflow bit if result is outside of floating-point range. Sets underflow bit if result is subnormal. Sets FZ bit if result $= plus.minus 0$. Sets FINF bit if result $= plus.minus infinity$. Sets FNAN bit if result is not-a-number.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [1], [0], [0], [0], [1], [1], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Fy], table.cell(colspan: 5)[Fz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 8)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x04 (Multiply Floating-Point) \
MUL c Fx Fy Fz \
Sets Fx := Fy \* Fz.\
Sets overflow bit if result is outside of floating-point range. Sets underflow bit if result is subnormal. Sets FZ bit if result $= plus.minus 0$. Sets FINF bit if result $= plus.minus infinity$. Sets FNAN bit if result is not-a-number.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [1], [0], [0], [1], [0], [0], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Fy], table.cell(colspan: 5)[Fz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 8)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x05 (Divide Floating-Point) \
DIV c Fx Fy Fz \
Sets Fx := Fy / Fz.\
Sets overflow bit if result is outside of floating-point range. Sets underflow bit if result is subnormal. Sets FZ bit if result $= plus.minus 0$. Sets FINF bit if result $= plus.minus infinity$. Sets FNAN bit if result is not-a-number.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [1], [0], [0], [1], [0], [1], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Fy], table.cell(colspan: 5)[Fz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 8)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x06 (Cast To Float) \
CST c Fx Ry \
Converts the value of Ry into a float and stores it in Fx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [1], [0], [0], [1], [1], [0], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Ry], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 13)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x07 (Cast From Float) \
CST c Rx Fy \
Converts the value of Fy into a signed integer (rounded towards zero) and stores it in Rx.\
Sets OVRF if $"Fy" > 2^31 - 1$. Sets UNDF if $"Fy" < -2^31$.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [1], [0], [0], [1], [1], [1], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Fy], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 13)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x08 (Negate Floating-Point) \
NEG c Fx Fy \
Sets Fx := -Fy.\
Sets overflow bit if result is outside of floating-point range. Sets underflow bit if result is subnormal. Sets FZ bit if result $= plus.minus 0$. Sets FINF bit if result $= plus.minus infinity$. Sets FNAN bit if result is not-a-number.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [1], [0], [1], [0], [0], [0], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Fy], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 13)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x09 (Absolute Value Floating-Point) \
ABS c Fx Fy \
Sets Fx := |Fy|.\
Sets overflow bit if result is outside of floating-point range. Sets underflow bit if result is subnormal. Sets FZ bit if result $= plus.minus 0$. Sets FINF bit if result $= plus.minus infinity$. Sets FNAN bit if result is not-a-number.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [1], [0], [1], [0], [0], [1], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Fy], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 13)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x0a (Round Floating-Point) \
RND c Fx Fy \
Sets Fx to Fy rounded to the nearest integer.\
Sets overflow bit if result is outside of floating-point range. Sets underflow bit if result is subnormal. Sets FZ bit if result $= plus.minus 0$. Sets FINF bit if result $= plus.minus infinity$. Sets FNAN bit if result is not-a-number.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [1], [0], [1], [0], [1], [0], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Fy], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 13)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x0b (Round To Zero Floating-Point) \
RNDZ c Fx Fy \
Sets Fx := trunc(Fy).\
Sets overflow bit if result is outside of floating-point range. Sets underflow bit if result is subnormal. Sets FZ bit if result $= plus.minus 0$. Sets FINF bit if result $= plus.minus infinity$. Sets FNAN bit if result is not-a-number.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [1], [0], [1], [0], [1], [1], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Fy], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 13)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x0c (Round To Infinity Floating-Point) \
RNDI c Fx Fy \
Sets Fx := trunc(Fy) + sgn(Fy).\
Sets overflow bit if result is outside of floating-point range. Sets underflow bit if result is subnormal. Sets FZ bit if result $= plus.minus 0$. Sets FINF bit if result $= plus.minus infinity$. Sets FNAN bit if result is not-a-number.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [1], [0], [1], [1], [0], [0], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Fy], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 13)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x0d (Square Root Floating-Point) \
ABS c Fx Fy \
Sets Fx := sqrt(Fy).\
Sets overflow bit if result is outside of floating-point range. Sets underflow bit if result is subnormal. Sets FZ bit if result $= plus.minus 0$. Sets FINF bit if result $= plus.minus infinity$. Sets FNAN bit if result is not-a-number.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [1], [0], [1], [1], [0], [1], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Fy], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 13)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x0e (Log Base 10 Floating-Point) \
LOG c Fx Fy \
Sets Fx := log(Fy).\
Sets overflow bit if result is outside of floating-point range. Sets underflow bit if result is subnormal. Sets FZ bit if result $= plus.minus 0$. Sets FINF bit if result $= plus.minus infinity$. Sets FNAN bit if result is not-a-number.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [1], [0], [1], [1], [1], [0], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Fy], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 13)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x0f (Log Natrual Floating-Point) \
LN c Fx Fy \
Sets Fx := ln(Fy).\
Sets overflow bit if result is outside of floating-point range. Sets underflow bit if result is subnormal. Sets FZ bit if result $= plus.minus 0$. Sets FINF bit if result $= plus.minus infinity$. Sets FNAN bit if result is not-a-number.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [1], [0], [1], [1], [1], [1], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Fy], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 13)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x10 (Exponential Floating-Point) \
EXP c Fx Fy \
Sets Fx := e^{Fy}.\
Sets overflow bit if result is outside of floating-point range. Sets underflow bit if result is subnormal. Sets FZ bit if result $= plus.minus 0$. Sets FINF bit if result $= plus.minus infinity$. Sets FNAN bit if result is not-a-number.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [1], [1], [0], [0], [0], [0], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Fy], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 13)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x11 (Sine Floating-Point) \
SIN c Fx Fy \
Sets Fx := sin(Fy).\
Sets overflow bit if result is outside of floating-point range. Sets underflow bit if result is subnormal. Sets FZ bit if result $= plus.minus 0$. Sets FINF bit if result $= plus.minus infinity$. Sets FNAN bit if result is not-a-number.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [1], [1], [0], [0], [0], [1], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Fy], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 13)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x12 (Cosine Floating-Point) \
COS c Fx Fy \
Sets Fx := cos(Fy).\
Sets overflow bit if result is outside of floating-point range. Sets underflow bit if result is subnormal. Sets FZ bit if result $= plus.minus 0$. Sets FINF bit if result $= plus.minus infinity$. Sets FNAN bit if result is not-a-number.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [1], [1], [0], [0], [1], [0], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Fy], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 13)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x13 (Tangent Floating-Point) \
SIN c Fx Fy \
Sets Fx := tan(Fy).\
Sets overflow bit if result is outside of floating-point range. Sets underflow bit if result is subnormal. Sets FZ bit if result $= plus.minus 0$. Sets FINF bit if result $= plus.minus infinity$. Sets FNAN bit if result is not-a-number.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [1], [1], [0], [0], [1], [1], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Fy], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 13)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x14 (Arcsine Floating-Point) \
ASN c Fx Fy \
Sets Fx := arcsin(Fy).\
Sets overflow bit if result is outside of floating-point range. Sets underflow bit if result is subnormal. Sets FZ bit if result $= plus.minus 0$. Sets FINF bit if result $= plus.minus infinity$. Sets FNAN bit if result is not-a-number.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [1], [1], [0], [1], [0], [0], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Fy], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 13)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x15 (Arccosine Floating-Point) \
ACS c Fx Fy \
Sets Fx := arccos(Fy).\
Sets overflow bit if result is outside of floating-point range. Sets underflow bit if result is subnormal. Sets FZ bit if result $= plus.minus 0$. Sets FINF bit if result $= plus.minus infinity$. Sets FNAN bit if result is not-a-number.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [1], [1], [0], [1], [0], [1], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Fy], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 13)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x16 (Arctangent Floating-Point) \
ATN c Fx Fy \
Sets Fx := arctan(Fy).\
Sets overflow bit if result is outside of floating-point range. Sets underflow bit if result is subnormal. Sets FZ bit if result $= plus.minus 0$. Sets FINF bit if result $= plus.minus infinity$. Sets FNAN bit if result is not-a-number.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [1], [1], [0], [1], [1], [0], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Fy], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 13)[Unused]
)}) \
== Type 110 (Timers) \
#block(breakable: false,{text[=== OPCODE: 0x00 (Set Timer) \
SETT Tx Ry \
Sets the value of the timer (timer) to the value of Rx]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [1], [0], [0], [0], [0], [0], [0], table.cell(colspan: 5)[Tx], table.cell(colspan: 5)[Ry], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Timer], table.cell(colspan: 5)[Source Register], table.cell(colspan: 14)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x01 (Get Current Timer) \
GETT Rx Ty \
Sets Rx to the value of the timer (timer)]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [1], [0], [0], [0], [0], [0], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ty], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Timer], table.cell(colspan: 14)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x02 (Check Timer) \
CHKT Tx \
Sets the EQ flag if the current value of timer (timer) is zero and if it is restart the timer to the previously set value.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [1], [0], [0], [0], [0], [1], [0], table.cell(colspan: 5)[Tx], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Timer], table.cell(colspan: 19)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x03 (Clear Timer) \
CLRT Tx \
Sets the timer (timer) to zero.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [1], [0], [0], [0], [0], [1], [1], table.cell(colspan: 5)[Tx], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Timer], table.cell(colspan: 19)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x04 (Stall Timer) \
STALL Tx \
Stalls the pipeline at execute until the timer is 0.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [1], [0], [0], [0], [1], [0], [0], table.cell(colspan: 5)[Tx], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Timer], table.cell(colspan: 19)[Unused]
)}) \
== Type 111 (RFU) \
This section is intentionally left blank as it is reserved for future use.\
All opcodes of this type will be interpreted as a NOP.
